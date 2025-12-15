use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter},
    time::{Duration, Instant},
};

use crate::vocab::owl;
use fluent_uri::Iri;
use futures::StreamExt;
use grapher::prelude::{
    ElementType, GraphDisplayData, OwlEdge, OwlNode, OwlType, RdfsEdge, RdfsNode, RdfsType,
};
use log::{debug, info, trace, warn};
use rdf_fusion::{
    execution::results::QuerySolutionStream,
    model::{Term, vocab::rdfs},
};
use webvowl_parser::errors::WebVowlStoreError;

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct Triple {
    /// The subject
    id: Term,
    /// The predicate
    element_type: Term,
    /// The object
    target: Option<Term>,
}
impl Display for Triple {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Triple {{")?;
        writeln!(f, "\tsubject: {}", self.id)?;
        writeln!(f, "\telement_type: {}", self.element_type,)?;
        writeln!(
            f,
            "\tobject: {}",
            self.target
                .as_ref()
                .map(|t| t.to_string())
                .unwrap_or_default(),
        )?;
        writeln!(f, "}}")
    }
}
pub struct GraphDisplayDataSolutionSerializer {
    blanknode_mapping: HashMap<String, String>,
    iricache: HashMap<String, usize>,
    mapped_to: HashMap<usize, HashSet<String>>,
    /// Stores indices of element instances.
    ///
    /// Used in cases where multiple elements should refer to a particular instance.
    /// E.g. multiple properties referring to the same instance of owl:Thing.
    global_element_mappings: HashMap<ElementType, usize>,
    /// Stores labels until it's corresponding class/property has been found.
    ///
    /// usize = element index in data_buffer.elements
    /// String = label
    labels: HashMap<String, String>,
}

impl GraphDisplayDataSolutionSerializer {
    pub fn new() -> Self {
        Self {
            blanknode_mapping: HashMap::new(),
            iricache: HashMap::new(),
            mapped_to: HashMap::new(),
            global_element_mappings: HashMap::new(),
            labels: HashMap::new(),
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
                // Labels are a separate solution without nodetype
                self.extract_label(solution.get("label"), id_term);
                continue;
            };

            // Handle cases where label is, in fact, not a separate solution.
            self.extract_label(solution.get("label"), id_term);

            let triple: Triple = Triple {
                id: id_term.to_owned(),
                element_type: node_type_term.to_owned(),
                target: solution.get("target").map(|term| term.to_owned()),
            };
            self.write_node_triple(data_buffer, triple);
            count += 1;
        }
        let finish_time = Instant::now()
            .checked_duration_since(start_time)
            .unwrap_or(Duration::new(0, 0))
            .as_secs_f32();
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
        debug!("{}", data_buffer);
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

    /// Extract label info from the query solution and store for later use.
    /// This is necessary as labels are currently emitted in a solution separate
    /// from the triple they belong to.
    fn extract_label(&mut self, label: Option<&Term>, id_term: &Term) {
        let iri = id_term.to_string();
        let result = match label {
            Some(label) => {
                if label.to_string() != "" {
                    Ok(self.labels.insert(id_term.to_string(), label.to_string()))
                } else {
                    Err(())
                }
            }
            None => match Iri::parse(iri.clone()) {
                Ok(id_iri) => match id_iri.fragment() {
                    Some(frag) => Ok(self.labels.insert(id_iri.to_string(), frag.to_string())),
                    // TODO: Handle path component (see label docs in this codebase)
                    None => Err(()),
                },
                Err(_) => Err(()),
            },
        };
        match result {
            Ok(_) => {}
            Err(_) => {
                self.labels.insert(iri.clone(), iri);
            }
        }
    }

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
        triple: &Triple,
    ) -> (Option<usize>, Option<usize>) {
        let resolved_subject = self.resolve(data_buffer, &triple.id.to_string());
        let resolved_object = match &triple.target {
            Some(target) => self.resolve(data_buffer, &target.to_string()),
            None => {
                warn!("Cannot resolve object of triple:\n {}", triple);
                None
            }
        };
        (resolved_subject, resolved_object)
    }

    fn insert_node(
        &mut self,
        data_buffer: &mut GraphDisplayData,
        triple: Triple,
        node_type: ElementType,
    ) {
        data_buffer.elements.push(node_type);
        self.insert_label(data_buffer, &triple, &node_type);
        self.iricache
            .insert(triple.id.to_string(), data_buffer.labels.len() - 1);
    }

    /// If no domain and/or range axiom is defined for a property, owl:Thing is used as domain and/or range.
    /// EXCEPT datatype properties without a defined range. Here, rdfs:Literal is used as range instead.
    ///
    /// Procedure:
    /// If NOT rdfs:Datatype:
    /// - Property missing domain OR range:
    ///   - Create new owl:Thing for this property.
    /// - Property missing domain AND range:
    ///   - Create a new owl:Thing (if not created previously) and use this instance for ALL edges in this category.
    /// IF rdfs:Datatype:
    /// - Property missing domain AND/OR range:
    ///   - Create new rdfs:Literal for this property
    fn insert_edge(
        &mut self,
        data_buffer: &mut GraphDisplayData,
        triple: &Triple,
        edge_type: ElementType,
    ) {
        let (subject_index, object_index) = self.resolve_so(data_buffer, &triple);
        // let mut edge: [usize; 3] = [];

        // if subject_index.is_none() {}
        // if object_index.is_none() {
        //     if let Some(thing) = self
        //         .global_element_mappings
        //         .get(ElementType::Owl(OwlType::Node(OwlNode::Thing)))
        //     {
        //         edge[1] = *thing;
        //     } else {
        //         edge[1] = subject_index.unwrap();
        //     }
        // }

        let edge_index = data_buffer.elements.len();
        data_buffer
            .edges
            .push([subject_index.unwrap(), edge_index, object_index.unwrap()]);
        data_buffer.elements.push(edge_type);
        self.insert_label(data_buffer, &triple, &edge_type);
    }

    /// Create a label for an element.
    ///
    /// Note: must be called AFTER adding the element to `databuffer.elements`.
    fn insert_label(
        &mut self,
        data_buffer: &mut GraphDisplayData,
        triple: &Triple,
        element_type: &ElementType,
    ) {
        if let Some(label) = self.labels.remove(&triple.id.to_string()) {
            data_buffer.labels.push(label);
        } else {
            // Fallback label as all elements must have a label.
            data_buffer.labels.push(element_type.to_string());
        }
    }

    /// Creates nodes as targets for edges without one in the solution.
    // fn handle_missing_edges(
    //     &mut self,
    //     data_buffer: &mut GraphDisplayData,
    //     edge_type: ElementType,
    //     subject_index: Option<usize>,
    //     object_index: Option<usize>,
    // ) -> [usize; 3] {
    //     match edge_type {
    //         ElementType::Rdfs(RdfsType::Node(RdfsNode::Datatype)) => {}
    //         _ => {
    //             let thing = ElementType::Owl(OwlType::Node(OwlNode::Thing));

    //             // Case: NOT rdfs:Datatype missing domain AND range
    //             if subject_index.is_none() && object_index.is_none() {
    //                 if let Some(thing_idx) = self.global_element_mappings.get(&thing) {
    //                     [*thing_idx, 0, *thing_idx]
    //                 } else {
    //                     let thing_idx = data_buffer.elements.len();
    //                     data_buffer.labels.push(thing.to_string());
    //                     data_buffer.elements.push(thing);
    //                     [thing_idx, 0, thing_idx]
    //                 }
    //             }

    //             // Case: NOT rdfs:Datatype missing domain OR range
    //             if let Some(sub_idx) = subject_index {
    //                 edge[0] = sub_idx;
    //             } else {
    //                 // TODO: Create owl:Thing
    //             }
    //             if let Some(obj_idx) = object_index {
    //                 edge[1] = obj_idx;
    //             } else {
    //                 // TODO: Create owl:Thing
    //             }
    //         }
    //     }
    // }

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

    fn write_node_triple(&mut self, data_buffer: &mut GraphDisplayData, triple: Triple) {
        // TODO: Collect errors and show to frontend
        let node_type = triple.element_type.clone();
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
                let value = literal.value();
                match value {
                    // "blanknode" => {}
                    &_ => {
                        warn!("Visualization of literal '{value}' is not supported");
                    }
                }
            }
            Term::NamedNode(uri) => {
                // NOTE: Only supports RDF 1.1
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
                            ElementType::Rdfs(RdfsType::Node(RdfsNode::Datatype)),
                        );
                    }
                    // rdfs::DOMAIN => {}
                    // rdfs::IS_DEFINED_BY => {}
                    // rdfs::LABEL => {}
                    rdfs::LITERAL => {
                        self.insert_node(
                            data_buffer,
                            triple,
                            ElementType::Rdfs(RdfsType::Node(RdfsNode::Literal)),
                        );
                    }
                    // rdfs::MEMBER => {}
                    // rdfs::RANGE => {}
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
                    // owl::COMPLEMENT_OF => {
                    //     // self.insert_edge(
                    //     // data_buffer,
                    //     // triple,
                    //     // ElementType::Owl(OwlType::Edge(OwlEdge::ComplementOf)),
                    //     // TODO: Implement
                    // }
                    // owl::DATATYPE_COMPLEMENT_OF => {}
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
                        trace!("blanknode_mapping | {:?}", self.blanknode_mapping);
                        trace!("mapped_to | {:?}", self.mapped_to);
                        trace!("iricache | {:?}", self.iricache);
                        trace!("triple: {:?}", triple);
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
                                    trace!("target_index: {:?}", target_index);
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
                    // owl::FUNCTIONAL_PROPERTY => {}
                    // owl::HAS_KEY => {}
                    // owl::HAS_SELF => {}
                    // owl::HAS_VALUE => {}
                    // owl::IMPORTS => {}
                    // owl::INCOMPATIBLE_WITH => {}
                    owl::INTERSECTION_OF => {
                        trace!("{}", triple);
                        trace!("{}", self);
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
                    // owl::NAMED_INDIVIDUAL => {}
                    // owl::NEGATIVE_PROPERTY_ASSERTION => {}
                    owl::NOTHING => {}
                    owl::OBJECT_PROPERTY => self.insert_edge(
                        data_buffer,
                        &triple,
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
                    // owl::REFLEXIVE_PROPERTY => {}
                    // owl::RESTRICTION => {}
                    // owl::SAME_AS => {}
                    // owl::SOME_VALUES_FROM => {}
                    // owl::SOURCE_INDIVIDUAL => {}
                    // owl::SYMMETRIC_PROPERTY => {}
                    // owl::TARGET_INDIVIDUAL => {}
                    // owl::TARGET_VALUE => {}
                    owl::THING => self.insert_node(
                        data_buffer,
                        triple,
                        ElementType::Owl(OwlType::Node(OwlNode::Thing)),
                    ),
                    // owl::TOP_DATA_PROPERTY => {}
                    // owl::TOP_OBJECT_PROPERTY => {}
                    // owl::TRANSITIVE_PROPERTY => {
                    //     // self.try_insert_characteristic(
                    //     // data_buffer,
                    //     // term,
                    //     // Characteristic::Transitive)
                    //     //
                    //     // TODO: Implement
                    // }
                    // owl::UNION_OF => {}
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
}
impl Display for GraphDisplayDataSolutionSerializer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (index, (element, label)) in self.iricache.iter().enumerate() {
            write!(f, "{index}: {element:?} -> {label}\n")?;
        }
        Ok(())
    }
}
