use grapher::prelude::ElementType;

pub fn filter<T>(elements: Vec<ElementType>, filters: Vec<T>) -> Vec<ElementType>
where
    T: Fn(ElementType) -> bool,
{
    elements
        .into_iter()
        .filter(|&elem| filters.iter().any(|f| f(elem)))
        .collect::<Vec<_>>()
}
