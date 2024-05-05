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
        let mut players_by_team: HashMap<String, Vec<&MergedData>> = HashMap::new();
    
        // Group players by team
        for data in merged_data {
            if data.player.season == 2022 {
                players_by_team
                    .entry(data.team.abbreviation.clone())
                    .or_insert_with(Vec::new)
                    .push(data);
            }
        }
    
        // Create nodes and edges
        for (_, team_players) in &players_by_team {
            let mut player_nodes: HashMap<u32, NodeIndex<u32>> = HashMap::new();
    
            for player_data in team_players {
                let player_id = player_data.player.id;
                let node_index = self.add_node(player_id);
                player_nodes.insert(player_id, node_index);
            }
    
            for (player1_id, player1_node) in &player_nodes {
                for (player2_id, player2_node) in &player_nodes {
                    if player1_id != player2_id {
                        let player1_data = team_players.iter().find(|data| &data.player.id == player1_id).unwrap();
                        let player2_data = team_players.iter().find(|data| &data.player.id == player2_id).unwrap();
                        let weight = Self::calculate_weight(player1_data, player2_data);
                        self.add_edge(*player1_id, *player2_id, weight);
                    }
                }
            }
        }
    }

    fn calculate_weight(player1: &MergedData, player2: &MergedData) -> f64 {
        let player1_fg_percent = player1.player.fg_percent;
        let player2_fg_percent = player2.player.fg_percent;
        let fg_percent_diff = (player1_fg_percent - player2_fg_percent).abs();

        let player1_points_per_game = player1.team.points_per_game;
        let player2_points_per_game = player2.team.points_per_game;
        let points_per_game_diff = (player1_points_per_game - player2_points_per_game).abs();

        let player1_three_point_percent = player1.player.fg_percent_from_x3p_range;
        let player2_three_point_percent = player2.player.fg_percent_from_x3p_range;
        let three_point_percent_diff = (player1_three_point_percent - player2_three_point_percent).abs();

        // Combine the differences using a weighted sum
        let weight = 0.5 * fg_percent_diff + 0.3 * points_per_game_diff + 0.2 * three_point_percent_diff;

        weight
    }

    pub fn get_node_weight(&self, node_id: u32) -> Option<&u32> {
        self.graph.node_weight(*self.node_map.get(&node_id)?)
    }

    pub fn get_neighbors(&self, node_id: u32) -> Vec<u32> {
        let node_idx = self.node_map.get(&node_id).unwrap();
        self.graph
            .neighbors_undirected(*node_idx)
            .map(|neighbor_idx| *self.graph.node_weight(neighbor_idx).unwrap())
            .collect()
    }
}