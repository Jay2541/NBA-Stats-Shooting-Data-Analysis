use crate::graph::Graph;
use rustworkx_core::centrality::{betweenness_centrality, closeness_centrality};
use std::collections::HashMap;

pub fn calculate_centrality(graph: &Graph) -> HashMap<u32, f64> {
    let mut centrality_scores = HashMap::new();

    // Betweenness centrality
    let betweenness_centrality_scores = betweenness_centrality(&graph.graph, true, true, graph.graph.node_count());
    for (node_id, score) in graph.graph.node_weights().zip(betweenness_centrality_scores.iter().flatten()) {
        *centrality_scores.entry(*node_id).or_insert(0.0) += score;
    }

    // Closeness centrality
    let closeness_centrality_scores = closeness_centrality(&graph.graph, false);
    for (node_id, score) in graph.graph.node_weights().zip(closeness_centrality_scores.iter().flatten()) {
        *centrality_scores.entry(*node_id).or_insert(0.0) += score;
    }

    // let degree_centrality_scores = degree_centrality(&graph);
    // for (node_id, score) in degree_centrality_scores {
    //     centrality_scores.insert(node_id, score);
    // }

    centrality_scores
}