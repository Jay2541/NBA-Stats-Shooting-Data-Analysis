use crate::data_structures::MergedData;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use csv::Writer;

#[derive(Clone)]
pub struct CorrelationResult {
    pub player_id: u32,
    pub player_name: String,
    pub team_name: String,
    pub statistic_name: String,
    pub correlation_coefficient: f64,
}

pub fn correlate_statistics(merged_data: &[MergedData]) -> Vec<CorrelationResult> {
    let mut results = Vec::new();

    for data in merged_data {
        let player = &data.player;
        let team = &data.team;

        let fg_percent_diff = player.fg_percent - team.fg_percentage;
        let fg_percent_ratio = player.fg_percent / team.fg_percentage;

        let fg_percent_from_x2p_range_diff = player.fg_percent_from_x2p_range - team.two_point_percentage;
        let fg_percent_from_x2p_range_ratio = player.fg_percent_from_x2p_range / team.two_point_percentage;

        let fg_percent_from_x3p_range_diff = player.fg_percent_from_x3p_range - team.three_point_percentage;
        let fg_percent_from_x3p_range_ratio = player.fg_percent_from_x3p_range / team.three_point_percentage;

        let fg_percent_diff_result = CorrelationResult {
            player_id: player.id,
            player_name: player.name.clone(),
            team_name: team.name.clone(),
            statistic_name: "FG Percent Diff".to_string(),
            correlation_coefficient: fg_percent_diff,
        };
        results.push(fg_percent_diff_result);

        let fg_percent_ratio_result = CorrelationResult {
            player_id: player.id,
            player_name: player.name.clone(),
            team_name: team.name.clone(),
            statistic_name: "FG Percent Ratio".to_string(),
            correlation_coefficient: fg_percent_ratio,
        };
        results.push(fg_percent_ratio_result);

        let fg_percent_from_x2p_range_diff_result = CorrelationResult {
            player_id: player.id,
            player_name: player.name.clone(),
            team_name: team.name.clone(),
            statistic_name: "FG Percent from 2P Range Diff".to_string(),
            correlation_coefficient: fg_percent_from_x2p_range_diff,
        };
        results.push(fg_percent_from_x2p_range_diff_result);

        let fg_percent_from_x2p_range_ratio_result = CorrelationResult {
            player_id: player.id,
            player_name: player.name.clone(),
            team_name: team.name.clone(),
            statistic_name: "FG Percent from 2P Range Ratio".to_string(),
            correlation_coefficient: fg_percent_from_x2p_range_ratio,
        };
        results.push(fg_percent_from_x2p_range_ratio_result);

        let fg_percent_from_x3p_range_diff_result = CorrelationResult {
            player_id: player.id,
            player_name: player.name.clone(),
            team_name: team.name.clone(),
            statistic_name: "FG Percent from 3P Range Diff".to_string(),
            correlation_coefficient: fg_percent_from_x3p_range_diff,
        };
        results.push(fg_percent_from_x3p_range_diff_result);

        let fg_percent_from_x3p_range_ratio_result = CorrelationResult {
            player_id: player.id,
            player_name: player.name.clone(),
            team_name: team.name.clone(),
            statistic_name: "FG Percent from 3P Range Ratio".to_string(),
            correlation_coefficient: fg_percent_from_x3p_range_ratio,
        };
        results.push(fg_percent_from_x3p_range_ratio_result);
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

    let mut fg_percent_values = Vec::new();
    let mut fg_percent_from_x2p_range_values = Vec::new();
    let mut fg_percent_from_x3p_range_values = Vec::new();
    let mut playoff_values = Vec::new();

    for data in merged_data {
        let player = &data.player;
        let team = &data.team;

        fg_percent_values.push(player.fg_percent);
        fg_percent_from_x2p_range_values.push(player.fg_percent_from_x2p_range);
        fg_percent_from_x3p_range_values.push(player.fg_percent_from_x3p_range);
        playoff_values.push(team.playoffs as u8 as f64);
    }

    let statistic_names = [
        "FG Percent",
        "FG Percent from 2P Range",
        "FG Percent from 3P Range",
    ];

    let x_values_array = [
        &fg_percent_values,
        &fg_percent_from_x2p_range_values,
        &fg_percent_from_x3p_range_values,
    ];

    for (i, x_values) in x_values_array.iter().enumerate() {
        let statistic_name = statistic_names[i].to_string();
        let correlation_coefficient = calculate_correlation(x_values, &playoff_values);

        let result = CorrelationResult {
            player_id: 0,
            player_name: "All Players".to_string(),
            team_name: "".to_string(),
            statistic_name,
            correlation_coefficient,
        };

        if correlation_coefficient > 0.0 {
            positive_correlations.insert(i as u32, result);
        } else {
            negative_correlations.insert(i as u32, result);
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
    let denominator = ((n * sum_x_sq - sum_x * sum_x) * (n * sum_y_sq - sum_y * sum_y)).sqrt();

    if denominator != 0.0 {
        numerator / denominator
    } else {
        f64::NAN
    }
}

pub fn write_correlations_to_csv(
    correlations: &[CorrelationResult],
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    let mut writer = Writer::from_path(file_path)?;

    // Write the header row
    writer.write_record(&["Player ID", "Player Name", "Team Name", "Statistic Name", "Correlation Coefficient"])?;

    for result in correlations {
        writer.write_record(&[
            result.player_id.to_string(),
            result.player_name.clone(),
            result.team_name.clone(),
            result.statistic_name.clone(),
            result.correlation_coefficient.to_string(),
        ])?;
    }

    writer.flush()?;
    Ok(())
}