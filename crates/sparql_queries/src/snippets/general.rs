//! Provides SPARQL query snippets for generic querying across vocabularies.

/// Flatten RDF lists. Currently only supports select OWL types.
pub const LIST_FLATTENING: &str = r#"{
            # BRIDGE: Start at the Named Class, jump to the intermediate node
            ?id ?connector ?intermediateNode .
            FILTER(isIRI(?id)) 

            # Match the logic property (unionOf, etc) on the intermediate node
            ?intermediateNode ?nodeType ?blanknode .

            # Flatten the list from the blanknode
            ?blanknode rdf:rest*/rdf:first ?target .

            # Filter for Logic Types
            FILTER(?nodeType IN (owl:intersectionOf, owl:unionOf, owl:oneOf, owl:disjointUnionOf, owl:disjointWith))
            FILTER(?target != rdf:nil)
            }"#;

/// External classes.
///
/// 1. Elements whose base URI differs from that of the visualized ontology.
///    p. 6 of https://www.semantic-web-journal.net/system/files/swj1114.pdf
/// 2. A base URI is EITHER `xml:base` OR that of the document.
///    https://www.w3.org/TR/rdf-syntax-grammar/#section-Syntax-ID-xml-base
pub const XML_BASE: &str = r#"{
            ?id xml:base ?base .
            BIND(xml:base AS ?nodeType)
            }"#;

/// Generic, deprecated OWL elements.
///
/// This query is still work-in-progress.
/// We need to figure out what type the deprecated element is.
/// It could be a class or a property!
pub const OWL_DEPRECATED: &str = r#"{
            # WIP: Identify OWL deprecated properties
            # ?id owl:deprecated "true"^^<http://www.w3.org/2001/XMLSchema#boolean>
            # BIND("DeprecatedProperty" AS ?nodeType)
        
            # DEPRECATED
            ?id owl:deprecated ?target .
            BIND(owl:deprecated AS ?nodeType)
            }"#;

/// Find labels for elements in the following order:
/// 1. Use rdfs:label, if exists.
///    https://www.w3.org/TR/rdf-schema/#ch_label
/// 2. Use rdf:resource, if exists.
///    https://www.w3.org/TR/rdf-syntax-grammar/#section-Syntax-empty-property-elements
/// 3. Use rdf:ID, if exists.
///    https://www.w3.org/TR/rdf-syntax-grammar/#section-Syntax-ID-xml-base
pub const LABEL: &str = r#"{
                OPTIONAL { ?id rdfs:label ?theLabel }
                OPTIONAL { ?id rdf:resource ?resLabel }
                OPTIONAL { ?id rdf:ID ?idLabel }
                BIND (
                    COALESCE(
                        IF( BOUND(?theLabel), ?theLabel, 1/0 ),
                        IF( BOUND(?resLabel), ?resLabel, 1/0 ),
                        IF( BOUND(?idLabel), ?idLabel, 1/0 ),
                        ""
                    ) AS ?label
                )
            }"#;
