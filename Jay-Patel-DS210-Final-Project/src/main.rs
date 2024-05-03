mod data_loader;
mod data_merger;
mod analytics;
mod visualization;
mod graph;
mod centrality;
mod data_structures;

use data_loader::{load_players, load_teams};
use data_merger::merge_data;
use analytics::{correlate_statistics, analyze_playoff_correlation};
use visualization::{visualize_centrality_scores};
use centrality::calculate_betweenness_centrality;
use graph::BasketballGraph;

fn main() {
    let player_data = load_players("data/player_shooting.csv");
    println!("{}", player_data);
    let team_data = load_teams("data/team_stats_per_game.csv");
    let merged_data = merge_data(&player_data, &team_data);

    let mut graph = BasketballGraph::new();
    graph.construct_from_data(&merged_data);

    let centrality_scores = calculate_betweenness_centrality(&graph);
    let correlation_results = correlate_statistics(&merged_data);
    let playoff_correlation_results = analyze_playoff_correlation(&merged_data);

    display_results(&correlation_results, &playoff_correlation_results);
    visualize_centrality_scores(&graph, &centrality_scores);
}
