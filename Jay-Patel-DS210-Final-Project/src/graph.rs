use std::collections::HashMap;
use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum NodeType {
    Player(String),
    Statistic(String),
    Team(String),
}

pub struct Graph {
    pub nodes: Vec<NodeType>,
    pub edges: HashMap<NodeType, Vec<NodeType>>,
}

impl Graph {
    pub fn new() -> Self {
        Self { nodes: Vec::new(), edges: HashMap::new() }
    }

    pub fn add_node(&mut self, node: NodeType) {
        if !self.nodes.contains(&node) {
            self.nodes.push(node);
        }
    }

    pub fn add_edge(&mut self, from: &NodeType, to: &NodeType) {
        self.edges.entry(from.clone()).or_insert_with(Vec::new).push(to.clone());
    }
}
