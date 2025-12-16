use crate::{default, general};

pub fn get_default_query() -> String {
    let patterns = vec![
        default::NAMED_CLASS,
        default::EXTERNAL_CLASS,
        default::DEPRECATED_CLASS,
        default::EQUIVALENT_CLASS,
        default::ANONYMOUS_CLASS,
        default::THING,
        default::RDFS_CLASS,
        default::RDFS_RESOURCE,
        default::RDFS_LITERAL,
        default::RDFS_DATATYPE,
        default::INTERSECTION_OF,
        default::UNION_OF,
        default::COMPLEMENT,
        default::DISJOINT_UNION,
        default::OBJECTPROPERTY,
        default::DATATYPEPROPERTY,
        default::SUBCLASSOF,
        default::INVERSEOF,
        default::DISJOINTWITH,
        default::RDFPROPERTY,
        default::DEPRECATEDPROPERTY,
        default::EXTERNALPROPERTY,
        default::VALUESFROM,
        general::EXTERNALS,
        general::DEPRECATED,
        general::LABEL,
    ];

    let union_clause = patterns.join(" UNION ");

    format!(
        r#"
        PREFIX owl: <http://www.w3.org/2002/07/owl#>
        PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
        PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
        PREFIX webvowl: <http://www.example.com/iri#>
        PREFIX xml: <http://www.w3.org/XML/1998/namespace>

        SELECT *
        WHERE {{
            {{
                {}
            }}
            BIND(
                IF(?nodeType = owl:Class, 1, 2)
                AS ?weight)
        }}
        ORDER BY ?weight
        "#,
        union_clause
    )
    .to_string()
}
