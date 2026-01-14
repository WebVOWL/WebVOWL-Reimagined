use grapher::prelude::{ElementType, OwlNode, OwlType};

pub fn is_set_operator(item: ElementType) -> bool {
    match item {
        ElementType::Owl(OwlType::Node(node)) => match node {
            OwlNode::Complement
            | OwlNode::DisjointUnion
            | OwlNode::IntersectionOf
            | OwlNode::UnionOf => true,
            _ => false,
        },
        _ => false,
    }
}
