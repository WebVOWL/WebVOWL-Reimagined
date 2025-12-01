use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    OWL,
    TTL,
    RDF,
    SPARQLJSON,
    SPARQLXML,
    /// fallback when type cant be determined
    UNKNOWN,
}

impl DataType {
    /// Map file extensions to datatypes
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "owl" => Self::OWL,
            "ttl" => Self::TTL,
            "rdf" => Self::RDF,
            "sparql" => Self::SPARQLJSON,
            _ => Self::UNKNOWN,
        }
    }

    // Fixed string literals called by reference as to not allocate new memory each time the function is called
    /// labels the data extension type
    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::OWL => "application/owl+xml",
            Self::TTL => "text/turtle",
            Self::RDF => "application/rdf+xml",
            Self::SPARQLJSON => "application/sparql-results+json",
            Self::SPARQLXML => "application/sparql-results+xml",
            Self::UNKNOWN => "application/octet-stream",
        }
    }
}