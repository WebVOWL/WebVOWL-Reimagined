mod serializers;
pub mod store;
pub use webvowl_sparql_queries;
mod vocab;
pub mod prelude {
    pub use crate::serializers::frontend::GraphDisplayDataSolutionSerializer;
    pub use rdf_fusion::execution::results::QueryResults;
}
