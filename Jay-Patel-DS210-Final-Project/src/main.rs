mod models;
mod utils;
mod graph;
mod centrality;

fn analyze_data(player_shooting: Vec<models::PlayerShooting>, team_stats: Vec<models::TeamStats>) {
    // Grouping player stats by team
    use std::collections::HashMap;
    let mut team_shooting: HashMap<String, Vec<models::PlayerShooting>> = HashMap::new();
    for shot in player_shooting {
        team_shooting.entry(shot.team_name.clone())
            .or_insert_with(Vec::new)
            .push(shot);
    }

    // Calculate average shooting performance per team
    let mut team_performance: HashMap<String, (f32, f32)> = HashMap::new(); // (average 2PT%, average 3PT%)
    for (team, shots) in team_shooting.iter() {
        let (total_2pt, count_2pt, total_3pt, count_3pt) = shots.iter().fold((0.0, 0, 0.0, 0), |acc, shot| {
            match shot.distance.as_ref() {
                "2PT" => (acc.0 + shot.field_goal_percentage, acc.1 + 1, acc.2, acc.3),
                "3PT" => (acc.0, acc.1, acc.2 + shot.field_goal_percentage, acc.3 + 1),
                _ => acc,
            }
        });
        let avg_2pt = if count_2pt > 0 { total_2pt / count_2pt as f32 } else { 0.0 };
        let avg_3pt = if count_3pt > 0 { total_3pt / count_3pt as f32 } else { 0.0 };
        team_performance.insert(team.clone(), (avg_2pt, avg_3pt));
    }

    // Correlate team performance with playoff appearances
    for team in team_stats {
        let playoff_status = &team.playoff_status;
        if let Some(&(avg_2pt, avg_3pt)) = team_performance.get(&team.team_name) {
            println!("Team: {}, 2PT%: {:.2}%, 3PT%: {:.2}%, Playoff: {}",
                     team.team_name, avg_2pt, avg_3pt, playoff_status);
        }
    }
}

fn main() {
    let player_shooting_path = "NBA Stats (1947-Present)/Player Shooting.csv";
    let team_stats_path = "NBA Stats (1947-Present)/Team Stats Per Game.csv";

    let player_shooting = utils::read_player_shooting(player_shooting_path).expect("Failed to read player shooting data");
    let team_stats = utils::read_team_stats(team_stats_path).expect("Failed to read team stats");

    analyze_data(player_shooting, team_stats);
}
