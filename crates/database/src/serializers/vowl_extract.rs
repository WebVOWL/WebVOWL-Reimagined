use std::{collections::HashMap, fmt::Display, hash::Hash, ops::Deref, str::FromStr};

use webvowl_parser::errors::WebVowlStoreError;

use crate::serializers::new_ser::is_iri;

#[derive(Debug, Clone)]
pub enum Node<T> {
    Class(T),
    ExternalClass(T),
    Thing(T),
    EquivalentClass(Vec<T>),
    Union(T),
    DisjointUnion(T),
    Intersection(T),
    Complement(T),
    DeprecatedClass(T),
    AnonymousClass(T),
    Literal(T),
    RdfsClass(T),
    RdfsResource(T),
    NoDraw,
}

impl<T> Node<T> {
    pub fn from_str(node_type: &str, id: T) -> Result<Self, WebVowlStoreError> {
        match node_type {
            "1Class" => Ok(Self::Class(id)),
            "2UnnamedClass" => Ok(Self::Thing(id)),
            "Intersection" => Ok(Self::Intersection(id)),
            "Union" => Ok(Self::Union(id)),
            "AnonymousClass" => Ok(Self::AnonymousClass(id)),
            "EquivalentClass" => Ok(Self::EquivalentClass(vec![id])),
            "Complement" => Ok(Self::Complement(id)),
            "DeprecatedClass" => Ok(Self::DeprecatedClass(id)),
            "Literal" => Ok(Self::Literal(id)),
            "RdfsClass" => Ok(Self::RdfsClass(id)),
            "RdfsResource" => Ok(Self::RdfsResource(id)),
            _ => Err(WebVowlStoreError::from(format!(
                "Invalid node type: {}",
                node_type
            ))),
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub enum Edge<T> {
    Datatype(T, T),
    ObjectProperty(T, T, T),
    DatatypeProperty(T, T),
    SubclassOf(T, T),
    InverseProperty(T, T),
    DisjointWith(T, T),
    RdfProperty(T, T),
    DeprecatedProperty(T, T),
    ExternalProperty(T, T),
    ValuesFrom(T, T),
    NoDraw,
}

#[derive(Debug)]
pub struct VowlExtract<A> {
    //ontology: ComponentMappedOntology<A, Rc<AnnotatedComponent<A>>>,
    nodes: Vec<Node<usize>>,
    // [from, edge_type, to]
    edges: Vec<Edge<usize>>,
    blanknode_mapping: HashMap<A, usize>,
    pub iricache: HashMap<A, usize>,
    pub irivec: Vec<A>,
}

impl<A> Default for VowlExtract<A> {
    fn default() -> Self {
        Self {
            nodes: vec![],
            edges: vec![],
            iricache: HashMap::new(),
            blanknode_mapping: HashMap::new(),
            irivec: vec![],
        }
    }
}

impl<A: Clone + Eq + Hash + AsRef<str>> VowlExtract<A> {
    pub fn insert(&mut self, x: A) -> usize {
        if self.resolve(&x).is_none() {
            let present = self.iricache.contains_key(&x);
            if !present {
                self.iricache
                    .insert(x.clone(), self.irivec.len() as usize);
                self.irivec.push(x.clone());
            }
        }
        self.iricache[&x]
    }
    pub fn resolve(&mut self, x: &A) -> Option<usize> {
        if self.blanknode_mapping.contains_key(x) {
            return self.resolve(&self.irivec[self.blanknode_mapping[x]].clone());
        } else if self.iricache.contains_key(x) {
            return Some(self.iricache[x]);
        } else {
            return None;
        }
    }
}

impl<A: Display> Display for Node<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Class(id) => write!(f, "Class({})", id),
            Self::ExternalClass(id) => write!(f, "ExternalClass({})", id),
            Self::Thing(id) => write!(f, "Thing({})", id),
            Self::EquivalentClass(ids) => write!(
                f,
                "EquivalentClass({})",
                ids.iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            Self::Union(id) => write!(f, "Union({})", id),
            Self::DisjointUnion(id) => write!(f, "DisjointUnion({})", id),
            Self::Intersection(id) => write!(f, "Intersection({})", id),
            Self::Complement(id) => write!(f, "Complement({})", id),
            Self::DeprecatedClass(id) => write!(f, "DeprecatedClass({})", id),
            Self::AnonymousClass(id) => write!(f, "AnonymousClass({})", id),
            Self::Literal(id) => write!(f, "Literal({})", id),
            Self::RdfsClass(id) => write!(f, "RdfsClass({})", id),
            Self::RdfsResource(id) => write!(f, "RdfsResource({})", id),
            Self::NoDraw => write!(f, "NoDraw"),
        }
    }
}
