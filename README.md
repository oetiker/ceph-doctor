# Ceph Doctor

A Ceph cluster analysis tool for helping with clusters where `ceph -s` shows `HEALTH_WARN` status.

## Overview

The `ceph-doctor` tool provides a `monitor` subcommand that displays a real-time view of the cluster's state, showing how Ceph is working towards resolving the `HEALTH_WARN` condition.

## Installation

```bash
cargo build --release
```

Or install directly:

```bash
cargo install --path .
```

## Usage

### Monitor Command

Monitor your local Ceph cluster:

```bash
ceph-doctor monitor
```

The monitor tool calls `ceph pg dump --format json-pretty` at a configurable interval (default: 5 seconds) to observe cluster activity and recovery progress.

#### Options

- `--interval <SECONDS>`: Set the refresh interval (default: 5)
- `--test`: Test mode using sample JSON files (`state-a.json` and `state-b.json`)
- `--prefix-command <COMMAND>`: Command prefix for remote execution

#### Remote Execution

For remote Ceph clusters, use the `--prefix-command` option:

```bash
# SSH to remote host with sudo
ceph-doctor monitor --prefix-command "ssh ceph-host sudo"

# SSH with custom user and sudo
ceph-doctor monitor --prefix-command "ssh user@ceph-host sudo"

# Kubernetes pod execution
ceph-doctor monitor --prefix-command "kubectl exec ceph-pod --"

# Docker container execution
ceph-doctor monitor --prefix-command "docker exec ceph-container"
```

#### Test Mode

Test the interface using sample data:

```bash
ceph-doctor monitor --test
```

## Features

The monitor displays:

- **Recovery Progress**: Shows active recovery operations with rates and ETAs
- **Placement Group States**: Summary of PG states across the cluster
- **OSD Data Movement**: Tracks data movement between OSDs
- **Inconsistent PGs**: Highlights placement groups requiring attention
- **Real-time Updates**: Responsive terminal interface with resize support

## Requirements

- Rust toolchain
- Access to a Ceph cluster (local or remote)
- Terminal with TTY support (required for the interactive interface)

## Controls

- **q**, **Ctrl+C**, or **Esc**: Quit the application
- Terminal resize is automatically handled

## Technical Details

Built with:
- `clap` for command-line parsing
- `serde` for JSON handling
- `tokio` for async operations
- `ratatui` for terminal UI
- `crossterm` for terminal backend
- `anyhow` for error handling
- `chrono` for time operations

The tool parses Ceph's JSON output to provide organized, real-time monitoring of cluster health and recovery progress.

## Development

### Development Commands

- `cargo ci-check` - Run all CI checks locally (formatting, linting, tests)
- `cargo fmt` - Auto-format code
- `cargo fmt-check` - Check code formatting
- `cargo clippy-check` - Run clippy lints
- `cargo test-all` - Run all tests

### Code Quality

This project enforces code quality through:
- **Automated formatting** with `rustfmt`
- **Linting** with `clippy` (configured in `Cargo.toml`)
- **Testing** with unit tests
- **CI/CD** via GitHub Actions

Before submitting changes, run:
```bash
cargo ci-check
```

This runs the same checks as the GitHub Actions CI pipeline.

### Testing

Run the full test suite:
```bash
cargo test-all
```

Test the monitor interface locally:
```bash
cargo run -- monitor --test
```

### Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run `cargo ci-check` to ensure code quality
5. Submit a pull request

All pull requests must pass CI checks including formatting, linting, and testing.