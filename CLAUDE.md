# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

**Note:** When working on this project, please use the context7 mcp.

## Project Overview

This is a Ceph cluster analysis tool for helping with ceph clusters where
`ceph -s` says `HEALTH WARN`.

## Functionality

The `ceph-doctor` tool has a `monitor` subcommand that displays a real-time view of the cluster's state.

### Monitor Tool

Invoked via `ceph-doctor monitor`, this tool shows how Ceph is working towards resolving the `HEALTH_WARN` state. It calls `ssh <host> sudo ceph pg dump_json pgs --format json` at a configurable interval to observe what the cluster is doing and how fast it is doing it.

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

The system is implemented in idiomatic Rust. The code is split into several files:

- `src/main.rs`: Handles command-line argument parsing using `clap`.
- `src/lib.rs`: Defines the data structures for the Ceph JSON output using `serde`.
- `src/monitor.rs`: Contains the logic for fetching, parsing, and displaying the cluster state. It uses `ratatui` with `crossterm` backend to create a modern terminal-based user interface. The UI uses color-blind friendly colors that work well with both light and dark terminal themes, and supports the `NO_COLOR` environment variable for accessibility.

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

