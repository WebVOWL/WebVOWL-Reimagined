mod store;

use grapher::prelude::GraphDisplayData;
use rdf_fusion::{execution::results::QueryResults, store::Store};
use std::path::Path;
use webvowl_database::{prelude::GraphDisplayDataSolutionSerializer, store::{ WebVOWLStore}};
use webvowl_database::sparql_queries::default::DEFAULT_QUERY;
use std::env;


#[tokio::main]
pub async fn main() {
    //let session = Store::open("oxigraph.db").unwrap();
    let session = Store::default();
    println!("Loaded {} quads", session.len().await.unwrap());
    // let path = Path::new("crates/database/owl1-compatible.owl");
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
        println!("{} -> {:?} -> {}", data_buffer.labels[edge[0]], data_buffer.elements[edge[1]], data_buffer.labels[edge[2]]);
    }
}
