use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter},
    time::{Duration, Instant},
};

use crate::vocab::owl;
use futures::StreamExt;
use grapher::prelude::{
    ElementType, GenericEdge, GraphDisplayData, OwlEdge, OwlNode, OwlType, RdfsEdge, RdfsNode, RdfsType
};
use log::{info, warn};
use rdf_fusion::{
    execution::results::QuerySolutionStream,
    model::{Term, vocab::rdfs},
};
use webvowl_parser::errors::WebVowlStoreError;

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct NodeTriple {
    id: Term,
    node_type: Term,
    target: Option<Term>,
}
impl Display for NodeTriple {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NodeTriple {{ id: {} \n node_type: {} \n target: {} }}",
            self.id,
            self.node_type,
            self.target
                .as_ref()
                .map(|t| t.to_string())
                .unwrap_or_default()
        )
    }
}
pub struct GraphDisplayDataSolutionSerializer {
    blanknode_mapping: HashMap<String, String>,
    iricache: HashMap<String, usize>,
    mapped_to: HashMap<usize, HashSet<String>>,
    unknown_buffer: HashSet<NodeTriple>,
    object_properties: HashMap<String, usize>,
    edges: HashSet<[usize; 3]>,
    doc_iri: String,
}

impl GraphDisplayDataSolutionSerializer {
    pub fn new() -> Self {
        Self {
            blanknode_mapping: HashMap::new(),
            iricache: HashMap::new(),
            mapped_to: HashMap::new(),
            object_properties: HashMap::new(),
            unknown_buffer: HashSet::new(),
            edges: HashSet::new(),
            doc_iri: String::new(),
        }
    }

    pub async fn serialize_nodes_stream(
        &mut self,
        data_buffer: &mut GraphDisplayData,
        mut solution_stream: QuerySolutionStream,
    ) -> Result<(), WebVowlStoreError> {
        let mut count: u32 = 0;
        info!("Serializing query solution stream...");
        let start_time = Instant::now();
        while let Some(solution) = solution_stream.next().await {
            let solution = solution?;
            let Some(id_term) = solution.get("id") else {
                continue;
            };
            let Some(node_type_term) = solution.get("nodeType") else {
                continue;
            };
            let triple: NodeTriple = NodeTriple {
                id: id_term.to_owned(),
                node_type: node_type_term.to_owned(),
                target: solution.get("label").map(|term| term.to_owned()),
            };
            self.write_node_triple(data_buffer, triple);
            count += 1;
        }
        let finish_time = start_time
            .checked_duration_since(Instant::now())
            .unwrap_or(Duration::new(0, 0))
            .as_secs();
        info!(
            "Serialization completed in {} s\n \
            \tTotal solutions: {count}\n \
            \tElements       : {}\n \
            \tEdges          : {}\n \
            \tLabels         : {}\n \
            \tCardinalities  : {}\n \
            \tCharacteristics: {}\n\n \
        ",
            finish_time,
            data_buffer.elements.len(),
            data_buffer.edges.len(),
            data_buffer.labels.len(),
            data_buffer.cardinalities.len(),
            data_buffer.characteristics.len()
        );
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
            return self.resolve(data_buffer, &self.blanknode_mapping[x].clone());
        } else if self.iricache.contains_key(x) {
            return Some(self.iricache[x]);
        }
        None
    }
    pub fn resolve_so(
        &mut self,
        data_buffer: &mut GraphDisplayData,
        triple: &NodeTriple,
    ) -> (Option<usize>, Option<usize>) {
        if triple.target.is_none() {
            warn!("Target is required for edge: {:?}", triple);
        }
        let resolved_subject = self.resolve(data_buffer, &triple.id.to_string());
        let resolved_object = match triple.target.as_ref() {
            Some(target) => self.resolve(data_buffer, &target.to_string()),
            None => {
                warn!("Target is required for edge: {:?}", triple);
                None
            }
        };

        (resolved_subject, resolved_object)
    }

    fn insert_node(
        &mut self,
        data_buffer: &mut GraphDisplayData,
        triple: NodeTriple,
        node_type: ElementType,
    ) {
        let iri = triple.id.to_string();
        let label = self.label_from_iri(&iri);
        data_buffer.labels.push(label);
        data_buffer.elements.push(node_type);
        self.iricache.insert(iri, data_buffer.labels.len() - 1);
        //self.check_insert_unknowns(data_buffer);
    }

    fn label_from_iri(&self, iri: &str) -> String {
        let cleaned_iri = iri.trim_matches(['<', '>']);
        let cleaned_doc_iri = self.doc_iri.trim_matches(['<', '>']);

        if !cleaned_doc_iri.is_empty() {
            if let Some(suffix) = cleaned_iri.strip_prefix(cleaned_doc_iri) {
                let trimmed = suffix.trim_start_matches(['#', '/']);
                if !trimmed.is_empty() {
                    return trimmed.to_string();
                }
            }
        }

        cleaned_iri
            .rsplit(['#', '/'])
            .next()
            .unwrap_or(cleaned_iri)
            .to_string()
    }

    fn check_insert_unknowns(&mut self, data_buffer: &mut GraphDisplayData) {
        let unknown_buffer = std::mem::take(&mut self.unknown_buffer);
        for node_triple in unknown_buffer.iter() {
            match self.resolve_so(data_buffer, node_triple) {
                (Some(_), Some(_)) => {
                    self.write_node_triple(data_buffer, node_triple.clone());
                }
                _ => {}
            }
        }
    }

    fn insert_edge(
        &mut self,
        data_buffer: &mut GraphDisplayData,
        triple: &NodeTriple,
        edge_type: ElementType,
    ) {
        let (index_s, index_o) = self.resolve_so(data_buffer, &triple);
        if index_s.is_none() || index_o.is_none() {
            self.unknown_buffer.insert(triple.clone());
        } else {
            let edge = [index_s.unwrap(), data_buffer.elements.len(), index_o.unwrap()];
            if !self.edges.contains(&edge) {
            data_buffer
                .edges
                    .push(edge.clone());
            data_buffer.elements.push(edge_type);
                self.edges.insert(edge);
            }
        }
    }

    fn replace_node(&mut self, _data_buffer: &mut GraphDisplayData, old: usize, new: usize) {
        //let old = data_buffer.labels[old];
        let iter = self.mapped_to.remove(&old);
        match iter {
            Some(iter) => {
                self.mapped_to.insert(new, iter.clone());
                for index in iter.clone() {
                    self.iricache.insert(index.clone(), new);
                }
            }
            None => {}
        }
    }

    fn map_to(&mut self, k: String, v: usize) {
        self.iricache.insert(k.clone(), v);
        if self.mapped_to.contains_key(&v) {
            self.mapped_to.get_mut(&v).unwrap().insert(k.clone());
        } else {
            self.mapped_to.insert(v, HashSet::from([k]));
        }
    }

    fn write_node_triple(&mut self, data_buffer: &mut GraphDisplayData, triple: NodeTriple) {
        // TODO: Collect errors and show to frontend
        let node_type = triple.node_type.clone();
        println!("{}", triple);
        match node_type {
            Term::BlankNode(bnode) => {
                // The query must never put blank nodes in the ?nodeType variable
                // TODO: Handle errors gracefully (and show to frontend)
                panic!(
                    "Illegal blank node during serialization: '{}'",
                    bnode.to_string()
                );
            }
            Term::Literal(literal) => {
                // NOTE: Any string literal goes here, e.g. 'EquivalentClass'.
                // That is, every BIND("someString" AS ?nodeType)
                //info!("Is literal: '{}'", literal.value());
                
                warn!("Visualization of literal '{}' is not supported", literal.value());
                    
            }
            Term::NamedNode(uri) => {
                // NOTE: Only supports RDF 1.1
                // info!("Is named node: '{}'", uri);
                // TODO: Finding external classes/properties:
                // 1. Elements whose base URI differs from that of the visualized ontology.
                // 2. A base URI is EITHER `xml:base` OR that of the document.
                // SOURCE (save this for the paper and documentation):
                // 1. p. 6 of https://www.semantic-web-journal.net/system/files/swj1114.pdf
                // 2. https://www.w3.org/TR/rdf-syntax-grammar/#section-Syntax-ID-xml-base
                match uri.as_ref() {
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
                    rdfs::DATATYPE => {
                        self.insert_edge(
                            data_buffer,
                            &triple,
                            ElementType::Rdfs(RdfsType::Edge(RdfsEdge::Datatype)),
                        );
                        println!("{}", triple);
                    }
                    rdfs::DOMAIN => {
                        println!("{}", triple);
                        if let Some(index) = self.object_properties.get(&triple.id.to_string()) {
                            self.insert_edge(
                                data_buffer,
                                &triple,
                                ElementType::Owl(OwlType::Edge(OwlEdge::ObjectProperty)),
                            );
                        } else {
                            self.unknown_buffer.insert(triple.clone());
                        }
                    }
                    // rdfs::IS_DEFINED_BY => {}
                    rdfs::LABEL => {
                        // TODO: Implement
                    }
                    rdfs::LITERAL => {
                        self.insert_node(
                            data_buffer,
                            triple,
                            ElementType::Rdfs(RdfsType::Node(RdfsNode::Literal)),
                        );
                    }
                    // rdfs::MEMBER => {}
                    rdfs::RANGE => {
                        // TODO: Implement
                    }
                    // rdfs::RESOURCE => {}
                    // rdfs::SEE_ALSO => {}
                    rdfs::SUB_CLASS_OF => self.insert_edge(
                        data_buffer,
                        &triple,
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
                        &triple,
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
                        &triple,
                        ElementType::Owl(OwlType::Edge(OwlEdge::DeprecatedProperty)),
                    ),
                    // owl::DIFFERENT_FROM => {}
                    // owl::DISJOINT_UNION_OF => {}
                    owl::DISJOINT_WITH => self.insert_edge(
                        data_buffer,
                        &triple,
                        ElementType::Owl(OwlType::Edge(OwlEdge::DisjointWith)),
                    ),
                    // owl::DISTINCT_MEMBERS => {}
                    owl::EQUIVALENT_CLASS => {
                        let index = self
                            .resolve(data_buffer, &triple.id.to_string())
                            .expect("Couldnt resolve for id");

                        if matches!(triple.target.as_ref(), Some(Term::NamedNode(_))) {
                            self.upgrade_node_type(
                                data_buffer,
                                index,
                                ElementType::Owl(OwlType::Node(OwlNode::EquivalentClass)),
                            );
                            if let Some(target) = triple.target.as_ref() {
                                if let Some(target_index) =
                                    self.resolve(data_buffer, &target.to_string())
                                {
                                    self.replace_node(data_buffer, index, target_index);
                                }
                            }
                        }
                        let target = triple
                            .target
                            .as_ref()
                            .expect("Target is required")
                            .to_string();
                        self.map_to(target, index);
                    }
                    // owl::EQUIVALENT_PROPERTY => {}
                    owl::FUNCTIONAL_PROPERTY => {}
                    // owl::HAS_KEY => {}
                    // owl::HAS_SELF => {}
                    // owl::HAS_VALUE => {}
                    // owl::IMPORTS => {}
                    // owl::INCOMPATIBLE_WITH => {}
                    owl::INTERSECTION_OF => {
                        let index = self.resolve(data_buffer, &triple.id.to_string());

                        let target = triple.target.as_ref().expect("Target is required");

                        if let Some(index) = index {
                            self.map_to(target.to_string(), index);
                        } else {
                            self.blanknode_mapping
                                .insert(triple.id.to_string(), target.to_string());
                        }
                    }
                    owl::INVERSE_FUNCTIONAL_PROPERTY => {
                        //self.try_insert_characteristic(
                        // data_buffer,
                        // term,
                        // Characteristic::InverseFunctionalProperty)
                        // TODO: Implement
                    }
                    owl::INVERSE_OF => self.insert_edge(
                        data_buffer,
                        &triple,
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
                    owl::OBJECT_PROPERTY => {
                        self.map_to(triple.id.to_string(), data_buffer.elements.len() - 1);
                        println!("{}", triple);
                        
                    },
                    // owl::ONE_OF => {}
                    owl::ONTOLOGY => {
                        self.doc_iri = uri.to_string();
                    }
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
                        warn!("Visualization of term '{}' is not supported", uri);
                    }
                };
            }
        }
    }

    fn upgrade_node_type(
        &self,
        data_buffer: &mut GraphDisplayData,
        index: usize,
        node_type: ElementType,
    ) {
        if data_buffer.elements[index] == ElementType::Owl(OwlType::Node(OwlNode::Class)) {
            data_buffer.elements[index] = node_type;
        }
    }
}

impl Display for GraphDisplayDataSolutionSerializer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (index, (element, label)) in self.iricache.iter().enumerate() {
            write!(f, "{index}: {element:?} -> {label}\n")?;
        }
        Ok(())
    }
}
