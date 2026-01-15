use std::{collections::{HashMap, HashSet}, fmt::{Display, Formatter}};

use grapher::prelude::{ElementType, GraphDisplayData};
use log::error;
use oxrdf::Term;

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
        write!(f, "Triple{{ ")?;
        write!(f, "{} - ", self.id)?;
        write!(f, "{} - ", self.element_type,)?;
        write!(
            f,
            "{}",
            self.target
                .as_ref()
                .map(|t| t.to_string())
                .unwrap_or_default(),
        )?;
        write!(f, "}}")
    }
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct Edge {
    /// The subject
    subject: String,
    /// The element type
    element_type: ElementType,
    /// The object
    object: String,
}
impl Display for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Edge{{ {} - {} - {} }}", self.subject, self.element_type, self.object)?;
        Ok(())
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
    /// 
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

    /// Stores labels of edges.
    ///
    /// - Key = The edge.
    /// - Value = The label.
    edge_label_buffer: HashMap<Edge, String>,
    
    /// Edges in graph, to avoid duplicates
    edge_buffer: HashSet<Edge>,
    /// Maps from edge to its characteristic.
    edge_characteristics: HashMap<Edge, Vec<String>>,

    /// Maps from node iri to its characteristics.
    node_characteristics: HashMap<String, Vec<String>>,

    /// Stores unresolved triples.
    ///
    /// - Key = The unresolved IRI of the triple 
    ///   can be either the subject, object or both (in this case, subject is used)
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
            edge_label_buffer: HashMap::new(),
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
            let maybe_label = self.edge_label_buffer.remove(&edge);

            match (subject_idx, object_idx, maybe_label) {
                (Some(subject_idx), Some(object_idx), Some(label)) => {
                    display_data.elements.push(edge.element_type);
                    display_data.labels.push(label);
                    display_data.edges.push([*subject_idx, display_data.elements.len() - 1, *object_idx]);
                }
                (Some(_), Some(_), None) => {
                    error!("Label in edge not found in iricache: {}", edge.subject);
                }
                (None, _, _) => {
                    error!("Subject in edge not found in iricache: {}", edge.subject);
                }
                (_, None, _) => {
                    error!("Object in edge not found in iricache: {}", edge.object);
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
                    display_data.characteristics.insert(*idx, characteristics.pop().unwrap());
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
        writeln!(f, "\tdocument_base: {}", self.document_base)?;
        writeln!(f, "\telement_buffer:")?;
        for (iri, element) in self.element_buffer.iter() {
            writeln!(f, "\t\t{} : {}", iri, element)?;
        }
        writeln!(f, "\tedge_redirection:")?;
        for (iri, subject) in self.edge_redirection.iter() {
            writeln!(f, "\t\t{} -> {}", iri, subject)?;
        }
        writeln!(f, "\tedges_include_map: ")?;
        for (iri, edges) in self.edges_include_map.iter() {
            writeln!(f, "\t\t{} : {{", iri)?;
            for edge in edges.iter() {
                writeln!(f, "\t\t\t{}", edge)?;
            }
            writeln!(f, "\t\t}}")?;
        }
        writeln!(f, "\tglobal_element_mappings:")?;
        for (element, index) in self.global_element_mappings.iter() {
            writeln!(f, "\t\t{} : {}", element, index)?;
        }
        writeln!(f, "\tlabel_buffer:")?;
        for (iri, label) in self.label_buffer.iter() {
            writeln!(f, "\t\t{} : {}", iri, label)?;
        }
        writeln!(f, "\tedge_buffer:")?;
        for edge in self.edge_buffer.iter() {
            writeln!(f, "\t\t{}", edge)?;
        }
        writeln!(f, "\tedge_characteristics: {:?}", self.edge_characteristics)?;
        writeln!(f, "\tnode_characteristics: {:?}", self.node_characteristics)?;
        writeln!(f, "\tunknown_buffer:")?;
        for (iri, triple) in self.unknown_buffer.iter() {
            writeln!(f, "\t\t{} : {}", iri, triple)?;
        }
        writeln!(f, "\tfailed_buffer:")?;
        for (triple, reason) in self.failed_buffer.iter() {
            writeln!(f, "\t\t{} : {}", triple, reason)?;
        }
        writeln!(f, "}}")
    }
}