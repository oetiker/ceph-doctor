# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### New

### Changed

### Fixed

## 0.1.0 - 2025-07-18
### New
- **Enhanced Error Handling**: Added comprehensive error popup system that displays detailed information when ceph commands fail. The popup shows the exact command that failed, exit code, and complete output (stdout/stderr), making it much easier to diagnose connectivity, authentication, or configuration issues.
- **Scrollable Error Messages**: Long error messages can now be scrolled using arrow keys (↑/↓) or vim-style navigation (k/j), ensuring all error details are accessible regardless of terminal size.

### Fixed
- **Loading Screen Freeze**: Fixed issue where application would get stuck on loading screen when ceph commands failed, leaving users with no indication of what went wrong.
- **Responsive Controls**: Fixed unresponsive popup controls that would only work during brief intervals, ensuring smooth and immediate response to keyboard input regardless of refresh interval settings.

## 0.0.1 - 2025-07-14

### New
- **Initial Release**: Ceph cluster analysis tool for helping with clusters in `HEALTH_WARN` state
- **Monitor Command**: Real-time monitoring of Ceph cluster recovery progress with `ceph-doctor monitor`
- **Comprehensive UI**: Terminal-based interface displaying:
  - Recovery progress with rates and ETAs
  - Placement group (PG) states and statistics
  - OSD data movement tracking
  - Inconsistent PG monitoring
  - Cluster health overview with timestamp and refresh interval
- **Remote Execution Support**: Generic `--prefix-command` option for flexible remote execution (e.g., `--prefix-command "ssh host sudo"`)
- **Test Mode**: Built-in test mode using sample data files (`--test` flag)
- **Responsive Interface**: Real-time terminal resize handling and responsive controls
- **Modular Architecture**: Clean separation of concerns with focused modules:
  - Data processing and calculation logic
  - UI components for different display areas
  - Centralized state management
  - Terminal event handling
- **JSON Data Processing**: Parses large Ceph cluster state JSON files (`ceph pg dump --format json-pretty`)
- **Configurable Refresh Interval**: Customizable data refresh intervals for monitoring
- **Graceful Shutdown**: Proper signal handling with Ctrl+C, 'q', and Esc key support

### Technical Details
- **Language**: Rust with idiomatic patterns and error handling
- **Libraries**: 
  - `clap` for command-line argument parsing
  - `serde` and `serde_json` for JSON processing
  - `ratatui` for modern terminal UI
  - `crossterm` for terminal operations
  - `tokio` for async operations
  - `anyhow` for error handling
  - `chrono` for time management
- **Architecture**: Event-driven terminal interface with modular component design
- **Requirements**: TTY environment for interactive terminal interface