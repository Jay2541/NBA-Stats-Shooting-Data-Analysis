use petgraph::Graph as PetGraph;
use petgraph::graph::NodeIndex;
use std::collections::HashMap;

// Assuming NodeData and EdgeData are defined as follows:
#[derive(Debug, Clone)]
pub enum NodeData {
    Player(Player),
    Team(Team),
}

#[derive(Debug, Clone)]
pub struct EdgeData {
    relationship: String,
    weight: f32,
}

pub struct BasketballGraph {
    pub graph: PetGraph<NodeData, EdgeData>,
    pub node_indices: HashMap<String, NodeIndex>,
}

impl BasketballGraph {
    pub fn new() -> Self {
        BasketballGraph {
            graph: PetGraph::new(),
            node_indices: HashMap::new(),
        }
    }

    // Constructs a graph from merged player and team data
    pub fn construct_from_data(&mut self, data: &[MergedData]) {
        for entry in data {
            let player_node = self.add_player(&entry.player);
            let team_node = self.add_team(&entry.team);

            // Example: Connect every player to their respective team
            self.graph.add_edge(player_node, team_node, EdgeData {
                relationship: "plays for".to_string(),
                weight: 1.0,  // Example weight, could be based on some specific statistics
            });
        }
    }

    pub fn add_player(&mut self, player: &Player) -> NodeIndex {
        let node = self.node_indices.entry(player.name.clone()).or_insert_with(|| {
            self.graph.add_node(NodeData::Player(player.clone()))
        });
        *node
    }

    pub fn add_team(&mut self, team: &Team) -> NodeIndex {
        let node = self.node_indices.entry(team.name.clone()).or_insert_with(|| {
            self.graph.add_node(NodeData::Team(team.clone()))
        });
        *node
    }
}