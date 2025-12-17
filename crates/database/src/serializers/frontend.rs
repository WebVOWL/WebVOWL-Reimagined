use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter},
    time::{Duration, Instant},
};

use crate::vocab::owl;
use fluent_uri::Iri;
use futures::StreamExt;
use grapher::prelude::{
    Characteristic, ElementType, GraphDisplayData, OwlEdge, OwlNode, OwlType, RdfEdge, RdfType,
    RdfsEdge, RdfsNode, RdfsType,
};
use log::{debug, error, info, trace, warn};
use oxrdf::vocab::rdf;
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
    /// Stores all resolved elements.
    ///
    /// These elements may mutate during serialization
    /// if new information regarding them is found.
    /// This also means an element can be completely removed!
    ///
    /// - Key = The subject IRI of a triple.
    /// - Value = The ElementType of `Key`.
    element_buffer: HashMap<String, ElementType>,
    /// Keeps track of edges that should point to a node different
    /// from their definition.
    ///
    /// Key
    /// ---
    /// The object IRI of an edge triple.
    ///
    /// The object is also called:
    /// - the target of an edge.
    /// - the range of an edge.
    ///
    /// Value
    /// -----
    /// The subject IRI of an edge triple.
    ///
    /// The subject is also called:
    /// - the source of an edge.
    /// - the domain of an edge.
    ///
    /// Example
    /// -------
    /// Consider the triples:
    /// ```sparql
    ///     ex:Mother owl:equivalentClass ex:blanknode1
    ///     ex:blanknode1 rdf:type owl:Class
    ///     ex:blanknode1 owl:intersectionOf ex:blanknode2
    /// ```
    /// Here `ex:Mother` is equivalent to `ex:blanknode1`,
    /// which means all edges referencing `ex:blanknode1` should
    /// be redirected to `ex:Mother`.
    ///
    /// Thus, the edges are redirected to:
    /// ```sparql
    ///     ex:Mother owl:intersectionOf ex:blanknode2
    /// ```
    /// In this case, `blanknode1` is effectively omitted from serialization.
    edge_redirection: HashMap<String, String>,
    /// IRI to index in data_buffer.
    ///
    /// - Key = The subject OR object IRI
    /// - Value = Index in data_buffer.elements
    iricache: HashMap<String, usize>,
    /// Reverse iricache, namely which IRIs are mapped to each index.
    ///
    /// Used to remap when nodes are merges.
    mapped_to: HashMap<usize, HashSet<String>>,
    /// Maps from element index to the indices of the edges that include it.
    ///
    /// Used to remap when nodes are merges.
    edges_include_map: HashMap<usize, HashSet<usize>>,
    /// Stores indices of element instances.
    ///
    /// Used in cases where multiple elements should refer to a particular instance.
    /// E.g. multiple properties referring to the same instance of owl:Thing.
    global_element_mappings: HashMap<ElementType, usize>,
    /// Stores labels of subject/object.
    ///
    /// - Key = The IRI the label belongs to.
    /// - Value = The label.
    label_buffer: HashMap<String, String>,
    object_properties: HashMap<String, usize>,
    /// Edges in graph, to avoid duplicates
    edges: HashSet<(usize, ElementType, usize)>,
    /// Stores unresolved triples.
    ///
    /// - Key = The subject IRI of the triple
    /// - Value = The unresolved triple.
    unknown_buffer: HashMap<String, Triple>,
    /// Stores triples that are impossible to serialize.
    ///
    /// This could be caused by various reasons, such as
    /// visualization of the triple is not supported.
    ///
    /// Each element is a tuple of:
    /// - 0 = The triple.
    /// - 1 = The reason it failed to serialize.
    failed_buffer: Vec<(Triple, String)>,
    /// The base IRI of the document.
    ///
    /// For instance: `http://purl.obolibrary.org/obo/envo.owl`
    document_base: String,
}

impl GraphDisplayDataSolutionSerializer {
    pub fn new() -> Self {
        Self {
            element_buffer: HashMap::new(),
            edge_redirection: HashMap::new(),
            iricache: HashMap::new(),
            mapped_to: HashMap::new(),
            edges_include_map: HashMap::new(),
            object_properties: HashMap::new(),
            global_element_mappings: HashMap::new(),
            label_buffer: HashMap::new(),
            edges: HashSet::new(),
            unknown_buffer: HashMap::new(),
            failed_buffer: Vec::new(),
            document_base: String::new(),
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
        warn!("Failed to serialize: {:#?}", self.failed_buffer);
        debug!("{}", data_buffer);
        Ok(())
    }

    /// Extract label info from the query solution and store until
    /// they can be mapped to their ElementType.
    fn extract_label(&mut self, label: Option<&Term>, id_term: &Term) {
        let iri = id_term.to_string();

        // Prevent overriding labels
        if self.label_buffer.contains_key(&iri) {
            return;
        }

        match label {
            // Case 1: Label is a rdfs:label OR rdfs:Resource OR rdf:ID
            Some(label) => {
                if label.to_string() != "" {
                    self.label_buffer
                        .insert(id_term.to_string(), label.to_string());
                } else {
                    debug!("Empty label detected for iri '{iri}'");
                }
            }
            // Case 2: Try parsing the iri
            None => {
                // Remove '<' and '>' from iri string to
                // comply with https://www.ietf.org/rfc/rfc3987.html (p. 12)
                let compliant_iri = iri[1..iri.len() - 1].to_string();
                match Iri::parse(compliant_iri) {
                    // Case 2.1: Look for fragments in the iri
                    Ok(id_iri) => match id_iri.fragment() {
                        Some(frag) => {
                            self.label_buffer
                                .insert(id_term.to_string(), frag.to_string());
                        }
                        // Case 2.2: Look for path in iri
                        None => {
                            debug!("No fragment found in iri '{iri}'");
                            match id_iri.path().rsplit_once('/') {
                                Some(path) => {
                                    self.label_buffer
                                        .insert(id_term.to_string(), path.1.to_string());
                                }
                                None => {
                                    debug!("No path found in iri '{iri}'");
                                }
                            }
                        }
                    },
                    Err(e) => {
                        // Do not make a 'warn!'. A parse error is allowed to happen (e.g. on blank nodes).
                        debug!("Failed to parse iri '{}':\n{:?}", iri, e);
                    }
                }
            }
        };
    }

    pub fn resolve(&mut self, data_buffer: &mut GraphDisplayData, x: &String) -> Option<usize> {
        if self.iricache.contains_key(x) {
            return Some(self.iricache[x]);
        } else if self.edge_redirection.contains_key(x) {
            return self.resolve(data_buffer, &self.edge_redirection[x].clone());
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

    pub fn insert_edge_include(
        &mut self,
        data_buffer: &mut GraphDisplayData,
        edge_index: usize,
        element_index: usize,
    ) {
        if self.edges_include_map.contains_key(&element_index) {
            self.edges_include_map
                .get_mut(&element_index)
                .unwrap()
                .insert(edge_index);
        } else {
            self.edges_include_map
                .insert(element_index, HashSet::from([edge_index]));
        }
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

    fn insert_edge(
        &mut self,
        data_buffer: &mut GraphDisplayData,
        triple: &Triple,
        edge_type: ElementType,
    ) {
        let (maybe_sub_idx, maybe_obj_idx) = self.resolve_so(data_buffer, &triple);

        // let mut edge = match edge_type {
        //     ElementType::Owl(OwlType::Edge(OwlEdge::DatatypeProperty)) => self
        //         .handle_missing_edges(
        //             data_buffer,
        //             ElementType::Rdfs(RdfsType::Node(RdfsNode::Literal)),
        //             maybe_sub_idx,
        //             maybe_obj_idx,
        //         ),
        //     _ => self.handle_missing_edges(
        //         data_buffer,
        //         ElementType::Owl(OwlType::Node(OwlNode::Thing)),
        //         maybe_sub_idx,
        //         maybe_obj_idx,
        //     ),
        // };
        match (maybe_sub_idx, maybe_obj_idx) {
            (Some(sub_idx), Some(obj_idx)) => {
                let edge = [sub_idx, data_buffer.elements.len(), obj_idx];
                if !self.edges.contains(&(edge[0], edge_type, edge[2])) {
                    data_buffer.edges.push(edge);

                    data_buffer.elements.push(edge_type);
                    self.edges.insert((edge[0], edge_type, edge[2]));
                    self.insert_edge_include(data_buffer, data_buffer.edges.len() - 1, edge[0]);
                    self.insert_edge_include(data_buffer, data_buffer.edges.len() - 1, edge[2]);
                    self.insert_label(data_buffer, &triple, &edge_type);
                }
            }
            (None, Some(_)) => {
                self.unknown_buffer
                    .insert(triple.target.as_ref().unwrap().to_string(), triple.clone());
                warn!("Cannot resolve subject of triple:\n {}", triple);
            }
            (Some(_), None) => {
                self.unknown_buffer
                    .insert(triple.id.to_string(), triple.clone());
                warn!("Cannot resolve object of triple:\n {}", triple);
            }
            _ => {
                self.unknown_buffer
                    .insert(triple.id.to_string(), triple.clone());
                warn!("Cannot resolve subject and object of triple:\n {}", triple);
            }
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
        if let Some(label) = self.label_buffer.remove(&triple.id.to_string()) {
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

    fn merge_nodes(&mut self, data_buffer: &mut GraphDisplayData, old: usize, new: usize) {
        //let old = data_buffer.labels[old];
        let iter = self.mapped_to.remove(&old);
        match iter {
            Some(iter) => {
                if let map = self.mapped_to.get_mut(&new) {
                    for index in iter {
                        // map.extend(index.clone());
                    }
                } else {
                    self.mapped_to.insert(new, iter.clone());
                }
            }
            None => {}
        }
        let iter = self.edges_include_map.remove(&old);
        match iter {
            Some(iter) => {
                for index in iter {
                    let mut edge = data_buffer.edges[index];
                    if edge[0] == old {
                        edge[0] = new;
                    } else if edge[2] == old {
                        edge[2] = new;
                    }
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
        debug!("upgrading {}", node_type);
        if data_buffer.elements[index] == ElementType::Owl(OwlType::Node(OwlNode::Class)) {
            info!(
                "upgrading from: {} to {}",
                data_buffer.labels[index], node_type
            );
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
        self.edge_redirection.insert(k.to_string(), v.to_string());
        //self.check_insert_unknowns(data_buffer);
    }

    fn extend_label(&mut self, data_buffer: &mut GraphDisplayData, index: usize, label: String) {
        data_buffer.labels[index].push_str(format!("\n{}", label).as_str());
    }

    /// Appends a string to an element's label.
    fn extend_element_label(&mut self, element: String, label_to_append: String) {
        if let Some(label) = self.label_buffer.get_mut(&element) {
            label.push_str(format!("\n{}", label).as_str());
        } else {
            self.label_buffer.insert(element, label_to_append);
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
                    "blanknode" => {
                        self.insert_node(
                            data_buffer,
                            triple,
                            ElementType::Owl(OwlType::Node(OwlNode::AnonymousClass)),
                        );
                    }
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
                        let (index_s, index_o) = self.resolve_so(data_buffer, &triple);
                        match (index_s, index_o) {
                            (Some(index), Some(_)) => {
                                self.upgrade_node_type(
                                    data_buffer,
                                    index,
                                    ElementType::Owl(OwlType::Node(OwlNode::Complement)),
                                );
                                self.insert_edge(data_buffer, &triple, ElementType::NoDraw);
                            }
                            (Some(_), None) => {
                                self.unknown_buffer
                                    .insert(triple.id.to_string(), triple.clone());
                            }
                            (None, Some(_)) => {
                                if let Some(target) = &triple.target {
                                    self.unknown_buffer
                                        .insert(target.to_string(), triple.clone());
                                } else {
                                    warn!("Target is required for: {}", triple);
                                }
                            }
                            _ => {
                                self.unknown_buffer
                                    .insert(triple.id.to_string(), triple.clone());
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
                        self.insert_edge(data_buffer, &triple, ElementType::NoDraw);
                        if let Some(index) = self.resolve(data_buffer, &triple.id.to_string()) {
                            self.upgrade_node_type(
                                data_buffer,
                                index,
                                ElementType::Owl(OwlType::Node(OwlNode::DisjointUnion)),
                            );
                        }
                    }
                    owl::DISJOINT_WITH => self.insert_edge(
                        data_buffer,
                        &triple,
                        ElementType::Owl(OwlType::Edge(OwlEdge::DisjointWith)),
                    ),
                    // owl::DISTINCT_MEMBERS => {}
                    owl::EQUIVALENT_CLASS => {
                        // NOTE: SPARQL query must be sorted as follows:
                        // 1. Classes
                        // 2. Equivalents
                        // ...
                        // n. Properties

                        // REVIEW: Is it possible for the subject to be a blank node??
                        match &triple.target {
                            Some(target) => {
                                // Generally, all equivalent classes should have
                                // their edges redirected.
                                self.edge_redirection
                                    .insert(target.to_string(), triple.id.to_string());

                                if target.is_named_node() {
                                    // Case 1:
                                    // The subject of an equivalentClass relation should
                                    // become a full-fledged equivalent class. This happens
                                    // if the subject and object of the equivalentClass relation
                                    // are both named classes (i.e. not blank nodes).
                                    //
                                    // In other words, the object must be removed from existence,
                                    // and have all references to it (incl. labels) point to
                                    // the subject.
                                    let target_str = target.to_string();

                                    // Move object label to subject.
                                    if let Some(label) = self.label_buffer.remove(&target_str) {
                                        self.extend_element_label(triple.id.to_string(), label);
                                    }

                                    // Remove object from existence.
                                    match self.element_buffer.remove(&target_str) {
                                        // Case 1.1: Object exists in the elememt buffer
                                        Some(_) => {
                                            // REVIEW: Anything that needs to be done here??
                                        }
                                        // Case 1.2: Look in the unknown buffer
                                        None => match self.unknown_buffer.remove(&target_str) {
                                            Some(_) => {
                                                // REVIEW: Anything that needs to be done here??
                                            }
                                            None => {
                                                self.failed_buffer.push((triple, "Failed to merge object of equivalence relation into subject: object was not found".to_string()));
                                                return;
                                            }
                                        },
                                    }

                                    // Upgrade subject to a full-fledged equivalent class
                                    let new_element =
                                        ElementType::Owl(OwlType::Node(OwlNode::EquivalentClass));
                                    match self
                                        .element_buffer
                                        .insert(triple.id.to_string(), new_element)
                                    {
                                        Some(old_elem) => {
                                            debug!(
                                                "Upgraded subject '{}' from {} to {}",
                                                triple.id, old_elem, new_element
                                            )
                                        }
                                        None => {
                                            warn!(
                                                "Upgraded unresolved subject '{}' to {}",
                                                triple.id, new_element
                                            )
                                        }
                                    }
                                } else if target.is_blank_node() {
                                    // Case 2:
                                    // The subject of an equivalentClass relation should
                                    // remain unchanged. This happens if the object of the
                                    // equivalentClass relation is a blank node.
                                    //
                                    // In other words, nothing to do here.
                                } else {
                                    self.failed_buffer.push((triple, "Visualization of equivalence relations between classes and literals is not supported".to_string()));
                                }
                            }
                            None => {
                                self.failed_buffer.push((
                                    triple,
                                    "Subject of equivalence relation is missing an object"
                                        .to_string(),
                                ));
                            }
                        }

                        // info!("blanknode_mapping | {:?}", self.edge_redirection);
                        // info!("mapped_to | {:?}", self.mapped_to);
                        // info!("iricache | {:?}", self.iricache);
                        // info!("triple: {}", triple);
                        // let (index_s, index_o) = self.resolve_so(data_buffer, &triple);
                        // this should work when reintroducing unknown buffer
                        // TODO: We should rework how unknowns are handled
                        // especially blank nodes.
                        // Instead of working directly with the databuffer, we should keep a temporary
                        // datastructure, which we can mutate while serializing, then converting said
                        // structure to the databuffer at the end. This will allow us to handle merging/unknowns gracefully.
                        // this will also allow us to use blanknode mapping (should be renamed), in conjunction with
                        // iricache to resolve.
                        // An example:
                        // Mother : equivalentClass : blanknode1
                        // blanknode1 : rdf:type : owl:Class
                        // blanknode1 : owl:intersectionOf : blanknode2
                        // blanknode2 : collection (which we flatten) : blanknode3
                        // blanknode3 : owl:intersectionOf : Parent
                        // blanknode3 : owl:intersectionOf : Warden
                        // etc.
                        // - When we first meet blanknode1: add to unknown
                        //

                        // When that's done, there are two cases to consider:
                        // -
                        // - The subject of an equivalentClass relation should
                        //   remain unchanged. This happens if the the object of the
                        //   equivalentClass relation is a blank node.

                        // match (index_s, index_o) {
                        //     (Some(index_s), Some(index_o)) => {
                        //         self.merge_nodes(data_buffer, index_o, index_s);
                        //         if !data_buffer.elements[index_s]
                        //             .eq(&ElementType::Owl(OwlType::Node(OwlNode::AnonymousClass)))
                        //         {
                        //             self.upgrade_node_type(
                        //                 data_buffer,
                        //                 index_s,
                        //                 ElementType::Owl(OwlType::Node(OwlNode::EquivalentClass)),
                        //             );
                        //             self.extend_label(
                        //                 data_buffer,
                        //                 index_s,
                        //                 data_buffer.labels[index_o].clone(),
                        //             );
                        //             self.map_to(
                        //                 data_buffer,
                        //                 data_buffer.labels[index_o].clone(),
                        //                 index_s,
                        //             );
                        //         } else {
                        //             self.map_to(
                        //                 data_buffer,
                        //                 data_buffer.labels[index_o].clone(),
                        //                 index_s,
                        //             );
                        //         }
                        //     }
                        //     _ => {
                        //         //self.unknown_buffer.insert(triple.clone());
                        //         warn!("Target is required for: {}", triple);
                        //     }
                        // }

                        // This works but i dont like it
                        // match triple.target.as_ref() {
                        //     Some(Term::NamedNode(nn)) => {
                        //         let target_index = self.resolve(data_buffer, &nn.to_string());
                        //         match target_index {
                        //             Some(target_index) => {
                        //                 info!("edges_include_map: {:?}", self.edges_include_map.get(&target_index));
                        //                 self.replace_node(data_buffer, target_index, index);
                        //                 self.upgrade_node_type(data_buffer, index, ElementType::Owl(OwlType::Node(OwlNode::EquivalentClass)));
                        //                 self.extend_label(data_buffer, index, data_buffer.labels[target_index].clone());
                        //                 self.map_to(data_buffer, nn.to_string(), index);

                        //             }
                        //             None => {
                        //                 warn!("Target is required for: {}", triple);
                        //             }
                        //         }
                        //     }
                        //     Some(Term::BlankNode(bn)) => {
                        //         let target_index = self.resolve(data_buffer, &bn.to_string());
                        //         match target_index {
                        //             Some(target_index) => {
                        //                 self.replace_node(data_buffer, target_index, index);
                        //                 self.map_to(data_buffer, bn.to_string(), target_index);
                        //             }
                        //             None => {
                        //                 warn!("Target is required for: {}", triple);
                        //             }
                        //         }
                        //     }
                        //     _ => {
                        //         warn!("Target is required for: {}", triple);
                        //     }
                        // }
                        // let target_index =
                        //     self.resolve(data_buffer, &triple.target.as_ref().unwrap().to_string());
                    }
                    // owl::EQUIVALENT_PROPERTY => {}
                    // owl::FUNCTIONAL_PROPERTY => {}
                    // owl::HAS_KEY => {}
                    // owl::HAS_SELF => {}
                    // owl::HAS_VALUE => {}
                    // owl::IMPORTS => {}
                    // owl::INCOMPATIBLE_WITH => {}
                    owl::INTERSECTION_OF => {
                        self.insert_edge(data_buffer, &triple, ElementType::NoDraw);
                        if let Some(index) = self.resolve(data_buffer, &triple.id.to_string()) {
                            self.upgrade_node_type(
                                data_buffer,
                                index,
                                ElementType::Owl(OwlType::Node(OwlNode::IntersectionOf)),
                            );
                        }
                    }
                    owl::INVERSE_FUNCTIONAL_PROPERTY => {
                        //self.try_insert_characteristic(
                        // data_buffer,
                        // term,
                        // Characteristic::InverseFunctionalProperty)
                        // TODO: Implement
                    }
                    owl::INVERSE_OF => {}
                    // owl::IRREFLEXIVE_PROPERTY => {}
                    // owl::MAX_CARDINALITY => {}
                    // owl::MAX_QUALIFIED_CARDINALITY => {}
                    // owl::MEMBERS => {}
                    // owl::MIN_CARDINALITY => {}
                    // owl::MIN_QUALIFIED_CARDINALITY => {}
                    // owl::NAMED_INDIVIDUAL => {}
                    // owl::NEGATIVE_PROPERTY_ASSERTION => {}
                    owl::NOTHING => {}
                    owl::OBJECT_PROPERTY => {}
                    // owl::ONE_OF => {}
                    owl::ONTOLOGY => {
                        // TODO: Base must be known before matching.
                        // Make it a separete variable in the query.
                        // self.doc_iri = uri.to_string();
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
                        self.insert_edge(data_buffer, &triple, ElementType::NoDraw);
                        data_buffer.characteristics.insert(
                            data_buffer.elements.len() - 1,
                            Characteristic::Transitive.to_string(),
                        );
                    }
                    owl::UNION_OF => {
                        self.insert_edge(data_buffer, &triple, ElementType::NoDraw);
                        if let Some(index) = self.resolve(data_buffer, &triple.id.to_string()) {
                            self.upgrade_node_type(
                                data_buffer,
                                index,
                                ElementType::Owl(OwlType::Node(OwlNode::UnionOf)),
                            );
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
        for (index, (element, label)) in self.edge_redirection.iter().enumerate() {
            write!(f, "{index}: {element:?} -> {label}\n")?;
        }
        for (index, (element, label)) in self.object_properties.iter().enumerate() {
            write!(f, "{index}: {element:?} -> {label}\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[allow(unused_must_use)]
mod test {
    use super::*;
    use oxrdf::NamedNode;

    #[test]
    fn test_replace_node() {
        let _ = env_logger::builder().is_test(true).try_init();
        let mut serializer = GraphDisplayDataSolutionSerializer::new();
        let mut data_buffer = GraphDisplayData::new();
        serializer.write_node_triple(
            &mut data_buffer,
            Triple {
                id: Term::NamedNode(NamedNode::new("http://example.com#").unwrap()),
                element_type: Term::NamedNode(
                    NamedNode::new("http://www.w3.org/2002/07/owl#Ontology").unwrap(),
                ),
                target: None,
            },
        );
        serializer.write_node_triple(
            &mut data_buffer,
            Triple {
                id: Term::NamedNode(NamedNode::new("http://example.com#Parent").unwrap()),
                element_type: Term::NamedNode(
                    NamedNode::new("http://www.w3.org/2002/07/owl#Class").unwrap(),
                ),
                target: None,
            },
        );
        serializer.write_node_triple(
            &mut data_buffer,
            Triple {
                id: Term::NamedNode(NamedNode::new("http://example.com#Mother").unwrap()),
                element_type: Term::NamedNode(
                    NamedNode::new("http://www.w3.org/2002/07/owl#Class").unwrap(),
                ),
                target: None,
            },
        );
        serializer.write_node_triple(
            &mut data_buffer,
            Triple {
                id: Term::NamedNode(NamedNode::new("http://example.com#Guardian").unwrap()),
                element_type: Term::NamedNode(
                    NamedNode::new("http://www.w3.org/2002/07/owl#Class").unwrap(),
                ),
                target: None,
            },
        );
        serializer.write_node_triple(
            &mut data_buffer,
            Triple {
                id: Term::NamedNode(NamedNode::new("http://example.com#Warden").unwrap()),
                element_type: Term::NamedNode(
                    NamedNode::new("http://www.w3.org/2002/07/owl#Class").unwrap(),
                ),
                target: None,
            },
        );
        serializer.write_node_triple(
            &mut data_buffer,
            Triple {
                id: Term::NamedNode(NamedNode::new("http://example.com#Warden").unwrap()),
                element_type: Term::NamedNode(
                    NamedNode::new("http://www.w3.org/2000/01/rdf-schema#subClassOf").unwrap(),
                ),
                target: Some(Term::NamedNode(
                    NamedNode::new("http://example.com#Guardian").unwrap(),
                )),
            },
        );
        serializer.write_node_triple(
            &mut data_buffer,
            Triple {
                id: Term::NamedNode(NamedNode::new("http://example.com#Mother").unwrap()),
                element_type: Term::NamedNode(
                    NamedNode::new("http://www.w3.org/2000/01/rdf-schema#subClassOf").unwrap(),
                ),
                target: Some(Term::NamedNode(
                    NamedNode::new("http://example.com#Parent").unwrap(),
                )),
            },
        );
        serializer.write_node_triple(
            &mut data_buffer,
            Triple {
                id: Term::NamedNode(NamedNode::new("http://example.com#Warden").unwrap()),
                element_type: Term::NamedNode(
                    NamedNode::new("http://www.w3.org/2002/07/owl#unionOf").unwrap(),
                ),
                target: Some(Term::NamedNode(
                    NamedNode::new("http://example.com#Guardian").unwrap(),
                )),
            },
        );

        print_graph_display_data(&data_buffer);
        println!("--------------------------------");

        let triple = Triple {
            id: Term::NamedNode(NamedNode::new("http://example.com#Guardian").unwrap()),
            element_type: Term::NamedNode(
                NamedNode::new("http://www.w3.org/2002/07/owl#equivalentClass").unwrap(),
            ),
            target: Some(Term::NamedNode(
                NamedNode::new("http://example.com#Warden").unwrap(),
            )),
        };
        serializer.write_node_triple(&mut data_buffer, triple);
        print_graph_display_data(&data_buffer);
        println!("data_buffer: {}", data_buffer);
        println!("serializer: {}", serializer);
    }

    pub fn print_graph_display_data(data_buffer: &GraphDisplayData) {
        for (index, (element, label)) in data_buffer
            .elements
            .iter()
            .zip(data_buffer.labels.iter())
            .enumerate()
        {
            println!("{index}: {label} -> {element:?}");
        }
        for edge in data_buffer.edges.iter() {
            println!(
                "{} -> {:?} -> {}",
                data_buffer.labels[edge[0]],
                data_buffer.elements[edge[1]],
                data_buffer.labels[edge[2]]
            );
        }
    }
}
