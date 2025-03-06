use petgraph::graph::{DiGraph, NodeIndex, EdgeIndex};
use petgraph::visit::EdgeRef;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)] //PartialEq allows enums to perform == and != operations
enum Signal {
    None,
    Plus,
    Minus,
}

#[derive(Debug, Clone)]
enum Interaction {
    Activate,
    Apoptosis,
    Consume,
    Decay,
    Differentiate,
    Infect,
    Inhibit,
    Kill,
    Phagocytosis,
    Polarize,
    Secrete,
    Replicate,
    Recruit,
}

// Structure of the link type
#[derive(Debug, Clone)]
struct LinkType {
    interaction: Interaction,
    signal: (Signal, Signal), 
}

// Structure to represent a path in the graph
#[derive(Debug, Clone)]
struct Path {
    edges: Vec<(EdgeIndex, Interaction)>, // List of tuples containing the edge ID and the interaction
}

// Structure of the path bank
#[derive(Debug, Clone)]
struct PathBank {
    paths: Vec<Path>,
}

// Structure for node properties
#[derive(Debug)]
struct NodeProperties {
    x: f32,
    y: f32,
    radius: f32,
    color: String,
}

impl PathBank {
    // New bank
    fn new() -> Self {
        PathBank { paths: Vec::new() }
    }
    
    fn add_path(&mut self, path: Path) {
        self.paths.push(path);
    }

    // Number of paths
    fn count(&self) -> usize {
        self.paths.len()
    }

    fn get_paths(&self) -> &Vec<Path> {
        &self.paths
    }

    fn negative_signal_edge(
        &self,
        graph: &DiGraph<&str, LinkType>,
    ) -> Vec<&Path> {
        self.paths
            .iter()
            .filter(|path| {
                path.edges.iter().any(|(edge_index, _)| {
                    if let Some(edge) = graph.edge_weight(*edge_index) {
                        edge.signal.1 == Signal::Minus 
                    } else {
                        false
                    }
                })
            })
            .collect()
    }

    fn non_negative_signal_edge(
        &self,
        graph: &DiGraph<&str, LinkType>,
    ) -> Vec<&Path> {
        self.paths
            .iter()
            .filter(|path| {
                path.edges.iter().any(|(edge_index, _)| {
                    if let Some(edge) = graph.edge_weight(*edge_index) {
                        edge.signal.1 == Signal::Plus || edge.signal.1 == Signal::None 
                    } else {
                        false
                    }
                })
            })
            .collect()
    }
}

fn nodes_without_negative_signal(graph: &DiGraph<&str, LinkType>) -> Vec<NodeIndex> {
    let mut reached_with_negative = HashSet::new();

    for edge in graph.edge_references() {
        if edge.weight().signal.1 == Signal::Minus {
            reached_with_negative.insert(edge.target());
        }
    }

    graph
        .node_indices()
        .filter(|node| !reached_with_negative.contains(node))
        .collect()
}


// Path implementation
impl Path {
    fn new() -> Self {
        Path { edges: Vec::new() }
    }

    fn add_edge(&mut self, edge: EdgeIndex, interaction: Interaction) {
        self.edges.push((edge, interaction));
    }

    fn length(&self) -> usize {
        self.edges.len()
    }
}

fn interaction_signal_map(interaction: &Interaction) -> (Signal, Signal) {
    match interaction {
        Interaction::Activate => (Signal::None, Signal::Plus),
        Interaction::Apoptosis => (Signal::None, Signal::Minus),
        Interaction::Consume => (Signal::None, Signal::Minus),
        Interaction::Decay => (Signal::None, Signal::Minus),
        Interaction::Differentiate => (Signal::Minus, Signal::Plus),
        Interaction::Infect => (Signal::None, Signal::Minus),
        Interaction::Inhibit => (Signal::None, Signal::Minus),
        Interaction::Kill => (Signal::None, Signal::Minus),
        Interaction::Phagocytosis => (Signal::None, Signal::Minus),
        Interaction::Polarize => (Signal::Minus, Signal::Plus),
        Interaction::Secrete => (Signal::None, Signal::Plus),
        Interaction::Replicate => (Signal::None, Signal::Plus),
        Interaction::Recruit => (Signal::None, Signal::Plus),
    }
}

fn non_positive_output(graph: &DiGraph<&str, LinkType>) -> Vec<NodeIndex> {
    let mut neg_links_s = Vec::new(); // Stores nodes with negative output links

    for node in graph.node_indices() {
        for edge in graph.edges(node) {
            if edge.weight().signal.1 == Signal::Minus {
                neg_links_s.push(node); // Adds nodes to the vector
                break;
            }
        }
    }
    neg_links_s
}

fn non_positive_input(graph: &DiGraph<&str, LinkType>) -> Vec<NodeIndex> {
    let mut neg_links_e = Vec::new(); // Stores nodes with non-positive input links

    for node in graph.node_indices() {
        for edge in graph.edges(node) {
            if edge.weight().signal.0 == Signal::Minus || edge.weight().signal.0 == Signal::None {
                neg_links_e.push(node); // Adds nodes to the vector
                break;
            }
        }
    }
    neg_links_e
}

fn non_negative_input(graph: &DiGraph<&str, LinkType>) -> Vec<NodeIndex> {
    let mut not_neg_e = Vec::new(); // Stores nodes with non-negative input links

    for node in graph.node_indices() {
        for edge in graph.edges(node) {
            if edge.weight().signal.0 == Signal::Plus || edge.weight().signal.0 == Signal::None {
                not_neg_e.push(node); // Adds nodes to the vector
                break;
            }
        }
    }
    not_neg_e
}

fn main() {
    // Creates a directed graph
    let mut g = DiGraph::<&str, LinkType>::new();

    // Adds nodes
    let a = g.add_node("Node A");
    let b = g.add_node("Node B");
    let c = g.add_node("Node C");
    let d = g.add_node("Node D");
    let e = g.add_node("Node E");

    // HashMap of nodes and properties
    let mut node_properties: HashMap<NodeIndex, NodeProperties> = HashMap::new();

    node_properties.insert(a, NodeProperties { x: 0.0, y: 0.5, radius: 1.0, color: "red".to_string() });
    node_properties.insert(b, NodeProperties { x: 5.0, y: 0.0, radius: 1.0, color: "blue".to_string() });
    node_properties.insert(c, NodeProperties { x: 0.0, y: -5.0, radius: 1.0, color: "green".to_string() });
    node_properties.insert(d, NodeProperties { x: -5.0, y: 0.0, radius: 1.0, color: "yellow".to_string() });
    node_properties.insert(e, NodeProperties { x: 0.0, y: 0.0, radius: 1.0, color: "purple".to_string() });

    // Displays node properties
    println!("Node properties:\n");
    for (node, props) in &node_properties {
        println!(
            "Node {:?}: position: ({}, {}), radius: {}, color: {}",
            node.index(),
            props.x,
            props.y,
            props.radius,
            props.color
        );
    }
    println!("\n");

    // Adds all interactions as EDGES
    let interactions = vec![
        Interaction::Activate,
        Interaction::Apoptosis,
        Interaction::Consume,
        Interaction::Decay,
        Interaction::Differentiate,
        Interaction::Infect,
        Interaction::Inhibit,
        Interaction::Kill,
        Interaction::Phagocytosis,
        Interaction::Polarize,
        Interaction::Secrete,
        Interaction::Replicate,
        Interaction::Recruit,
    ];

    // Linking nodes
    let nodes = vec![a, b, c, d, e];
    for (i, interaction) in interactions.iter().enumerate() {
        let source = nodes[i % nodes.len()];
        let target = nodes[(i + 2) % nodes.len()];

        g.add_edge(
            source,
            target,
            LinkType {
                interaction: interaction.clone(),
                signal: interaction_signal_map(interaction),
            },
        );
        if i > 3 {
            break;
        }
    }

    let neg_links_s = non_positive_output(&g);
    println!("Nodes with negative output links:\n");
    for node in neg_links_s {
        println!("{:?}\n", g[node]);
    }

    let neg_links_e = non_positive_input(&g);
    println!("Nodes with non-positive input links:\n");
    for node in neg_links_e {
        println!("{:?}\n", g[node]);
    }

    let not_neg_e = non_negative_input(&g);
    println!("Nodes with non-negative input links:\n");
    for node in not_neg_e {
        println!("{:?}\n", g[node]);
    }

    // Creates the path bank
    let mut path_bank = PathBank::new();

    // Adds paths to the bank
    for i in 0..4 {
        let mut path = Path::new();
        for edge in g.edge_references().skip(i).take(3) {
            path.add_edge(edge.id(), edge.weight().interaction.clone());
        }
        path_bank.add_path(path);
    }
    println!("Path bank created with {} paths.\n", path_bank.count());
    for (i, path) in path_bank.get_paths().iter().enumerate() {
        println!("Path {}: {:?}, Length: {}", i + 1, path, path.length());
    }
    println!("\n");

    let negative_path = path_bank.negative_signal_edge(&g);
    println!(
        "Set of paths with at least one edge having a negative signal associated with the target node:"
    );
    for (i, path) in negative_path.iter().enumerate() {
        println!("Path {}: {:?}, Length: {}", i + 1, path, path.length());
    }
    println!("\n");

    let non_negative_path = path_bank.non_negative_signal_edge(&g);
    println!(
        "Set of paths with at least one edge having a non-negative signal associated with the target node:"
    );
    for (i, path) in non_negative_path.iter().enumerate() {
        println!("Path {}: {:?}, Length: {}", i + 1, path, path.length());
    }
    println!("\n");

    let unreachable_nodes = nodes_without_negative_signal(&g);
    println!("Nodes that are not reached by paths with a negative signal:");
    for node in unreachable_nodes {
        println!("{:?}\n", g[node]);
    }

    // Displays the graph    
    for edge in g.edge_references() {
        println!(
            "Edge from {:?} to {:?} has interaction {:?} with signals {:?}\n",
            edge.source(),
            edge.target(),
            edge.weight().interaction,
            edge.weight().signal
        );
    }
}
