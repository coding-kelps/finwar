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

pub fn plot_pie_files(files: &Vec<String>) -> Result<Plot, PolarsError> {
    use std::fs;

    // Shift up the size bins in bits
    let bins = [
        (0, 8_192),          // 0-8KB
        (8_192, 81_920),     // 8KB-80KB
        (81_920, 819_200),   // 80KB-800KB
        (819_200, u64::MAX), // 800KB+
    ];
    let mut counts = vec![0; bins.len()];

    // Count files in each bin
    for file in files {
        let path = format!("./local/data/Stocks/{}", file);
        let metadata = fs::metadata(&path);
        if let Ok(meta) = metadata {
            let size = meta.len(); // bytes
            let size_bits = size * 8;
            for (i, (min, max)) in bins.iter().enumerate() {
                if size_bits >= *min && size_bits < *max {
                    counts[i] += 1;
                    break;
                }
            }
        }
    }

    // Create labels with counts (after counts are calculated)
    let bin_labels: Vec<String> = bins
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let label = match i {
                0 => "0-8KB",
                1 => "8KB-80KB",
                2 => "80KB-800KB",
                3 => "800KB+",
                _ => "Other",
            };
            format!("{} ({})", label, counts[i])
        })
        .collect();

    let trace = Pie::new(counts.clone()).labels(bin_labels);

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(a_nice_layout("Files Size Distribution"));
    Ok(plot)
}
