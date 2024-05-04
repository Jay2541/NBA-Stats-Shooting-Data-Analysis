mod data_loader;
mod data_merger;
mod analytics;
mod visualization;
mod graph;
mod centrality;
mod data_structures;

use data_loader::{load_player_data, load_team_data};
use data_merger::merge_data;
use analytics::{correlate_statistics, analyze_playoff_correlation, write_correlations_to_csv};
use centrality::calculate_centrality;
use graph::Graph;

fn main() {
    let player_data = load_player_data("NBA Stats (1947-Present)/Player Shooting.csv").unwrap();
    let team_data = load_team_data("NBA Stats (1947-Present)/Team Stats Per Game.csv").unwrap();
    let merged_data = merge_data(&player_data, &team_data);

    let mut graph = Graph::new();
    graph.construct_from_data(&merged_data);

    let petgraph = graph.into();
    let centrality_scores = calculate_centrality(&petgraph);

    let correlation_results = correlate_statistics(&merged_data);
    let playoff_correlation_results = analyze_playoff_correlation(&merged_data);

    write_correlations_to_csv(&correlation_results, "correlation_results.csv").unwrap();

    let positive_correlations: Vec<_> = playoff_correlation_results.positive_correlations.values().cloned().collect();
    write_correlations_to_csv(&positive_correlations, "positive_correlations.csv").unwrap();

    let negative_correlations: Vec<_> = playoff_correlation_results.negative_correlations.values().cloned().collect();
    write_correlations_to_csv(&negative_correlations, "negative_correlations.csv").unwrap();

    // display_results(&correlation_results, &playoff_correlation_results);
    // visualize_graph(&graph, &centrality_scores);
}