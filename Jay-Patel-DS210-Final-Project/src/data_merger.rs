use crate::data_structures::{Player, Team, MergedData};
use std::collections::HashMap;

pub fn merge_data(players: Vec<Player>, teams: Vec<Team>) -> Vec<MergedData> {
    let team_map: HashMap<String, Team> = teams.into_iter()
        .map(|team| (team.abbreviation.clone(), team))
        .collect();

    players.into_iter().filter_map(|player| {
        team_map.get(&player.team_abbreviation).map(|team| {
            MergedData {
                player,
                team: team.clone(),
            }
        })
    }).collect()
}