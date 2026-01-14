pub const DEFAULT_PREFIXES: [&str; 6] = [
    "owl: <http://www.w3.org/2002/07/owl#>",
    "rdfs: <http://www.w3.org/2000/01/rdf-schema#>",
    "rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>",
    "xsd: <http://www.w3.org/2001/XMLSchema#>",
    "vowlr: <http://www.example.com/iri#>",
    "xml: <http://www.w3.org/XML/1998/namespace>",
];

pub struct QueryAssembler;

impl QueryAssembler {
    /// Construct a SPARQL query from URI prefixes and SPARQL snippets.
    ///
    /// `prefixes` is the collection of prefixes to use.
    /// An example of a prefix is: `owl: <http://www.w3.org/2002/07/owl#>`.
    ///
    /// `snippets` is the collection of SPARQL snippets to use, defined in the module [`snippets.rs`].
    pub fn assemble_query(prefixes: Vec<&str>, snippets: Vec<&'static str>) -> String {
        format!(
            r#"
            {}
            SELECT ?id ?nodeType ?target ?label
            WHERE {{
                {}
                BIND(
                    IF(?nodeType = owl:Class, 1, 2)
                    AS ?weight
                    )
            }}
            ORDER BY ?weight
        "#,
            prefixes
                .iter()
                .map(|item| format!("PREFIX {item}"))
                .collect::<Vec<_>>()
                .join("\n"),
            snippets
                .iter()
                .map(|item| item.to_string())
                .filter(|item| item.len() > 0)
                .collect::<Vec<_>>()
                .join(" UNION "),
        )
    }
}
