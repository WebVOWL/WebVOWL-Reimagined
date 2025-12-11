mod store;

use grapher::prelude::GraphDisplayData;
use rdf_fusion::{execution::results::QueryResults, store::Store};
use std::path::Path;
use webvowl_database::prelude::serializers::frontend::GraphDisplayDataSolutionSerializer;
use webvowl_database::store::{DEFAULT_QUERY, WebVOWLStore};

#[tokio::main]
pub async fn main() {
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
    println!("{}", solution_serializer);
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
    for edge in data_buffer.edges.iter() {
        println!("{edge:?}");
    }
}
