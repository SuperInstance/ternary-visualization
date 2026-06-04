# Future Integration: ternary-visualization

## Current State
Provides visualization data generation for ternary systems: heatmap data on ternary grids, time-series plots, 3D surface plots, ASCII rendering, and color mapping with red/gray/green for {-1, 0, +1}.

## Integration Opportunities

### With open-tui (Terminal Dashboard)
`ternary-visualization` generates data; `open-tui` renders it. `HeatmapData::render_ascii()` is already terminal-compatible. The color hex strings ("#FF4444", "#444444", "#44FF44") map to terminal color codes. Together: real-time room state dashboards in the terminal, no GUI required.

### With ternary-som
SOM produces U-matrix data. `ternary-visualization` renders U-matrices as heatmaps. `SurfacePlotData` captures 3D SOM topology for visualization. The SOM room map is rendered as an interactive terminal dashboard showing room clustering.

### With ternary-cell
Cell grid state IS heatmap data. Each cell's ternary value maps to a color. `HeatmapData` wraps the grid; `render_ascii()` displays it. Cell tick cycles produce time-series data for `TimeSeriesData`. Together: real-time visualization of cell populations evolving.

## Potential in Mature Systems
In room-as-codespace, `ternary-visualization` is the monitoring layer. Every room has a dashboard: current state (heatmap), history (time series), and fleet overview (3D surface). All rendered in the terminal via open-tui. No external dependencies, no web servers, no GUI frameworks. Pure terminal monitoring.

## Cross-Pollination Ideas
- ASCII art heatmaps for SMS/Slack/Discord room status alerts
- Time-series sparklines for room health at a glance
- 3D surface plots for fleet resource topology visualization

## Dependencies for Next Steps
- Integration with open-tui for terminal rendering
- Integration with ternary-som for U-matrix visualization
- Integration with ternary-cell for live cell grid dashboards
