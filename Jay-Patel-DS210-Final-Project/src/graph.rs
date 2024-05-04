use petgraph::{Graph as PetGraph, graph::NodeIndex};
use std::collections::HashMap;
use crate::data_structures::MergedData;

pub struct Graph {
    pub graph: PetGraph<u32, f64>,
    node_map: HashMap<u32, NodeIndex<u32>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            graph: PetGraph::new(),
            node_map: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: u32) -> NodeIndex<u32> {
        let node = self.graph.add_node(id);
        self.node_map.insert(id, node);
        node
    }

    pub fn add_edge(&mut self, source: u32, target: u32, weight: f64) {
        let source_node = self.node_map.get(&source).unwrap();
        let target_node = self.node_map.get(&target).unwrap();
        self.graph.add_edge(*source_node, *target_node, weight);
    }

    pub fn construct_from_data(&mut self, merged_data: &[MergedData]) {
        for data in merged_data {
            self.add_node(data.player.id);
            self.add_node(data.team.abbreviation.as_bytes().iter().map(|&b| b as u32).sum());
        }

        for data in merged_data {
            let player_node = self.node_map.get(&data.player.id).unwrap();
            let team_node = self.node_map.get(&data.team.abbreviation.as_bytes().iter().map(|&b| b as u32).sum()).unwrap();
            self.graph.add_edge(*player_node, *team_node, data.player.fg_percent);
        }
    }

    pub fn get_node_weight(&self, node_id: u32) -> Option<&u32> {
        self.graph.node_weight(*self.node_map.get(&node_id)?)
    }

    pub fn get_edge_weight(&self, source: u32, target: u32) -> Option<&f64> {
        let source_node = self.node_map.get(&source)?;
        let target_node = self.node_map.get(&target)?;
    
        self.graph
            .edge_weight(self.graph.find_edge(*source_node, *target_node).unwrap())
    }

    pub fn get_neighbors(&self, node_id: u32) -> Vec<u32> {
        let node_idx = self.node_map.get(&node_id).unwrap();
        self.graph
            .neighbors_undirected(*node_idx)
            .map(|neighbor_idx| *self.graph.node_weight(neighbor_idx).unwrap())
            .collect()
    }
}