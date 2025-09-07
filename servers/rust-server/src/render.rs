use plotly::{
    layout::{Axis, themes::BuiltinTheme},
    *,
};
use polars::{error::PolarsError, frame::DataFrame};

use plotly::{Candlestick, Plot};

pub fn a_nice_layout(title: &str) -> Layout {
    let theme = BuiltinTheme::PlotlyDark;
    let theme_template = theme.build();
    Layout::new()
        .title(title)
        // .x_axis(Axis::new().title("Time"))
        .y_axis(Axis::new().title("Price (USD $)"))
        .font(plotly::common::Font::new().size(14))
        // .width(1200)
        // .height(800)
        .template(theme_template)
}

pub fn plot_candlesticks(
    df: DataFrame, df_name: Option<&str>,
) -> Result<Plot, PolarsError> {
    let x = df.column("Date")?.date()?.to_string("%Y-%m-%d")?;
    let a = x.into_no_null_iter().map(|s| s.to_string()).collect::<Vec<_>>();
    let open =
        df.column("Open")?.f64()?.into_no_null_iter().collect::<Vec<_>>();
    let high =
        df.column("High")?.f64()?.into_no_null_iter().collect::<Vec<_>>();
    let low = df.column("Low")?.f64()?.into_no_null_iter().collect::<Vec<_>>();
    let close =
        df.column("Close")?.f64()?.into_no_null_iter().collect::<Vec<_>>();

    let trace = Candlestick::new(a, open, high, low, close);
    let mut plot = Plot::new();
    let title = df_name.unwrap_or("Some stocks example");
    let df_name = title.to_uppercase();
    plot.add_trace(trace);
    plot.set_layout(a_nice_layout(
        format!("{} Stock Prices", df_name).as_str(),
    ));
    Ok(plot)
}
