use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PlayerShooting {
    #[serde(rename = "seas_id")]
    pub season_id: i32,

    #[serde(rename = "season")]
    pub season: i32,

    #[serde(rename = "player_id")]
    pub player_id: i32,

    #[serde(rename = "player")]
    pub player_name: String,

    #[serde(rename = "birth_year")]
    pub birth_year: Option<i32>, // Using Option since "NA" values can be present

    #[serde(rename = "pos")]
    pub position: String,

    #[serde(rename = "age")]
    pub age: i32,

    #[serde(rename = "experience")]
    pub experience: i32,

    #[serde(rename = "lg")]
    pub league: String,

    #[serde(rename = "tm")]
    pub team: String,

    #[serde(rename = "g")]
    pub games: i32,

    #[serde(rename = "mp")]
    pub minutes_played: Option<i32>, // Using Option since "NA" values can be present

    #[serde(rename = "fg_percent")]
    pub field_goal_percentage: Option<f32>, // Using Option since "NA" values can be present

    #[serde(rename = "avg_dist_fga")]
    pub average_shot_distance: Option<f32>, // Using Option since "NA" values can be present

    // The percentage of field goal attempts from different distances
    #[serde(rename = "percent_fga_from_x2p_range")]
    pub percent_fga_2pt_range: Option<f32>,

    #[serde(rename = "percent_fga_from_x0_3_range")]
    pub percent_fga_0_3_range: Option<f32>,

    #[serde(rename = "percent_fga_from_x3_10_range")]
    pub percent_fga_3_10_range: Option<f32>,

    #[serde(rename = "percent_fga_from_x10_16_range")]
    pub percent_fga_10_16_range: Option<f32>,

    #[serde(rename = "percent_fga_from_x16_3p_range")]
    pub percent_fga_16_3pt_range: Option<f32>,

    #[serde(rename = "percent_fga_from_x3p_range")]
    pub percent_fga_3pt_range: Option<f32>,

    // The field goal percentage from different distances
    #[serde(rename = "fg_percent_from_x2p_range")]
    pub fg_percent_2pt_range: Option<f32>,

    #[serde(rename = "fg_percent_from_x0_3_range")]
    pub fg_percent_0_3_range: Option<f32>,

    #[serde(rename = "fg_percent_from_x3_10_range")]
    pub fg_percent_3_10_range: Option<f32>,

    #[serde(rename = "fg_percent_from_x10_16_range")]
    pub fg_percent_10_16_range: Option<f32>,

    #[serde(rename = "fg_percent_from_x16_3p_range")]
    pub fg_percent_16_3pt_range: Option<f32>,

    #[serde(rename = "fg_percent_from_x3p_range")]
    pub fg_percent_3pt_range: Option<f32>,

    // Other shot characteristics
    #[serde(rename = "percent_assisted_x2p_fg")]
    pub percent_assisted_2pt_fg: Option<f32>,

    #[serde(rename = "percent_assisted_x3p_fg")]
    pub percent_assisted_3pt_fg: Option<f32>,

    #[serde(rename = "percent_dunks_of_fga")]
    pub percent_dunks_of_fga: Option<f32>,

    #[serde(rename = "num_of_dunks")]
    pub number_of_dunks: Option<i32>,

    #[serde(rename = "percent_corner_3s_of_3pa")]
    pub percent_corner_3s_of_3pa: Option<f32>,

    #[serde(rename = "corner_3_point_percent")]
    pub corner_3_point_percentage: Option<f32>,

    #[serde(rename = "num_heaves_attempted")]
    pub number_of_heaves_attempted: Option<i32>,

    #[serde(rename = "num_heaves_made")]
    pub number_of_heaves_made: Option<i32>,
}


#[derive(Debug, Deserialize)]
pub struct TeamStats {
    pub team_name: String,
    pub win_percentage: f32,
    pub playoff_status: String,  // "Made Playoffs" or "Did Not Make Playoffs"
}