use std::{
    fs::File,
    io::{BufReader, Cursor, Write},
    path::Path,
};

use futures::{StreamExt, stream::BoxStream};
use horned_owl::{
    io::{rdf::reader::ConcreteRDFOntology, *},
    model::{RcAnnotatedComponent, RcStr},
    ontology::component_mapped::RcComponentMappedOntology,
};
use rdf_fusion::{
    execution::results::QuadStream,
    io::{JsonLdProfileSet, RdfFormat, RdfParser, RdfSerializer},
    model::{GraphName},
};
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::errors::{WebVowlStoreError, WebVowlStoreErrorKind};
use std::io;

#[derive(Debug)]
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
    pub fn from_path(path: &Path) -> Result<Self, WebVowlStoreError> {
        std::fs::read(path)
            .map(ParserInput::File)
            .map_err(WebVowlStoreError::from)
    }

    pub fn as_slice(&self) -> &[u8] {
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
pub fn format_from_resource_type(resource_type: &ResourceType) -> Option<RdfFormat> {
    match resource_type {
        ResourceType::RDF => Some(RdfFormat::RdfXml),
        ResourceType::TTL => Some(RdfFormat::Turtle),
        ResourceType::NTriples => Some(RdfFormat::NTriples),
        ResourceType::NQuads => Some(RdfFormat::NQuads),
        ResourceType::TriG => Some(RdfFormat::TriG),
        ResourceType::JsonLd => Some(RdfFormat::JsonLd {
            profile: JsonLdProfileSet::default(),
        }),
        ResourceType::N3 => Some(RdfFormat::N3),
        ResourceType::OWL => Some(RdfFormat::RdfXml),
        _ => None,
    }
}
pub async fn parse_stream_to(
    mut stream: QuadStream,
    output_type: ResourceType,
) -> Result<BoxStream<'static, Result<Vec<u8>, WebVowlStoreError>>, WebVowlStoreError> {
    match output_type {
        ResourceType::OFN | ResourceType::OWX | ResourceType::OWL => {
            let (tx, rx) = mpsc::unbounded_channel();
            let mut buf = Vec::new();
            let mut serializer =
                RdfSerializer::from_format(format_from_resource_type(&ResourceType::OWL).ok_or(
                    WebVowlStoreErrorKind::InvalidInput(format!(
                        "Unsupported output type: {:?}",
                        output_type
                    )),
                )?)
                .for_writer(&mut buf);
            while let Some(quad) = stream.next().await {
                serializer.serialize_quad(&quad?)?;
            }
            serializer.finish()?;

            let mut reader = BufReader::new(Cursor::new(buf));
            tokio::task::spawn_blocking(move || {
                let mut writer = ChannelWriter { sender: tx.clone() };
                let result = (|| match output_type {
                    ResourceType::OFN => {
                        let (ont, prefix)
                        : (RcComponentMappedOntology, _) = ofn::reader::read(
                                &mut reader, ParserConfiguration::default())?;
                        ofn::writer::write(
                            &mut writer, 
                            &ont, 
                            Some(&prefix))?;
                        writer.flush()?;
                        Ok(writer)
                    }
                    ResourceType::OWX => {
                        let (ont, prefix): (RcComponentMappedOntology, _) =
                            owx::reader::read(&mut reader, ParserConfiguration::default())?;
                        owx::writer::write(
                            &mut writer, 
                            &ont, 
                            Some(&prefix))?;
                        writer.flush()?;
                        Ok(writer)
                    }
                    ResourceType::OWL => {
                        let (ont, _): (ConcreteRDFOntology<RcStr, RcAnnotatedComponent>, _) =
                            rdf::reader::read(&mut reader, ParserConfiguration::default())?;
                        rdf::writer::write(&mut writer, &ont.into())?;
                        writer.flush()?;
                        Ok(writer)
                    }
                    _ => Err(WebVowlStoreError::from(
                        WebVowlStoreErrorKind::InvalidInput(format!(
                            "Unsupported output type: {:?}",
                            output_type
                        )),
                    )),
                })();

                if let Err(e) = result {
                    let _ = tx.send(Err(e.into()));
                }
            });
            Ok(UnboundedReceiverStream::new(rx)
            .map(|result| result.map_err(WebVowlStoreError::from)).boxed())
        }
        _ => {
            let (tx, rx) = mpsc::unbounded_channel();
            tokio::task::spawn(async move {
                let mut writer = ChannelWriter { sender: tx.clone() };
                let result = (|| async {
                    let mut serializer =
                    RdfSerializer::from_format(format_from_resource_type(&output_type).ok_or(
                        WebVowlStoreErrorKind::InvalidInput(format!(
                            "Unsupported output type: {:?}",
                            output_type
                        )),
                    )?)
                    .for_writer(&mut writer);
                    while let Some(quad) = stream.next().await {
                        serializer.serialize_quad(&quad?)?;
                    }
                    serializer.finish()?;
                    Ok::<ChannelWriter, WebVowlStoreError>(writer)
                })();

                if let Err(e) = result.await {
                    let _ = tx.send(Err(e.into()));
                }
            });
            Ok(UnboundedReceiverStream::new(rx)
            .map(|result| result.map_err(WebVowlStoreError::from)).boxed())
        }
    }
}

pub fn parser_from_format(path: &Path, lenient: bool) -> Result<PreparedParser, WebVowlStoreError> {
    let make_parser = |fmt| {
        let path_str = path.to_str().unwrap();
        // TODO: Handle non default graph
        let parser = RdfParser::from_format(fmt).with_default_graph(GraphName::DefaultGraph);
        //.with_default_graph(NamedNode::new(format!("file:://{}", path_str)).unwrap());
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
        Some(ResourceType::OWL) => {
            let b = horned_owl::model::Build::<RcStr>::new();
            let iri = horned_owl::resolve::path_to_file_iri(&b, path);
            let (ontology, _) = rdf::closure_reader::read::<
                RcStr,
                RcAnnotatedComponent,
                ConcreteRDFOntology<RcStr, RcAnnotatedComponent>,
            >(&iri, ParserConfiguration::default())?;

            let mut buf = Vec::new();
            rdf::writer::write(&mut buf, &ontology.into())?;

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
        _ => Err(WebVowlStoreErrorKind::InvalidInput(format!(
            "Unsupported parser: {}",
            path.display()
        ))),
    };
    Ok(prepared?)
}
struct ChannelWriter {
    sender: UnboundedSender<Result<Vec<u8>, io::Error>>,
}

impl Write for ChannelWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let data = buf.to_vec();

        self.sender
            .send(Ok(data))
            .map_err(|_| io::Error::new(io::ErrorKind::BrokenPipe, "Stream receiver dropped"))?;

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(()) // No internal buffering to flush
    }
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
            let _ = session
                .load_from_reader(parser.parser, parser.input.as_slice())
                .await;
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
            let _ = session
                .load_from_reader(parser.parser, parser.input.as_slice())
                .await;
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
            let _ = session
                .load_from_reader(parser.parser, parser.input.as_slice())
                .await;
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
