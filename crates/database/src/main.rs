mod store;

use env_logger::Env;
use grapher::prelude::GraphDisplayData;
use rdf_fusion::{execution::results::QueryResults, store::Store};
use std::path::Path;
use webvowl_database::serializers::frontend::GraphDisplayDataSolutionSerializer;
use webvowl_database::store::{DEFAULT_QUERY, WebVOWLStore};

#[tokio::main]
pub async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();
    //let session = Store::open("oxigraph.db").unwrap();
    let session = Store::default();
    println!("Loaded {} quads", session.len().await.unwrap());
    // let path = Path::new("crates/database/owl1-compatible.owl");
    let path = Path::new("crates/database/owl1-compatible.owl");
    let webvowl = WebVOWLStore::new(session);
    webvowl
        .insert_file(&path, false)
        .await
        .expect("Error inserting file");
    println!("Loaded {} quads", webvowl.session.len().await.unwrap());
    /*
    let mut stream = parse_stream_to(webvowl.session.stream().await.unwrap(), ResourceType::OWL)
        .await
        .unwrap();
    let mut out = vec![];
    while let Some(result) = stream.next().await {
        out.extend(result.unwrap());
    }*/
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
    print_graph_display_data(&data_buffer);
    //let mut serializer = NewSerializer::<String>::default();
    //serializer.serialize(webvowl.session).await.unwrap();
    //println!("{}", String::from_utf8_lossy(&out));
    println!("Written to Output.owl");
}

pub fn print_graph_display_data(data_buffer: &GraphDisplayData) {
    for (index, (element, label)) in data_buffer
        .elements
        .iter()
        .zip(data_buffer.labels.iter())
        .enumerate()
    {
        println!("{index}: {element:?} -> {label}");
    }
}
