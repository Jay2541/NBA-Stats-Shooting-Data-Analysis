use petgraph::algo::floyd_warshall;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::prelude::*;
use std::collections::HashMap;

pub fn calculate_betweenness_centrality<N, E>(graph: &UnGraph<N, E>) -> HashMap<NodeIndex, f64> 
where N: Clone, E: Clone + Into<f64> {
    let mut centrality_scores = HashMap::new();
    let node_count = graph.node_count();
    let shortest_paths = floyd_warshall(&graph, |edge| *edge.weight());

    // Initialize centrality score for each node
    for node in graph.node_indices() {
        centrality_scores.insert(node, 0.0);
    }

    // Calculate betweenness centrality
    for (s, paths) in shortest_paths.iter().enumerate() {
        let s_index = NodeIndex::new(s);
        for (t, paths_to_t) in paths.iter().enumerate() {
            let t_index = NodeIndex::new(t);
            if s_index != t_index {
                for (v, path_count) in paths_to_t.iter().enumerate() {
                    let v_index = NodeIndex::new(v);
                    if v_index != s_index && v_index != t_index {
                        let through_v = path_count / paths_to_t[t];
                        let current_centrality = centrality_scores.get_mut(&v_index).unwrap();
                        *current_centrality += through_v;
                    }
                }
            }
        }
    }

    // Normalize centrality scores
    for centrality in centrality_scores.values_mut() {
        *centrality /= (node_count - 1) as f64 * (node_count - 2) as f64;
    }

    centrality_scores
}