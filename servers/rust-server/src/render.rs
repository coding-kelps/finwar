use plotly::{
    color::Rgba,
    common::{Line, Marker},
    layout::{Axis, themes::BuiltinTheme},
    *,
};
use polars::{error::PolarsError, frame::DataFrame};

use plotly::histogram::HistFunc;
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

// fn colored_and_styled_histograms(show: bool, file_name: &str) {
//     let n = 500;
//     let x1 = sample_uniform_distribution(n, 0.0, 5.0);
//     let x2 = sample_uniform_distribution(n, 0.0, 10.0);
//     let y1 = sample_uniform_distribution(n, 0.0, 1.0);
//     let y2 = sample_uniform_distribution(n, 0.0, 2.0);

//     let trace1 = Histogram::new_xy(x1, y1)
//         .name("control")
//         .hist_func(HistFunc::Count)
//         .marker(
//             Marker::new()
//                 .color(Rgba::new(255, 100, 102, 0.7))
//                 .line(Line::new().color(Rgba::new(255, 100, 102, 1.0)).width(1.0)),
//         )
//         .opacity(0.5)
//         .auto_bin_x(false)
//         .x_bins(Bins::new(0.5, 2.8, 0.06));
//     let trace2 = Histogram::new_xy(x2, y2)
//         .name("experimental")
//         .hist_func(HistFunc::Count)
//         .marker(
//             Marker::new()
//                 .color(Rgba::new(100, 200, 102, 0.7))
//                 .line(Line::new().color(Rgba::new(100, 200, 102, 1.0)).width(1.0)),
//         )
//         .opacity(0.75)
//         .auto_bin_x(false)
//         .x_bins(Bins::new(-3.2, 4.0, 0.06));
//     let layout = Layout::new()
//         .title("Colored and Styled Histograms")
//         .x_axis(Axis::new().title("Value"))
//         .y_axis(Axis::new().title("Count"))
//         .bar_mode(BarMode::Overlay)
//         .bar_gap(0.05)
//         .bar_group_gap(0.2);

//     let mut plot = Plot::new();
//     plot.set_layout(layout);
//     plot.add_trace(trace1);
//     plot.add_trace(trace2);

//     let path = write_example_to_html(&plot, file_name);
//     if show {
//         plot.show_html(path);

pub fn plot_histogram(
    dfs: &Vec<DataFrame>, names: &Vec<String>,
) -> Result<Plot, PolarsError> {
    let colors = [
        Rgba::new(255, 100, 102, 0.7),
        Rgba::new(100, 200, 102, 0.7),
        Rgba::new(100, 100, 255, 0.7),
        Rgba::new(255, 200, 100, 0.7),
    ];

    let mut plot = Plot::new();

    for (i, df) in dfs.iter().enumerate() {
        let close =
            df.column("Close")?.f64()?.into_no_null_iter().collect::<Vec<_>>();
        let trace =
            Histogram::new(close)
                .name(format!(
                    "Stock {}",
                    names.get(i).unwrap_or(&"Unknown".to_string())
                ))
                .hist_func(HistFunc::Count)
                .marker(Marker::new().color(colors[i % colors.len()]).line(
                    Line::new().color(colors[i % colors.len()]).width(1.0),
                ))
                .opacity(0.7)
                .auto_bin_x(false);
        plot.add_trace(trace);
    }

    plot.set_layout(a_nice_layout("Close Price Distribution"));
    Ok(plot)
}
