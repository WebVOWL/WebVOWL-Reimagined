use std::path::Path;
use rdf_fusion::store::Store;
use webvowl_database::store::WebVOWLStore;

mod store;

#[tokio::main]
pub async fn main() {
    //let session = Store::open("oxigraph.db").unwrap();
    let session = Store::default();
    println!("Loaded {} quads", session.len().await.unwrap());
    let path = Path::new("data/ONTOAD.owl");
    let webvowl = WebVOWLStore::new(session);
    webvowl
        .insert_file(&path, false)
        .await
        .expect("Error inserting file");
    println!("Loaded {} quads", webvowl.session.len().await.unwrap());
    webvowl
        .serialize_to_file(Path::new("data/Output.owl"))
        .await
        .unwrap();
    println!("Written to Output.owl");
}
