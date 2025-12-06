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
pub struct VowlExtractData {
    // the values are the indices of the nodes in the nodes vector
    pub nodes: Vec<Node<usize>>,
    // the values are the indices of the edges in the node vector
    pub edges: Vec<Edge<usize>>,
    pub irivec: Vec<String>,
}

impl<A> Default for VowlExtractData<A> {
    fn default() -> Self {
        Self {
            nodes: vec![],
            edges: vec![],
            irivec: vec![],
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
