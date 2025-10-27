use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};

use charming::Chart;
use charming::{
    component::{Axis, DataZoom, Legend},
    element::{
        AreaStyle, AxisPointer, AxisPointerType, AxisType, DataBackground,
        LineStyle, SplitLine, TextStyle, Tooltip, Trigger,
    },
    renderer::HtmlRenderer,
    series::{Candlestick, Line},
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};

use crate::{error::AppError, state::AppState};
use entity::stocks_history;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    chart: String,
}

fn calculate_ma(period: usize, data: &[Vec<f64>]) -> Vec<f64> {
    data.windows(period)
        .map(|w| w.iter().map(|d| d[1]).sum::<f64>() / period as f64)
        .collect()
}

pub async fn chart(state: &AppState) -> Result<Chart, AppError> {
    let records = stocks_history::Entity::find()
        .filter(stocks_history::Column::Symbol.eq("AAPL"))
        .order_by_asc(stocks_history::Column::Time)
        .limit(300)
        .all(&state.db)
        .await?;

    let dates: Vec<String> = records
        .iter()
        .map(|r| r.time.format("%Y-%m-%d %H:%M").to_string())
        .collect();

    let data: Vec<Vec<f64>> = records
        .iter()
        .map(|r| {
            vec![
                r.close.unwrap_or(0.0),
                r.open.unwrap_or(0.0),
                r.low.unwrap_or(0.0),
                r.high.unwrap_or(0.0),
            ]
        })
        .collect();

    let data_background = DataBackground::new()
        .area_style(AreaStyle::new().color("#8392A5"))
        .line_style(LineStyle::new().color("#8392A5").opacity(0.8));
    Ok(Chart::new()
        .legend(Legend::new().inactive_color("#777").data(vec!["MA5"]))
        .tooltip(
            Tooltip::new().trigger(Trigger::Axis).axis_pointer(
                AxisPointer::new()
                    .animation(true)
                    .type_(AxisPointerType::Cross)
                    .line_style(
                        LineStyle::new().color("#376df4").width(2).opacity(1),
                    ),
            ),
        )
        .x_axis(Axis::new().type_(AxisType::Category).data(dates))
        .y_axis(
            Axis::new().scale(true).split_line(SplitLine::new().show(false)),
        )
        .data_zoom(
            DataZoom::new()
                .start(78)
                .end(98)
                .text_style(TextStyle::new().color("#8392A5"))
                .data_background(
                    DataBackground::new()
                        .area_style(AreaStyle::new().color("#8392A5"))
                        .line_style(
                            LineStyle::new().color("#8392A5").opacity(0.8),
                        ),
                )
                .brush_select(true)
                .selected_data_background(data_background),
        )
        .series(Candlestick::new().data(data.clone()))
        .series(
            Line::new()
                .name("MA5")
                .data(calculate_ma(5, &data))
                .smooth(true)
                .show_symbol(false)
                .line_style(LineStyle::new().width(1)),
        ))
}

pub async fn home(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let chart = chart(&state).await?;

    let renderer = HtmlRenderer::new("Shanghai Candlestick Chart", 1220, 800);
    let html = renderer.render(&chart).map_err(|e| AppError::Charming(e))?;
    let template = HomeTemplate { chart: html };
    Ok(Html(template.render()?))
}
