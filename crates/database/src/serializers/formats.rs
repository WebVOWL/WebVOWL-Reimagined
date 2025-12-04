pub mod graph_display {
    use std::collections::HashMap;

    use grapher::web::prelude::ElementType;

    /// Struct containing graph data for RustGrapher
    pub struct GraphDisplayData {
        /// Labels annotate classes and properties
        ///
        /// The index into this vector is the ID of the node/edge having a label.
        /// The ID is defined by the indices of `elements`.
        pub labels: Vec<String>,
        /// Elements are the nodes and edge types for which visualization is supported.
        ///
        /// The index into this vector determines the unique ID of each element.
        pub elements: Vec<ElementType>,
        /// An array of three elements: `source node`, `edge`, and `target node`.
        ///
        /// The elements of the array are node/edge IDs.
        /// They are defined by the indices of `elements`.
        pub edges: Vec<[usize; 3]>,
        /// Cardinalities of edges.
        ///
        /// The tuple consists of 2 elements:
        ///     - u32: The ID of the edge. Defined by the indices of `elements`.
        ///     - (String, Option<String>):
        ///         - String: The min cardinality of the edge.
        ///         - Option<String>: The max cardinality of the target edge.
        pub cardinalities: Vec<(u32, (String, Option<String>))>,
        /// Special node types. For instance "transitive" or "inverse functional".
        ///
        /// The hashmap consists of:
        ///     - usize: The ID of the node. Defined by the indices of `elements`.
        ///     - String: The name of the node type. E.g. "transitive".
        pub characteristics: HashMap<usize, String>,
    }

    impl GraphDisplayData {
        pub fn new_empty() -> Self {
            Self {
                labels: Vec::new(),
                elements: Vec::new(),
                edges: Vec::new(),
                cardinalities: Vec::new(),
                characteristics: HashMap::new(),
            }
        }
    }
}
