use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub team_abbreviation: String,
    pub fg_percent_3p: f32,
    pub fg_percent_2p: f32,
    pub avg_dist_fga: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub abbreviation: String,
    pub avg_points_per_game: f32,
    pub made_playoffs: bool,
}

#[derive(Debug, Clone)]
pub struct MergedData {
    pub player: Player,
    pub team: Team,
}

#[derive(Debug)]
pub struct CorrelationResults {
    pub player_name: String,
    pub fg_3p_pts_correlation: f64,
    pub fg_2p_pts_correlation: f64,
}

#[derive(Debug)]
pub struct PlayoffCorrelationResults {
    pub correlation_with_playoffs: f64,
}