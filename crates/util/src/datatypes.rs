use rkyv::{Archive, Deserialize as RDeserialize, Serialize as RSerialize};
use serde::{Deserialize, Serialize};

/// Supported content types.
#[repr(C)]
#[derive(Archive, RDeserialize, RSerialize, Deserialize, Serialize, Debug, Clone)]
pub enum DataType {
    OWL,
    OFN,
    OWX,
    TTL,
    RDF,
    NTriples,
    NQuads,
    TriG,
    JsonLd,
    N3,
    SPARQLJSON,
    SPARQLXML,
    SPARQLCSV,
    SPARQLTSV,
    /// Fallback when type can't be determined.
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
            "nt" => Self::NTriples,
            "nq" => Self::NQuads,
            "trig" => Self::TriG,
            "jsonld" => Self::JsonLd,
            "n3" => Self::N3,
            "srj" | "json" => Self::SPARQLJSON,
            "srx" | "xml" => Self::SPARQLXML,
            "src" | "csv" => Self::SPARQLCSV,
            "tsv" => Self::SPARQLTSV, //TODO: Figure out file extension for TSV and if the file extension of TSV SPARQL Query Result differs.
            _ => Self::UNKNOWN,
        }
    }

    // Fixed string literals called by reference as to not allocate new memory each time the function is called
    /// Get mime type of the data.
    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::OWL => "application/owl+xml",
            Self::OFN => "text/ofn",
            Self::OWX => "application/owl+xml",
            Self::TTL => "text/turtle",
            Self::RDF => "application/rdf+xml",
            Self::NTriples => "application/n-triples",
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
