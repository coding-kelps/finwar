use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};

use crate::{
    error::AppError,
    render::{PlotlyHtml, plot_candlesticks, plot_histogram},
    state::AppState,
};

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    plots: Vec<PlotlyHtml>,
}

pub async fn home(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let dataframes = state.dataframes;
    let random_files = state.randos;
    let plot_pie = state.pie_distrubtion;
    let histogram = plot_histogram(&dataframes, &random_files)?;

    let mut plots: Vec<PlotlyHtml> = dataframes
        .iter()
        .zip(random_files.iter())
        .map(|(df, file)| {
            let plot = plot_candlesticks(df.clone(), Some(&file))?;
            Ok(plot.to_inline_html(Some(&file)))
        })
        .collect::<Result<_, AppError>>()?;

    plots.insert(0, plot_pie);
    plots.insert(1, histogram.to_inline_html(Some("histogram")));

    let template = HomeTemplate { plots };
    Ok(Html(template.render()?))
}
