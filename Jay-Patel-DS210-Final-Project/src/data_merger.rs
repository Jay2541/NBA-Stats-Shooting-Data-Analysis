use crate::data_structures::{MergedData, Player, Team};
use std::collections::HashMap;

pub fn merge_data(player_data: &[Player], team_data: &[Team]) -> Vec<MergedData> {
    let mut merged_data = Vec::new();

    // Group players by team abbreviation
    let mut players_by_team: HashMap<String, Vec<Player>> = HashMap::new();
    for player in player_data {
        players_by_team
            .entry(player.team_abbreviation.clone())
            .or_insert_with(Vec::new)
            .push(player.clone());
    }

    // Match teams with their players
    for team in team_data {
        if let Some(team_players) = players_by_team.get(&team.abbreviation) {
            for player in team_players {
                merged_data.push(MergedData {
                    player: player.clone(),
                    team: team.clone(),
                });
            }
        }
    }

    merged_data
}