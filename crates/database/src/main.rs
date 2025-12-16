mod store;
use env_logger::Env;
use grapher::prelude::GraphDisplayData;
use log::info;
use rdf_fusion::{execution::results::QueryResults, store::Store};
use std::path::Path;
use webvowl_database::prelude::GraphDisplayDataSolutionSerializer;
use webvowl_database::store::{DEFAULT_QUERY, WebVOWLStore};

#[tokio::main]
pub async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();
    let session = Store::default();
    let args = env::args().collect::<Vec<String>>();
    let path;
    if args.len() > 1 {
        path = Path::new(&args[1]);
    } else {
        path = Path::new("crates/database/owl1-unions-simple.owl");
    }
    let webvowl = WebVOWLStore::new(session);
    webvowl
        .insert_file(&path, false)
        .await
        .expect("Error inserting file");
    info!("Loaded {} quads", webvowl.session.len().await.unwrap());

    let mut data_buffer = GraphDisplayData::new();
    let mut solution_serializer = GraphDisplayDataSolutionSerializer::new();
    let query_stream = webvowl.session.query(DEFAULT_QUERY).await.unwrap();
    if let QueryResults::Solutions(solutions) = query_stream {
        solution_serializer
            .serialize_nodes_stream(&mut data_buffer, solutions)
            .await
            .unwrap();
    } else {
        panic!("Query stream is not a solutions stream");
    }
    info!("--- GraphDisplayData ---");
    print_graph_display_data(&data_buffer);
    info!("--- SolutionSerializer ---");
    info!("{}", solution_serializer);
}

pub fn print_graph_display_data(data_buffer: &GraphDisplayData) {
    for (index, (element, label)) in data_buffer
        .elements
        .iter()
        .zip(data_buffer.labels.iter())
        .enumerate()
    {
        info!("{index}: {element:?} -> {label}");
    }
    for edge in data_buffer.edges.iter() {
        info!("{} -> {:?} -> {}", data_buffer.labels[edge[0]], data_buffer.elements[edge[1]], data_buffer.labels[edge[2]]);
    }
}
