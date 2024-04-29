// ... other imports ...

use crate::models::PlayerShooting;
use csv::ReaderBuilder;
use std::fs::File;
use std::io::{self, BufReader};

pub fn read_player_shooting(file_path: &str) -> io::Result<Vec<PlayerShooting>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);

    let mut data: Vec<PlayerShooting> = Vec::new();
    for result in csv_reader.deserialize() {
        match result {
            Ok(record) => data.push(record),
            Err(e) => {
                eprintln!("Failed to deserialize a row: {}", e);
                // Depending on your needs, you might want to continue processing
                // or return an error.
            }
        }
    }
    Ok(data)
}


pub fn read_team_stats(file_path: &str) -> io::Result<Vec<TeamStats>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);

    let mut data: Vec<TeamStats> = Vec::new();
    for result in csv_reader.deserialize() {
        let record: TeamStats = result?;
        data.push(record);
    }
    Ok(data)
}
