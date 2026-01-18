use grapher::prelude::{ElementType, GenericType, OwlType, RdfType, RdfsType};

use crate::blocks::workbench::filter_menu::special_operators::is_set_operator;

pub fn is_owl_class(item: ElementType) -> bool {
    let class = match item {
        ElementType::Owl(OwlType::Node(_)) => true,
        _ => false,
    };
    class && !is_set_operator(item)
}

pub fn is_rdf_class(item: ElementType) -> bool {
    match item {
        ElementType::Rdfs(RdfsType::Node(_)) => true,
        _ => false,
    }
}

pub fn is_generic_class(item: ElementType) -> bool {
    match item {
        ElementType::Generic(GenericType::Node(_)) => true,
        _ => false,
    }
}
