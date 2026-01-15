use std::{
    collections::HashSet,
    ops::DerefMut,
    time::{Duration, Instant},
};

use super::{Edge, SerializationDataBuffer, Triple};
use crate::vocab::owl;
use fluent_uri::Iri;
use futures::StreamExt;
use grapher::prelude::{
    Characteristic, ElementType, GraphDisplayData, OwlEdge, OwlNode, OwlType, RdfEdge, RdfType,
    RdfsEdge, RdfsNode, RdfsType,
};
use log::{debug, info, warn};
use oxrdf::{BlankNode, NamedNode, vocab::rdf};
use rdf_fusion::{
    execution::results::QuerySolutionStream,
    model::{Term, vocab::rdfs},
};
use vowlr_parser::errors::WebVowlStoreError;

pub struct GraphDisplayDataSolutionSerializer {}

impl GraphDisplayDataSolutionSerializer {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn serialize_nodes_stream(
        &self,
        data: &mut GraphDisplayData,
        mut solution_stream: QuerySolutionStream,
    ) -> Result<(), WebVowlStoreError> {
        let mut count: u32 = 0;
        info!("Serializing query solution stream...");
        let start_time = Instant::now();
        let mut data_buffer = SerializationDataBuffer::new();
        while let Some(solution) = solution_stream.next().await {
            let solution = solution?;
            let Some(id_term) = solution.get("id") else {
                continue;
            };
            let Some(node_type_term) = solution.get("nodeType") else {
                continue;
            };

            self.extract_label(&mut data_buffer, solution.get("label"), id_term);

            let triple: Triple = Triple {
                id: id_term.to_owned(),
                element_type: node_type_term.to_owned(),
                target: solution.get("target").map(|term| term.to_owned()),
            };
            self.write_node_triple(&mut data_buffer, triple);
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
            data_buffer.element_buffer.len(),
            data_buffer.edge_buffer.len(),
            data_buffer.label_buffer.len(),
            data_buffer.edge_characteristics.len() + data_buffer.node_characteristics.len(),
            0
        );
        if !data_buffer.failed_buffer.is_empty() {
            warn!("Failed to serialize: {:#?}", data_buffer.failed_buffer);
        }
        debug!("{}", data_buffer);
        *data = data_buffer.into();
        Ok(())
    }

    /// Extract label info from the query solution and store until
    /// they can be mapped to their ElementType.
    fn extract_label(
        &self,
        data_buffer: &mut SerializationDataBuffer,
        label: Option<&Term>,
        id_term: &Term,
    ) {
        let iri = id_term.to_string();

        // Prevent overriding labels
        if data_buffer.label_buffer.contains_key(&iri) {
            return;
        }

        match label {
            // Case 1: Label is a rdfs:label OR rdfs:Resource OR rdf:ID
            Some(label) => {
                if label.to_string() != "" {
                    data_buffer
                        .label_buffer
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
                            data_buffer
                                .label_buffer
                                .insert(id_term.to_string(), frag.to_string());
                        }
                        // Case 2.2: Look for path in iri
                        None => {
                            debug!("No fragment found in iri '{iri}'");
                            match id_iri.path().rsplit_once('/') {
                                Some(path) => {
                                    data_buffer
                                        .label_buffer
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

    pub fn resolve(
        &self,
        data_buffer: &mut SerializationDataBuffer,
        mut x: String,
    ) -> Option<String> {
        if data_buffer.element_buffer.contains_key(&x) {
            info!(
                "resolved: {}: {}",
                x,
                data_buffer.element_buffer.get(&x).unwrap()
            );
            return Some(x);
        }
        while let Some(redirected) = data_buffer.edge_redirection.get(&x) {
            let new_x = redirected.clone();
            if data_buffer.element_buffer.contains_key(&new_x) {
                info!(
                    "resolved: {}: {}",
                    new_x,
                    data_buffer.element_buffer.get(&new_x).unwrap()
                );
                return Some(new_x);
            }
            info!("checked: {} ", x);
            x = new_x;
        }
        None
    }
    pub fn resolve_so(
        &self,
        data_buffer: &mut SerializationDataBuffer,
        triple: &Triple,
    ) -> (Option<String>, Option<String>) {
        let resolved_subject = self.resolve(data_buffer, triple.id.to_string());
        let resolved_object = match &triple.target {
            Some(target) => self.resolve(data_buffer, target.to_string()),
            None => {
                warn!("Cannot resolve object of triple:\n {}", triple);
                info!("triple: {}", triple);
                for (k, v) in data_buffer.unknown_buffer.iter() {
                    info!("unknown: {} -> {}", k, v);
                }
                info!("edge_redirection: ");
                for (k, v) in data_buffer.edge_redirection.iter() {
                    info!("edge_redirection: {} -> {}", k, v);
                }
                None
            }
        };
        (resolved_subject, resolved_object)
    }

    /// Insert an edge into the element's edge set.
    pub fn insert_edge_include(
        &self,
        data_buffer: &mut SerializationDataBuffer,
        element_iri: String,
        edge: Edge,
    ) {
        if data_buffer.edges_include_map.contains_key(&element_iri) {
            data_buffer
                .edges_include_map
                .get_mut(&element_iri)
                .unwrap()
                .insert(edge);
        } else {
            data_buffer
                .edges_include_map
                .insert(element_iri, HashSet::from([edge]));
        }
    }

    fn insert_node(
        &self,
        data_buffer: &mut SerializationDataBuffer,
        triple: Triple,
        node_type: ElementType,
    ) {
        // Skip insertion if this node was already merged into another node
        if data_buffer
            .edge_redirection
            .contains_key(&triple.id.to_string())
        {
            debug!(
                "Skipping insert_node for '{}': already redirected",
                triple.id
            );
            return;
        }

        data_buffer
            .element_buffer
            .insert(triple.id.to_string(), node_type);

        if let Some(triple) = data_buffer.unknown_buffer.remove(&triple.id.to_string()) {
            info!("checking triple in unknown buffer: {}", triple);
            self.write_node_triple(data_buffer, triple);
        } else if let Some(target) = triple.target {
            if let Some(triple) = data_buffer.unknown_buffer.remove(&target.to_string()) {
                info!("checking triple in unknown buffer: {}", triple);
                self.write_node_triple(data_buffer, triple);
            }
        }
    }

    fn insert_edge(
        &self,
        data_buffer: &mut SerializationDataBuffer,
        triple: &Triple,
        edge_type: ElementType,
    ) -> Option<Edge> {
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
            (Some(sub_iri), Some(obj_iri)) => {
                let edge = Edge {
                    subject: sub_iri.clone(),
                    element_type: edge_type,
                    object: obj_iri.clone(),
                };
                data_buffer.edge_buffer.insert(edge.clone());
                self.insert_edge_include(data_buffer, sub_iri, edge.clone());
                self.insert_edge_include(data_buffer, obj_iri, edge.clone());
                data_buffer
                    .edge_label_buffer
                    .insert(edge.clone(), edge_type.to_string());
                Some(edge)
            }
            (None, Some(obj_iri)) => {
                warn!("Cannot resolve subject of triple:\n {}", triple);
                data_buffer.unknown_buffer.insert(obj_iri, triple.clone());
                None
            }
            (Some(sub_iri), None) => {
                data_buffer.unknown_buffer.insert(sub_iri, triple.clone());
                warn!("Cannot resolve object of triple:\n {}", triple);
                None
            }
            _ => {
                data_buffer
                    .unknown_buffer
                    .insert(triple.id.to_string(), triple.clone());
                None
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
        // if let Some(label) = self.label_buffer.remove(&triple.id.to_string()) {
        //     data_buffer.labels.push(label);
        // } else {
        //     // Fallback label as all elements must have a label.
        //     data_buffer.labels.push(element_type.to_string());
        // }
    }

    // /// Creates nodes as targets for edges without one in the solution.
    // ///
    // /// If no domain and/or range axiom is defined for a property, owl:Thing is used as domain and/or range.
    // /// EXCEPT datatype properties without a defined range. Here, rdfs:Literal is used as range instead.
    // ///
    // /// Procedure:
    // /// If NOT rdfs:Datatype:
    // /// - Property missing domain OR range:
    // ///   - Create new owl:Thing for this property.
    // /// - Property missing domain AND range:
    // ///   - Create a new owl:Thing (if not created previously) and use this instance for ALL edges in this category.
    // /// IF rdfs:Datatype:
    // /// - Property missing domain AND/OR range:
    // ///   - Create new rdfs:Literal for this property
    // fn handle_missing_edges(
    //     &mut self,
    //     data_buffer: &mut GraphDisplayData,
    //     node_to_create: ElementType,
    //     maybe_sub_idx: Option<usize>,
    //     maybe_obj_idx: Option<usize>,
    // ) -> [usize; 3] {
    //     // Case: missing domain AND range
    //     // if maybe_sub_idx.is_none() && maybe_obj_idx.is_none() {
    //     //     if let Some(global_idx) = self.global_element_mappings.get(&node_to_create) {
    //     //         [*global_idx, 0, *global_idx]
    //     //     } else {
    //     //         // Create global node type
    //     //         let global_idx = self.insert_global(data_buffer, node_to_create);
    //     //         [global_idx, 0, global_idx]
    //     //     }
    //     // // Case: missing domain OR range
    //     // } else {
    //     //     let sub_idx = maybe_sub_idx.unwrap_or(self.insert_local(data_buffer, node_to_create));
    //     //     let obj_idx = maybe_obj_idx.unwrap_or(self.insert_local(data_buffer, node_to_create));
    //     //     [sub_idx, 0, obj_idx]
    //     // }
    // }

    // /// Insert an ElementType into the global element mapping.
    // ///
    // /// May only be called once for each ElementType!
    // ///
    // /// Use [`insert_local`] if multiple calls are required for each ElementType.
    // fn insert_global(
    //     &mut self,
    //     data_buffer: &mut GraphDisplayData,
    //     element_to_create: ElementType,
    // ) -> usize {
    //     // let elem_idx = data_buffer.elements.len();
    //     // self.global_element_mappings
    //     //     .insert(element_to_create, elem_idx);
    //     // data_buffer.labels.push(element_to_create.to_string());
    //     // data_buffer.elements.push(element_to_create);
    //     // elem_idx
    // }

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

    fn merge_nodes(&self, data_buffer: &mut SerializationDataBuffer, old: String, new: String) {
        data_buffer.element_buffer.remove(&old);
        self.update_edges(data_buffer, &old, &new);
        data_buffer.edge_redirection.insert(old.to_string(), new);
    }

    fn update_edges(&self, data_buffer: &mut SerializationDataBuffer, old: &String, new: &String) {
        let old_edges = data_buffer.edges_include_map.remove(old);
        if let Some(old_edges) = old_edges {
            info!("old_edges: ");
            for edge in old_edges.iter() {
                info!("edge: {} ", edge);
            }

            for mut edge in old_edges.into_iter() {
                data_buffer.edge_buffer.remove(&edge);
                if edge.object == *old {
                    edge.object = new.clone();
                }
                if edge.subject == *old {
                    edge.subject = new.clone();
                }
                data_buffer
                    .edges_include_map
                    .get_mut(new)
                    .unwrap()
                    .insert(edge.clone());
                data_buffer.edge_buffer.insert(edge);
            }
            info!("new_edges: ");
            for edge in data_buffer.edge_buffer.iter() {
                info!("edge: {} ", edge);
            }
        }
    }

    fn upgrade_node_type(
        &self,
        data_buffer: &mut SerializationDataBuffer,
        iri: String,
        new_element: ElementType,
    ) {
        let old_elem_opt = data_buffer.element_buffer.get(&iri).cloned();
        match old_elem_opt {
            Some(old_elem) => {
                if old_elem == ElementType::Owl(OwlType::Node(OwlNode::Class)) {
                    data_buffer.element_buffer.insert(iri.clone(), new_element);
                }
                debug!(
                    "Upgraded subject '{}' from {} to {}",
                    iri, old_elem, new_element
                )
            }
            None => {
                warn!("Upgraded unresolved subject '{}' to {}", iri, new_element)
            }
        }
    }

    /// Appends a string to an element's label.
    fn extend_element_label(
        &self,
        data_buffer: &mut SerializationDataBuffer,
        element: String,
        label_to_append: String,
    ) {
        info!(
            "extending element: {} with label: {}",
            element, label_to_append
        );
        if let Some(label) = data_buffer.label_buffer.get_mut(&element) {
            label.push_str(format!("\n{}", label_to_append).as_str());
            info!("appended to: {}", label);
        } else {
            data_buffer
                .label_buffer
                .insert(element.clone(), label_to_append.clone());
            info!("inserted k v: {} -> {}", element, label_to_append);
        }
    }

    fn write_node_triple(&self, data_buffer: &mut SerializationDataBuffer, triple: Triple) {
        // TODO: Collect errors and show to frontend
        let node_type = triple.element_type.clone();
        let test_triple = triple.clone();
        info!("{}", test_triple);
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
                    rdfs::RESOURCE => {
                        self.insert_node(
                            data_buffer,
                            triple,
                            ElementType::Rdfs(RdfsType::Node(RdfsNode::Resource)),
                        );
                    }
                    // rdfs::SEE_ALSO => {}
                    rdfs::SUB_CLASS_OF => {
                        self.insert_edge(
                            data_buffer,
                            &triple,
                            ElementType::Rdfs(RdfsType::Edge(RdfsEdge::SubclassOf)),
                        );
                    }
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
                            (Some(index), Some(target)) => {
                                self.upgrade_node_type(
                                    data_buffer,
                                    target,
                                    ElementType::Owl(OwlType::Node(OwlNode::Complement)),
                                );
                                self.insert_edge(data_buffer, &triple, ElementType::NoDraw);
                            }
                            (Some(_), None) => {
                                data_buffer
                                    .unknown_buffer
                                    .insert(triple.id.to_string(), triple.clone());
                            }
                            (None, Some(_)) => {
                                if let Some(target) = &triple.target {
                                    data_buffer
                                        .unknown_buffer
                                        .insert(target.to_string(), triple.clone());
                                } else {
                                    warn!("Target is required for: {}", triple);
                                }
                            }
                            _ => {
                                data_buffer
                                    .unknown_buffer
                                    .insert(triple.id.to_string(), triple.clone());
                            }
                        }
                    }
                    owl::DATATYPE_COMPLEMENT_OF => {}
                    owl::DATATYPE_PROPERTY => {
                        self.insert_edge(
                            data_buffer,
                            &triple,
                            ElementType::Owl(OwlType::Edge(OwlEdge::DatatypeProperty)),
                        );
                    }
                    // owl::DATA_RANGE => {}
                    // owl::DEPRECATED => {}
                    owl::DEPRECATED_CLASS => self.insert_node(
                        data_buffer,
                        triple,
                        ElementType::Owl(OwlType::Node(OwlNode::DeprecatedClass)),
                    ),
                    owl::DEPRECATED_PROPERTY => {
                        self.insert_edge(
                            data_buffer,
                            &triple,
                            ElementType::Owl(OwlType::Edge(OwlEdge::DeprecatedProperty)),
                        );
                    }
                    // owl::DIFFERENT_FROM => {}
                    owl::DISJOINT_UNION_OF => {
                        self.insert_edge(data_buffer, &triple, ElementType::NoDraw);
                        if let Some(_) = triple.target {
                            if let Some(index) = self.resolve(data_buffer, triple.id.to_string()) {
                                self.upgrade_node_type(
                                    data_buffer,
                                    index,
                                    ElementType::Owl(OwlType::Node(OwlNode::DisjointUnion)),
                                );
                            }
                        }
                    }
                    owl::DISJOINT_WITH => {
                        self.insert_edge(
                            data_buffer,
                            &triple,
                            ElementType::Owl(OwlType::Edge(OwlEdge::DisjointWith)),
                        );
                    }
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

                                if target.is_named_node() {
                                    debug!("is named node");
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
                                    if let Some(label) =
                                        data_buffer.label_buffer.remove(&target_str)
                                    {
                                        info!("removed label: {}", label);
                                        self.extend_element_label(
                                            data_buffer,
                                            triple.id.to_string(),
                                            label,
                                        );
                                    }

                                    // Remove object from existence.
                                    match data_buffer.element_buffer.remove(&target_str) {
                                        // Case 1.1: Object exists in the elememt buffer
                                        Some(_) => {
                                            self.merge_nodes(
                                                data_buffer,
                                                target_str,
                                                triple.id.to_string(),
                                            );
                                            // REVIEW: Anything that needs to be done here??
                                        }
                                        // Case 1.2: Look in the unknown buffer
                                        None => {
                                            match data_buffer.unknown_buffer.remove(&target_str) {
                                                Some(_) => {
                                                    // REVIEW: Anything that needs to be done here??
                                                }
                                                None => {
                                                    data_buffer.failed_buffer.push((triple, "Failed to merge object of equivalence relation into subject: object was not found".to_string()));
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                    self.upgrade_node_type(
                                        data_buffer,
                                        triple.id.to_string(),
                                        ElementType::Owl(OwlType::Node(OwlNode::EquivalentClass)),
                                    );
                                } else if target.is_blank_node() {
                                    // Case 2:
                                    // The subject of an equivalentClass relation should
                                    // could either be start of a collection or anon class
                                    debug!("is blank node");
                                    let (index_s, index_o) = self.resolve_so(data_buffer, &triple);
                                    match (index_s, index_o) {
                                        (Some(index_s), Some(index_o)) => {
                                            debug!("Some, Some -> merging");
                                            info!("merging nodes: {} -> {}", index_o, index_s);
                                            self.merge_nodes(data_buffer, index_o, index_s);
                                        }
                                        (Some(index_s), None) => {
                                            debug!("Some, None -> redirecting");
                                            data_buffer.edge_redirection.insert(
                                                triple.target.unwrap().to_string(),
                                                index_s,
                                            );
                                        }
                                        _ => {
                                            debug!("None -> unknown buffer");
                                            data_buffer
                                                .unknown_buffer
                                                .insert(target.to_string(), triple);
                                        }
                                    }
                                } else {
                                    data_buffer.failed_buffer.push((triple, "Visualization of equivalence relations between classes and literals is not supported".to_string()));
                                }
                            }
                            None => {
                                data_buffer.failed_buffer.push((
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
                        if let Some(index) = self.resolve(data_buffer, triple.id.to_string()) {
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
                        match self.insert_edge(data_buffer, &triple, ElementType::NoDraw) {
                            Some(edge) => {
                                data_buffer
                                    .edge_characteristics
                                    .insert(edge, vec![Characteristic::Transitive.to_string()]);
                            }
                            _ => {}
                        }
                    }
                    owl::UNION_OF => {
                        self.insert_edge(data_buffer, &triple, ElementType::NoDraw);
                        if let Some(index) = self.resolve(data_buffer, triple.id.to_string()) {
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

#[cfg(test)]
#[allow(unused_must_use)]
mod test {
    use super::*;
    use oxrdf::{Literal, NamedNode};

    #[test]
    fn test_replace_node() {
        let _ = env_logger::builder().is_test(true).try_init();
        let serializer = GraphDisplayDataSolutionSerializer {};
        let mut data_buffer = SerializationDataBuffer::new();
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
                id: Term::NamedNode(NamedNode::new("http://example.com#Warden1").unwrap()),
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
                id: Term::BlankNode(BlankNode::new("e1013e66f734c508511575854b0c9396").unwrap()),
                element_type: Term::Literal(Literal::new_simple_literal("blanknode".to_string())),
                target: None,
            },
        );
        serializer.write_node_triple(
            &mut data_buffer,
            Triple {
                id: Term::NamedNode(NamedNode::new("http://example.com#Warden1").unwrap()),
                element_type: Term::NamedNode(
                    NamedNode::new("http://www.w3.org/2002/07/owl#unionOf").unwrap(),
                ),
                target: Some(Term::NamedNode(
                    NamedNode::new("http://example.com#Warden").unwrap(),
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
        for (k, v) in data_buffer.element_buffer.iter() {
            println!("element_buffer: {} -> {}", k, v);
        }
        for (k, v) in data_buffer.edges_include_map.iter() {
            println!("edges_include_map: {} -> {:?}", k, v);
        }
        for (k, v) in data_buffer.edge_redirection.iter() {
            println!("edge_redirection: {} -> {}", k, v);
        }
        assert!(
            data_buffer
                .element_buffer
                .contains_key("<http://example.com#Guardian>")
        );
        assert!(
            !data_buffer
                .element_buffer
                .contains_key("<http://example.com#Warden>")
        );
        assert!(
            data_buffer
                .element_buffer
                .contains_key("<http://example.com#Warden1>")
        );
        assert!(
            data_buffer
                .edges_include_map
                .contains_key("<http://example.com#Warden1>")
        );
        assert!(
            *data_buffer
                .edge_redirection
                .get("<http://example.com#Warden>")
                .unwrap()
                == "<http://example.com#Guardian>".to_string()
        );
        assert!(data_buffer.edge_buffer.contains(&Edge {
            subject: "<http://example.com#Warden1>".to_string(),
            element_type: ElementType::NoDraw,
            object: "<http://example.com#Guardian>".to_string()
        }));
        assert!(
            data_buffer
                .edge_redirection
                .contains_key("<http://example.com#Warden>")
        );
        assert_eq!(
            data_buffer
                .edge_redirection
                .get("<http://example.com#Warden>")
                .unwrap(),
            "<http://example.com#Guardian>"
        );
        serializer.write_node_triple(
            &mut data_buffer,
            Triple {
                id: Term::NamedNode(NamedNode::new("http://example.com#Guardian").unwrap()),
                element_type: Term::NamedNode(
                    NamedNode::new("http://www.w3.org/2002/07/owl#equivalentClass").unwrap(),
                ),
                target: Some(Term::BlankNode(
                    BlankNode::new("e1013e66f734c508511575854b0c9396").unwrap(),
                )),
            },
        );
        let s = serializer.resolve(
            &mut data_buffer,
            "_:e1013e66f734c508511575854b0c9396".to_string(),
        );
        assert!(s.is_some());
        for (k, v) in data_buffer.element_buffer.iter() {
            println!("element_buffer: {} -> {}", k, v);
        }
        for (k, v) in data_buffer.edge_redirection.iter() {
            println!("edge_redirection: {} -> {}", k, v);
        }
        assert!(s.unwrap() == "<http://example.com#Guardian>".to_string());
        assert!(
            !data_buffer
                .edges_include_map
                .contains_key("_:e1013e66f734c508511575854b0c9396")
        );
        assert!(!data_buffer.edges_include_map.contains_key("Warden"));
        print_graph_display_data(&data_buffer);
        println!("data_buffer: {}", data_buffer);
    }

    pub fn print_graph_display_data(data_buffer: &SerializationDataBuffer) {
        for (index, (element, label)) in data_buffer.element_buffer.iter().enumerate() {
            println!("{index}: {label} -> {element:?}");
        }
        for edge in data_buffer.edge_buffer.iter() {
            println!(
                "{} -> {:?} -> {}",
                edge.subject, edge.element_type, edge.object
            );
        }
    }
}
