use crate::data_structures::{MergedData, Player, Team};

pub fn merge_data(player_data: &[Player], team_data: &[Team]) -> Vec<MergedData> {
    let mut merged_data = Vec::new();

    for player in player_data {
        for team in team_data {
            if player.team_abbreviation == team.abbreviation {
                merged_data.push(MergedData {
                    player: player.clone(),
                    team: team.clone(),
                });
            }
        }
    }

    merged_data
}