use serde::{Serialize, Deserialize, Deserializer};
use serde::de::{self, Visitor};
use std::fmt;

fn deserialize_option_f32<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: Deserializer<'de>,
{
    struct OptionF32Visitor;

    impl<'de> Visitor<'de> for OptionF32Visitor {
        type Value = Option<f32>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a float, 'NA', or an empty string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match v.trim() {
                "NA" | "" => Ok(None),
                other => other.parse().map(Some).map_err(E::custom),
            }
        }

        fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(v))
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }
    }

    deserializer.deserialize_option(OptionF32Visitor)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    #[serde(rename = "player")]
    pub player: String,
    #[serde(rename = "tm")]
    pub tm: String,
    #[serde(rename = "fg_percent_from_x3p_range")]
    #[serde(deserialize_with = "deserialize_option_f32")]
    pub fg_percent_from_x3p_range: Option<f32>,
    #[serde(rename = "fg_percent_from_x2p_range")]
    #[serde(deserialize_with = "deserialize_option_f32")]
    pub fg_percent_from_x2p_range: Option<f32>,
    #[serde(rename = "avg_dist_fga")]
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