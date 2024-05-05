#[derive(Clone, Debug)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub team_abbreviation: String,
    pub season: u32,
    pub fg_percent: f64,
    pub fg_percent_from_x2p_range: f64,
    pub fg_percent_from_x3p_range: f64,
}

#[derive(Clone, Debug)]
pub struct Team {
    pub abbreviation: String,
    pub name: String,
    pub season: u32,
    pub playoffs: bool,
    pub fg_percentage: f64,
    pub two_point_percentage: f64,
    pub three_point_percentage: f64,
    pub points_per_game: f64,
}

#[derive(Clone, Debug)]
pub struct MergedData {
    pub player: Player,
    pub team: Team,
}