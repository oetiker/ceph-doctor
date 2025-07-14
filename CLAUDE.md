# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

**Note:** When working on this project, please use the context7 mcp.

## Project Overview

This is a Ceph cluster analysis tool for helping with ceph clusters where
`ceph -s` says `HEALTH WARN`.

## Functionality

The `ceph-doctor` tool has a `monitor` subcommand that displays a real-time view of the cluster's state.

### Monitor Tool

Invoked via `ceph-doctor monitor`, this tool shows how Ceph is working towards resolving the `HEALTH_WARN` state. It calls `ceph pg dump --format json-pretty` at a configurable interval to observe what the cluster is doing and how fast it is doing it. For remote execution, use `--prefix-command` (e.g., `--prefix-command "ssh host sudo"`).

There is also a test mode, invoked with `ceph-doctor monitor --test`, which uses the `state-a.json` and `state-b.json` files to simulate the output of the `ceph` command.

### Data Structure

The JSON files contain Ceph cluster state with the following key components:

- `pg_ready`: Boolean indicating if placement groups are ready
- `pg_map`: Contains comprehensive cluster state including:
  - `pg_stats`: Individual placement group statistics
  - `pg_stats_sum`: Aggregated statistics across all PGs
  - `osd_stats_sum`: Object Storage Daemon statistics
  - Performance metrics, recovery data, and health alerts

### Working with the Data

The JSON files are large (6.8MB each) and contain single-line JSON. The tool parses this JSON to display a summary of the cluster's state.

### File Format

- `state-a.json` and `state-b.json`: Single-line JSON files containing complete Ceph cluster state snapshots.
- Both files follow the same schema structure.
- The files appear to be different time snapshots of the same cluster for comparison analysis.

## Implementation

The system is implemented in idiomatic Rust using a modular architecture:

### Core Files
- `src/main.rs`: Handles command-line argument parsing using `clap`.
- `src/lib.rs`: Defines the data structures for the Ceph JSON output using `serde`.

### Monitor Module (`src/monitor/`)
The monitor functionality is organized into focused modules:

- `mod.rs`: Main coordination and public API for monitor functionality
- `terminal.rs`: Terminal management, event handling, and TTY operations
- `state.rs`: Centralized state management replacing static variables
- `data/`: Data processing and utilities
  - `calculator.rs`: Data calculation logic (OSD movements, recovery progress, inconsistent PGs)
  - `formatter.rs`: Number, time, and bytes formatting utilities (with unit tests)
- `ui/`: User interface components
  - `header.rs`: Header rendering with timestamp and interval info
  - `footer.rs`: Footer with control instructions
  - `error.rs`: Error message display
  - `recovery.rs`: Recovery progress table with rates and ETAs
  - `pg_table.rs`: Placement group states table
  - `osd_table.rs`: OSD data movement table and inconsistent PGs table

### Architecture Benefits
- **Modular design**: Each file has a single responsibility
- **State management**: Centralized `MonitorState` struct for persistent data
- **Event handling**: Responsive terminal resize and quit event handling
- **Testability**: Individual components can be unit tested
- **Maintainability**: Changes are localized to specific modules

The project uses the following key libraries:
- `clap` for command-line argument parsing.
- `serde` and `serde_json` for JSON deserialization.
- `tokio` for asynchronous operations.
- `anyhow` for error handling.
- `ratatui` for terminal UI with modern widgets and layout system.
- `crossterm` as the backend for ratatui terminal operations.
- `chrono` for date and time manipulation.

## Important Development Notes

**TTY Requirement**: The monitor tool requires a TTY (terminal) to run because it uses `ratatui` for the interactive terminal interface. This means:

- The application cannot be run directly via `cargo run` in environments without a proper TTY
- When testing or debugging, use a proper terminal session
- The application will fail with "No such device or address" errors in non-TTY environments
- For automated testing, consider mocking the terminal interface or using integration tests that don't require the full UI

**Event Handling**: The terminal event system supports responsive resize handling:
- Terminal resize events trigger immediate UI redraws
- Quit events (Ctrl+C, 'q', Esc) are handled during both polling and sleep periods
- Event checking happens every 250ms during sleep intervals for responsiveness

**State Management**: The `MonitorState` struct centralizes all persistent data:
- Recovery history for rate calculations
- OSD movement tracking with historical data
- Inconsistent PG progress monitoring
- All state is properly managed and can be tested

**Testing**: Limited test coverage currently exists:
- `cargo test` runs 3 basic formatter function tests only
- Most business logic (calculator, state management) lacks tests
- Individual modules are structured for testability but need tests written
- Integration tests should mock the terminal interface (not yet implemented)

## Development Guidelines

**Code Organization**: The codebase follows a modular architecture with clear separation of concerns:

### Working with the Monitor Module

**Adding New UI Components:**
- Create new files in `src/monitor/ui/` for new display components
- Export them through `src/monitor/ui/mod.rs`
- Follow the existing pattern of accepting `Frame`, `Rect`, and data parameters

**Adding New Calculations:**
- Add calculation logic to `src/monitor/data/calculator.rs`
- Use the `MonitorState` for persistent data across refreshes
- Return results that can be consumed by UI components

**Modifying State:**
- All persistent state goes through `src/monitor/state.rs`
- Use the provided getters/setters for proper encapsulation
- Use the centralized state manager for consistency

**Event Handling:**
- Terminal events are managed in `src/monitor/terminal.rs`
- The `SleepResult` enum handles quit, resize, and continue events
- Resize events trigger immediate redraws for responsiveness

### Future Enhancement Opportunities

**UI Improvements:**
- Consider `Gauge` widgets for visual progress indicators
- Add `Chart` widgets for historical data visualization
- Implement `Tabs` for multiple view modes
- Use `Scrollbar` for long data lists

**Performance Optimizations:**
- Cache expensive calculations between refreshes
- Implement data streaming for very large clusters
- Add configuration for update intervals per component

**Testing Strategy (Needs Implementation):**
- Expand unit tests beyond current formatter tests to cover calculator and state modules
- Add integration tests with mocked terminal interface
- Consider property-based tests for data consistency
- Priority areas: `calculate_osd_data_movement()`, `MonitorState` operations, event handling

### TUI Library Notes

**Current Setup**: The project uses `ratatui = "0.29"` which is the correct and modern choice. `ratatui` is the active successor to the now-archived `tui-rs` library.

**Currently Using:**
- `Block`, `Paragraph`, `Table`, `Row`, `Cell`
- Layout system with constraints
- Styling and colors with `NO_COLOR` support
- Border types

**Enhancement Opportunities:**
- `Gauge` widget for visual progress bars (recovery progress)
- `Chart` widget for historical data visualization
- `List` widget for better data display
- `Tabs` widget for multiple views
- `Scrollbar` for long lists
- Custom widgets for complex displays

**Example Enhancement:**
```rust
use ratatui::widgets::Gauge;

// Replace text-based progress with visual gauge
let progress = Gauge::default()
    .block(Block::default().borders(Borders::ALL).title("Recovery Progress"))
    .gauge_style(Style::default().fg(Color::Green))
    .percent(recovery_percentage)
    .label(format!("{:.1}%", recovery_percentage));
```

