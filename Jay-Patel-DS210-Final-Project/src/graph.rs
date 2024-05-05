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
    
        for data in merged_data {
            if data.player.season == 2022 {
                players_by_team
                    .entry(data.team.abbreviation.clone())
                    .or_insert_with(Vec::new)
                    .push(data);
            }
        }
    
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_structures::{Player, Team, MergedData};

    #[test]
    fn test_add_node() {
        let mut graph = Graph::new();
        let node_id = 1;
        let node_index = graph.add_node(node_id);
        assert_eq!(graph.graph.node_count(), 1);
        assert_eq!(graph.graph.node_weight(node_index), Some(&node_id));
    }

    #[test]
    fn test_add_edge() {
        let mut graph = Graph::new();
        let source_id = 1;
        let target_id = 2;
        let weight = 0.5;
        let source_node = graph.add_node(source_id);
        let target_node = graph.add_node(target_id);
        graph.add_edge(source_id, target_id, weight);
        assert_eq!(graph.graph.edge_count(), 1);
        if let Some(edge_index) = graph.graph.find_edge(source_node, target_node) {
            assert_eq!(graph.graph.edge_weight(edge_index), Some(&weight));
        } else {
            panic!("Edge not found between source and target nodes");
        }
    }

    #[test]
    fn test_construct_from_data() {
        let mut graph = Graph::new();
        let player1 = Player {
            id: 1,
            name: "Player 1".to_string(),
            team_abbreviation: "TEA".to_string(),
            season: 2022,
            fg_percent: 0.5,
            fg_percent_from_x2p_range: 0.6,
            fg_percent_from_x3p_range: 0.4,
        };
        let player2 = Player {
            id: 2,
            name: "Player 2".to_string(),
            team_abbreviation: "TEA".to_string(),
            season: 2022,
            fg_percent: 0.6,
            fg_percent_from_x2p_range: 0.7,
            fg_percent_from_x3p_range: 0.5,
        };
        let team = Team {
            abbreviation: "TEA".to_string(),
            name: "Team A".to_string(),
            season: 2022,
            playoffs: true,
            fg_percentage: 0.55,
            two_point_percentage: 0.65,
            three_point_percentage: 0.45,
            points_per_game: 100.0,
        };
        let merged_data = vec![
            MergedData { player: player1, team: team.clone() },
            MergedData { player: player2, team },
        ];
        graph.construct_from_data(&merged_data);
        assert_eq!(graph.graph.node_count(), 2);
        assert_eq!(graph.graph.edge_count(), 2);
    }

    #[test]
    fn test_get_node_weight() {
        let mut graph = Graph::new();
        let node_id = 1;
        let node_index = graph.add_node(node_id);
        assert_eq!(graph.get_node_weight(node_id), Some(&node_id));
    }

    #[test]
    fn test_get_neighbors() {
        let mut graph = Graph::new();
        let node1_id = 1;
        let node2_id = 2;
        let node3_id = 3;
        graph.add_node(node1_id);
        graph.add_node(node2_id);
        graph.add_node(node3_id);
        graph.add_edge(node1_id, node2_id, 0.5);
        graph.add_edge(node1_id, node3_id, 0.7);
        let neighbors = graph.get_neighbors(node1_id);
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&node2_id));
        assert!(neighbors.contains(&node3_id));
    }
}