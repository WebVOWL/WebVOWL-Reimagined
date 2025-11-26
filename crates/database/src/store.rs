use horned_owl::{
    io::{rdf::reader::ConcreteRDFOntology, *},
    model::{ForIRI, RcAnnotatedComponent, RcStr},
    ontology::component_mapped::RcComponentMappedOntology,
};
use rdf_fusion:: {
    store::Store,
    io::{RdfFormat, RdfParser, JsonLdProfileSet},
    model::NamedNode,
};
use std::{
    fs::File,
    io::{BufReader, Cursor},
    marker::PhantomData,
    path::Path,
};

use crate::errors::{
    errors::{WebVowlStoreError, WebVowlStoreErrorKind}
};

pub struct WebVOWLStore<A> {
    pub session: Store,
    phantom: PhantomData<A>,
}
impl<A: ForIRI> WebVOWLStore<A> {
    pub fn new(session: Store) -> Self {
        Self {
            session,
            phantom: PhantomData,
        }
    }

    // TTL format -> (oxittl) RDF XML quads -> (horned_owl) Normalize OWL/RDF -> Quads -> Insert into Oxigraph
    pub async fn insert_file(&self, fs: &Path, lenient: bool) -> Result<(), WebVowlStoreError> {
        let parser = parser_from_format(fs, lenient)?;
        
        self.session.load_from_reader(parser.parser, parser.input.as_slice()).await?;

        Ok(())
    }
}
pub enum ResourceType {
    OFN,
    OWX,
    RDF,
    OWL,
    TTL,
    NTriples,
    NQuads,
    TriG,
    JsonLd,
    N3,
}
pub enum ParserInput {
    File(Vec<u8>),
    Buffer(Cursor<Vec<u8>>),
}
impl ParserInput {
    fn from_path(path: &Path) -> Result<Self, WebVowlStoreError> {
        std::fs::read(path)
            .map(ParserInput::File)
            .map_err(WebVowlStoreError::from)
    }

    fn as_slice(&self) -> &[u8] {
        match self {
            ParserInput::Buffer(cursor) => cursor.get_ref().as_slice(),
            ParserInput::File(bytes) => bytes.as_slice(),
        }
    }
}

pub struct PreparedParser {
    pub parser: RdfParser,
    pub input: ParserInput,
}

pub fn path_type(path: &Path) -> Option<ResourceType> {
    match path.extension().and_then(|s| s.to_str()) {
        Some("ofn") => Some(ResourceType::OFN),
        Some("owx") => Some(ResourceType::OWX),
        Some("rdf") => Some(ResourceType::RDF),
        Some("owl") => Some(ResourceType::OWL),
        Some("ttl") => Some(ResourceType::TTL),
        Some("nt") => Some(ResourceType::NTriples),
        Some("nq") => Some(ResourceType::NQuads),
        Some("trig") => Some(ResourceType::TriG),
        Some("jsonld") => Some(ResourceType::JsonLd),
        Some("n3") => Some(ResourceType::N3),
        _ => None,
    }
}
pub fn parser_from_format(path: &Path, lenient: bool) -> Result<PreparedParser, WebVowlStoreError> {
    let make_parser = |fmt| {
        let path_str = path.to_str().unwrap();
        // TODO: Handle non default graph
        let parser = RdfParser::from_format(fmt)
            .with_default_graph(NamedNode::new(format!("file:://{}", path_str)).unwrap());
        if lenient { parser.lenient() } else { parser }
    };
    let t_pat = path_type(path);
    let prepared = match t_pat {
        Some(ResourceType::OFN) => {
            let file = File::open(path)?;
            let mut reader = BufReader::new(file);
            let (ont, _): (RcComponentMappedOntology, _) =
                ofn::reader::read(&mut reader, ParserConfiguration::default())?;

            let mut buf = Vec::new();
            rdf::writer::write(&mut buf, &ont)?;
            Ok(PreparedParser {
                parser: make_parser(RdfFormat::RdfXml),
                input: ParserInput::Buffer(Cursor::new(buf)),
            })
        }
        Some(ResourceType::OWX) => {
            let file = File::open(path)?;
            let mut reader = BufReader::new(file);
            let ontology = owx::reader::read::<
                RcStr,
                ConcreteRDFOntology<RcStr, RcAnnotatedComponent>,
                _,
            >(&mut reader, ParserConfiguration::default())?;

            let mut buf = Vec::new();
            rdf::writer::write(&mut buf, &ontology.0.into())?;

            Ok(PreparedParser {
                parser: make_parser(RdfFormat::RdfXml),
                input: ParserInput::Buffer(Cursor::new(buf)),
            })
        }
        Some(ResourceType::TTL) => {
            let input = ParserInput::from_path(path)?;
            Ok(PreparedParser {
                parser: make_parser(RdfFormat::Turtle),
                input,
            })
        }
        Some(ResourceType::NTriples) => {
            let input = ParserInput::from_path(path)?;
            Ok(PreparedParser {
                parser: make_parser(RdfFormat::NTriples),
                input,
            })
        }
        Some(ResourceType::NQuads) => {
            let input = ParserInput::from_path(path)?;
            Ok(PreparedParser {
                parser: make_parser(RdfFormat::NQuads),
                input,
            })
        }
        Some(ResourceType::TriG) => {
            let input = ParserInput::from_path(path)?;
            Ok(PreparedParser {
                parser: make_parser(RdfFormat::TriG),
                input,
            })
        }
        Some(ResourceType::JsonLd) => {
            let input = ParserInput::from_path(path)?;
            Ok(PreparedParser {
                parser: make_parser(RdfFormat::JsonLd {
                    profile: JsonLdProfileSet::default(),
                }),
                input,
            })
        }
        Some(ResourceType::N3) => {
            let input = ParserInput::from_path(path)?;
            Ok(PreparedParser {
                parser: make_parser(RdfFormat::N3),
                input,
            })
        }
        Some(ResourceType::OWL) => {
            let input = ParserInput::from_path(path)?;
            Ok(PreparedParser {
                parser: make_parser(RdfFormat::RdfXml),
                input,
            })
        }
        _ => Err(HornedOxiErrorKind::InvalidInput(format!(
            "Unsupported parser: {}",
            path.display()
        ))),
    };
    Ok(prepared?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_ofn_parser() {
        let resources = resources_with_suffix("data/owl-functional", "ofn");
        use rdf_fusion::store::Store;
        let session = Store::default();
        for resource in resources {
            let parser = parser_from_format(Path::new(&resource), false).unwrap();
            let _ = session.load_from_reader(parser.parser, parser.input.as_slice()).await;
            assert_ne!(
                session.len().await.unwrap(),
                0,
                "Expected non-zero quads for: {}",
                resource
            );
        }
        
    }

    #[tokio::test]
    async fn test_owl_parser() {
        use rdf_fusion::store::Store;
        let session = Store::default();
        let resources = resources_with_suffix("data/owl-rdf", "owl");
        for resource in resources {
            let parser = parser_from_format(Path::new(&resource), false).unwrap();
            let _ = session.load_from_reader(parser.parser, parser.input.as_slice()).await;
            assert_ne!(
                session.len().await.unwrap(),
                0,
                "Expected non-zero quads for: {}",
                resource
            );
        }
    }
    #[tokio::test]
    async fn test_ttl_parser() {
        use rdf_fusion::store::Store;
        let session = Store::default();
        let resources = resources_with_suffix("data/owl-ttl", "ttl");
        for resource in resources {
            let parser = parser_from_format(Path::new(&resource), false).unwrap();
            let _ = session.load_from_reader(parser.parser, parser.input.as_slice()).await;
            assert_ne!(
                session.len().await.unwrap(),
                0,
                "Expected non-zero quads for: {}",
                resource
            );
        }
    }
    fn resources_with_suffix(relative_dir: &str, suffix: &str) -> Vec<String> {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let base_dir = Path::new(manifest_dir).join(relative_dir);
        let suffix = suffix.trim_start_matches(|c| c == '.' || c == '*');
        let mut resources = Vec::new();

        let entries = std::fs::read_dir(&base_dir).unwrap_or_else(|err| {
            panic!(
                "Failed to read resources directory {}: {}",
                base_dir.display(),
                err
            )
        });

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file()
                && path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| ext == suffix)
                    .unwrap_or(false)
            {
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    let dir = relative_dir.trim_end_matches('/');
                    resources.push(format!("{}/{}", dir, file_name));
                }
            }
        }

        resources.sort();
        resources
    }
}