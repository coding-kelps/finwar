use axum::{
    response::{Html},
};

use plotly::{layout::{themes::BuiltinTheme, Axis}, *};
use polars::prelude::*;

use plotly::{Plot, Layout};

fn read_csv(path: &str) -> Result<DataFrame, PolarsError> {
    CsvReadOptions::default()
        .with_has_header(true)
        .with_parse_options(
            CsvParseOptions::default().with_try_parse_dates(true),
        )
        .try_into_reader_with_file_path(Some(path.into()))?
        .finish()
}

fn a_nice_layout(title: &str) -> Layout {
    let theme = BuiltinTheme::PlotlyDark;
    let theme_template = theme.build();
    Layout::new()
        .title(title)
        .x_axis(Axis::new().title("Time"))
        .y_axis(Axis::new().title("Price (USD $)"))
        .font(plotly::common::Font::new().size(18))
        .width(1200)
        .height(800)
        .template(theme_template)
}

pub async fn home() -> Html<String> {
    let df = read_csv("./local/data/Stocks/aapl.us.txt").expect("Failed to read CSV file");
    
    let x  = df
        .column("Date").expect("No 'Date' column")
        .date().expect("'Date' column is not date").to_string("%Y-%m-%d");
    let a = x.unwrap().into_no_null_iter().map(|s| s.to_string()).collect::<Vec<_>>();
    let open = df.column("Open").expect("No 'Open' column").f64().expect("'Open' column is not f64").into_no_null_iter().collect::<Vec<_>>();
    let high = df.column("High").expect("No 'High' column").f64().expect("'High' column is not f64").into_no_null_iter().collect::<Vec<_>>();
    let low = df.column("Low").expect("No 'Low' column").f64().expect("'Low' column is not f64").into_no_null_iter().collect::<Vec<_>>();
    let close = df.column("Close").expect("No 'Close' column").f64().expect("'Close' column is not f64").into_no_null_iter().collect::<Vec<_>>();
    // println!("{:?}", df);

    let trace = Candlestick::new(a, open, high, low, close);
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(a_nice_layout("AAPL Stock Prices"));
    Html(plot.to_html())
}