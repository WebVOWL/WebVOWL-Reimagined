use super::data::OwlToWovlJSON;
use petgraph::Graph;
use petgraph::graph::NodeIndex;
use std::collections::HashMap;

pub struct GraphContainer {
    pub graph: Graph<String, ()>,
    pub graph_struct: OwlToWovlJSON,
    pub node_map: HashMap<String, NodeIndex>,
}

impl GraphContainer {
    pub fn new(
        graph: Graph<String, ()>,
        graph_struct: OwlToWovlJSON,
        node_map: HashMap<String, NodeIndex>,
    ) -> Self {
        GraphContainer {
            graph,
            graph_struct,
            node_map,
        }
    }
}

pub fn build_graph(graph_struct: OwlToWovlJSON) -> GraphContainer {
    // * Create the graph */
    let mut graph = Graph::<String, ()>::new();

    //Hashmap is necessary to store the nodes references with their ids
    let mut hashmap = HashMap::new();

    for edge in &graph_struct.propertyAttribute {
        let domain_node_id = edge.domain.clone();
        let range_node_id = edge.range.clone();
        let dom_index;
        let ran_index;

        // * Check if the node already exists */
        // * If it does, get the index        */
        // * If it doesn't, create the node   */
        let opt_dm_index = hashmap.get(&domain_node_id);
        match opt_dm_index {
            Some(&x) => {
                dom_index = x;
            }
            None => {
                dom_index = graph.add_node(edge.domain.clone());
                hashmap.insert(domain_node_id, dom_index);
            }
        }

        let opt_rn_index = hashmap.get(&edge.range);
        match opt_rn_index {
            Some(&x) => {
                ran_index = x;
            }
            None => {
                ran_index = graph.add_node(edge.range.clone());
                hashmap.insert(range_node_id, ran_index);
            }
        }
        graph.add_edge(dom_index, ran_index, ());
    }

    for node in &graph_struct.class {
        let node_id = node.id.clone();
        let opt_index = hashmap.get(&node.id);
        match opt_index {
            Some(_) => {}
            None => {
                hashmap.insert(node_id, graph.add_node(node.id.clone()));
            }
        }
    }

    for attr in &graph_struct.classAttribute {
        let attr_id = attr.id.clone();
        let opt_index = hashmap.get(&attr.id);

        let dom_index;
        match opt_index {
            Some(&x) => {
                dom_index = x;
            }
            None => {
                println!("WARNING! Node does not exist, creating node: {:?}", attr.id);
                dom_index = graph.add_node(attr.id.clone());
                hashmap.insert(attr_id, dom_index);
            }
        }
        make_edges(
            dom_index,
            attr.superClasses.clone(),
            &mut graph,
            &mut hashmap,
        );
        make_edges(dom_index, attr.subClasses.clone(), &mut graph, &mut hashmap);
        make_edges(dom_index, attr.complement.clone(), &mut graph, &mut hashmap);
        make_edges(dom_index, attr.union.clone(), &mut graph, &mut hashmap);
        make_edges(
            dom_index,
            attr.intersection.clone(),
            &mut graph,
            &mut hashmap,
        );
        make_edges(dom_index, attr.equivalent.clone(), &mut graph, &mut hashmap);
        make_edges(
            dom_index,
            attr.disjointUnion.clone(),
            &mut graph,
            &mut hashmap,
        );
    }
    return GraphContainer::new(graph, graph_struct, hashmap);
}

pub fn make_edges(
    domain: NodeIndex,
    opt_vector: Option<Vec<String>>,
    graph: &mut Graph<String, ()>,
    hashmap: &mut HashMap<String, NodeIndex>,
) {
    match opt_vector {
        Some(vector) => {
            for id_string in vector {
                let opt_index = hashmap.get(&id_string);
                match opt_index {
                    Some(ran_index) => {
                        graph.add_edge(domain, *ran_index, ());
                    }
                    None => {
                        println!(
                            "WARNING! Node does not exist, creating node: {:?}",
                            id_string
                        );
                        let ran_index = graph.add_node(id_string.clone());
                        hashmap.insert(id_string, ran_index);
                        graph.add_edge(domain, ran_index, ());
                    }
                }
            }
        }
        None => {}
    }
}
