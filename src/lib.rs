#![forbid(unsafe_code)]

//! Visualization data generation for ternary systems.

use std::collections::HashMap;

/// Ternary value for visualization.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Trit {
    Neg,
    Zero,
    Pos,
}

impl Trit {
    pub fn value(self) -> i8 {
        match self {
            Trit::Neg => -1,
            Trit::Zero => 0,
            Trit::Pos => 1,
        }
    }

    pub fn from_value(v: i8) -> Option<Self> {
        match v {
            -1 => Some(Trit::Neg),
            0 => Some(Trit::Zero),
            1 => Some(Trit::Pos),
            _ => None,
        }
    }
}

/// Heatmap data on a ternary grid mapped to color values.
pub struct HeatmapData {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<Trit>>,
}

impl HeatmapData {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![vec![Trit::Zero; height]; width],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, val: Trit) {
        if x < self.width && y < self.height {
            self.grid[x][y] = val;
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Trit> {
        if x < self.width && y < self.height {
            Some(self.grid[x][y])
        } else {
            None
        }
    }

    /// Map ternary value to a color hex string.
    pub fn to_color(trit: Trit) -> &'static str {
        match trit {
            Trit::Neg => "#FF4444",   // red
            Trit::Zero => "#444444",  // gray
            Trit::Pos => "#44FF44",   // green
        }
    }

    /// Render as ASCII art.
    pub fn render_ascii(&self) -> String {
        let mut lines = Vec::new();
        for y in (0..self.height).rev() {
            let mut row = String::new();
            for x in 0..self.width {
                let ch = match self.grid[x][y] {
                    Trit::Neg => '-',
                    Trit::Zero => '.',
                    Trit::Pos => '+',
                };
                row.push(ch);
            }
            lines.push(row);
        }
        lines.join("\n")
    }

    /// Count values by type.
    pub fn count_values(&self) -> HashMap<Trit, usize> {
        let mut counts = HashMap::new();
        for col in &self.grid {
            for &val in col {
                *counts.entry(val).or_insert(0) += 1;
            }
        }
        counts
    }
}

/// A 2D scatter point with ternary classification.
#[derive(Clone, Debug)]
pub struct ScatterPoint2D {
    pub x: f64,
    pub y: f64,
    pub label: Trit,
}

impl ScatterPoint2D {
    pub fn new(x: f64, y: f64, label: Trit) -> Self {
        Self { x, y, label }
    }
}

/// A 3D scatter point with ternary classification.
#[derive(Clone, Debug)]
pub struct ScatterPoint3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub label: Trit,
}

impl ScatterPoint3D {
    pub fn new(x: f64, y: f64, z: f64, label: Trit) -> Self {
        Self { x, y, z, label }
    }
}

/// Scatter data collection for ternary points.
pub struct ScatterData {
    pub points_2d: Vec<ScatterPoint2D>,
    pub points_3d: Vec<ScatterPoint3D>,
}

impl ScatterData {
    pub fn new() -> Self {
        Self {
            points_2d: Vec::new(),
            points_3d: Vec::new(),
        }
    }

    pub fn add_2d(&mut self, point: ScatterPoint2D) {
        self.points_2d.push(point);
    }

    pub fn add_3d(&mut self, point: ScatterPoint3D) {
        self.points_3d.push(point);
    }

    /// Count 2D points by label.
    pub fn count_2d_by_label(&self) -> HashMap<Trit, usize> {
        let mut counts = HashMap::new();
        for p in &self.points_2d {
            *counts.entry(p.label).or_insert(0) += 1;
        }
        counts
    }

    /// Compute bounding box of 2D points.
    pub fn bbox_2d(&self) -> Option<((f64, f64), (f64, f64))> {
        if self.points_2d.is_empty() {
            return None;
        }
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        for p in &self.points_2d {
            min_x = min_x.min(p.x);
            min_y = min_y.min(p.y);
            max_x = max_x.max(p.x);
            max_y = max_y.max(p.y);
        }
        Some(((min_x, min_y), (max_x, max_y)))
    }

    /// Render 2D scatter as ASCII art.
    pub fn render_ascii_2d(&self, width: usize, height: usize) -> String {
        let bbox = match self.bbox_2d() {
            Some(b) => b,
            None => return String::new(),
        };
        let (min_x, min_y) = bbox.0;
        let (max_x, max_y) = bbox.1;
        let dx = (max_x - min_x).max(1e-10);
        let dy = (max_y - min_y).max(1e-10);

        let mut grid = vec![vec![b' '; width]; height];
        for p in &self.points_2d {
            let px = ((p.x - min_x) / dx * (width as f64 - 1.0)) as usize;
            let py = ((p.y - min_y) / dy * (height as f64 - 1.0)) as usize;
            if px < width && py < height {
                let ch = match p.label {
                    Trit::Neg => b'-',
                    Trit::Zero => b'.',
                    Trit::Pos => b'+',
                };
                grid[py][px] = ch;
            }
        }

        let mut lines = Vec::new();
        for row in grid.iter().rev() {
            lines.push(String::from_utf8_lossy(row).to_string());
        }
        lines.join("\n")
    }

    pub fn len_2d(&self) -> usize {
        self.points_2d.len()
    }

    pub fn len_3d(&self) -> usize {
        self.points_3d.len()
    }

    pub fn is_empty(&self) -> bool {
        self.points_2d.is_empty() && self.points_3d.is_empty()
    }
}

impl Default for ScatterData {
    fn default() -> Self {
        Self::new()
    }
}

/// Time series data point with ternary value.
#[derive(Clone, Debug)]
pub struct TimeSeriesPoint {
    pub time: f64,
    pub value: Trit,
}

impl TimeSeriesPoint {
    pub fn new(time: f64, value: Trit) -> Self {
        Self { time, value }
    }
}

/// Time series of ternary values.
pub struct TimeSeriesData {
    pub name: String,
    pub points: Vec<TimeSeriesPoint>,
}

impl TimeSeriesData {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            points: Vec::new(),
        }
    }

    pub fn push(&mut self, time: f64, value: Trit) {
        self.points.push(TimeSeriesPoint::new(time, value));
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Time span of the series.
    pub fn time_range(&self) -> Option<(f64, f64)> {
        if self.points.is_empty() {
            return None;
        }
        let mut min_t = f64::INFINITY;
        let mut max_t = f64::NEG_INFINITY;
        for p in &self.points {
            min_t = min_t.min(p.time);
            max_t = max_t.max(p.time);
        }
        Some((min_t, max_t))
    }

    /// Count transitions between ternary states.
    pub fn count_transitions(&self) -> usize {
        if self.points.len() < 2 {
            return 0;
        }
        let mut count = 0;
        for i in 1..self.points.len() {
            if self.points[i].value != self.points[i - 1].value {
                count += 1;
            }
        }
        count
    }

    /// Render as ASCII time series.
    pub fn render_ascii(&self, width: usize) -> String {
        if self.points.is_empty() {
            return String::new();
        }
        let (min_t, max_t) = self.time_range().unwrap();
        let dt = (max_t - min_t).max(1e-10);

        let mut grid = vec![vec![b' '; width]; 3]; // 3 rows: Pos, Zero, Neg
        for p in &self.points {
            let col = ((p.time - min_t) / dt * (width as f64 - 1.0)) as usize;
            if col < width {
                let row = match p.value {
                    Trit::Pos => 2,
                    Trit::Zero => 1,
                    Trit::Neg => 0,
                };
                grid[row][col] = b'*';
            }
        }

        let labels = ["-", "0", "+"];
        let mut lines = Vec::new();
        for (i, row) in grid.iter().enumerate() {
            let line = format!("{}|{}", labels[i], String::from_utf8_lossy(row));
            lines.push(line);
        }
        lines.join("\n")
    }
}

/// Chart configuration.
#[derive(Clone, Debug)]
pub struct ChartConfig {
    pub title: String,
    pub width: usize,
    pub height: usize,
    pub show_legend: bool,
    pub x_label: String,
    pub y_label: String,
}

impl ChartConfig {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            width: 60,
            height: 20,
            show_legend: true,
            x_label: String::new(),
            y_label: String::new(),
        }
    }

    pub fn with_dimensions(mut self, w: usize, h: usize) -> Self {
        self.width = w;
        self.height = h;
        self
    }

    pub fn with_labels(mut self, x: &str, y: &str) -> Self {
        self.x_label = x.to_string();
        self.y_label = y.to_string();
        self
    }
}

/// SVG generation for ternary diagrams.
pub struct TernarySvg {
    pub width: f64,
    pub height: f64,
}

impl TernarySvg {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    /// Generate an SVG heatmap.
    pub fn heatmap_svg(&self, data: &HeatmapData) -> String {
        let cell_w = self.width / data.width as f64;
        let cell_h = self.height / data.height as f64;
        let mut svg = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\">\n",
            self.width, self.height
        );
        for y in 0..data.height {
            for x in 0..data.width {
                let color = HeatmapData::to_color(data.grid[x][y]);
                svg.push_str(&format!(
                    "  <rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{}\"/>\n",
                    x as f64 * cell_w,
                    y as f64 * cell_h,
                    cell_w,
                    cell_h,
                    color
                ));
            }
        }
        svg.push_str("</svg>");
        svg
    }

    /// Generate an SVG ternary triangle diagram.
    pub fn ternary_triangle_svg(&self, points: &[(f64, f64, f64)]) -> String {
        // Equilateral triangle vertices
        let cx = self.width / 2.0;
        let base_y = self.height * 0.9;
        let top_y = self.height * 0.1;
        let half_base = self.width * 0.4;

        let (ax, ay) = (cx, top_y);           // top vertex (component 1)
        let (bx, by) = (cx - half_base, base_y); // bottom-left (component 2)
        let (ex, ey) = (cx + half_base, base_y); // bottom-right (component 3)

        let mut svg = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\">\n",
            self.width, self.height
        );
        // Draw triangle
        svg.push_str(&format!(
            "  <polygon points=\"{:.1},{:.1} {:.1},{:.1} {:.1},{:.1}\" fill=\"none\" stroke=\"black\" stroke-width=\"2\"/>\n",
            ax, ay, bx, by, ex, ey
        ));
        // Plot points (barycentric coords)
        for (a, b, c) in points {
            let sum = a + b + c;
            if sum.abs() < 1e-10 {
                continue;
            }
            let an = a / sum;
            let bn = b / sum;
            let cn = c / sum;
            let px = an * ax + bn * bx + cn * ex;
            let py = an * ay + bn * by + cn * ey;
            svg.push_str(&format!(
                "  <circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"3\" fill=\"blue\"/>\n",
                px, py
            ));
        }
        svg.push_str("</svg>");
        svg
    }

    /// Generate an SVG scatter plot for 2D ternary data.
    pub fn scatter_svg(&self, data: &ScatterData) -> String {
        let bbox = match data.bbox_2d() {
            Some(b) => b,
            None => {
                return format!(
                    "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\"/>",
                    self.width, self.height
                )
            }
        };
        let (min_x, min_y) = bbox.0;
        let (max_x, max_y) = bbox.1;
        let dx = (max_x - min_x).max(1e-10);
        let dy = (max_y - min_y).max(1e-10);
        let margin = 20.0;
        let plot_w = self.width - 2.0 * margin;
        let plot_h = self.height - 2.0 * margin;

        let mut svg = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\">\n",
            self.width, self.height
        );
        for p in &data.points_2d {
            let px = margin + (p.x - min_x) / dx * plot_w;
            let py = self.height - margin - (p.y - min_y) / dy * plot_h;
            let color = match p.label {
                Trit::Neg => "red",
                Trit::Zero => "gray",
                Trit::Pos => "green",
            };
            svg.push_str(&format!(
                "  <circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"{}\"/>\n",
                px, py, color
            ));
        }
        svg.push_str("</svg>");
        svg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trit_values() {
        assert_eq!(Trit::Neg.value(), -1);
        assert_eq!(Trit::Zero.value(), 0);
        assert_eq!(Trit::Pos.value(), 1);
    }

    #[test]
    fn test_trit_from_value() {
        assert_eq!(Trit::from_value(1), Some(Trit::Pos));
        assert_eq!(Trit::from_value(5), None);
    }

    #[test]
    fn test_heatmap_set_get() {
        let mut hm = HeatmapData::new(3, 3);
        hm.set(1, 1, Trit::Pos);
        assert_eq!(hm.get(1, 1), Some(Trit::Pos));
        assert_eq!(hm.get(0, 0), Some(Trit::Zero));
        assert_eq!(hm.get(5, 5), None);
    }

    #[test]
    fn test_heatmap_ascii() {
        let mut hm = HeatmapData::new(3, 2);
        hm.set(0, 0, Trit::Pos);
        hm.set(2, 1, Trit::Neg);
        let ascii = hm.render_ascii();
        assert!(ascii.contains('+'));
        assert!(ascii.contains('-'));
        assert!(ascii.contains('.'));
    }

    #[test]
    fn test_heatmap_count_values() {
        let mut hm = HeatmapData::new(2, 2);
        hm.set(0, 0, Trit::Pos);
        hm.set(1, 1, Trit::Pos);
        let counts = hm.count_values();
        assert_eq!(counts.get(&Trit::Pos), Some(&2));
        assert_eq!(counts.get(&Trit::Zero), Some(&2));
    }

    #[test]
    fn test_heatmap_to_color() {
        assert_eq!(HeatmapData::to_color(Trit::Neg), "#FF4444");
        assert_eq!(HeatmapData::to_color(Trit::Zero), "#444444");
        assert_eq!(HeatmapData::to_color(Trit::Pos), "#44FF44");
    }

    #[test]
    fn test_scatter_2d() {
        let mut sd = ScatterData::new();
        sd.add_2d(ScatterPoint2D::new(1.0, 2.0, Trit::Pos));
        sd.add_2d(ScatterPoint2D::new(3.0, 4.0, Trit::Neg));
        assert_eq!(sd.len_2d(), 2);
        assert!(!sd.is_empty());
    }

    #[test]
    fn test_scatter_bbox() {
        let mut sd = ScatterData::new();
        sd.add_2d(ScatterPoint2D::new(1.0, 2.0, Trit::Pos));
        sd.add_2d(ScatterPoint2D::new(5.0, 8.0, Trit::Neg));
        let bbox = sd.bbox_2d().unwrap();
        assert_eq!(bbox.0, (1.0, 2.0));
        assert_eq!(bbox.1, (5.0, 8.0));
    }

    #[test]
    fn test_scatter_count_by_label() {
        let mut sd = ScatterData::new();
        sd.add_2d(ScatterPoint2D::new(0.0, 0.0, Trit::Pos));
        sd.add_2d(ScatterPoint2D::new(1.0, 1.0, Trit::Pos));
        sd.add_2d(ScatterPoint2D::new(2.0, 2.0, Trit::Neg));
        let counts = sd.count_2d_by_label();
        assert_eq!(counts.get(&Trit::Pos), Some(&2));
        assert_eq!(counts.get(&Trit::Neg), Some(&1));
    }

    #[test]
    fn test_scatter_ascii() {
        let mut sd = ScatterData::new();
        sd.add_2d(ScatterPoint2D::new(0.0, 0.0, Trit::Pos));
        sd.add_2d(ScatterPoint2D::new(10.0, 10.0, Trit::Neg));
        let ascii = sd.render_ascii_2d(10, 10);
        assert!(!ascii.is_empty());
    }

    #[test]
    fn test_timeseries_push_and_len() {
        let mut ts = TimeSeriesData::new("test");
        ts.push(0.0, Trit::Pos);
        ts.push(1.0, Trit::Zero);
        ts.push(2.0, Trit::Neg);
        assert_eq!(ts.len(), 3);
    }

    #[test]
    fn test_timeseries_time_range() {
        let mut ts = TimeSeriesData::new("test");
        ts.push(1.0, Trit::Pos);
        ts.push(5.0, Trit::Neg);
        let range = ts.time_range().unwrap();
        assert_eq!(range, (1.0, 5.0));
    }

    #[test]
    fn test_timeseries_transitions() {
        let mut ts = TimeSeriesData::new("test");
        ts.push(0.0, Trit::Pos);
        ts.push(1.0, Trit::Pos);
        ts.push(2.0, Trit::Neg);
        ts.push(3.0, Trit::Zero);
        assert_eq!(ts.count_transitions(), 2);
    }

    #[test]
    fn test_timeseries_no_transitions() {
        let mut ts = TimeSeriesData::new("test");
        ts.push(0.0, Trit::Pos);
        ts.push(1.0, Trit::Pos);
        assert_eq!(ts.count_transitions(), 0);
    }

    #[test]
    fn test_timeseries_ascii() {
        let mut ts = TimeSeriesData::new("test");
        ts.push(0.0, Trit::Pos);
        ts.push(1.0, Trit::Neg);
        let ascii = ts.render_ascii(10);
        assert!(ascii.contains('*'));
    }

    #[test]
    fn test_chart_config() {
        let config = ChartConfig::new("Test Chart")
            .with_dimensions(80, 25)
            .with_labels("Time", "Value");
        assert_eq!(config.title, "Test Chart");
        assert_eq!(config.width, 80);
        assert_eq!(config.height, 25);
        assert_eq!(config.x_label, "Time");
    }

    #[test]
    fn test_svg_heatmap() {
        let mut hm = HeatmapData::new(2, 2);
        hm.set(0, 0, Trit::Pos);
        let svg_gen = TernarySvg::new(100.0, 100.0);
        let svg = svg_gen.heatmap_svg(&hm);
        assert!(svg.contains("<svg"));
        assert!(svg.contains("#44FF44"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_svg_ternary_triangle() {
        let svg_gen = TernarySvg::new(200.0, 200.0);
        let svg = svg_gen.ternary_triangle_svg(&[(0.5, 0.3, 0.2), (0.1, 0.8, 0.1)]);
        assert!(svg.contains("<svg"));
        assert!(svg.contains("<circle"));
        assert!(svg.contains("<polygon"));
    }

    #[test]
    fn test_svg_scatter() {
        let mut sd = ScatterData::new();
        sd.add_2d(ScatterPoint2D::new(0.0, 0.0, Trit::Pos));
        sd.add_2d(ScatterPoint2D::new(1.0, 1.0, Trit::Neg));
        let svg_gen = TernarySvg::new(200.0, 200.0);
        let svg = svg_gen.scatter_svg(&sd);
        assert!(svg.contains("green"));
        assert!(svg.contains("red"));
    }

    #[test]
    fn test_scatter_3d() {
        let p = ScatterPoint3D::new(1.0, 2.0, 3.0, Trit::Pos);
        assert_eq!(p.label, Trit::Pos);
        let mut sd = ScatterData::new();
        sd.add_3d(p);
        assert_eq!(sd.len_3d(), 1);
    }

    #[test]
    fn test_empty_scatter_ascii() {
        let sd = ScatterData::new();
        let ascii = sd.render_ascii_2d(10, 10);
        assert!(ascii.is_empty());
    }

    #[test]
    fn test_timeseries_empty() {
        let ts = TimeSeriesData::new("empty");
        assert!(ts.is_empty());
        assert!(ts.time_range().is_none());
        assert_eq!(ts.count_transitions(), 0);
        let ascii = ts.render_ascii(10);
        assert!(ascii.is_empty());
    }
}
