use grapher::web::prelude::NodeType;

/// Struct containing graph data for RustGrapher
pub struct GraphDisplayData {
    labels: Vec<String>,
    nodes: Vec<NodeType>,
    edges: Vec<[usize; 3]>,
}
