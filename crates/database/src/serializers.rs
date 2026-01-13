use grapher::prelude::{ElementType, GraphDisplayData};
use log::error;
use oxrdf::Term;
use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter},
};
pub mod frontend;

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

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct Edge {
    /// The IRI of the edge
    edge_iri: String,
    /// The subject IRI
    subject: String,
    /// The element type
    element_type: ElementType,
    /// The object IRI
    object: String,
}
impl Display for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Edge {{")?;
        writeln!(f, "\tsubject: {}", self.subject)?;
        writeln!(f, "\telement_type: {}", self.element_type)?;
        writeln!(f, "\tobject: {}", self.object)?;
        writeln!(f, "}}")
    }
}

pub struct SerializationDataBuffer {
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

    /// Maps from element IRI to a set of the edges that include it.
    ///
    /// Used to remap when nodes are merges.
    edges_include_map: HashMap<String, HashSet<Edge>>,
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

    /// Edges in graph, to avoid duplicates
    edge_buffer: HashSet<Edge>,
    /// Maps from edge to its characteristic.
    edge_characteristics: HashMap<Edge, Vec<String>>,

    /// Maps from node iri to its characteristics.
    node_characteristics: HashMap<String, Vec<String>>,

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
impl SerializationDataBuffer {
    pub fn new() -> Self {
        Self {
            element_buffer: HashMap::new(),
            edge_redirection: HashMap::new(),
            edges_include_map: HashMap::new(),
            global_element_mappings: HashMap::new(),
            label_buffer: HashMap::new(),
            edge_buffer: HashSet::new(),
            unknown_buffer: HashMap::new(),
            failed_buffer: Vec::new(),
            document_base: String::new(),
            edge_characteristics: HashMap::new(),
            node_characteristics: HashMap::new(),
        }
    }
}

impl Into<GraphDisplayData> for SerializationDataBuffer {
    fn into(mut self) -> GraphDisplayData {
        let mut display_data = GraphDisplayData::new();
        let mut iricache: HashMap<String, usize> = HashMap::new();
        for (iri, element) in self.element_buffer.into_iter() {
            let label = self.label_buffer.remove(&iri);
            match label {
                Some(label) => {
                    display_data.labels.push(label);
                    display_data.elements.push(element);
                    iricache.insert(iri, display_data.elements.len() - 1);
                }
                None => {
                    error!("Label not found for iri: {}", iri);
                }
            }
        }

        for edge in self.edge_buffer.iter() {
            let subject_idx = iricache.get(&edge.subject);
            let object_idx = iricache.get(&edge.object);
            let maybe_label = self.label_buffer.remove(&edge.subject);

            match (subject_idx, object_idx, maybe_label) {
                (Some(subject_idx), Some(object_idx), Some(label)) => {
                    display_data.elements.push(edge.element_type);
                    display_data.labels.push(label);
                    display_data.edges.push([
                        *subject_idx,
                        display_data.elements.len() - 1,
                        *object_idx,
                    ]);
                }
                _ => {
                    error!("Edge not found in iricache: {}", edge);
                }
            }
        }

        for (iri, mut characteristics) in self.node_characteristics.into_iter() {
            let idx = iricache.get(&iri);
            match idx {
                Some(idx) => {
                    display_data
                        .characteristics
                        .insert(*idx, characteristics.pop().unwrap());
                }
                None => {
                    error!("Characteristic not found for node in iricache: {}", iri);
                }
            }
        }
        // TODO: handle cardinalities

        display_data
    }
}

impl Display for SerializationDataBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "SerializationDataBuffer {{")?;
        writeln!(f, "\telement_buffer: {:?}", self.element_buffer)?;
        writeln!(f, "\tedge_redirection: {:?}", self.edge_redirection)?;
        writeln!(f, "\tedges_include_map: {:?}", self.edges_include_map)?;
        writeln!(
            f,
            "\tglobal_element_mappings: {:?}",
            self.global_element_mappings
        )?;
        writeln!(f, "\tlabel_buffer: {:?}", self.label_buffer)?;
        writeln!(f, "\tedge_buffer: {:?}", self.edge_buffer)?;
        writeln!(f, "\tedge_characteristics: {:?}", self.edge_characteristics)?;
        writeln!(f, "\tnode_characteristics: {:?}", self.node_characteristics)?;
        writeln!(f, "\tunknown_buffer: {:?}", self.unknown_buffer)?;
        writeln!(f, "\tfailed_buffer: {:?}", self.failed_buffer)?;
        writeln!(f, "\tdocument_base: {:?}", self.document_base)?;
        writeln!(f, "}}")
    }
}
