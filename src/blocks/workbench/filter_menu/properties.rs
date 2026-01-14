use grapher::prelude::{ElementType, GenericType, OwlType, RdfType, RdfsType};

pub fn is_property(item: ElementType) -> bool {
    match item {
        ElementType::Generic(GenericType::Edge(_)) => true,
        ElementType::Owl(OwlType::Edge(_)) => true,
        ElementType::Rdf(RdfType::Edge(_)) => true,
        ElementType::Rdfs(RdfsType::Edge(_)) => true,
        _ => false,
    }
}
