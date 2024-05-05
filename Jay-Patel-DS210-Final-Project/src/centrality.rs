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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;
    use std::collections::HashMap;

    #[test]
    fn test_calculate_centrality() {
        let mut graph = Graph::new();

        let node1 = graph.add_node(1);
        let node2 = graph.add_node(2);
        let node3 = graph.add_node(3);
        let node4 = graph.add_node(4);

        graph.add_edge(1, 2, 1.0);
        graph.add_edge(1, 3, 2.0);
        graph.add_edge(2, 3, 3.0);
        graph.add_edge(3, 4, 4.0);

        let node_labels = HashMap::from([
            (1, "Node 1".to_string()),
            (2, "Node 2".to_string()),
            (3, "Node 3".to_string()),
            (4, "Node 4".to_string()),
        ]);

        let result = calculate_centrality(&graph, &node_labels, "Centrality Test.csv");
        assert!(result.is_ok());
        let file_contents = std::fs::read_to_string("Centrality Test.csv").unwrap();

        assert!(file_contents.contains("Node ID,Label,Betweenness Centrality,Closeness Centrality"));
        assert!(file_contents.contains("1,Node 1,0.25,0.0"));
        assert!(file_contents.contains("2,Node 2,0.25,1"));
        assert!(file_contents.contains("3,Node 3,0.41666666666666663,1"));
        assert!(file_contents.contains("4,Node 4,0.25,0.6"));

        std::fs::remove_file("Centrality Test.csv").unwrap();
    }
}