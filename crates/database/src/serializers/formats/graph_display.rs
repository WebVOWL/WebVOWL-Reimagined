use grapher::web::prelude::NodeType;

/// Struct containing graph data for RustGrapher
pub struct GraphDisplayData {
    pub labels: Vec<String>,
    pub nodes: Vec<NodeType>,
    pub edges: Vec<[usize; 3]>,
}

impl GraphDisplayData {
    pub fn new_empty() -> Self {
        Self {
            labels: Vec::new(),
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}
