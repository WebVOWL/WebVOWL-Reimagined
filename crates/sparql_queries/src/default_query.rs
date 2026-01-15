use crate::{default, general};
use const_format::{concatcp, formatcp};

pub const DEFAULT_QUERY: &str = formatcp!(
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
    concatcp!(
        default::NAMED_CLASS,
        " UNION ",
        default::EXTERNAL_CLASS,
        " UNION ",
        default::DEPRECATED_CLASS,
        " UNION ",
        default::EQUIVALENT_CLASS,
        " UNION ",
        default::ANONYMOUS_CLASS,
        " UNION ",
        default::THING,
        " UNION ",
        default::RDFS_CLASS,
        " UNION ",
        default::RDFS_RESOURCE,
        " UNION ",
        default::RDFS_LITERAL,
        " UNION ",
        default::RDFS_DATATYPE,
        " UNION ",
        default::INTERSECTION_OF,
        " UNION ",
        default::UNION_OF,
        " UNION ",
        default::COMPLEMENT,
        " UNION ",
        default::DISJOINT_UNION,
        " UNION ",
        default::OBJECTPROPERTY,
        " UNION ",
        default::DATATYPEPROPERTY,
        " UNION ",
        default::SUBCLASSOF,
        " UNION ",
        default::INVERSEOF,
        " UNION ",
        default::DISJOINTWITH,
        " UNION ",
        default::RDFPROPERTY,
        " UNION ",
        default::DEPRECATEDPROPERTY,
        " UNION ",
        default::EXTERNALPROPERTY,
        " UNION ",
        default::VALUESFROM,
        " UNION ",
        general::EXTERNALS,
        " UNION ",
        general::DEPRECATED,
        " UNION ",
        general::LABEL
    )
);
