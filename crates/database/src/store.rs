use rdf_fusion::store::Store;
use std::path::Path;

use webvowl_parser::{errors::WebVowlStoreError, parser_util::parser_from_format};

pub struct WebVOWLStore {
    pub session: Store,
}
impl WebVOWLStore {
    pub fn new(session: Store) -> Self {
        Self { session }
    }

    // TTL format -> (oxittl) RDF XML quads -> (horned_owl) Normalize OWL/RDF -> Quads -> Insert into Oxigraph
    pub async fn insert_file(&self, fs: &Path, lenient: bool) -> Result<(), WebVowlStoreError> {
        let parser = parser_from_format(fs, lenient)?;

        self.session
            .load_from_reader(parser.parser, parser.input.as_slice())
            .await?;

        Ok(())
    }
}
