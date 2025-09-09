use askama::Template;
use axum::response::{Html, IntoResponse};

use crate::{
    data::{list_files_in_folder, read_csv},
    error::AppError,
    render::{plot_candlesticks, plot_pie_files},
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
    let plot_pie = plot_pie_files(&potentials)?;
    let mut rng = rand::rng();
    potentials.shuffle(&mut rng);
    let random_files = potentials.into_iter().take(3).collect::<Vec<_>>();
    let mut plots: Vec<PlotlyHtml> = random_files
        .iter()
        .map(|file| {
            let df = read_csv(&format!("./local/data/Stocks/{}", file))?;
            let plot = plot_candlesticks(df, Some(&file))?;
            Ok(plot.to_inline_html(Some(&file)))
        })
        .collect::<Result<_, AppError>>()?;
    plots.insert(0, plot_pie.to_inline_html(Some("distribution")));
    let template = HomeTemplate { plots };
    Ok(Html(template.render()?))
}
