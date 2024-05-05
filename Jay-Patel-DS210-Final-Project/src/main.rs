mod data_loader;
mod analytics;
mod graph;
mod centrality;
mod data_structures;

use data_loader::{load_player_data, load_team_data};
use data_structures::{Player, Team, MergedData};
use analytics::{
    correlate_statistics, analyze_playoff_correlation, write_correlations_to_csv,
    CorrelationResult, PlayoffCorrelationResults,
};
use centrality::calculate_centrality;
use graph::Graph;
use std::collections::HashMap;

fn filter_data_by_season(
    player_data: &[Player],
    team_data: &[Team],
    season: u32,
) -> (HashMap<String, Vec<Player>>, Vec<Team>) {
    let mut players_by_team: HashMap<String, Vec<Player>> = HashMap::new();
    let filtered_teams: Vec<Team> = team_data
        .iter()
        .filter(|t| t.season == season)
        .cloned()
        .collect();

    for player in player_data.iter().filter(|p| p.season == season) {
        players_by_team
            .entry(player.team_abbreviation.clone())
            .or_insert_with(Vec::new)
            .push(player.clone());
    }

    (players_by_team, filtered_teams)
}

use petgraph::visit::EdgeRef;

fn main() {
    let player_data = load_player_data("NBA Stats (1947-Present)/Player Shooting.csv").unwrap();
    let team_data = load_team_data("NBA Stats (1947-Present)/Team Stats Per Game.csv").unwrap();

    let season = 2022;
    let (players_by_team, filtered_teams) = filter_data_by_season(&player_data, &team_data, season);

    let mut correlation_merged_data = Vec::new();
    for team in &filtered_teams {
        if let Some(team_players) = players_by_team.get(&team.abbreviation) {
            let player = &team_players[0];
            correlation_merged_data.push(MergedData {
                player: player.clone(),
                team: team.clone(),
            });
        }
    }

    let mut centrality_merged_data = Vec::new();
    for team in &filtered_teams {
        if let Some(team_players) = players_by_team.get(&team.abbreviation) {
            for player in team_players {
                centrality_merged_data.push(MergedData {
                    player: player.clone(),
                    team: team.clone(),
                });
            }
        }
    }

    let mut graph = Graph::new();
    graph.construct_from_data(&centrality_merged_data);

    let mut node_labels = HashMap::new();
    for data in &centrality_merged_data {
        node_labels.insert(data.player.id, format!("{} (Player)", data.player.name));
        node_labels.insert(data.team.abbreviation.as_bytes().iter().map(|&b| b as u32).sum(), format!("{} (Team)", data.team.abbreviation));
    }

    let _ = calculate_centrality(&graph, &node_labels, "centrality_scores.csv").unwrap();

    let player_analytics = correlate_statistics(&correlation_merged_data);
    let playoff_correlation = analyze_playoff_correlation(&correlation_merged_data);

    write_correlations_to_csv(&player_analytics, "Player Shooting Stats Analytics.csv", true).unwrap();

    let all_players_playoffs: Vec<CorrelationResult> = playoff_correlation
        .all_players_correlation
        .values()
        .cloned()
        .collect();
    write_correlations_to_csv(&all_players_playoffs, "Players' Contribution To Team.csv", false).unwrap();
}