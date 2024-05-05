use std::error::Error;
use csv::Writer;
use rustworkx_core::centrality::{betweenness_centrality, closeness_centrality};
use std::collections::HashMap;
use crate::graph::Graph;

pub fn calculate_centrality(
    graph: &Graph,
    node_labels: &HashMap<u32, String>,
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    let betweenness_scores: HashMap<u32, f64> = betweenness_centrality(&graph.graph, true, true, graph.graph.node_count())
        .into_iter()
        .flatten()
        .zip(graph.graph.node_weights())
        .map(|(score, node_id)| (*node_id, score))
        .collect();

    let closeness_scores: HashMap<u32, f64> = closeness_centrality(&graph.graph, false)
        .into_iter()
        .flatten()
        .zip(graph.graph.node_weights())
        .map(|(score, node_id)| (*node_id, score))
        .collect();

    let mut writer = Writer::from_path(file_path)?;
    writer.write_record(&["Node ID", "Label", "Betweenness Centrality", "Closeness Centrality"])?;

    for node_id in graph.graph.node_weights() {
        let label = node_labels.get(node_id).cloned().unwrap_or_else(|| "Unknown".to_string());
        let betweenness_score = *betweenness_scores.get(node_id).unwrap_or(&0.0);
        let closeness_score = *closeness_scores.get(node_id).unwrap_or(&0.0);

        let betweenness_score_str = if betweenness_score != 0.0 {
            betweenness_score.to_string()
        } else {
            "0.0".to_string()
        };

        let closeness_score_str = if closeness_score != 0.0 {
            closeness_score.to_string()
        } else {
            "0.0".to_string()
        };

        writer.write_record(&[
            node_id.to_string(),
            label,
            betweenness_score_str,
            closeness_score_str,
        ])?;
    }

    writer.flush()?;
    Ok(())
}