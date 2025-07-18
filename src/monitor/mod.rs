pub mod data;
pub mod state;
pub mod terminal;
pub mod ui;

use crate::common::CephPgDump;
use crate::Result;
use data::*;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
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

    // Draw initial loading screen
    render_current_state(
        terminal_manager.terminal(),
        None,
        None,
        interval,
        &mut state,
    )?;

    loop {
        // Handle events
        if terminal_manager.poll_event(std::time::Duration::from_millis(100))? {
            let event = terminal_manager.read_event()?;

            // Always handle quit events
            if terminal_manager.should_quit(&event) {
                break;
            }

            // Route events based on popup state
            if state.has_command_error_popup() {
                // Modal popup event handling - only handle popup-specific events
                if terminal_manager.should_close_popup(&event) {
                    state.clear_command_error_popup();
                    render_current_state(
                        terminal_manager.terminal(),
                        last_data.as_ref(),
                        error_message.as_ref(),
                        interval,
                        &mut state,
                    )?;
                } else if terminal_manager.is_scroll_up(&event) {
                    state.scroll_popup_up();
                    render_current_state(
                        terminal_manager.terminal(),
                        last_data.as_ref(),
                        error_message.as_ref(),
                        interval,
                        &mut state,
                    )?;
                } else if terminal_manager.is_scroll_down(&event) {
                    state.scroll_popup_down();
                    render_current_state(
                        terminal_manager.terminal(),
                        last_data.as_ref(),
                        error_message.as_ref(),
                        interval,
                        &mut state,
                    )?;
                } else if matches!(event, crossterm::event::Event::Resize(_, _)) {
                    render_current_state(
                        terminal_manager.terminal(),
                        last_data.as_ref(),
                        error_message.as_ref(),
                        interval,
                        &mut state,
                    )?;
                }
                // All other events are ignored when popup is active
            } else {
                // Normal application event handling
                if matches!(event, crossterm::event::Event::Resize(_, _)) {
                    render_current_state(
                        terminal_manager.terminal(),
                        last_data.as_ref(),
                        error_message.as_ref(),
                        interval,
                        &mut state,
                    )?;
                }
            }
        }

        // Load data
        match std::fs::read_to_string(files[file_index]) {
            Ok(json_str) => match serde_json::from_str::<CephPgDump>(&json_str) {
                Ok(data) => {
                    last_data = Some(data);
                    error_message = None;
                    render_current_state(
                        terminal_manager.terminal(),
                        last_data.as_ref(),
                        error_message.as_ref(),
                        interval,
                        &mut state,
                    )?;
                    file_index = (file_index + 1) % files.len();
                }
                Err(e) => {
                    error_message = Some(format!("Parse error: {e}"));
                    render_current_state(
                        terminal_manager.terminal(),
                        last_data.as_ref(),
                        error_message.as_ref(),
                        interval,
                        &mut state,
                    )?;
                }
            },
            Err(e) => {
                error_message = Some(format!("File read error: {e}"));
                render_current_state(
                    terminal_manager.terminal(),
                    last_data.as_ref(),
                    error_message.as_ref(),
                    interval,
                    &mut state,
                )?;
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
                render_current_state(
                    terminal_manager.terminal(),
                    last_data.as_ref(),
                    error_message.as_ref(),
                    interval,
                    &mut state,
                )?;
            }
            SleepResult::PopupClose => {
                state.clear_command_error_popup();
                render_current_state(
                    terminal_manager.terminal(),
                    last_data.as_ref(),
                    error_message.as_ref(),
                    interval,
                    &mut state,
                )?;
            }
            SleepResult::PopupScrollUp => {
                state.scroll_popup_up();
                render_current_state(
                    terminal_manager.terminal(),
                    last_data.as_ref(),
                    error_message.as_ref(),
                    interval,
                    &mut state,
                )?;
            }
            SleepResult::PopupScrollDown => {
                state.scroll_popup_down();
                render_current_state(
                    terminal_manager.terminal(),
                    last_data.as_ref(),
                    error_message.as_ref(),
                    interval,
                    &mut state,
                )?;
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

    // Draw initial loading screen
    render_current_state(
        terminal_manager.terminal(),
        None,
        None,
        interval,
        &mut state,
    )?;

    loop {
        // Handle events
        if terminal_manager.poll_event(std::time::Duration::from_millis(100))? {
            let event = terminal_manager.read_event()?;

            // Always handle quit events
            if terminal_manager.should_quit(&event) {
                break;
            }

            // Route events based on popup state
            if state.has_command_error_popup() {
                // Modal popup event handling - only handle popup-specific events
                if terminal_manager.should_close_popup(&event) {
                    state.clear_command_error_popup();
                    render_current_state(
                        terminal_manager.terminal(),
                        last_data.as_ref(),
                        error_message.as_ref(),
                        interval,
                        &mut state,
                    )?;
                } else if terminal_manager.is_scroll_up(&event) {
                    state.scroll_popup_up();
                    render_current_state(
                        terminal_manager.terminal(),
                        last_data.as_ref(),
                        error_message.as_ref(),
                        interval,
                        &mut state,
                    )?;
                } else if terminal_manager.is_scroll_down(&event) {
                    state.scroll_popup_down();
                    render_current_state(
                        terminal_manager.terminal(),
                        last_data.as_ref(),
                        error_message.as_ref(),
                        interval,
                        &mut state,
                    )?;
                } else if matches!(event, crossterm::event::Event::Resize(_, _)) {
                    render_current_state(
                        terminal_manager.terminal(),
                        last_data.as_ref(),
                        error_message.as_ref(),
                        interval,
                        &mut state,
                    )?;
                }
                // All other events are ignored when popup is active
            } else {
                // Normal application event handling
                if matches!(event, crossterm::event::Event::Resize(_, _)) {
                    render_current_state(
                        terminal_manager.terminal(),
                        last_data.as_ref(),
                        error_message.as_ref(),
                        interval,
                        &mut state,
                    )?;
                }
            }
        }

        // Don't fetch data if popup is open
        if state.has_command_error_popup() {
            // Just sleep and check for events
            match terminal::sleep_with_event_check(interval, &terminal_manager).await? {
                SleepResult::Quit => {
                    terminal_manager.cleanup()?;
                    return Ok(());
                }
                SleepResult::Resize => {
                    // Trigger immediate redraw on resize
                    render_current_state(
                        terminal_manager.terminal(),
                        last_data.as_ref(),
                        error_message.as_ref(),
                        interval,
                        &mut state,
                    )?;
                }
                SleepResult::PopupClose => {
                    state.clear_command_error_popup();
                    render_current_state(
                        terminal_manager.terminal(),
                        last_data.as_ref(),
                        error_message.as_ref(),
                        interval,
                        &mut state,
                    )?;
                }
                SleepResult::PopupScrollUp => {
                    state.scroll_popup_up();
                    render_current_state(
                        terminal_manager.terminal(),
                        last_data.as_ref(),
                        error_message.as_ref(),
                        interval,
                        &mut state,
                    )?;
                }
                SleepResult::PopupScrollDown => {
                    state.scroll_popup_down();
                    render_current_state(
                        terminal_manager.terminal(),
                        last_data.as_ref(),
                        error_message.as_ref(),
                        interval,
                        &mut state,
                    )?;
                }
                SleepResult::Continue => {
                    // Normal flow, continue to next iteration
                }
            }
            continue;
        }

        // Fetch data
        match fetch_ceph_pg_dump(prefix_args).await {
            Ok(data) => {
                last_data = Some(data);
                error_message = None;
                render_current_state(
                    terminal_manager.terminal(),
                    last_data.as_ref(),
                    error_message.as_ref(),
                    interval,
                    &mut state,
                )?;
            }
            Err(e) => {
                // Check if this is a CommandError (special format)
                if let Some(cmd_error_str) = e.to_string().strip_prefix("CommandError:") {
                    // This is a command error, show it in popup
                    if let Ok(cmd_error) = parse_command_error(cmd_error_str) {
                        state.set_command_error_popup(cmd_error);
                        error_message = None; // Clear regular error message
                    } else {
                        error_message = Some(e.to_string());
                    }
                } else {
                    error_message = Some(e.to_string());
                }
                render_current_state(
                    terminal_manager.terminal(),
                    last_data.as_ref(),
                    error_message.as_ref(),
                    interval,
                    &mut state,
                )?;
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
                render_current_state(
                    terminal_manager.terminal(),
                    last_data.as_ref(),
                    error_message.as_ref(),
                    interval,
                    &mut state,
                )?;
            }
            SleepResult::PopupClose => {
                state.clear_command_error_popup();
                render_current_state(
                    terminal_manager.terminal(),
                    last_data.as_ref(),
                    error_message.as_ref(),
                    interval,
                    &mut state,
                )?;
            }
            SleepResult::PopupScrollUp => {
                state.scroll_popup_up();
                render_current_state(
                    terminal_manager.terminal(),
                    last_data.as_ref(),
                    error_message.as_ref(),
                    interval,
                    &mut state,
                )?;
            }
            SleepResult::PopupScrollDown => {
                state.scroll_popup_down();
                render_current_state(
                    terminal_manager.terminal(),
                    last_data.as_ref(),
                    error_message.as_ref(),
                    interval,
                    &mut state,
                )?;
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
        let stdout_str = String::from_utf8_lossy(&output.stdout);
        let stderr_str = String::from_utf8_lossy(&output.stderr);

        let command_str = if let Some(args) = prefix_args.filter(|args| !args.is_empty()) {
            format!("{} ceph pg dump --format json-pretty", args.join(" "))
        } else {
            "ceph pg dump --format json-pretty".to_string()
        };

        // Create the command error in our simple format
        let cmd_error_str = format!(
            "CommandError:{}|{}|{}|{}",
            command_str,
            output.status.code().unwrap_or(-1),
            stdout_str,
            stderr_str
        );

        return Err(cmd_error_str.into());
    }

    let json_str = String::from_utf8(output.stdout)?;

    let data: CephPgDump = serde_json::from_str(&json_str).map_err(|e| {
        let stderr_str = String::from_utf8_lossy(&output.stderr);
        let mut error_msg = format!(
            "Failed to parse ceph command output as JSON: {}\n\nCommand output was:\n{}",
            e,
            json_str.trim()
        );

        if !stderr_str.trim().is_empty() {
            error_msg.push_str(&format!("\n\nStderr output:\n{}", stderr_str.trim()));
        }

        error_msg
    })?;

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

fn render_loading_screen(f: &mut ratatui::Frame, interval: u64) {
    use ratatui::prelude::*;
    use ratatui::widgets::*;

    let size = f.area();

    // Create main layout
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Content
            Constraint::Length(3), // Footer
        ])
        .split(size);

    // Header
    let header_block = Block::default()
        .borders(Borders::ALL)
        .title("Ceph Doctor - Monitor")
        .title_style(Style::default().add_modifier(Modifier::BOLD));

    let header_content = format!("Refresh interval: {interval} seconds");
    let header_paragraph = Paragraph::new(header_content)
        .block(header_block)
        .style(Style::default());

    f.render_widget(header_paragraph, main_layout[0]);

    // Loading message
    let loading_block = Block::default()
        .borders(Borders::ALL)
        .title("Status")
        .title_style(Style::default().add_modifier(Modifier::BOLD));

    let loading_text = "Loading cluster data...\n\nFetching: ceph pg dump --format json-pretty\n\nPress 'q', Ctrl+C, or Esc to quit";
    let loading_paragraph = Paragraph::new(loading_text)
        .block(loading_block)
        .style(Style::default())
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);

    f.render_widget(loading_paragraph, main_layout[1]);

    // Footer
    render_footer(f, main_layout[2]);
}

fn render_current_state(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    data: Option<&CephPgDump>,
    error: Option<&String>,
    interval: u64,
    state: &mut MonitorState,
) -> Result<()> {
    terminal.draw(|f| {
        // First render the main UI
        match (data, error) {
            (Some(data), _) => {
                // Has data - render main UI (may also show error overlay)
                let error_opt = error.cloned();
                render_main_ui(f, data, None, interval, &error_opt, state);
            }
            (None, Some(error)) => {
                // No data but has error - render error screen
                render_error_screen(f, error, interval);
            }
            (None, None) => {
                // No data and no error - render loading screen
                render_loading_screen(f, interval);
            }
        }

        // Then render popup overlay if there's a command error popup
        if let Some(cmd_error) = state.get_command_error_popup() {
            render_command_error_popup(f, cmd_error);
        }
    })?;
    Ok(())
}

fn render_error_screen(f: &mut ratatui::Frame, error: &str, interval: u64) {
    use ratatui::prelude::*;
    use ratatui::widgets::*;

    let size = f.area();

    // Create main layout
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Content
            Constraint::Length(3), // Footer
        ])
        .split(size);

    // Header
    let header_block = Block::default()
        .borders(Borders::ALL)
        .title("Ceph Doctor - Monitor")
        .title_style(Style::default().add_modifier(Modifier::BOLD));

    let header_content = format!("Refresh interval: {interval} seconds");
    let header_paragraph = Paragraph::new(header_content)
        .block(header_block)
        .style(Style::default());

    f.render_widget(header_paragraph, main_layout[0]);

    // Error message
    let error_block = Block::default()
        .borders(Borders::ALL)
        .title("Error")
        .title_style(Style::default().add_modifier(Modifier::BOLD));

    let error_text =
        format!("Failed to fetch cluster data:\n\n{error}\n\nPress 'q', Ctrl+C, or Esc to quit");
    let error_paragraph = Paragraph::new(error_text)
        .block(error_block)
        .style(Style::default())
        .wrap(Wrap { trim: true });

    f.render_widget(error_paragraph, main_layout[1]);

    // Footer
    render_footer(f, main_layout[2]);
}

fn render_command_error_popup(f: &mut ratatui::Frame, cmd_error: &state::CommandError) {
    use ratatui::prelude::*;
    use ratatui::widgets::*;

    let area = f.area();

    // Create a centered popup area (85% of screen width, 80% of screen height)
    let popup_width = (area.width as f32 * 0.85) as u16;
    let popup_height = (area.height as f32 * 0.8) as u16;
    let popup_x = (area.width - popup_width) / 2;
    let popup_y = (area.height - popup_height) / 2;

    let popup_area = Rect::new(popup_x, popup_y, popup_width, popup_height);

    // Clear the popup area
    f.render_widget(Clear, popup_area);

    // Create unified popup block
    let popup_block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" Command Error: {} ", cmd_error.command))
        .title_style(Style::default().add_modifier(Modifier::BOLD))
        .border_style(Style::default().fg(Color::Red));

    let inner_area = popup_block.inner(popup_area);
    f.render_widget(popup_block, popup_area);

    // Create content layout within the unified block
    let content_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Content
            Constraint::Length(1), // Empty separator line
            Constraint::Length(1), // Footer
        ])
        .split(inner_area);

    // Build content text
    let mut content_lines = Vec::new();

    if cmd_error.exit_code != 0 {
        content_lines.push(Line::from(vec![
            Span::styled("Exit code: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::from(cmd_error.exit_code.to_string()),
        ]));
        content_lines.push(Line::from(""));
    }

    if !cmd_error.stdout.trim().is_empty() {
        content_lines.push(Line::from(Span::styled(
            "Command output:",
            Style::default().add_modifier(Modifier::BOLD),
        )));
        for line in cmd_error.stdout.lines() {
            content_lines.push(Line::from(line));
        }
        content_lines.push(Line::from(""));
    }

    if !cmd_error.stderr.trim().is_empty() {
        content_lines.push(Line::from(Span::styled(
            "Error output:",
            Style::default().add_modifier(Modifier::BOLD),
        )));
        for line in cmd_error.stderr.lines() {
            content_lines.push(Line::from(line));
        }
        content_lines.push(Line::from(""));
    }

    content_lines.push(Line::from(Span::styled(
        "This error suggests a problem with your Ceph configuration or connectivity.",
        Style::default().add_modifier(Modifier::ITALIC),
    )));

    let content_paragraph = Paragraph::new(content_lines)
        .wrap(Wrap { trim: true })
        .scroll((cmd_error.scroll_offset, 0));

    f.render_widget(content_paragraph, content_layout[0]);

    // Empty separator line (content_layout[1] is left empty)

    // Footer instructions on the last line
    let footer_text =
        "[Esc/Enter/Space] Close popup • [↑/k ↓/j] Scroll • [q/Ctrl+C] Quit application";
    let footer_paragraph = Paragraph::new(footer_text)
        .style(Style::default().add_modifier(Modifier::ITALIC))
        .alignment(Alignment::Center);

    f.render_widget(footer_paragraph, content_layout[2]);
}

fn parse_command_error(error_str: &str) -> Result<state::CommandError> {
    // Simple parsing - in a real implementation you might use JSON or a proper format
    // For now, let's create a simple format: command|exit_code|stdout|stderr
    let parts: Vec<&str> = error_str.split('|').collect();
    if parts.len() >= 4 {
        Ok(state::CommandError {
            command: parts[0].to_string(),
            exit_code: parts[1].parse().unwrap_or(-1),
            stdout: parts[2].to_string(),
            stderr: parts[3].to_string(),
            scroll_offset: 0,
        })
    } else {
        Err("Invalid command error format".into())
    }
}
