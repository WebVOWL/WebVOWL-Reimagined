use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter},
    time::{Duration, Instant},
};

use crate::vocab::owl;
use fluent_uri::Iri;
use futures::StreamExt;
use grapher::prelude::{
    Characteristic, ElementType, GenericEdge, GraphDisplayData, OwlEdge, OwlNode, OwlType, RdfsEdge, RdfsNode, RdfsType
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
    object_properties: HashMap<String, usize>,
    edges: HashSet<(usize, ElementType, usize)>,
    doc_iri: String,
}

impl GraphDisplayDataSolutionSerializer {
    pub fn new() -> Self {
        Self {
            blanknode_mapping: HashMap::new(),
            iricache: HashMap::new(),
            mapped_to: HashMap::new(),
            object_properties: HashMap::new(),
            global_element_mappings: HashMap::new(),
            labels: HashMap::new(),
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
                // Labels are a separate solution without nodetype
                self.extract_label(solution.get("label"), id_term);
                continue;
            };

            // Handle cases where label is, in fact, not a separate solution.
            //self.extract_label(solution.get("label"), id_term);

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
        if triple.target.is_none() {
            warn!("Target is required for edge: {:?}", triple);
        }
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

    fn insert_edge(
        &mut self,
        data_buffer: &mut GraphDisplayData,
        triple: &Triple,
        edge_type: ElementType,
    ) {
        let (maybe_sub_idx, maybe_obj_idx) = self.resolve_so(data_buffer, &triple);

        let mut edge = match edge_type {
            ElementType::Owl(OwlType::Edge(OwlEdge::DatatypeProperty)) => self
                .handle_missing_edges(
                    data_buffer,
                    ElementType::Rdfs(RdfsType::Node(RdfsNode::Literal)),
                    maybe_sub_idx,
                    maybe_obj_idx,
                ),
            _ => self.handle_missing_edges(
                data_buffer,
                ElementType::Owl(OwlType::Node(OwlNode::Thing)),
                maybe_sub_idx,
                maybe_obj_idx,
            ),
        };
        edge[1] = data_buffer.elements.len();
        if !self.edges.contains(&(edge[0], edge_type, edge[2])) {
            data_buffer
                .edges
                .push(edge);

            data_buffer.elements.push(edge_type);
            self.edges.insert((edge[0], edge_type, edge[2]));
            self.insert_label(data_buffer, &triple, &edge_type);
        }
        
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
    ///
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
    fn handle_missing_edges(
        &mut self,
        data_buffer: &mut GraphDisplayData,
        node_to_create: ElementType,
        maybe_sub_idx: Option<usize>,
        maybe_obj_idx: Option<usize>,
    ) -> [usize; 3] {
        // Case: missing domain AND range
        if maybe_sub_idx.is_none() && maybe_obj_idx.is_none() {
            if let Some(global_idx) = self.global_element_mappings.get(&node_to_create) {
                [*global_idx, 0, *global_idx]
            } else {
                // Create global node type
                let global_idx = self.insert_global(data_buffer, node_to_create);
                [global_idx, 0, global_idx]
            }
        // Case: missing domain OR range
        } else {
            let sub_idx = maybe_sub_idx.unwrap_or(self.insert_local(data_buffer, node_to_create));
            let obj_idx = maybe_obj_idx.unwrap_or(self.insert_local(data_buffer, node_to_create));
            [sub_idx, 0, obj_idx]
        }
    }

    /// Insert an ElementType into the global element mapping.
    ///
    /// May only be called once for each ElementType!
    ///
    /// Use [`insert_local`] if multiple calls are required for each ElementType.
    fn insert_global(
        &mut self,
        data_buffer: &mut GraphDisplayData,
        element_to_create: ElementType,
    ) -> usize {
        let elem_idx = data_buffer.elements.len();
        self.global_element_mappings
            .insert(element_to_create, elem_idx);
        data_buffer.labels.push(element_to_create.to_string());
        data_buffer.elements.push(element_to_create);
        elem_idx
    }

    /// Create an ElementType for use in one solution.
    ///
    /// No call restrictions!
    fn insert_local(
        &mut self,
        data_buffer: &mut GraphDisplayData,
        element_to_create: ElementType,
    ) -> usize {
        let elem_idx = data_buffer.elements.len();
        data_buffer.labels.push(element_to_create.to_string());
        data_buffer.elements.push(element_to_create);
        elem_idx
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

    fn upgrade_node_type(
        &self,
        data_buffer: &mut GraphDisplayData,
        index: usize,
        node_type: ElementType,
    ) {
        info!("upgrading {}", node_type);
        if data_buffer.elements[index] == ElementType::Owl(OwlType::Node(OwlNode::Class)) {
            debug!("upgrading node: {} to {}", data_buffer.labels[index], node_type);
            data_buffer.elements[index] = node_type;
        }
    }

    fn map_to(&mut self, data_buffer: &mut GraphDisplayData, k: String, v: usize) {
        self.iricache.insert(k.clone(), v);
        if self.mapped_to.contains_key(&v) {
            self.mapped_to.get_mut(&v).unwrap().insert(k.clone());
        } else {
            self.mapped_to.insert(v, HashSet::from([k]));
        }
        //self.check_insert_unknowns(data_buffer);
    }

    fn map_bnode(&mut self, data_buffer: &mut GraphDisplayData, k: Term, v: Term) {
        self.blanknode_mapping.insert(k.to_string(), v.to_string());
        //self.check_insert_unknowns(data_buffer);
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
                    rdf::PROPERTY => {
                        self.insert_edge(
                            data_buffer,
                            &triple,
                            ElementType::Rdf(RdfType::Edge(RdfEdge::RdfProperty)),
                        );
                    }
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
                        self.insert_node(
                            data_buffer,
                            triple,
                            ElementType::Rdfs(RdfsType::Node(RdfsNode::Datatype)),
                        );
                    }
                    // TODO: 
                    // - in sparql query, sort domain before range
                    // - add domain to domain_map HashMap<String, String>, key is subject, value is object
                    // - add range to range_map HashMap<String, String>, key is subject, value is object
                    // - when matching ie. TransitiveProperty, check if domain and range are available, 
                    //   if so create edge with characteristics
                    //   otherwise, add to unknown_buffer
                    // - At the end of serialization, unknown_buffer is handled, inserting missing node sources/targets as Things
                    //  (call handle_missing_edges) 
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
                    owl::COMPLEMENT_OF => {
                        let (index_s, index_o)  = self.resolve_so(data_buffer, &triple);
                        match (index_s, index_o) {
                            (Some(index), Some(_)) => {
                                self.upgrade_node_type(data_buffer, index, ElementType::Owl(OwlType::Node(OwlNode::Complement)));
                                self.insert_edge(data_buffer, &triple, ElementType::NoDraw);
                            }
                            _ => {
                                //self.unknown_buffer.insert(triple.clone());
                            }
                        }
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
                    owl::DISJOINT_UNION_OF => {
                        self.insert_edge(
                            data_buffer,
                            &triple,
                            ElementType::NoDraw);
                        if let Some(index) = self.resolve(data_buffer, &triple.id.to_string()) {
                        self.upgrade_node_type(data_buffer, index, ElementType::Owl(OwlType::Node(OwlNode::DisjointUnion)));
                        }
                    }
                    owl::DISJOINT_WITH => self.insert_edge(
                        data_buffer,
                        &triple,
                        ElementType::Owl(OwlType::Edge(OwlEdge::DisjointWith)),
                    ),
                    // owl::DISTINCT_MEMBERS => {}
                    
                    owl::EQUIVALENT_CLASS => {
                        info!("blanknode_mapping | {:?}", self.blanknode_mapping);
                        info!("mapped_to | {:?}", self.mapped_to);
                        info!("iricache | {:?}", self.iricache);
                        info!("triple: {:?}", triple);
                        let index = self
                            .resolve(data_buffer, &triple.id.to_string())
                            .expect("Couldnt resolve for id");
                        /* 
                        match triple.target.as_ref() {
                            Some(Term::NamedNode(nn)) => {
                                let target_index = self.resolve(data_buffer, &nn.to_string());
                                match target_index {
                                    Some(target_index) => {
                                        self.map_to(data_buffer, nn.to_string(), target_index);

                                    }
                                    None => {
                                        warn!("Target is required for: {:?}", triple);
                                    }
                                }
                            }
                            Some(Term::BlankNode(bn)) => {
                                let target_index = self.resolve(data_buffer, &bn.to_string());
                                match target_index {
                                    Some(target_index) => {
                                        self.map_to(data_buffer, bn.to_string(), target_index);
                                    }
                                    None => {
                                        warn!("Target is required for: {:?}", triple);
                                    }
                                }
                                self.map_to(data_buffer, bn.to_string(), target_index);
                            }
                            _ => {
                                warn!("Target is required for: {:?}", triple);
                            }
                        }*/
                        let target_index = self.resolve(data_buffer, &triple.target.as_ref().unwrap().to_string());

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
                                } else {
                                    self.map_to(data_buffer, target.to_string(), index);
                                }
                            }
                        } else if matches!(triple.target.as_ref(), Some(Term::BlankNode(_))) {
                            self.map_bnode(data_buffer, triple.target.clone().unwrap(), triple.id.clone());
                        }
                        let target = triple
                            .target
                            .as_ref()
                            .expect("Target is required")
                            .to_string();
                        self.map_to(data_buffer, target, index);

                    }
                    // owl::EQUIVALENT_PROPERTY => {}
                    // owl::FUNCTIONAL_PROPERTY => {}
                    // owl::HAS_KEY => {}
                    // owl::HAS_SELF => {}
                    // owl::HAS_VALUE => {}
                    // owl::IMPORTS => {}
                    // owl::INCOMPATIBLE_WITH => {}
                    owl::INTERSECTION_OF => {
                        self.insert_edge(
                            data_buffer,
                            &triple,
                            ElementType::NoDraw);
                        if let Some(index) = self.resolve(data_buffer, &triple.id.to_string()) {
                            self.upgrade_node_type(data_buffer, index, ElementType::Owl(OwlType::Node(OwlNode::IntersectionOf)));
                        }
                    }
                    owl::INVERSE_FUNCTIONAL_PROPERTY => {
                        //self.try_insert_characteristic(
                        // data_buffer,
                        // term,
                        // Characteristic::InverseFunctionalProperty)
                        // TODO: Implement
                    }
                    owl::INVERSE_OF => {
                        
                    },
                    // owl::IRREFLEXIVE_PROPERTY => {}
                    // owl::MAX_CARDINALITY => {}
                    // owl::MAX_QUALIFIED_CARDINALITY => {}
                    // owl::MEMBERS => {}
                    // owl::MIN_CARDINALITY => {}
                    // owl::MIN_QUALIFIED_CARDINALITY => {}
                    // owl::NAMED_INDIVIDUAL => {}
                    // owl::NEGATIVE_PROPERTY_ASSERTION => {}
                    owl::NOTHING => {}
                    owl::OBJECT_PROPERTY => {
                    
                        
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
                    owl::TRANSITIVE_PROPERTY => {
                        self.insert_edge(
                            data_buffer,
                            &triple,
                            ElementType::NoDraw);
                        data_buffer.characteristics.insert(data_buffer.elements.len() - 1, Characteristic::Transitive.to_string());
                    }
                    owl::UNION_OF => {
                        self.insert_edge(
                            data_buffer,
                            &triple,
                            ElementType::NoDraw);
                        if let Some(index) = self.resolve(data_buffer, &triple.id.to_string()) {
                            self.upgrade_node_type(data_buffer, index, ElementType::Owl(OwlType::Node(OwlNode::UnionOf)));
                        }
                    }
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
        for (index, (element, label)) in self.blanknode_mapping.iter().enumerate() {
            write!(f, "{index}: {element:?} -> {label}\n")?;
        }
        for (index, (element, label)) in self.object_properties.iter().enumerate() {
            write!(f, "{index}: {element:?} -> {label}\n")?;
        }
        Ok(())
    }
}
