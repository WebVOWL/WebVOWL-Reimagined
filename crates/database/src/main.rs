use std::path::Path;

use futures::StreamExt;
use rdf_fusion::store::Store;
use webvowl_database::{serializers::new_ser::NewSerializer, store::WebVOWLStore};
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
    /*
    let mut stream = parse_stream_to(webvowl.session.stream().await.unwrap(), ResourceType::OWL)
        .await
        .unwrap();
    let mut out = vec![];
    while let Some(result) = stream.next().await {
        out.extend(result.unwrap());
    }*/
    let mut serializer = NewSerializer::<String>::default();
    serializer.serialize(webvowl.session).await.unwrap();
    //println!("{}", String::from_utf8_lossy(&out));
    println!("Written to Output.owl");
}
