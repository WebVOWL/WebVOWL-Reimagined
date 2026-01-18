use grapher::prelude::{
    ElementType, GenericEdge, GenericNode, GenericType, OwlEdge, OwlNode, OwlType, RdfEdge,
    RdfType, RdfsEdge, RdfsNode, RdfsType,
};

pub trait ElementLegend {
    /// Get the legend of `self`.
    fn legend(self) -> Option<String>;
}

impl ElementLegend for ElementType {
    fn legend(self) -> Option<String> {
        match self {
            ElementType::NoDraw => None,
            ElementType::Rdf(RdfType::Edge(edge)) => edge.legend(),
            ElementType::Rdfs(RdfsType::Node(node)) => node.legend(),
            ElementType::Rdfs(RdfsType::Edge(edge)) => edge.legend(),
            ElementType::Owl(OwlType::Node(node)) => node.legend(),
            ElementType::Owl(OwlType::Edge(edge)) => edge.legend(),
            ElementType::Generic(GenericType::Node(node)) => node.legend(),
            ElementType::Generic(GenericType::Edge(edge)) => edge.legend(),
        }
    }
}

impl ElementLegend for GenericNode {
    fn legend(self) -> Option<String> {
        match self {
            GenericNode::Generic => None,
        }
    }
}

impl ElementLegend for GenericEdge {
    fn legend(self) -> Option<String> {
        match self {
            GenericEdge::Generic => None,
        }
    }
}

impl ElementLegend for RdfsNode {
    fn legend(self) -> Option<String> {
        match self {
            RdfsNode::Class => Some("/node_legends/RdfsClass.png".to_string()),
            RdfsNode::Literal => Some("/node_legends/Literal.png".to_string()),
            RdfsNode::Resource => Some("/node_legends/RdfsResource.png".to_string()),
            RdfsNode::Datatype => Some("/node_legends/Datatype.png".to_string()),
        }
    }
}

impl ElementLegend for RdfsEdge {
    fn legend(self) -> Option<String> {
        match self {
            RdfsEdge::SubclassOf => Some("/node_legends/SubclassOf.png".to_string()),
        }
    }
}

impl ElementLegend for RdfEdge {
    fn legend(self) -> Option<String> {
        match self {
            RdfEdge::RdfProperty => None,
        }
    }
}

impl ElementLegend for OwlNode {
    fn legend(self) -> Option<String> {
        match self {
            OwlNode::AnonymousClass => Some("/node_legends/AnonymousClass.png".to_string()),
            OwlNode::Class => Some("/node_legends/Class.png".to_string()),
            OwlNode::Complement => Some("/node_legends/Complement.png".to_string()),
            OwlNode::DeprecatedClass => Some("/node_legends/DeprecatedClass.png".to_string()),
            OwlNode::ExternalClass => Some("/node_legends/ExternalClass.png".to_string()),
            OwlNode::EquivalentClass => Some("/node_legends/EquivalentClass.png".to_string()),
            OwlNode::DisjointUnion => Some("/node_legends/DisjointUnion.png".to_string()),
            OwlNode::IntersectionOf => Some("/node_legends/Intersection.png".to_string()),
            OwlNode::Thing => Some("/node_legends/Thing.png".to_string()),
            OwlNode::UnionOf => Some("/node_legends/Union.png".to_string()),
        }
    }
}

impl ElementLegend for OwlEdge {
    fn legend(self) -> Option<String> {
        match self {
            OwlEdge::DatatypeProperty => Some("/node_legends/DatatypeProperty.png".to_string()),
            OwlEdge::DisjointWith => Some("/node_legends/Disjoint.png".to_string()),
            OwlEdge::DeprecatedProperty => Some("/node_legends/DeprecatedProperty.png".to_string()),
            OwlEdge::ExternalProperty => Some("/node_legends/ExternalProperty.png".to_string()),
            OwlEdge::InverseOf => None,
            OwlEdge::ObjectProperty => None,
            OwlEdge::ValuesFrom => None,
        }
    }
}
