use std::collections::HashMap;

use crate::vocab::owl;
use futures::StreamExt;
use grapher::web::prelude::{
    Characteristic, ElementType, GenericNode, GenericType, GraphDisplayData, OwlEdge, OwlNode,
    OwlType, RdfEdge, RdfType, RdfsEdge, RdfsNode, RdfsType,
};
use log::{info, warn};
use rdf_fusion::{
    execution::results::QuerySolutionStream,
    model::{
        BlankNodeRef, Term, TermRef, VariableRef,
        vocab::{rdf, rdfs, xsd},
    },
};
use smallvec::SmallVec;
use webvowl_parser::errors::WebVowlStoreError;

pub struct GraphDisplayDataSolutionSerializer {
    blanknode_mapping: HashMap<String, usize>,
    iricache: HashMap<String, usize>,
}
#[derive(Debug)]
pub struct NodeTriple<'a> {
    id: TermRef<'a>,
    node_type: TermRef<'a>,
    target: Option<TermRef<'a>>,
}

pub struct EdgeTriple<'a> {
    source: TermRef<'a>,
    edge_type: TermRef<'a>,
    target: TermRef<'a>,
}

impl<'a> GraphDisplayDataSolutionSerializer {
    pub fn new() -> Self {
        Self {
            blanknode_mapping: HashMap::new(),
            iricache: HashMap::new(),
        }
    }

    pub async fn serialize_nodes_stream(
        &mut self,
        data_buffer: &mut GraphDisplayData,
        mut solution_stream: QuerySolutionStream,
    ) -> Result<(), WebVowlStoreError> {
        info!("Serializing nodes stream");
        while let Some(solution) = solution_stream.next().await {
            let solution = solution?;
            let Some(id_term) = solution.get("id") else {
                continue;
            };
            let Some(node_type_term) = solution.get("nodeType") else {
                continue;
            };
            let triple = NodeTriple {
                id: id_term.into(),
                node_type: node_type_term.into(),
                target: solution.get("label").map_or(None, |term| Some(term.into())),
            };

            self.write_node_triple(data_buffer, triple);
            info!("{}", data_buffer);
        }
        Ok(())
    }

    /*
    pub fn insert_iri(
        &mut self,
        data_buffer: &mut GraphDisplayData,
        x: &TermRef<'a>
    ) -> usize {
        if self.resolve(data_buffer, &x).is_none() {
            let present = self.iricache.contains_key(&x);
            if !present {
                self.iricache.insert(x.clone(), data_buffer.irivec.len() as usize);
                data_buffer.irivec.push(x.clone());
            }
        }
        self.iricache[&x]
    }*/

    pub fn resolve(&mut self, data_buffer: &mut GraphDisplayData, x: &String) -> Option<usize> {
        if self.blanknode_mapping.contains_key(x) {
            return self.resolve(
                data_buffer,
                &data_buffer.labels[self.blanknode_mapping[x]].clone(),
            );
        } else if self.iricache.contains_key(x) {
            return Some(self.iricache[x]);
        }
        None
    }
    pub fn resolve_so(
        &mut self,
        data_buffer: &mut GraphDisplayData,
        triple: NodeTriple,
    ) -> (Option<usize>, Option<usize>) {
        let resolved_subject = self.resolve(data_buffer, &triple.id.to_string());
        let resolved_object = self.resolve(
            data_buffer,
            &triple.target.expect("Target is required").to_string(),
        );
        (resolved_subject, resolved_object)
    }

    fn insert_node(
        &mut self,
        data_buffer: &mut GraphDisplayData,
        triple: NodeTriple,
        node_type: ElementType,
    ) {
        data_buffer.elements.push(node_type);
        data_buffer.labels.push(triple.id.to_string());
    }

    /// NOTE: The edge array is overwritten.
    /// This means if a solution has multiple edge terms, the last one seen wins.
    fn insert_edge(
        &mut self,
        data_buffer: &mut GraphDisplayData,
        triple: NodeTriple,
        edge_type: ElementType,
    ) {
        let (index_s, index_o) = self.resolve_so(data_buffer, triple);
        let edge_index = data_buffer.elements.len();
        data_buffer
            .edges
            .push([index_s.unwrap(), edge_index, index_o.unwrap()]);
        data_buffer.elements.push(edge_type);
    }

    fn write_node_triple(&mut self, data_buffer: &mut GraphDisplayData, triple: NodeTriple) {
        // TODO: Collect errors and show to frontend
        let term = triple.node_type;
        match term {
            TermRef::BlankNode(bnode) => {
                info!("Is blank node: '{}'", bnode.to_string());
                // This would never be relevant, since the query should never put blank nodes in the ?nodeType variable
            }
            TermRef::Literal(literal) => {
                info!("Is literal: '{}'", literal.value());
            }
            TermRef::NamedNode(uri) => {
                // NOTE: Only supports RDF 1.1
                info!("Is named node: '{}'", uri);
                match uri {
                    // ----------- RDF ----------- //

                    // rdf::ALT => {}
                    // rdf::BAG => {}
                    // rdf::FIRST => {}
                    // rdf::HTML => {}
                    // rdf::LANG_STRING => {}
                    // rdf::LIST => {}
                    // rdf::NIL => {}
                    // rdf::OBJECT => {}
                    // rdf::PREDICATE => {}
                    // rdf::PROPERTY => {}
                    // rdf::REST => {}
                    // rdf::SEQ => {}
                    // rdf::STATEMENT => {}
                    // rdf::SUBJECT => {}
                    // rdf::TYPE => {}
                    // rdf::VALUE => {}
                    // rdf::XML_LITERAL => {}

                    // ----------- RDFS ----------- //
                    rdfs::CLASS => self.insert_node(
                        data_buffer,
                        triple,
                        ElementType::Rdfs(RdfsType::Node(RdfsNode::Class)),
                    ),
                    // rdfs::COMMENT => {}
                    // rdfs::CONTAINER => {}
                    // rdfs::CONTAINER_MEMBERSHIP_PROPERTY => {}
                    // rdfs::DATATYPE => {},
                    rdfs::DOMAIN => {
                        // TODO: Implement
                    }
                    // rdfs::IS_DEFINED_BY => {}
                    rdfs::LABEL => {
                        // TODO: Implement
                    }
                    rdfs::LITERAL => {
                        // TODO: Implement
                    }
                    // rdfs::MEMBER => {}
                    rdfs::RANGE => {
                        // TODO: Implement
                    }
                    // rdfs::RESOURCE => {}
                    // rdfs::SEE_ALSO => {}
                    rdfs::SUB_CLASS_OF => self.insert_edge(
                        data_buffer,
                        triple,
                        ElementType::Rdfs(RdfsType::Edge(RdfsEdge::SubclassOf)),
                    ),
                    // rdfs::SUB_PROPERTY_OF => {},

                    // ----------- OWL 2 ----------- //

                    // owl::ALL_DIFFERENT => {},
                    // owl::ALL_DISJOINT_CLASSES => {},
                    // owl::ALL_DISJOINT_PROPERTIES => {},
                    // owl::ALL_VALUES_FROM => {}
                    // owl::ANNOTATED_PROPERTY => {},
                    // owl::ANNOTATED_SOURCE => {},
                    // owl::ANNOTATED_TARGET => {},
                    // owl::ANNOTATION => {},
                    // owl::ANNOTATION_PROPERTY => {},
                    // owl::ASSERTION_PROPERTY => {},
                    // owl::ASYMMETRIC_PROPERTY => {},
                    // owl::AXIOM => {},
                    // owl::BACKWARD_COMPATIBLE_WITH => {},
                    // owl::BOTTOM_DATA_PROPERTY => {},
                    // owl::BOTTOM_OBJECT_PROPERTY => {},
                    // owl::CARDINALITY => {}
                    owl::CLASS => self.insert_node(
                        data_buffer,
                        triple,
                        ElementType::Owl(OwlType::Node(OwlNode::Class)),
                    ),
                    // ComplementOf missing, oversight?
                    owl::COMPLEMENT_OF => {
                        // self.insert_edge(
                        // data_buffer,
                        // triple,
                        // ElementType::Owl(OwlType::Edge(OwlEdge::ComplementOf)),
                        // TODO: Implement
                    }
                    owl::DATATYPE_COMPLEMENT_OF => {}
                    owl::DATATYPE_PROPERTY => self.insert_edge(
                        data_buffer,
                        triple,
                        ElementType::Owl(OwlType::Edge(OwlEdge::DatatypeProperty)),
                    ),
                    // owl::DATA_RANGE => {}
                    // owl::DEPRECATED => {}
                    owl::DEPRECATED_CLASS => self.insert_node(
                        data_buffer,
                        triple,
                        ElementType::Owl(OwlType::Node(OwlNode::DeprecatedClass)),
                    ),
                    owl::DEPRECATED_PROPERTY => self.insert_edge(
                        data_buffer,
                        triple,
                        ElementType::Owl(OwlType::Edge(OwlEdge::DeprecatedProperty)),
                    ),
                    // owl::DIFFERENT_FROM => {}
                    // owl::DISJOINT_UNION_OF => {}
                    owl::DISJOINT_WITH => self.insert_edge(
                        data_buffer,
                        triple,
                        ElementType::Owl(OwlType::Edge(OwlEdge::DisjointWith)),
                    ),
                    // owl::DISTINCT_MEMBERS => {}
                    owl::EQUIVALENT_CLASS => {
                        self.blanknode_mapping.insert(
                            triple.target.unwrap().to_string(),
                            self.iricache[&triple.id.to_string()],
                        );
                        self.insert_node(
                            data_buffer,
                            triple,
                            ElementType::Owl(OwlType::Node(OwlNode::EquivalentClass)),
                        );
                    }
                    // owl::EQUIVALENT_PROPERTY => {}
                    owl::FUNCTIONAL_PROPERTY => {
                        // self.try_insert_characteristic(
                        // data_buffer,
                        // term,
                        // Characteristic::FunctionalProperty)
                        // TODO: Implement
                    }
                    // owl::HAS_KEY => {}
                    // owl::HAS_SELF => {}
                    // owl::HAS_VALUE => {}
                    // owl::IMPORTS => {}
                    // owl::INCOMPATIBLE_WITH => {}
                    owl::INTERSECTION_OF => self.insert_node(
                        data_buffer,
                        triple,
                        ElementType::Owl(OwlType::Node(OwlNode::IntersectionOf)),
                    ),
                    owl::INVERSE_FUNCTIONAL_PROPERTY => {
                        //self.try_insert_characteristic(
                        // data_buffer,
                        // term,
                        // Characteristic::InverseFunctionalProperty)
                        // TODO: Implement
                    }
                    owl::INVERSE_OF => self.insert_edge(
                        data_buffer,
                        triple,
                        ElementType::Owl(OwlType::Edge(OwlEdge::InverseOf)),
                    ),
                    // owl::IRREFLEXIVE_PROPERTY => {}
                    // owl::MAX_CARDINALITY => {}
                    // owl::MAX_QUALIFIED_CARDINALITY => {}
                    // owl::MEMBERS => {}
                    // owl::MIN_CARDINALITY => {}
                    // owl::MIN_QUALIFIED_CARDINALITY => {}
                    owl::NAMED_INDIVIDUAL => {}
                    // owl::NEGATIVE_PROPERTY_ASSERTION => {}
                    owl::NOTHING => {}
                    owl::OBJECT_PROPERTY => self.insert_edge(
                        data_buffer,
                        triple,
                        ElementType::Owl(OwlType::Edge(OwlEdge::ObjectProperty)),
                    ),
                    // owl::ONE_OF => {}
                    // owl::ONTOLOGY => {}
                    // owl::ONTOLOGY_PROPERTY => {}
                    // owl::ON_CLASS => {}
                    // owl::ON_DATARANGE => {}
                    // owl::ON_DATATYPE => {}
                    // owl::ON_PROPERTIES => {}
                    // owl::ON_PROPERTY => {}
                    // owl::PRIOR_VERSION => {}
                    // owl::PROPERTY_CHAIN_AXIOM => {}
                    // owl::PROPERTY_DISJOINT_WITH => {}
                    // owl::QUALIFIED_CARDINALITY => {}
                    owl::REFLEXIVE_PROPERTY => {}
                    // owl::RESTRICTION => {}
                    // owl::SAME_AS => {}
                    // owl::SOME_VALUES_FROM => {}
                    // owl::SOURCE_INDIVIDUAL => {}
                    owl::SYMMETRIC_PROPERTY => {}
                    // owl::TARGET_INDIVIDUAL => {}
                    // owl::TARGET_VALUE => {}
                    owl::THING => self.insert_node(
                        data_buffer,
                        triple,
                        ElementType::Owl(OwlType::Node(OwlNode::Thing)),
                    ),
                    // owl::TOP_DATA_PROPERTY => {}
                    // owl::TOP_OBJECT_PROPERTY => {}
                    owl::TRANSITIVE_PROPERTY => {
                        // self.try_insert_characteristic(
                        // data_buffer,
                        // term,
                        // Characteristic::Transitive)
                        //
                        // TODO: Implement
                    }
                    owl::UNION_OF => {}
                    // owl::VERSION_INFO => {}
                    // owl::VERSION_IRI => {}
                    // owl::WITH_RESTRICTIONS => {}
                    _ => {
                        // Visualization of this element is not supported
                        warn!("Visualization of term '{term}' is not supported");
                    }
                };
            }
        }
    }
}
