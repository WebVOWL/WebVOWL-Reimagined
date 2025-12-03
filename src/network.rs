use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    OWL,
    OFN,
    OWX,
    TTL,
    RDF,
    Ntriples,
    NQuads,
    TriG,
    JsonLd,
    N3,
    SPARQLJSON,
    SPARQLXML,
    SPARQLCSV,
    SPARQLTSV,
    /// fallback when type cant be determined
    UNKNOWN,
}

impl DataType {
    /// Map file extensions to datatypes
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "owl" => Self::OWL,
            "ofn" => Self::OFN,
            "owx" => Self::OWX,
            "ttl" => Self::TTL,
            "rdf" => Self::RDF,
            "nt" => Self::Ntriples,
            "nq" => Self::NQuads,
            "trig" => Self::TriG,
            "jsonld" => Self::JsonLd,
            "n3" => Self::N3,
            "srj" | "json" => Self::SPARQLJSON,
            "srx" | "xml" => Self::SPARQLXML,
            "src" | "csv" => Self::SPARQLCSV,
            "srtsv" | "tsv" => Self::SPARQLTSV,
            _ => Self::UNKNOWN,
        }
    }

    // Fixed string literals called by reference as to not allocate new memory each time the function is called
    /// labels the data extension type
    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::OWL => "application/owl+xml",
            Self::OFN => "text/ofn",
            Self::OWX => "application/owl+xml",
            Self::TTL => "text/turtle",
            Self::RDF => "application/rdf+xml",
            Self::Ntriples => "application/n-triples",
            Self::NQuads => "application/n-quads",
            Self::TriG => "application/trig",
            Self::JsonLd => "application/ld+json",
            Self::N3 => "text/n3",
            Self::SPARQLJSON => "application/sparql-results+json",
            Self::SPARQLXML => "application/sparql-results+xml",
            Self::SPARQLCSV => "text/csv",
            Self::SPARQLTSV => "text/tab-separated-values",
            Self::UNKNOWN => "application/octet-stream",
        }
    }
}