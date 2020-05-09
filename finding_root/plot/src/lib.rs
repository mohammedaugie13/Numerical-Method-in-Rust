use plotly::common::{DashType, Line, Mode, Title};
use plotly::layout::Axis;
use plotly::{Layout, Plot, Scatter};

pub fn scatter_plot(x: Vec<f64>, y: Vec<f64>, c: &str, title: &str) {
    let trace1 = Scatter::new(x, y).name(c).mode(Mode::Markers);

    let layout = Layout::new()
        .title(Title::new(title))
        .xaxis(Axis::new().title(Title::new("X")).show_grid(true).zero_line(true))
        .yaxis(Axis::new().title(Title::new("Y")).show_line(false));

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.set_layout(layout);
    plot.show();
}

#[allow(dead_code)]
pub fn line_dash(x: Vec<f32>, y: Vec<f32>, c: &str, title: &str) {
    let trace1 = Scatter::new(x, y)
        .name(c)
        .mode(Mode::LinesMarkers)
        .line(Line::new().dash(DashType::DashDot));

    let layout = Layout::new()
        .title(Title::new(title))
        .xaxis(Axis::new().title(Title::new("X")).show_grid(true).zero_line(true))
        .yaxis(Axis::new().title(Title::new("Y")).show_line(false));

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.set_layout(layout);
    plot.show();
}
