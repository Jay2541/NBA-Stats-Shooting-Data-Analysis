use csv::Reader;
use std::error::Error;
use crate::data_structures::{Player, Team};

pub fn load_player_data(file_path: &str) -> Result<Vec<Player>, Box<dyn Error>> {
    let mut reader = Reader::from_path(file_path)?;
    let mut players = Vec::new();

    // Skip the header row
    reader.records().next();

    for result in reader.records() {
        let record = result?;

        let player = Player {
            id: match record[2].parse() {
                Ok(id) => id,
                Err(e) => {
                    eprintln!("Error parsing player ID '{}': {:?}", record[2].to_string(), e);
                    continue;
                }
            },
            name: record[3].to_string(),
            team_abbreviation: record[9].to_string(),
            fg_percent: match record[12].parse() {
                Ok(percent) => percent,
                Err(e) => {
                    eprintln!("Error parsing FG percent '{}': {:?}", record[12].to_string(), e);
                    continue;
                }
            },
            avg_dist_fga: match record[13].parse() {
                Ok(dist) => dist,
                Err(e) => {
                    eprintln!("Error parsing average distance FGA '{}': {:?}", record[13].to_string(), e);
                    continue;
                }
            },
            fg_percent_from_x2p_range: match record[20].parse() {
                Ok(percent) => percent,
                Err(_) => 0.0, // Use a default value of 0.0 if parsing fails
            },
            fg_percent_from_x3p_range: match record[21].parse() {
                Ok(percent) => percent,
                Err(_) => 0.0, // Use a default value of 0.0 if parsing fails
            },
        };

        players.push(player);
    }

    Ok(players)
}

pub fn load_team_data(file_path: &str) -> Result<Vec<Team>, Box<dyn Error>> {
    let mut reader = Reader::from_path(file_path)?;
    let mut teams = Vec::new();

    // Skip the header row
    reader.records().next();

    for result in reader.records() {
        let record = result?;

        let team = Team {
            abbreviation: record[2].to_string(),
            name: record[3].to_string(),
            playoffs: match record[4].parse() {
                Ok(playoffs) => playoffs,
                Err(e) => {
                    eprintln!("Error parsing playoffs '{}': {:?}", record[4].to_string(), e);
                    continue;
                }
            },
            fg_percentage: match record[9].parse() {
                Ok(percentage) => percentage,
                Err(e) => {
                    eprintln!("Error parsing FG percentage '{}': {:?}", record[9].to_string(), e);
                    continue;
                }
            },
            two_point_percentage: match record[15].parse() {
                Ok(percentage) => percentage,
                Err(e) => {
                    eprintln!("Error parsing 2P percentage '{}': {:?}", record[15].to_string(), e);
                    continue;
                }
            },
            three_point_percentage: match record[12].parse() {
                Ok(percentage) => percentage,
                Err(e) => {
                    eprintln!("Error parsing 3P percentage '{}': {:?}", record[12].to_string(), e);
                    continue;
                }
            },
            points_per_game: match record[27].parse() {
                Ok(points) => points,
                Err(e) => {
                    eprintln!("Error parsing points per game '{}': {:?}", record[27].to_string(), e);
                    continue;
                }
            },
        };

        teams.push(team);
    }

    Ok(teams)
}