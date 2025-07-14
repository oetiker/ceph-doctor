pub mod data;
pub mod state;
pub mod terminal;
pub mod ui;

use crate::common::CephPgDump;
use crate::Result;
use data::*;
use state::MonitorState;
use std::env;
use std::process::Command;
use terminal::{SleepResult, TerminalManager};
use ui::*;

pub async fn run_test(interval: u64) -> Result<()> {
    let files = ["state-a.json", "state-b.json"];
    let mut file_index = 0;

    let mut terminal_manager = TerminalManager::new()?;
    let mut last_data: Option<CephPgDump> = None;
    let mut error_message: Option<String> = None;
    let mut state = MonitorState::new();

    loop {
        // Handle events
        if terminal_manager.poll_event(std::time::Duration::from_millis(100))? {
            let event = terminal_manager.read_event()?;
            if terminal_manager.should_quit(&event) {
                break;
            }

            // Handle resize events immediately
            if matches!(event, crossterm::event::Event::Resize(_, _)) {
                if let Some(ref data) = last_data {
                    terminal_manager.terminal().draw(|f| {
                        render_main_ui(f, data, None, interval, &error_message, &mut state);
                    })?;
                }
            }
        }

        // Load data
        match std::fs::read_to_string(files[file_index]) {
            Ok(json_str) => match serde_json::from_str::<CephPgDump>(&json_str) {
                Ok(data) => {
                    terminal_manager.terminal().draw(|f| {
                        render_main_ui(
                            f,
                            &data,
                            last_data.as_ref(),
                            interval,
                            &error_message,
                            &mut state,
                        );
                    })?;
                    last_data = Some(data);
                    error_message = None;
                    file_index = (file_index + 1) % files.len();
                }
                Err(e) => {
                    error_message = Some(format!("Parse error: {e}"));
                }
            },
            Err(e) => {
                error_message = Some(format!("File read error: {e}"));
            }
        }

        // Sleep with event checking
        match terminal::sleep_with_event_check(interval, &terminal_manager).await? {
            SleepResult::Quit => {
                terminal_manager.cleanup()?;
                return Ok(());
            }
            SleepResult::Resize => {
                // Trigger immediate redraw on resize
                if let Some(ref data) = last_data {
                    terminal_manager.terminal().draw(|f| {
                        render_main_ui(f, data, None, interval, &error_message, &mut state);
                    })?;
                }
            }
            SleepResult::Continue => {
                // Normal flow, continue to next iteration
            }
        }
    }

    terminal_manager.cleanup()?;
    Ok(())
}

pub async fn run(interval: u64, prefix_args: Option<&[String]>) -> Result<()> {
    let mut terminal_manager = TerminalManager::new()?;
    let mut last_data: Option<CephPgDump> = None;
    let mut error_message: Option<String> = None;
    let mut state = MonitorState::new();

    loop {
        // Handle events
        if terminal_manager.poll_event(std::time::Duration::from_millis(100))? {
            let event = terminal_manager.read_event()?;
            if terminal_manager.should_quit(&event) {
                break;
            }

            // Handle resize events immediately
            if matches!(event, crossterm::event::Event::Resize(_, _)) {
                if let Some(ref data) = last_data {
                    terminal_manager.terminal().draw(|f| {
                        render_main_ui(f, data, None, interval, &error_message, &mut state);
                    })?;
                }
            }
        }

        // Fetch data
        match fetch_ceph_pg_dump(prefix_args).await {
            Ok(data) => {
                terminal_manager.terminal().draw(|f| {
                    render_main_ui(
                        f,
                        &data,
                        last_data.as_ref(),
                        interval,
                        &error_message,
                        &mut state,
                    );
                })?;
                last_data = Some(data);
                error_message = None;
            }
            Err(e) => {
                error_message = Some(e.to_string());
                if let Some(ref data) = last_data {
                    terminal_manager.terminal().draw(|f| {
                        render_main_ui(f, data, None, interval, &error_message, &mut state);
                    })?;
                }
            }
        }

        // Sleep with event checking
        match terminal::sleep_with_event_check(interval, &terminal_manager).await? {
            SleepResult::Quit => {
                terminal_manager.cleanup()?;
                return Ok(());
            }
            SleepResult::Resize => {
                // Trigger immediate redraw on resize
                if let Some(ref data) = last_data {
                    terminal_manager.terminal().draw(|f| {
                        render_main_ui(f, data, None, interval, &error_message, &mut state);
                    })?;
                }
            }
            SleepResult::Continue => {
                // Normal flow, continue to next iteration
            }
        }
    }

    terminal_manager.cleanup()?;
    Ok(())
}

async fn fetch_ceph_pg_dump(prefix_args: Option<&[String]>) -> Result<CephPgDump> {
    let mut command = if let Some(args) = prefix_args.filter(|args| !args.is_empty()) {
        let mut cmd = Command::new(&args[0]);
        for arg in &args[1..] {
            cmd.arg(arg);
        }
        cmd.arg("ceph");
        cmd
    } else {
        Command::new("ceph")
    };

    // Add the ceph command arguments
    command.args(["pg", "dump", "--format", "json-pretty"]);

    let output = command.output()?;

    if !output.status.success() {
        return Err(format!(
            "ceph command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    let json_str = String::from_utf8(output.stdout)?;
    let data: CephPgDump = serde_json::from_str(&json_str)?;

    Ok(data)
}

fn count_unique_pg_states(pg_stats: &[crate::common::PgStats]) -> usize {
    use std::collections::HashSet;

    let mut unique_states = HashSet::new();
    for pg in pg_stats {
        unique_states.insert(&pg.state);
    }

    // Return the number of unique states, but show at least top 10
    std::cmp::min(unique_states.len(), 10)
}

fn render_main_ui(
    f: &mut ratatui::Frame,
    data: &CephPgDump,
    _last_data: Option<&CephPgDump>,
    interval: u64,
    error_msg: &Option<String>,
    state: &mut MonitorState,
) {
    // Check for NO_COLOR environment variable
    let use_colors = env::var("NO_COLOR").is_err();
    let size = f.area();

    // Create main layout
    let main_layout = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            ratatui::layout::Constraint::Length(3), // Header
            ratatui::layout::Constraint::Min(0),    // Content
            ratatui::layout::Constraint::Length(3), // Footer
        ])
        .split(size);

    // Render header
    render_header(f, main_layout[0], data, interval);

    // Render error message if present
    let content_area = if let Some(ref error) = error_msg {
        let error_layout = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Length(3), // Error
                ratatui::layout::Constraint::Min(0),    // Content
            ])
            .split(main_layout[1]);

        render_error(f, error_layout[0], error, use_colors);
        error_layout[1]
    } else {
        main_layout[1]
    };

    // Calculate dynamic heights
    let inconsistent_pgs = calculate_inconsistent_pg_progress(data, state, interval);
    let recovery_progress_height = calculate_recovery_progress_height(data);

    // Calculate PG states height dynamically based on content
    let pg_states_count = count_unique_pg_states(&data.pg_map.pg_stats);
    let pg_states_height = (pg_states_count + 3).clamp(4, 15) as u16;

    // Split content area into sections based on whether there are inconsistent PGs
    let (content_layout, osd_layout_index) = if inconsistent_pgs.is_empty() {
        // No inconsistent PGs - hide the block entirely
        let layout = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Length(recovery_progress_height), // Recovery progress (dynamic)
                ratatui::layout::Constraint::Length(pg_states_height), // PG states (dynamic)
                ratatui::layout::Constraint::Min(0),                   // OSD Data Movement
            ])
            .split(content_area);
        (layout, 2)
    } else {
        // Show inconsistent PGs block
        let inconsistent_pgs_height = (inconsistent_pgs.len() + 3).clamp(4, 15) as u16;
        let layout = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Length(recovery_progress_height), // Recovery progress (dynamic)
                ratatui::layout::Constraint::Length(pg_states_height), // PG states (dynamic)
                ratatui::layout::Constraint::Length(inconsistent_pgs_height), // Inconsistent PGs (dynamic)
                ratatui::layout::Constraint::Min(0),                          // OSD Data Movement
            ])
            .split(content_area);
        (layout, 3)
    };

    // Render sections
    render_recovery_progress(f, content_layout[0], data, state, interval);
    render_pg_states(f, content_layout[1], data, use_colors);

    // Only render inconsistent PGs table if there are inconsistent PGs
    if !inconsistent_pgs.is_empty() {
        render_inconsistent_pgs_table(f, content_layout[2], inconsistent_pgs, use_colors);
    }

    let osd_data_movements = calculate_osd_data_movement(data, state, interval);
    render_osd_data_movement_table(
        f,
        content_layout[osd_layout_index],
        osd_data_movements,
        use_colors,
    );

    // Render footer
    render_footer(f, main_layout[2]);
}
