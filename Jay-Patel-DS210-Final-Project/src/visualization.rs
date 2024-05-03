use plotters::prelude::*;
use std::collections::HashMap;

pub fn visualize_centrality_scores(centrality_scores: &HashMap<i32, f64>) -> Result<(), Box<dyn std::error::Error>> {
    // Define the path to the image we will generate
    let root = BitMapBackend::new("centrality_scores.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_centrality_score = centrality_scores.values().cloned().fold(f64::MIN, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .caption("Centrality Scores", ("sans-serif", 40))
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..centrality_scores.len(), 0.0..max_centrality_score)?;

    chart.configure_mesh().draw()?;

    // We will use different colors to make the plot more interesting
    let colors = [RED, BLUE, GREEN, CYAN, MAGENTA, YELLOW];

    // Drawing bars
    let bars: Vec<_> = centrality_scores.iter().map(|(node, &score)| (*node, score)).collect();

    for (i, (node, score)) in bars.iter().enumerate() {
        chart.draw_series(std::iter::once(Rectangle::new(
            [(i, 0.0), (i + 1, *score)],
            colors[i % colors.len()].filled(),
        )))?;
    }

    // Label the bars with node indices
    chart.configure_series_labels()
        .border_style(&BLACK)
        .draw()?;

    root.present().expect("Unable to write result to file");
    println!("Centrality scores visualization generated.");
    Ok(())
}
