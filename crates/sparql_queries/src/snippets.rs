pub mod general;
pub mod owl;
pub mod rdf_vocab;

use crate::element_type_injection::SparqlSnippet;
use grapher::prelude::IntoEnumIterator;

pub fn snippets_from_enum<T>() -> Vec<&'static str>
where
    T: IntoEnumIterator + SparqlSnippet,
{
    T::iter().map(|item| item.snippet()).collect::<Vec<_>>()
}
