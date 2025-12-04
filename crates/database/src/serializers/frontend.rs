use std::collections::HashMap;

use crate::serializers::formats::graph_display::GraphDisplayData;
use crate::vocab::owl;
use fluent_uri::Iri;
use grapher::web::prelude::{
    Characteristic, ElementType, GenericNode, GenericType, OwlEdge, OwlNode, OwlType, RdfEdge,
    RdfNode, RdfType, RdfsEdge, RdfsNode, RdfsType,
};
use log::{info, warn};
use rdf_fusion::model::{
    BlankNodeRef, TermRef, VariableRef,
    vocab::{rdf, rdfs, xsd},
};
use smallvec::SmallVec;

// TODO: Use the structure of RDF Fusion's JSON serializer for this module
pub struct GraphDisplayDataSolutionSerializer {
    /// Maps a term to an index in the `elements` field of [`GraphDisplayData`].
    term_indices: HashMap<&TermRef<'a>, usize>,
    /// Maps a variable to its term.
    variable_terms: HashMap<String, &TermRef<'a>>,
}

impl GraphDisplayDataSolutionSerializer {
    pub fn new() -> Self {
        Self {
            term_indices: HashMap::new(),
            variable_terms: HashMap::new(),
        }
    }

    pub fn serialize<'a>(
        &mut self,
        data_buffer: &mut GraphDisplayData,
        solution: impl IntoIterator<Item = (VariableRef<'a>, TermRef<'a>)>,
    ) {
        let mut knowns: Vec<&TermRef<'a>> = Vec::with_capacity(8);
        let mut unknowns: Vec<&TermRef<'a>> = Vec::with_capacity(8);
        let mut edges: [usize; 3] = [];
        for (variable, term) in solution {
            self.variable_terms.insert(variable.into_string(), &term);
            match self.write_term(data_buffer, &variable, &term, &edges, &knowns) {
                Ok => knowns.push(&term),
                Err => unknowns.push(&term),
            }
        }

        let edge_len = edges.len();
        if edge_len == 3 {
            data_buffer.edges.push(edges);
        } else if edge_len > 0 {
            warn!("Edge array of size {edge_len} differs from the expected length of 3. Skipping");
        }
    }

    fn update_term_index(&mut self, term: &TermRef<'a>, index: usize) {
        match self.term_indices.insert(term, index) {
            Some(i) => {
                warn!("Overwriting index {i} of term '{term}' with {index}");
            }
            _ => {}
        }
    }

    fn try_insert_characteristic(
        self,
        data_buffer: &mut GraphDisplayData,
        term: &TermRef<'a>,
        characteristic: Characteristic,
    ) -> Result<(), ()> {
        match self.term_indices.get(term) {
            Some(index) => {
                data_buffer
                    .characteristics
                    .insert(index, characteristic.to_string());
                Ok(())
            }
            None => Err(()),
        }
    }

    fn insert_node(
        &mut self,
        data_buffer: &mut GraphDisplayData,
        variable: VariableRef<'a>,
        term: &TermRef<'a>,
        edges: &mut [usize; 3],
        node_type: ElementType,
    ) -> Result<(), ()> {
        let index = data_buffer.elements.len();
        self.update_term_index(&term, index);
        match variable.as_str() {
            "s" => edges[0] = index,
            "o" => edges[2] = index,
        }
        data_buffer.elements.push(node_type);
        Ok(())
    }

    /// NOTE: The edge array is overwritten.
    /// This means if a solution has multiple edge terms, the last one seen wins.
    fn insert_edge(
        &mut self,
        data_buffer: &mut GraphDisplayData,
        term: &TermRef<'a>,
        edges: &mut [usize; 3],
        edge_type: ElementType,
    ) -> Result<(), ()> {
        let index = data_buffer.elements.len();
        self.update_term_index(&term, index);
        edges[1] = index;
        data_buffer.elements.push(edge_type);
        Ok(())
    }

    fn write_term<'a>(
        &mut self,
        data_buffer: &mut GraphDisplayData,
        variable: VariableRef<'a>,
        term: &TermRef<'a>,
        edges: &mut [usize; 3],
        knowns: &Vec<&TermRef<'a>>,
    ) -> Result<(), ()> {
        // TODO: Collect errors and show to frontend
        // let iri = Iri::parse(value.as_str()).unwrap();
        // let path = iri.path();
        // let haystack = path.as_str().as_bytes();

        // A prefix determines what namespace the element belongs to
        let prefixes = [
            "22-rdf-syntax-ns", // RDF
            "rdf-schema",       // RDFS
            "owl",              // OWL
        ];

        // Doesnt work
        // let iri_type = memchr2("rdf", "owl", haystack);

        match term {
            TermRef::BlankNode(bnode) => {
                // REVIEW: Test if this works
                if let Some(_) = bnode.unique_id() {
                    // Anonymous individual
                    self.insert_node(
                        data_buffer,
                        variable,
                        term,
                        edges,
                        ElementType::Owl(OwlNode::AnonymousClass),
                    )
                } else {
                    // RDFS Named individual
                }
            }
            TermRef::Literal(literal) => {
                info!("Is literal: '{}'", literal.value());
                // match literal {

                // }
            }
            TermRef::NamedNode(uri) => {
                // NOTE: Only supports RDF 1.1
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
                    rdf::PROPERTY => self.insert_edge(
                        data_buffer,
                        term,
                        edges,
                        ElementType::Rdf(RdfType::Edge(RdfEdge::RdfProperty)),
                    ),
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
                        variable,
                        term,
                        edges,
                        ElementType::Rdfs(RdfsType::Node(RdfsNode::Class)),
                    ),
                    // rdfs::COMMENT => {}
                    // rdfs::CONTAINER => {}
                    // rdfs::CONTAINER_MEMBERSHIP_PROPERTY => {}
                    rdfs::DATATYPE => self.insert_edge(
                        data_buffer,
                        term,
                        edges,
                        ElementType::Rdfs(RdfsType::Edge(RdfsEdge::Datatype)),
                    ),
                    rdfs::DOMAIN => {
                        for item in knowns {
                            match item {
                                owl::OBJECT_PROPERTY => self.insert_edge(
                                    data_buffer,
                                    term,
                                    edges,
                                    ElementType::Owl(OwlType::Edge(OwlEdge::ObjectProperty)),
                                ),
                                owl::DATATYPE_PROPERTY => self.insert_edge(
                                    data_buffer,
                                    term,
                                    edges,
                                    ElementType::Owl(OwlType::Edge(OwlEdge::DatatypeProperty)),
                                ),
                                // owl::ANNOTATION_PROPERTY => {}
                                _ => Err(()),
                            }
                        }
                    }
                    // rdfs::IS_DEFINED_BY => {}
                    rdfs::LABEL => {
                        match variable {
                            //
                            "slabel" => match self.variable_terms.get("s") {
                                Some(t) => {}
                                _ => Err(()),
                            },
                            "plabel" => {}
                            "olabel" => {}
                        }
                    }
                    rdfs::LITERAL => self.insert_node(
                        data_buffer,
                        variable,
                        term,
                        edges,
                        ElementType::Rdfs(RdfsType::Node(RdfsNode::Literal)),
                    ),
                    // rdfs::MEMBER => {}
                    rdfs::RANGE => {
                        for item in knowns {
                            match item {
                                owl::OBJECT_PROPERTY => self.insert_edge(
                                    data_buffer,
                                    term,
                                    edges,
                                    ElementType::Owl(OwlType::Edge(OwlEdge::ObjectProperty)),
                                ),
                                owl::DATATYPE_PROPERTY => self.insert_edge(
                                    data_buffer,
                                    term,
                                    edges,
                                    ElementType::Owl(OwlType::Edge(OwlEdge::DatatypeProperty)),
                                ),
                                // owl::ANNOTATION_PROPERTY => {}
                                _ => Err(()),
                            }
                        }
                    }
                    // rdfs::RESOURCE => {}
                    // rdfs::SEE_ALSO => {}
                    rdfs::SUB_CLASS_OF => self.insert_edge(
                        data_buffer,
                        term,
                        edges,
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
                        variable,
                        term,
                        edges,
                        ElementType::Owl(OwlType::Node(OwlNode::Class)),
                    ),
                    owl::COMPLEMENT_OF => {}
                    owl::DATATYPE_COMPLEMENT_OF => {}
                    owl::DATATYPE_PROPERTY => self.insert_edge(
                        data_buffer,
                        term,
                        edges,
                        ElementType::Owl(OwlType::Edge(OwlEdge::DatatypeProperty)),
                    ),
                    // owl::DATA_RANGE => {}
                    // owl::DEPRECATED => {}
                    owl::DEPRECATED_CLASS => self.insert_node(
                        data_buffer,
                        variable,
                        term,
                        edges,
                        ElementType::Owl(OwlType::Node(OwlNode::DeprecatedClass)),
                    ),
                    owl::DEPRECATED_PROPERTY => self.insert_edge(
                        data_buffer,
                        term,
                        edges,
                        ElementType::Owl(OwlType::Edge(OwlEdge::DeprecatedProperty)),
                    ),
                    // owl::DIFFERENT_FROM => {}
                    // owl::DISJOINT_UNION_OF => {}
                    owl::DISJOINT_WITH => self.insert_edge(
                        data_buffer,
                        term,
                        edges,
                        ElementType::Owl(OwlType::Edge(OwlEdge::DisjointWith)),
                    ),
                    // owl::DISTINCT_MEMBERS => {}
                    owl::EQUIVALENT_CLASS => self.insert_node(
                        data_buffer,
                        variable,
                        term,
                        edges,
                        ElementType::Owl(OwlType::Node(OwlNode::EquivalentClass)),
                    ),
                    // owl::EQUIVALENT_PROPERTY => {}
                    owl::FUNCTIONAL_PROPERTY => self.try_insert_characteristic(
                        data_buffer,
                        term,
                        Characteristic::FunctionalProperty,
                    ),
                    // owl::HAS_KEY => {}
                    // owl::HAS_SELF => {}
                    // owl::HAS_VALUE => {}
                    // owl::IMPORTS => {}
                    // owl::INCOMPATIBLE_WITH => {}
                    owl::INTERSECTION_OF => self.insert_node(
                        data_buffer,
                        variable,
                        term,
                        edges,
                        ElementType::Owl(OwlType::Node(OwlNode::Intersection)),
                    ),
                    owl::INVERSE_FUNCTIONAL_PROPERTY => self.try_insert_characteristic(
                        data_buffer,
                        term,
                        Characteristic::InverseFunctionalProperty,
                    ),
                    owl::INVERSE_OF => self.insert_edge(
                        data_buffer,
                        term,
                        edges,
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
                        term,
                        edges,
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
                        variable,
                        term,
                        edges,
                        ElementType::Owl(OwlType::Node(OwlNode::Thing)),
                    ),
                    // owl::TOP_DATA_PROPERTY => {}
                    // owl::TOP_OBJECT_PROPERTY => {}
                    owl::TRANSITIVE_PROPERTY => self.try_insert_characteristic(
                        data_buffer,
                        term,
                        Characteristic::Transitive,
                    ),
                    owl::UNION_OF => {}
                    // owl::VERSION_INFO => {}
                    // owl::VERSION_IRI => {}
                    // owl::WITH_RESTRICTIONS => {}
                    _ => {
                        // Visualization of this element is not supported
                        info!("Visualization of term '{term}' is not supported");
                        Err(())
                    }
                };
            }
        }
    }
}
