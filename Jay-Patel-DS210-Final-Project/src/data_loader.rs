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
            season: record[1].parse()?,
            fg_percent: if &record[12] == "NA" {
                0.0
            } else {
                record[12].parse().unwrap_or_else(|_| {
                    eprintln!("Error parsing FG percent '{}': Invalid value", record[12].to_string());
                    0.0
                })
            },
            // avg_dist_fga: if &record[13] == "NA" {
            //     0.0
            // } else {
            //     record[13].parse().unwrap_or_else(|_| {
            //         eprintln!("Error parsing average distance FGA '{}': Invalid value", record[13].to_string());
            //         0.0
            //     })
            // },
            fg_percent_from_x2p_range: if &record[20] == "NA" {
                0.0
            } else {
                record[20].parse().unwrap_or_else(|_| {
                    eprintln!("Error parsing FG percent from 2P range '{}': Invalid value", record[20].to_string());
                    0.0
                })
            },
            fg_percent_from_x3p_range: if &record[25] == "NA" {
                0.0
            } else {
                record[25].parse().unwrap_or_else(|_| {
                    eprintln!("Error parsing FG percent from 3P range '{}': Invalid value", record[25].to_string());
                    0.0
                })
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
    //reader.records().next();

    for result in reader.records() {
        let record = result?;

        let playoffs = match record[4].to_lowercase().as_str() {
            "true" => true,
            "false" => false,
            _ => {
                eprintln!("Error parsing playoffs '{}': Invalid value", record[4].to_string());
                continue;
            }
        };

        let three_point_percentage = if record[12].to_string() == "NA" {
            0.0
        } else {
            match record[12].parse() {
                Ok(percentage) => percentage,
                Err(e) => {
                    eprintln!("Error parsing 3P percentage '{}': {:?}", record[12].to_string(), e);
                    continue;
                }
            }
        };

        let fg_percentage = if record[9].to_string() == "NA" {
            0.0
        } else {
            match record[9].parse() {
                Ok(percentage) => percentage,
                Err(e) => {
                    eprintln!("Error parsing FG percentage '{}': {:?}", record[9].to_string(), e);
                    continue;
                }
            }
        };

        let two_point_percentage = if record[15].to_string() == "NA" {
            0.0
        } else {
            match record[15].parse() {
                Ok(percentage) => percentage,
                Err(e) => {
                    eprintln!("Error parsing 2P percentage '{}': {:?}", record[15].to_string(), e);
                    continue;
                }
            }
        };

        let points_per_game = if record[27].to_string() == "NA" {
            0.0
        } else {
            match record[27].parse() {
                Ok(points) => points,
                Err(e) => {
                    eprintln!("Error parsing points per game '{}': {:?}", record[27].to_string(), e);
                    continue;
                }
            }
        };

        let team = Team {
            abbreviation: record[3].to_string(),
            name: record[2].to_string(),
            season: record[0].parse()?,
            playoffs,
            fg_percentage,
            two_point_percentage,
            three_point_percentage,
            points_per_game,
        };

        teams.push(team);
    }

    Ok(teams)
}