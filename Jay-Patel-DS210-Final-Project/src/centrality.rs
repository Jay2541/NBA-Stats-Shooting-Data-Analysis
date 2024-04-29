use crate::graph::{Graph, NodeType};
use std::collections::HashMap;

pub fn calculate_degree_centrality(graph: &Graph) -> HashMap<NodeType, usize> {
    graph.edges.iter().map(|(node, edges)| (node.clone(), edges.len())).collect()
}

pub fn calculate_closeness_centrality(graph: &Graph) -> HashMap<NodeType, f64> {
    let mut closeness_scores = HashMap::new();

    for node in &graph.nodes {
        let total_distance = graph.nodes.iter().filter_map(|other| {
            if node != other {
                Some(1.0) // Dummy implementation, replace with actual pathfinding logic
            } else {
                None
            }
        }).sum::<f64>();

        let closeness = if total_distance > 0.0 {
            1.0 / total_distance
        } else {
            0.0
        };

        closeness_scores.insert(node.clone(), closeness);
    }

    closeness_scores
}
