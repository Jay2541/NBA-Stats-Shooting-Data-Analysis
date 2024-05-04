use crate::data_structures::{MergedData};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone)]
pub struct CorrelationResult {
    pub player_id: u32,
    pub correlation_coefficient: f64,
}

pub fn correlate_statistics(merged_data: &[MergedData]) -> Vec<CorrelationResult> {
    let mut results = Vec::new();
    for data in merged_data {
        let player = &data.player;
        let team = &data.team;
        // Calculate correlation between player statistics and team points per game
        let x_values = [
            player.fg_percent,
            player.avg_dist_fga,
            player.fg_percent_from_x2p_range,
            player.fg_percent_from_x3p_range,
        ];
        let y_value = team.points_per_game;
        let correlation_coefficient = calculate_correlation(&x_values, &[y_value]);
        results.push(CorrelationResult {
            player_id: player.id,
            correlation_coefficient,
        });
    }
    results
}

pub struct PlayoffCorrelationResults {
    pub positive_correlations: HashMap<u32, CorrelationResult>,
    pub negative_correlations: HashMap<u32, CorrelationResult>,
}

pub fn analyze_playoff_correlation(merged_data: &[MergedData]) -> PlayoffCorrelationResults {
    let mut positive_correlations = HashMap::new();
    let mut negative_correlations = HashMap::new();
    for data in merged_data {
        let player = &data.player;
        let team = &data.team;
        // Calculate correlation between player statistics and team playoff qualification
        let x_values = [
            player.fg_percent,
            player.avg_dist_fga,
            player.fg_percent_from_x2p_range,
            player.fg_percent_from_x3p_range,
        ];
        let y_value = if team.playoffs { 1.0 } else { 0.0 };
        let correlation_coefficient = calculate_correlation(&x_values, &[y_value]);
        let result = CorrelationResult {
            player_id: player.id,
            correlation_coefficient,
        };
        if correlation_coefficient > 0.0 {
            positive_correlations.insert(player.id, result);
        } else {
            negative_correlations.insert(player.id, result);
        }
    }
    PlayoffCorrelationResults {
        positive_correlations,
        negative_correlations,
    }
}

fn calculate_correlation(x_values: &[f64], y_values: &[f64]) -> f64 {
    let n = x_values.len() as f64;
    let sum_x = x_values.iter().sum::<f64>();
    let sum_y = y_values.iter().sum::<f64>();
    let sum_x_sq = x_values.iter().map(|x| x * x).sum::<f64>();
    let sum_y_sq = y_values.iter().map(|y| y * y).sum::<f64>();
    let sum_xy = x_values.iter().zip(y_values.iter()).map(|(x, y)| x * y).sum::<f64>();

    let numerator = n * sum_xy - sum_x * sum_y;
    let denominator = (n * sum_x_sq - sum_x * sum_x).sqrt() * (n * sum_y_sq - sum_y * sum_y).sqrt();

    if denominator == 0.0 {
        0.0
    } else {
        numerator / denominator
    }
}

pub fn write_correlations_to_csv(
    correlations: &[CorrelationResult],
    filename: &str,
) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(b"player_id,correlation_coefficient\n")?;

    for result in correlations {
        let line = format!("{},{}\n", result.player_id, result.correlation_coefficient);
        file.write_all(line.as_bytes())?;
    }

    Ok(())
}