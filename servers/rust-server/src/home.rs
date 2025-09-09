use askama::Template;
use axum::response::{Html, IntoResponse};
use plotly::histogram;

use crate::{
    data::{list_files_in_folder, read_csv},
    error::AppError,
    render::{plot_candlesticks, plot_histogram, plot_pie_files},
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

    let dataframes = random_files
        .iter()
        .map(|file| read_csv(&format!("./local/data/Stocks/{}", file)))
        .collect::<Result<Vec<_>, _>>()?;

    let histogram = plot_histogram(&dataframes, &random_files)?;

    let mut plots: Vec<PlotlyHtml> = dataframes
        .iter()
        .zip(random_files.iter())
        .map(|(df, file)| {
            let plot = plot_candlesticks(df.clone(), Some(&file))?;
            Ok(plot.to_inline_html(Some(&file)))
        })
        .collect::<Result<_, AppError>>()?;

    plots.insert(0, plot_pie.to_inline_html(Some("distribution")));
    plots.insert(1, histogram.to_inline_html(Some("histogram")));

    let template = HomeTemplate { plots };
    Ok(Html(template.render()?))
}
