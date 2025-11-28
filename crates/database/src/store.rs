use rdf_fusion::store::Store;
use std::fs::File;
use std::path::Path;

use webvowl_parser::{
    errors::WebVowlStoreError,
    errors::WebVowlStoreErrorKind,
    parser_util::{ResourceType, parse_stream_to, parser_from_format},
};

static GLOBAL_STORE: std::sync::OnceLock<Store> = std::sync::OnceLock::new();

pub struct WebVOWLStore {
    pub session: Store,
    upload_handle: Option<tempfile::NamedTempFile>,
}
impl WebVOWLStore {
    pub fn new(session: Store) -> Self {
        Self {
            session,
            upload_handle: None,
        }
    }

    pub fn default() -> Self {
        let session = GLOBAL_STORE.get_or_init(Store::default).clone();
        Self {
            session,
            upload_handle: None,
        }
    }

    // TTL format -> (oxittl) RDF XML quads -> (horned_owl) Normalize OWL/RDF -> Quads -> Insert into Oxigraph
    pub async fn insert_file(&self, fs: &Path, lenient: bool) -> Result<(), WebVowlStoreError> {
        let parser = parser_from_format(fs, lenient)?;

        self.session
            .load_from_reader(parser.parser, parser.input.as_slice())
            .await?;

        Ok(())
    }
    pub async fn serialize_to_file(&self, path: &Path) -> Result<(), WebVowlStoreError> {
        let mut file = File::create(path)?;
        let results = parse_stream_to(self.session.stream().await?, ResourceType::OWL).await?;
        std::io::Write::write_all(&mut file, &results)?;
        Ok(())
    }

    pub async fn serialize_to_string(&self) -> Result<String, WebVowlStoreError> {
        println!(
            "Store size before export: {}",
            self.session.len().await.unwrap_or(0)
        );
        let results = parse_stream_to(self.session.stream().await?, ResourceType::OWL).await?;
        String::from_utf8(results)
            .map_err(|e| WebVowlStoreErrorKind::InvalidInput(e.to_string()).into())
    }
    pub async fn start_upload(&mut self, filename: &str) -> Result<(), WebVowlStoreError> {
        let extension = Path::new(filename)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("owl");
        let file = tempfile::Builder::new()
            .suffix(&format!(".{}", extension))
            .tempfile()?;
        self.upload_handle = Some(file);
        Ok(())
    }

    pub async fn upload_chunk(&mut self, data: &[u8]) -> Result<(), WebVowlStoreError> {
        if let Some(file) = &mut self.upload_handle {
            std::io::Write::write_all(file, data)?;
            Ok(())
        } else {
            println!("Warning: upload_chunk called without start_upload");
            Ok(())
        }
    }

    pub async fn complete_upload(&mut self) -> Result<(), WebVowlStoreError> {
        if let Some(file) = &mut self.upload_handle {
            std::io::Write::flush(file)?;
            let path = file.path();
            let parser = parser_from_format(path, false)?;
            self.session
                .load_from_reader(parser.parser, parser.input.as_slice())
                .await?;
            println!("Loaded ontology");
        }
        self.upload_handle = None;
        Ok(())
    }
}
