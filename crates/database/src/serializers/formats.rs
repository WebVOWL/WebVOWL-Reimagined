pub mod graph_display {
    use grapher::web::prelude::ElementType;

    /// Struct containing graph data for RustGrapher
    pub struct GraphDisplayData {
        /// Labels annotate classes and properties
        pub labels: Vec<String>,
        /// Elements are the nodes and edge types for which visualization is supported.
        ///
        /// The index into this vector determines the unique ID of each element
        /// and is used by `edges`.
        pub elements: Vec<ElementType>,
        /// An array of three elements: `source node`, `edge`, and `target node`.
        ///
        /// The elements of the array are the indices of `elements`.
        pub edges: Vec<[usize; 3]>,
    }

    impl GraphDisplayData {
        pub fn new_empty() -> Self {
            Self {
                labels: Vec::new(),
                elements: Vec::new(),
                edges: Vec::new(),
            }
        }
    }
}
