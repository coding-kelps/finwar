use askama::Template;
use axum::response::{Html, IntoResponse};

use crate::{
    data::{list_files_in_folder, read_csv},
    error::AppError,
    render::plot_candlesticks,
};
use rand::seq::SliceRandom;

type PlotlyHtml = String;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    plots: Vec<PlotlyHtml>,
}

pub async fn home() -> Result<impl IntoResponse, AppError> {
    let mut potentials = list_files_in_folder("./local/data/Stocks")?;
    let mut rng = rand::rng();
    potentials.shuffle(&mut rng);
    let random_files = potentials.into_iter().take(3).collect::<Vec<_>>();
    let plots: Result<Vec<PlotlyHtml>, AppError> = random_files
        .iter()
        .map(|file| {
            let df = read_csv(&format!("./local/data/Stocks/{}", file))?;
            let plot = plot_candlesticks(df, Some(&file))?;
            Ok(plot.to_inline_html(Some(&file)))
        })
        .collect();

    let template = HomeTemplate { plots: plots? };
    Ok(Html(template.render()?))
}
