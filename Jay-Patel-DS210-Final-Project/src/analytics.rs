use ndarray::Array1;
use ndarray_stats::CorrelationExt;
use std::collections::HashMap;
use crate::data_structures::{MergedData, CorrelationResults, PlayoffCorrelationResults};

/// Calculates Pearson correlation coefficients for player shooting percentages to team points per game.
pub fn correlate_statistics(merged_data: &[MergedData]) -> Vec<CorrelationResults> {
    merged_data.iter().map(|data| {
        let player = &data.player;
        let team = &data.team;

        let fg_3p_list = Array1::from(
            merged_data.iter()
                .filter(|x| x.team.abbreviation == team.abbreviation)
                .map(|x| x.player.fg_percent_3p as f64)
                .collect::<Vec<f64>>()
        );
        let fg_2p_list = Array1::from(
            merged_data.iter()
                .filter(|x| x.team.abbreviation == team.abbreviation)
                .map(|x| x.player.fg_percent_2p as f64)
                .collect::<Vec<f64>>()
        );
        let pts_list = Array1::from(
            merged_data.iter()
                .filter(|x| x.team.abbreviation == team.abbreviation)
                .map(|x| x.team.pts_per_game as f64)
                .collect::<Vec<f64>>()
        );

        let fg_3p_pts_corr = fg_3p_list.pearson_correlation(&pts_list).unwrap_or(0.0);
        let fg_2p_pts_corr = fg_2p_list.pearson_correlation(&pts_list).unwrap_or(0.0);

        CorrelationResults {
            player_name: player.name.clone(),
            fg_3p_pts_correlation: fg_3p_pts_corr,
            fg_2p_pts_correlation: fg_2p_pts_corr,
        }
    }).collect()
}

/// Analyzes correlation between team success metrics and making the playoffs.
pub fn analyze_playoff_correlation(merged_data: &[MergedData]) -> PlayoffCorrelationResults {
    let pts_list: Vec<f64> = merged_data.iter().map(|x| x.team.pts_per_game as f64).collect();
    let playoff_list: Vec<f64> = merged_data.iter().map(|x| x.team.made_playoffs as u8 as f64).collect();

    let correlation_with_playoffs = pearson_correlation(&pts_list, &playoff_list).unwrap_or(0.0);

    PlayoffCorrelationResults {
        correlation_with_playoffs,
    }
}
