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
        FilterNode::Owl(OwlNode::Class) => Some(
            r#"{
            ?id a owl:Class .
            FILTER(isIRI(?id))
            BIND(owl:Class AS ?nodeType)
            OPTIONAL { ?id rdfs:label ?label }
            }"#
            .to_string(),
        ),
        // FilterNode::Owl(OwlNode::ExternalClass) => Some(
        //     "{ ?externalClass rdf:type owl:Class . ?externalClass rdfs:isDefinedBy ?definedBy . OPTIONAL { ?externalClass rdfs:label ?label } }"
        //         .to_string(),
        // ),
        FilterNode::Owl(OwlNode::EquivalentClass) => Some(
            r#"{
            ?id owl:equivalentClass ?label
            BIND("EquivalentClass" AS ?nodeType)
            }"#
            .to_string(),
        ),
        // FilterNode::Owl(OwlNode::DeprecatedClass) => Some(
        //     "{ ?deprecatedClass rdf:type owl:Class . ?deprecatedClass owl:deprecated true . OPTIONAL { ?deprecatedClass rdfs:label ?label } }"
        //         .to_string(),
        // ),
        FilterNode::Owl(OwlNode::AnonymousClass) => Some(
            r#"{
            ?id a owl:Class
            FILTER(!isIRI(?id))
            BIND("AnonymousClass" AS ?nodeType)
            OPTIONAL { ?id rdfs:label ?label }
            }"#
            .to_string(),
        ),
        // FilterNode::Owl(OwlNode::Thing) => Some("{ VALUES ?thing { <http://www.w3.org/2002/07/owl#Thing> } }".to_string()),
        // FilterNode::Rdfs(RdfsNode::Class) => Some(
        //     "{ ?rdfsClass rdf:type rdfs:Class . OPTIONAL { ?rdfsClass rdfs:label ?label } }"
        //         .to_string(),
        // ),
        // FilterNode::Rdfs(RdfsNode::Resource) => Some(
        //     "{ ?rdfsResource rdf:type rdfs:Resource . OPTIONAL { ?rdfsResource rdfs:label ?label } }"
        //         .to_string(),
        // ),
        // FilterNode::Rdfs(RdfsNode::Literal) => Some(
        //     "{ ?literal rdf:type rdfs:Datatype . OPTIONAL { ?literal rdfs:label ?label } }"
        //         .to_string(),
        // ),
        FilterNode::Owl(OwlNode::UnionOf) => Some(
            r#"{
            ?id owl:unionOf ?label .
            BIND("UnionOf" AS ?nodeType)
            }"#
            .to_string(),
        ),
        FilterNode::Owl(OwlNode::IntersectionOf) => Some(
            r#"{
            ?id owl:intersectionOf ?label .
            BIND("IntersectionOf" AS ?nodeType)
            }"#
            .to_string(),
        ),
        // FilterNode::Owl(OwlNode::Complement) => Some(
        //     "{ ?complementOf rdf:type owl:Class . FILTER(EXISTS { ?complementOf owl:complementOf ?v }) . OPTIONAL { ?complementOf rdfs:label ?label } . OPTIONAL { ?owner owl:equivalentClass ?complementOf . ?owner rdfs:label ?ownerLabel } }"
        //         .to_string(),
        // ),
        // FilterNode::Owl(OwlNode::DisjointUnion) => Some(
        //     "{ ?disjointUnionOf rdf:type owl:Class . FILTER(EXISTS { ?disjointUnionOf owl:disjointUnionOf ?v }) . OPTIONAL { ?disjointUnionOf rdfs:label ?label } . OPTIONAL { ?owner owl:equivalentClass ?disjointUnionOf . ?owner rdfs:label ?ownerLabel } }"
        //         .to_string(),
        // ),
        _ => None,
    }
}

fn get_edge_pattern(edge: &FilterEdge) -> Option<String> {
    match edge {
        //  FilterEdge::Owl(OwlEdge::ObjectProperty) => Some(
        //     "{ ?objectProperty rdf:type owl:ObjectProperty . OPTIONAL { ?objectProperty rdfs:label ?label } }"
        //         .to_string(),
        // ),
        // FilterEdge::Owl(OwlEdge::DatatypeProperty) => Some(
        //     "{ ?datatypeProperty rdf:type owl:DatatypeProperty . OPTIONAL { ?datatypeProperty rdfs:label ?label } }"
        //         .to_string(),
        // ),
        // FilterEdge::Rdfs(RdfsEdge::SubclassOf) => Some(
        //     "{ ?subClassOf rdf:type owl:Class . FILTER(EXISTS { ?subClassOf rdfs:subClassOf ?v }) }"
        //         .to_string(),
        // ),
        // FilterEdge::Owl(OwlEdge::InverseOf) => Some(
        //     "{ ?inverseOf rdf:type owl:ObjectProperty . FILTER(EXISTS { ?inverseOf owl:inverseOf ?v }) }"
        //         .to_string(),
        // ),
        // FilterEdge::Owl(OwlEdge::DisjointWith) => Some(
        //     "{ ?disjointWith rdf:type owl:Class . FILTER(EXISTS { ?disjointWith owl:disjointWith ?v }) }"
        //         .to_string(),
        // ),
        // FilterEdge::Rdf(RdfEdge::RdfProperty) => Some(
        //     "{ ?rdfProperty rdf:type rdf:Property . OPTIONAL { ?rdfProperty rdfs:label ?label } }"
        //         .to_string(),
        // ),
        // FilterEdge::Owl(OwlEdge::DeprecatedProperty) => Some(
        //     "{ ?deprecatedProperty rdf:type owl:DeprecatedProperty . OPTIONAL { ?deprecatedProperty rdfs:comment ?comment } }"
        //         .to_string(),
        // ),
        // FilterEdge::Owl(OwlEdge::ExternalProperty) => Some(
        //     "{ ?externalProperty rdf:type owl:Property . ?externalProperty rdfs:isDefinedBy ?definedBy . OPTIONAL { ?externalProperty rdfs:label ?label } }"
        //         .to_string(),
        // ),
        // FilterEdge::Owl(OwlEdge::ValuesFrom) => Some(
        //     "{ ?valuesFrom rdf:type owl:Restriction . FILTER (EXISTS { ?valuesFrom owl:someValuesFrom ?v }) . ?valuesFrom owl:someValuesFrom ?someValuesFrom }"
        //         .to_string(),
        // ),
        _ => None,
    }
}

// TODO: Define actual patterns for characteristics.
fn get_characteristic_pattern(characteristic: &Characteristic) -> Option<String> {
    match characteristic {
        // Characteristic::Transitive => Some("{ ?p rdf:type owl:TransitiveProperty }".to_string()),
        // Characteristic::FunctionalProperty => {
        //     Some("{ ?p rdf:type owl:FunctionalProperty }".to_string())
        // }
        // Characteristic::InverseFunctionalProperty => {
        //     Some("{ ?p rdf:type owl:InverseFunctionalProperty }".to_string())
        // }
        // Characteristic::Symmetric => Some("{ ?p rdf:type owl:SymmetricProperty }".to_string()),
        // Characteristic::Asymmetric => Some("{ ?p rdf:type owl:AsymmetricProperty }".to_string()),
        // Characteristic::Reflexive => Some("{ ?p rdf:type owl:ReflexiveProperty }".to_string()),
        // Characteristic::Irreflexive => Some("{ ?p rdf:type owl:IrreflexiveProperty }".to_string()),
        _ => None,
    }
}

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

    for (char, &checked) in char_checks.iter() {
        if checked {
            if let Some(pattern) = get_characteristic_pattern(char) {
                patterns.push(pattern);
            }
        }
    }

    let union_clause = if patterns.is_empty() {
        r#"
            BIND(<http://example.org/nothing> AS ?id)
            BIND(<http://example.org/nothing> AS ?nodeType)
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

        SELECT ?id ?nodeType ?label
        {{
            {}
        }}
        ORDER BY ?nodeType
        "#,
        union_clause
    )
    .to_string()
}
