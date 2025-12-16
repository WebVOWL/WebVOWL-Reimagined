use crate::{default, general};
use grapher::prelude::{
    Characteristic, GenericEdge, GenericNode, OwlEdge, OwlNode, RdfEdge, RdfsEdge, RdfsNode,
};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FilterNode {
    Owl(OwlNode),
    Rdfs(RdfsNode),
    Generic(GenericNode),
}

impl std::fmt::Display for FilterNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterNode::Owl(n) => n.fmt(f),
            FilterNode::Rdfs(n) => n.fmt(f),
            FilterNode::Generic(n) => n.fmt(f),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FilterEdge {
    Owl(OwlEdge),
    Rdf(RdfEdge),
    Rdfs(RdfsEdge),
    Generic(GenericEdge),
}

impl std::fmt::Display for FilterEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterEdge::Owl(e) => e.fmt(f),
            FilterEdge::Rdf(e) => e.fmt(f),
            FilterEdge::Rdfs(e) => e.fmt(f),
            FilterEdge::Generic(e) => e.fmt(f),
        }
    }
}

fn get_node_pattern(node: &FilterNode) -> Option<String> {
    match node {
        FilterNode::Owl(OwlNode::Class) => Some(default::NAMED_CLASS.to_string()),
        FilterNode::Owl(OwlNode::ExternalClass) => Some(default::EXTERNAL_CLASS.to_string()),
        FilterNode::Owl(OwlNode::EquivalentClass) => Some(default::EQUIVALENT_CLASS.to_string()),
        FilterNode::Owl(OwlNode::DeprecatedClass) => Some(default::DEPRECATED_CLASS.to_string()),
        FilterNode::Owl(OwlNode::AnonymousClass) => Some(default::ANONYMOUS_CLASS.to_string()),
        FilterNode::Owl(OwlNode::Thing) => Some(default::THING.to_string()),
        FilterNode::Owl(OwlNode::UnionOf) => Some(default::UNION_OF.to_string()),
        FilterNode::Owl(OwlNode::IntersectionOf) => Some(default::INTERSECTION_OF.to_string()),
        FilterNode::Owl(OwlNode::Complement) => Some(default::COMPLEMENT.to_string()),
        FilterNode::Owl(OwlNode::DisjointUnion) => Some(default::DISJOINT_UNION.to_string()),
        FilterNode::Rdfs(RdfsNode::Class) => Some(default::RDFS_CLASS.to_string()),
        FilterNode::Rdfs(RdfsNode::Resource) => Some(default::RDFS_RESOURCE.to_string()),
        FilterNode::Rdfs(RdfsNode::Literal) => Some(default::RDFS_LITERAL.to_string()),
        _ => None,
    }
}

fn get_edge_pattern(edge: &FilterEdge) -> Option<String> {
    match edge {
        FilterEdge::Owl(OwlEdge::ObjectProperty) => Some(default::OBJECTPROPERTY.to_string()),
        FilterEdge::Owl(OwlEdge::DatatypeProperty) => Some(default::DATATYPEPROPERTY.to_string()),
        FilterEdge::Owl(OwlEdge::InverseOf) => Some(default::INVERSEOF.to_string()),
        FilterEdge::Owl(OwlEdge::DisjointWith) => Some(default::DISJOINTWITH.to_string()),
        FilterEdge::Owl(OwlEdge::DeprecatedProperty) => {
            Some(default::DEPRECATEDPROPERTY.to_string())
        }
        FilterEdge::Owl(OwlEdge::ExternalProperty) => Some(default::EXTERNALPROPERTY.to_string()),
        FilterEdge::Owl(OwlEdge::ValuesFrom) => Some(default::VALUESFROM.to_string()),
        FilterEdge::Rdf(RdfEdge::RdfProperty) => Some(default::RDFPROPERTY.to_string()),
        FilterEdge::Rdfs(RdfsEdge::SubclassOf) => Some(default::SUBCLASSOF.to_string()),
        _ => None,
    }
}

// TODO: Define actual patterns for characteristics.
// fn get_characteristic_pattern(characteristic: &Characteristic) -> Option<String> {
//     match characteristic {
//         Characteristic::Transitive => Some("{ ?p rdf:type owl:TransitiveProperty }".to_string()),
//         Characteristic::FunctionalProperty => {
//             Some("{ ?p rdf:type owl:FunctionalProperty }".to_string())
//         }
//         Characteristic::InverseFunctionalProperty => {
//             Some("{ ?p rdf:type owl:InverseFunctionalProperty }".to_string())
//         }
//         Characteristic::Symmetric => Some("{ ?p rdf:type owl:SymmetricProperty }".to_string()),
//         Characteristic::Asymmetric => Some("{ ?p rdf:type owl:AsymmetricProperty }".to_string()),
//         Characteristic::Reflexive => Some("{ ?p rdf:type owl:ReflexiveProperty }".to_string()),
//         Characteristic::Irreflexive => Some("{ ?p rdf:type owl:IrreflexiveProperty }".to_string()),
//         _ => None,
//     }
// }

pub fn generate_sparql_query(
    node_checks: &HashMap<FilterNode, bool>,
    edge_checks: &HashMap<FilterEdge, bool>,
    char_checks: &HashMap<Characteristic, bool>,
) -> String {
    let mut patterns: Vec<String> = Vec::new();

    for (node, &checked) in node_checks.iter() {
        if checked {
            if let Some(pattern) = get_node_pattern(node) {
                patterns.push(pattern);
            }
        }
    }

    for (edge, &checked) in edge_checks.iter() {
        if checked {
            if let Some(pattern) = get_edge_pattern(edge) {
                patterns.push(pattern);
            }
        }
    }

    // for (char, &checked) in char_checks.iter() {
    //     if checked {
    //         if let Some(pattern) = get_characteristic_pattern(char) {
    //             patterns.push(pattern);
    //         }
    //     }
    // }

    let union_clause = if patterns.is_empty() {
        r#"
            BIND(<http://example.org/nothing> AS ?id)
            BIND(<http://example.org/nothing> AS ?nodeType)
            BIND(<http://example.org/nothing> AS ?target)
            BIND("" AS ?label)
            FILTER(false)
        "#
        .to_string()
    } else {
        patterns.join(" UNION ")
    };

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
            UNION
            {{
                {}
            }}
            UNION
            {{
                {}
            }}
            UNION
            {{
                {}
            }}

            BIND(
                IF(?nodeType = owl:Class, 1, 2)
                AS ?weight)
        }}
        ORDER BY ?weight
        "#,
        union_clause,
        general::EXTERNALS,
        general::DEPRECATED,
        general::LABEL
    )
    .to_string()
}
