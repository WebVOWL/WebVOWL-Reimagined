mod serializers;
pub mod store;
mod vocab;
pub mod prelude {
    pub use crate::serializers::frontend::GraphDisplayDataSolutionSerializer;
    pub use rdf_fusion::execution::results::QueryResults;
}

