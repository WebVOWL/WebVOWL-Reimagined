use std::path::Path;

use rdf_fusion::store::Store;
use webvowl_database::store::WebVOWLStore;
use webvowl_parser::parser_util::{ResourceType, parse_stream_to};

mod store;


#[tokio::main]
pub async fn main() {
    //let session = Store::open("oxigraph.db").unwrap();
    let session = Store::default();
    println!("Loaded {} quads", session.len().await.unwrap());
    let path = Path::new("crates/database/owl1-compatible.owl");
    let webvowl = WebVOWLStore::new(session);
    webvowl
        .insert_file(&path, false)
        .await
        .expect("Error inserting file");
    println!("Loaded {} quads", webvowl.session.len().await.unwrap());
    let results = parse_stream_to(webvowl.session.stream().await.unwrap(), ResourceType::OWL).await.unwrap();
    println!("{}", String::from_utf8(results).unwrap());
}