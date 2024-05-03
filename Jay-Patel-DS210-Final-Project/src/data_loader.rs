use csv::ReaderBuilder;
use serde::Deserialize;
use std::fs::File;
use std::io::Result;
use crate::data_structures::{Player, Team};

pub fn load_players(filepath: &str) -> Result<Vec<Player>> {
    let file = File::open(filepath)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    rdr.deserialize().collect()
}

pub fn load_teams(filepath: &str) -> Result<Vec<Team>> {
    let file = File::open(filepath)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    rdr.deserialize().collect()
}