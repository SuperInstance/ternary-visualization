# ternary-visualization

Rendering ternary data as ASCII art, SVG heatmaps, scatter plots, ternary triangle diagrams, and time series charts.

## Why This Exists

Ternary data {-1, 0, +1} needs its own visualization primitives. Standard charting libraries assume continuous ranges or large categorical sets. When your data has exactly three states, you want three colors, three ASCII symbols, and chart types designed for three-valued grids — like ternary triangle diagrams for compositional data. This crate generates those outputs from ternary data structures, producing both ASCII (for terminals and logs) and SVG (for web and documents).

## Core Concepts

- **Trit** — The core ternary value: Neg (−1), Zero (0), Pos (+1). Displayed as `−`/`.`/`+` in ASCII and red/gray/green in SVG.
- **HeatmapData** — A 2D grid of Trits with ASCII rendering and per-cell color mapping. Colors: Neg → `#FF4444` (red), Zero → `#444444` (gray), Pos → `#44FF44` (green).
- **ScatterData** — Collections of 2D and 3D points, each labeled with a Trit. Supports bounding box computation, label counting, and ASCII scatter rendering.
- **TimeSeriesData** — A named sequence of (time, Trit) points. Counts state transitions and renders as a 3-row ASCII chart (one row per trit value).
- **TernarySvg** — SVG generator for heatmaps, scatter plots, and ternary triangle diagrams. The triangle diagram plots barycentric coordinates on an equilateral triangle.
- **Barycentric coordinates** — A point in a ternary triangle is specified by three weights (a, b, c) summing to 1. The position is computed as a weighted average of the triangle vertices. Used in geology, chemistry, and materials science for three-component compositional data.
- **ChartConfig** — Dimensions, title, axis labels, and legend toggle for configuring chart output.

## Quick Start

```toml
# Cargo.toml
[dependencies]
ternary-visualization = "0.1"
```

```rust
use ternary_visualization::*;

fn main() {
    // ASCII heatmap
    let mut heatmap = HeatmapData::new(5, 3);
    heatmap.set(0, 0, Trit::Pos);
    heatmap.set(2, 1, Trit::Neg);
    heatmap.set(4, 2, Trit::Pos);
    println!("{}", heatmap.render_ascii());

    // SVG scatter plot
    let mut scatter = ScatterData::new();
    scatter.add_2d(ScatterPoint2D::new(0.0, 0.0, Trit::Pos));
    scatter.add_2d(ScatterPoint2D::new(5.0, 3.0, Trit::Neg));
    scatter.add_2d(ScatterPoint2D::new(2.0, 2.0, Trit::Zero));
    let svg = TernarySvg::new(400.0, 300.0).scatter_svg(&scatter);
    println!("{}", svg);

    // ASCII time series
    let mut ts = TimeSeriesData::new("signal");
    ts.push(0.0, Trit::Pos);
    ts.push(1.0, Trit::Pos);
    ts.push(2.0, Trit::Neg);
    ts.push(3.0, Trit::Zero);
    println!("{}", ts.render_ascii(40));
}
```

## API Overview

| Type | Description |
|------|-------------|
| `Trit` | Core ternary value (Neg/Zero/Pos) |
| `HeatmapData` | 2D ternary grid with ASCII and color rendering |
| `ScatterPoint2D` | 2D point with ternary label |
| `ScatterPoint3D` | 3D point with ternary label |
| `ScatterData` | Collection of scatter points with bounding box and ASCII render |
| `TimeSeriesPoint` | (time, Trit) datum |
| `TimeSeriesData` | Named time series with transition counting and ASCII rendering |
| `ChartConfig` | Chart dimensions, labels, title, legend toggle |
| `TernarySvg` | SVG generator for heatmaps, scatter, and triangle diagrams |

## How It Works

**ASCII heatmap** iterates rows top-to-bottom (y reversed so y=0 is the bottom), columns left-to-right. Each cell maps to `−` (Neg), `.` (Zero), or `+` (Pos).

**ASCII scatter** normalizes point coordinates to the bounding box, maps them to integer grid positions, and places the appropriate symbol. Points outside the grid are dropped. Multiple points mapping to the same cell overwrite (last wins).

**ASCII time series** uses a 3-row layout: Pos at top, Zero in the middle, Neg at the bottom. Time is the horizontal axis. Each data point places a `*` at the corresponding row and time column.

**SVG heatmap** generates one `<rect>` per cell with the ternary color fill. Cell dimensions are `svg_width / grid_width` × `svg_height / grid_height`.

**SVG scatter** plots each point as a `<circle>` with radius 4, colored by label (red/gray/green). Coordinates are mapped to the SVG viewport with a 20-pixel margin.

**SVG ternary triangle** draws an equilateral triangle and plots barycentric points as circles. Given a point `(a, b, c)`, it normalizes to `a/(a+b+c), b/(a+b+c), c/(a+b+c)` and computes the Cartesian position as a weighted average of the triangle vertices.

## Known Limitations

- **No interactive features.** SVG output is static. No hover, zoom, or click handlers.
- **Color palette is hard-coded.** Red/gray/green only. No custom color mapping or colorblind-safe palettes.
- **ASCII scatter has resolution limits.** Points close together map to the same character cell and overwrite each other. Information is lost.
- **No legends, axis ticks, or labels in SVG.** The SVG output is the raw plot — no text annotations, no axis lines, no grid lines.
- **Barycentric coordinates are not validated.** Points with negative weights or zero sum produce incorrect positions. The triangle renderer skips zero-sum points but doesn't validate individual components.

## Use Cases

- **Terminal dashboards** — Render ternary sensor grids, voting results, or cell tissue states as ASCII art in log output or CLI tools.
- **Web reports** — Generate SVG heatmaps and scatter plots for browser display or PDF embedding.
- **Scientific compositional data** — Plot three-component mixtures on a ternary triangle diagram (e.g., soil composition: sand/silt/clay).

## Ecosystem Context

Part of the SuperInstance ternary crate family. `ternary-visualization` is the rendering layer. It consumes data from `ternary-cell` (tissue grids), `ternary-database` (query results), `ternary-voting` (election tallies), and `ternary-robotics` (sensor arrays and paths). It's a leaf crate that depends on no other ternary crates.

## License

MIT
