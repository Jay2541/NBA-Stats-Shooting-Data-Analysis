// // visualization.rs
// use plotters::prelude::Text;
// use crate::graph::Graph;
// use std::collections::HashMap;
// use plotters::backend::{BitMapBackend};
// use plotters::prelude::WHITE;
// use crate::analytics::CorrelationResult;
// use crate::analytics::PlayoffCorrelationResults;

// pub fn visualize_graph(graph: &Graph, centrality_scores: &HashMap<u32, f64>) {
//     let mut nodes = Vec::new();
//     let mut edges = Vec::new();

//     for (node_id, centrality_score) in centrality_scores {
//         let node_label = format!("{} ({:.2})", node_id, centrality_score);
//         nodes.push(Text::new(node_label, (0, 0), ("sans-serif", 10)));
//     }

//     for edge in graph.graph.edge_indices() {
//         let source = graph.graph.node_weight(edge.source()).unwrap();
//         let target = graph.graph.node_weight(edge.target()).unwrap();
//         let edge_weight = graph.graph.edge_weight(edge).unwrap();
//         let edge_label = format!("{:.2}", edge_weight);
//         edges.push(BitMapBackendImage::new(edge_label).into_drawing_area().into_element());
//     }

//     let layout = std::iter::once(nodes.into_iter())
//         .chain(std::iter::once(edges.into_iter()))
//         .collect::<std::vec::Vec<_>>();

//     let root_area = BitMapBackend::new("graph.png", (1024, 768))
//         .into_drawing_area();
//     root_area.fill(&WHITE).unwrap();
//     root_area.draw(&layout.as_slice());
// }

// pub fn display_results(
//     correlation_results: &[CorrelationResult],
//     playoff_correlation_results: &PlayoffCorrelationResults,
// ) {
//     let mut positive_correlations = Vec::new();
//     let mut negative_correlations = Vec::new();

//     for result in correlation_results {
//         println!(
//             "Player ID: {}, Correlation Coefficient: {:.2}",
//             result.player_id, result.correlation_coefficient
//         );
//     }

//     for (player_id, result) in &playoff_correlation_results.positive_correlations {
//         positive_correlations.push(
//             BitMapBackendImage::new(format!(
//                 "{}: {:.2}",
//                 player_id, result.correlation_coefficient
//             ))
//             .into_drawing_area()
//             .into_element(),
//         );
//     }

//     for (player_id, result) in &playoff_correlation_results.negative_correlations {
//         negative_correlations.push(
//             BitMapBackendImage::new(format!(
//                 "{}: {:.2}",
//                 player_id, result.correlation_coefficient
//             ))
//             .into_drawing_area()
//             .into_element(),
//         );
//     }

//     let layout = std::iter::once(positive_correlations.into_iter())
//         .chain(std::iter::once(negative_correlations.into_iter()))
//         .collect::<std::vec::Vec<_>>();

//     let root_area = BitMapBackend::new("correlations.png", (1024, 768))
//         .into_drawing_area();
//     root_area.fill(&WHITE).unwrap();
//     root_area.draw(&layout.as_slice());
// }