use crate::common::{CephPgDump, OsdDataMovement, InconsistentPgProgress};
use crate::Result;
use std::process::Command;
use std::time::Duration;
use tokio::time;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use std::io;
use std::env;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Row, Table, Wrap, Cell},
    Frame, Terminal,
};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

pub async fn run_test(interval: u64) -> Result<()> {
    let files = ["state-a.json", "state-b.json"];
    let mut file_index = 0;
    
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let mut last_data: Option<CephPgDump> = None;
    let mut error_message: Option<String> = None;
    
    loop {
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    match key {
                        KeyEvent {
                            code: KeyCode::Char('c'),
                            modifiers: KeyModifiers::CONTROL,
                            ..
                        }
                        | KeyEvent {
                            code: KeyCode::Char('q'),
                            ..
                        }
                        | KeyEvent {
                            code: KeyCode::Esc,
                            ..
                        } => {
                            break;
                        }
                        _ => {}
                    }
                }
                Event::Resize(_, _) => {
                    // Trigger a redraw on resize
                    if let Some(ref data) = last_data {
                        terminal.draw(|f| {
                            render_ui(f, data, None, interval, &error_message);
                        })?;
                    }
                }
                _ => {}
            }
        }
        
        match std::fs::read_to_string(files[file_index]) {
            Ok(json_str) => {
                match serde_json::from_str::<CephPgDump>(&json_str) {
                    Ok(data) => {
                        terminal.draw(|f| {
                            render_ui(f, &data, last_data.as_ref(), interval, &error_message);
                        })?;
                        last_data = Some(data);
                        error_message = None;
                        file_index = (file_index + 1) % files.len();
                    }
                    Err(e) => {
                        error_message = Some(format!("Parse error: {}", e));
                    }
                }
            }
            Err(e) => {
                error_message = Some(format!("File read error: {}", e));
            }
        }
        
        // Sleep for the interval, but check for quit events every second
        let mut remaining_time = interval;
        while remaining_time > 0 {
            let sleep_duration = std::cmp::min(remaining_time, 1); // Sleep max 1 second at a time
            time::sleep(Duration::from_secs(sleep_duration)).await;
            remaining_time -= sleep_duration;
            
            // Check for quit events during sleep
            if event::poll(Duration::from_millis(0))? {
                match event::read()? {
                    Event::Key(key) => {
                        match key {
                            KeyEvent {
                                code: KeyCode::Char('c'),
                                modifiers: KeyModifiers::CONTROL,
                                ..
                            }
                            | KeyEvent {
                                code: KeyCode::Char('q'),
                                ..
                            }
                            | KeyEvent {
                                code: KeyCode::Esc,
                                ..
                            } => {
                                disable_raw_mode()?;
                                execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                                return Ok(());
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    
    Ok(())
}

pub async fn run(interval: u64) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let mut last_data: Option<CephPgDump> = None;
    let mut error_message: Option<String> = None;
    
    loop {
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    match key {
                        KeyEvent {
                            code: KeyCode::Char('c'),
                            modifiers: KeyModifiers::CONTROL,
                            ..
                        }
                        | KeyEvent {
                            code: KeyCode::Char('q'),
                            ..
                        }
                        | KeyEvent {
                            code: KeyCode::Esc,
                            ..
                        } => {
                            break;
                        }
                        _ => {}
                    }
                }
                Event::Resize(_, _) => {
                    // Trigger a redraw on resize
                    if let Some(ref data) = last_data {
                        terminal.draw(|f| {
                            render_ui(f, data, None, interval, &error_message);
                        })?;
                    }
                }
                _ => {}
            }
        }
        
        match fetch_ceph_pg_dump().await {
            Ok(data) => {
                terminal.draw(|f| {
                    render_ui(f, &data, last_data.as_ref(), interval, &error_message);
                })?;
                last_data = Some(data);
                error_message = None;
            }
            Err(e) => {
                error_message = Some(e.to_string());
                if let Some(ref data) = last_data {
                    terminal.draw(|f| {
                        render_ui(f, data, None, interval, &error_message);
                    })?;
                }
            }
        }
        
        // Sleep for the interval, but check for quit events every 0.25 seconds
        let mut remaining_time = interval as f32;
        while remaining_time > 0.0 {
            let sleep_duration = remaining_time.min(0.25); // Sleep max 0.25 seconds at a time
            time::sleep(Duration::from_secs_f32(sleep_duration)).await;
            remaining_time -= sleep_duration;
            
            // Check for quit events during sleep
            if event::poll(Duration::from_millis(0))? {
                match event::read()? {
                    Event::Key(key) => {
                        match key {
                            KeyEvent {
                                code: KeyCode::Char('c'),
                                modifiers: KeyModifiers::CONTROL,
                                ..
                            }
                            | KeyEvent {
                                code: KeyCode::Char('q'),
                                ..
                            }
                            | KeyEvent {
                                code: KeyCode::Esc,
                                ..
                            } => {
                                disable_raw_mode()?;
                                execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                                return Ok(());
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    
    Ok(())
}

async fn fetch_ceph_pg_dump() -> Result<CephPgDump> {
    let output = Command::new("ssh")
        .args(&["ceph11-adm", "sudo", "ceph", "pg", "dump", "--format", "json-pretty"])
        .output()?;
    
    if !output.status.success() {
        return Err(format!("ceph command failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }
    
    let json_str = String::from_utf8(output.stdout)?;
    let data: CephPgDump = serde_json::from_str(&json_str)?;
    
    Ok(data)
}

fn render_ui(
    f: &mut Frame,
    data: &CephPgDump,
    last_data: Option<&CephPgDump>,
    interval: u64,
    error_msg: &Option<String>,
) {
    // Check for NO_COLOR environment variable
    let use_colors = env::var("NO_COLOR").is_err();
    let size = f.area();
    
    // Create main layout
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(0),     // Content
            Constraint::Length(3),  // Footer
        ])
        .split(size);
    
    // Render header
    render_header(f, main_layout[0], data, interval);
    
    // Render error message if present
    let content_area = if let Some(ref error) = error_msg {
        let error_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Error
                Constraint::Min(0),     // Content
            ])
            .split(main_layout[1]);
        
        render_error(f, error_layout[0], error, use_colors);
        error_layout[1]
    } else {
        main_layout[1]
    };
    
    // Calculate dynamic heights
    let pg_states = count_pg_states(&data.pg_map.pg_stats);
    let inconsistent_pgs = calculate_inconsistent_pg_progress(data, last_data, interval);
    let recovery_progress_height = calculate_recovery_progress_height(data);
    
    let pg_states_height = std::cmp::min(pg_states.len() + 3, 15) as u16; // +3 for title, header, and borders, max 15
    
    // Split content area into sections based on whether there are inconsistent PGs
    let (content_layout, osd_layout_index) = if inconsistent_pgs.is_empty() {
        // No inconsistent PGs - hide the block entirely
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(recovery_progress_height), // Recovery progress (dynamic)
                Constraint::Length(pg_states_height), // PG states (dynamic)
                Constraint::Min(0),     // OSD Data Movement
            ])
            .split(content_area);
        (layout, 2)
    } else {
        // Show inconsistent PGs block
        let inconsistent_pgs_height = std::cmp::max(std::cmp::min(inconsistent_pgs.len() + 3, 15), 4) as u16; // +3 for title, header, borders, min 4, max 15
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(recovery_progress_height), // Recovery progress (dynamic)
                Constraint::Length(pg_states_height), // PG states (dynamic)
                Constraint::Length(inconsistent_pgs_height), // Inconsistent PGs (dynamic)
                Constraint::Min(0),     // OSD Data Movement
            ])
            .split(content_area);
        (layout, 3)
    };
    
    // Render sections
    render_recovery_progress(f, content_layout[0], data, last_data, interval);
    render_pg_states(f, content_layout[1], data, use_colors);
    
    // Only render inconsistent PGs table if there are inconsistent PGs
    if !inconsistent_pgs.is_empty() {
        render_inconsistent_pgs_table(f, content_layout[2], inconsistent_pgs, use_colors);
    }

    let osd_data_movements = calculate_osd_data_movement(data, last_data, interval);
    render_osd_data_movement_table(f, content_layout[osd_layout_index], osd_data_movements, use_colors);
    
    // Render footer
    render_footer(f, main_layout[2]);
}

fn render_header(f: &mut Frame, area: Rect, data: &CephPgDump, interval: u64) {
    let now = Utc::now();
    let timestamp = DateTime::parse_from_rfc3339(&data.pg_map.stamp)
        .unwrap_or_else(|_| now.into())
        .format("%Y-%m-%d %H:%M:%S UTC");
    
    let title = format!("CEPH DOCTOR - Cluster Monitor ({}s interval)", interval);
    let subtitle = format!("Last Update: {}", timestamp);
    
    let header_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(title)
        .title_style(Style::default().add_modifier(Modifier::BOLD));
    
    let header_text = Paragraph::new(subtitle)
        .block(header_block)
        .style(Style::default())
        .wrap(Wrap { trim: true });
    
    f.render_widget(header_text, area);
}

fn render_error(f: &mut Frame, area: Rect, error: &str, use_colors: bool) {
    let error_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("ERROR")
        .title_style(Style::default().add_modifier(Modifier::BOLD))
        .border_style(if use_colors { Style::default().fg(Color::Red) } else { Style::default() });
    
    let error_text = Paragraph::new(error)
        .block(error_block)
        .style(if use_colors { Style::default().fg(Color::Red) } else { Style::default() })
        .wrap(Wrap { trim: true });
    
    f.render_widget(error_text, area);
}


fn render_recovery_progress(f: &mut Frame, area: Rect, data: &CephPgDump, _last_data: Option<&CephPgDump>, interval: u64) {
    use std::sync::Mutex;
    use std::sync::LazyLock;
    
    #[derive(Debug, Clone)]
    struct RecoveryData {
        objects: i64,
        bytes: i64,
    }
    
    static RECOVERY_HISTORY: LazyLock<Mutex<HashMap<String, Vec<RecoveryData>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
    const HISTORY_SIZE: usize = 20;
    
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Recovery Progress")
        .title_style(Style::default().add_modifier(Modifier::BOLD));
    
    let stats_sum = &data.pg_map.pg_stats_sum.stat_sum;
    
    // Calculate average object size for estimation
    let avg_object_size = if stats_sum.num_objects > 0 {
        stats_sum.num_bytes as f64 / stats_sum.num_objects as f64
    } else {
        0.0
    };
    
    // Define recovery categories with current values
    let categories = vec![
        ("Missing", stats_sum.num_objects_missing),
        ("Unfound", stats_sum.num_objects_unfound),
        ("Misplaced", stats_sum.num_objects_misplaced),
        ("Degraded", stats_sum.num_objects_degraded),
    ];
    
    // Filter categories with values > 0
    let active_categories: Vec<_> = categories
        .into_iter()
        .filter(|(_, value)| *value > 0)
        .collect();
    
    // If no active categories, show a message
    if active_categories.is_empty() {
        let text = Paragraph::new("No recovery operations in progress")
            .block(block)
            .style(Style::default())
            .wrap(Wrap { trim: true });
        f.render_widget(text, area);
        return;
    }
    
    // Calculate rates for each category
    let mut history_guard = RECOVERY_HISTORY.lock().unwrap();
    let mut rows = Vec::new();
    
    for (category, current_objects) in active_categories {
        let history_key = category.to_string();
        let history = history_guard.entry(history_key).or_insert_with(Vec::new);
        
        // Calculate estimated bytes for this category
        let estimated_bytes = (current_objects as f64 * avg_object_size) as i64;
        
        // Add current data to history
        let current_data = RecoveryData {
            objects: current_objects,
            bytes: estimated_bytes,
        };
        history.push(current_data);
        if history.len() > HISTORY_SIZE {
            history.remove(0);
        }
        
        // Calculate rates (objects per second and bytes per second)
        let (object_rate, data_rate) = if history.len() >= 2 {
            let oldest_data = &history[0];
            let time_elapsed = (history.len() - 1) as f64 * interval as f64;
            
            if time_elapsed > 0.0 {
                let object_change = current_objects - oldest_data.objects;
                let byte_change = estimated_bytes - oldest_data.bytes;
                (
                    object_change as f64 / time_elapsed,
                    byte_change as f64 / time_elapsed,
                )
            } else {
                (0.0, 0.0)
            }
        } else {
            (0.0, 0.0)
        };
        
        // Calculate ETA
        let eta = if object_rate < 0.0 && current_objects > 0 {
            let remaining_time = (current_objects as f64 / -object_rate) as u64;
            format_time(remaining_time)
        } else if current_objects > 0 {
            "calculating...".to_string()
        } else {
            "complete".to_string()
        };
        
        // Format rate displays (show positive values for recovery progress)
        let object_rate_display = if object_rate.abs() < 0.01 {
            "0.0/s".to_string()
        } else {
            format!("{:.1}/s", object_rate.abs())
        };
        
        let data_rate_display = if data_rate.abs() < 1024.0 {
            "0B/s".to_string()
        } else {
            format_bytes_per_second(data_rate.abs())
        };
        
        rows.push(Row::new(vec![
            category.to_string(),
            format_number(current_objects),
            object_rate_display,
            data_rate_display,
            eta,
        ]));
    }
    
    let header = Row::new(vec!["Category", "Count", "Obj/s", "Data/s", "ETA"])
        .style(Style::default().add_modifier(Modifier::BOLD))
        .height(1);
    
    let table = Table::new(
        rows,
        [
            Constraint::Length(10),  // Category
            Constraint::Length(8),   // Count
            Constraint::Length(8),   // Obj/s
            Constraint::Length(10),  // Data/s
            Constraint::Length(12),  // ETA
        ],
    )
    .header(header)
    .block(block);
    
    f.render_widget(table, area);
}

fn render_pg_states(f: &mut Frame, area: Rect, data: &CephPgDump, use_colors: bool) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Placement Group States")
        .title_style(Style::default().add_modifier(Modifier::BOLD));
    
    let pg_states = count_pg_states(&data.pg_map.pg_stats);
    let mut sorted_states: Vec<_> = pg_states.iter().collect();
    sorted_states.sort_by(|a, b| b.1.cmp(a.1));
    
    let rows: Vec<Row> = sorted_states.iter()
        .take(10)  // Show top 10 states
        .map(|(state, count)| {
            let color = if use_colors {
                match state.as_str() {
                    s if s.contains("inconsistent") || s.contains("incomplete") || s.contains("down") => Color::Red,
                    s if s == "active+clean" => Color::Green,
                    s if s.contains("backfilling") || s.contains("recovery") || s.contains("remapped") => Color::Yellow,
                    _ => Color::Reset,
                }
            } else {
                Color::Reset
            };
            
            Row::new(vec![
                count.to_string(),
                state.to_string(),
            ]).style(Style::default().fg(color))
        })
        .collect();
    
    let table = Table::new(
        rows,
        [Constraint::Length(8), Constraint::Min(0)]
    )
    .block(block)
    .header(
        Row::new(vec!["Count", "State"])
            .style(Style::default().add_modifier(Modifier::BOLD))
    );
    
    f.render_widget(table, area);
}




fn render_footer(f: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Controls")
        .title_style(Style::default().add_modifier(Modifier::BOLD));
    
    let text = Paragraph::new("Press 'q', 'Esc', or 'Ctrl+C' to exit")
        .block(block)
        .style(Style::default())
        .wrap(Wrap { trim: true });
    
    f.render_widget(text, area);
}

fn calculate_osd_data_movement(
    current_data: &CephPgDump,
    _last_data: Option<&CephPgDump>,
    interval: u64,
) -> HashMap<u32, OsdDataMovement> {
    // Use a static HashMap to persist OSD movements across calls
    use std::sync::Mutex;
    use std::collections::HashMap;
    
    static OSD_MOVEMENTS: Mutex<Option<HashMap<u32, OsdDataMovement>>> = Mutex::new(None);
    const HISTORY_SIZE: usize = 20; // Keep 20 data points for rate calculation
    
    let mut osd_movements_guard = OSD_MOVEMENTS.lock().unwrap();
    let mut osd_movements = osd_movements_guard.take().unwrap_or_else(HashMap::new);

    // Initialize all OSDs that are currently up
    for osd_stat in &current_data.pg_map.osd_stats {
        osd_movements.entry(osd_stat.osd).or_insert_with(|| OsdDataMovement { 
            osd_id: osd_stat.osd, 
            ..Default::default() 
        });
    }

    // Reset current counts for all OSDs
    for movement in osd_movements.values_mut() {
        movement.incoming_objects = 0;
        movement.outgoing_objects = 0;
        movement.missing_objects = 0;
        movement.excess_objects = 0;
        movement.missing_objects_waiting = 0;
        movement.excess_objects_waiting = 0;
        movement.missing_objects_active = 0;
        movement.excess_objects_active = 0;
    }

    // Process ALL PGs to sum up misplaced objects per OSD (not just active/waiting ones)
    for current_pg in &current_data.pg_map.pg_stats {
        // Check if PG has misplaced objects
        if current_pg.stat_sum.num_objects_misplaced > 0 {
            let current_up_set: HashSet<u32> = current_pg.up.iter().cloned().collect();
            let current_acting_set: HashSet<u32> = current_pg.acting.iter().cloned().collect();

            // OSDs that are in 'up' but not in 'acting' (need data - missing objects)
            let missing_osds: Vec<u32> = current_up_set.difference(&current_acting_set).cloned().collect();
            // OSDs that are in 'acting' but not in 'up' (have excess data)
            let excess_osds: Vec<u32> = current_acting_set.difference(&current_up_set).cloned().collect();

            // Use actual misplaced object count from PG stats
            let pg_misplaced_objects = current_pg.stat_sum.num_objects_misplaced;

            // Determine state for categorization
            let is_actively_moving = current_pg.state.contains("recovering") ||
                                    current_pg.state.contains("backfilling");
            let is_waiting_to_move = current_pg.state.contains("backfill_wait");

            // Sum up misplaced objects for OSDs that need data
            for &osd_id in &missing_osds {
                let entry = osd_movements.entry(osd_id).or_insert_with(|| OsdDataMovement { osd_id, ..Default::default() });
                entry.incoming_objects += pg_misplaced_objects;
                entry.missing_objects += pg_misplaced_objects;
                
                if is_actively_moving {
                    entry.missing_objects_active += pg_misplaced_objects;
                } else if is_waiting_to_move {
                    entry.missing_objects_waiting += pg_misplaced_objects;
                }
            }

            // Sum up misplaced objects for OSDs that have excess data
            for &osd_id in &excess_osds {
                let entry = osd_movements.entry(osd_id).or_insert_with(|| OsdDataMovement { osd_id, ..Default::default() });
                entry.outgoing_objects += pg_misplaced_objects;
                entry.excess_objects += pg_misplaced_objects;
                
                if is_actively_moving {
                    entry.excess_objects_active += pg_misplaced_objects;
                } else if is_waiting_to_move {
                    entry.excess_objects_waiting += pg_misplaced_objects;
                }
            }
        }
    }

    // Update historical data and calculate ETA
    for (_osd_id, movement) in osd_movements.iter_mut() {
        // Add current missing objects count to history
        movement.missing_objects_history.push(movement.missing_objects);
        if movement.missing_objects_history.len() > HISTORY_SIZE {
            movement.missing_objects_history.remove(0);
        }

        // Add current excess objects count to history
        movement.excess_objects_history.push(movement.excess_objects);
        if movement.excess_objects_history.len() > HISTORY_SIZE {
            movement.excess_objects_history.remove(0);
        }

        // Calculate ETA using oldest vs current entry (need at least 3 data points)
        // Only calculate ETA if there are active missing objects being moved
        if movement.missing_objects_history.len() >= 3 && movement.missing_objects_active > 0 {
            let oldest_missing = movement.missing_objects_history[0];
            let current_missing = movement.missing_objects;
            let time_elapsed = (movement.missing_objects_history.len() - 1) as f64 * interval as f64;
            
            if oldest_missing > current_missing && time_elapsed > 0.0 {
                let rate = (oldest_missing - current_missing) as f64 / time_elapsed;
                movement.incoming_rate = Some(rate);
                if rate > 0.0 && current_missing > 0 {
                    let remaining_time_secs = (current_missing as f64 / rate) as u64;
                    movement.incoming_predicted_time_secs = Some(remaining_time_secs);
                }
            }
        }

        // Only calculate ETA if there are active excess objects being moved
        if movement.excess_objects_history.len() >= 3 && movement.excess_objects_active > 0 {
            let oldest_excess = movement.excess_objects_history[0];
            let current_excess = movement.excess_objects;
            let time_elapsed = (movement.excess_objects_history.len() - 1) as f64 * interval as f64;
            
            if oldest_excess > current_excess && time_elapsed > 0.0 {
                let rate = (oldest_excess - current_excess) as f64 / time_elapsed;
                movement.outgoing_rate = Some(rate);
                if rate > 0.0 && current_excess > 0 {
                    let remaining_time_secs = (current_excess as f64 / rate) as u64;
                    movement.outgoing_predicted_time_secs = Some(remaining_time_secs);
                }
            }
        }
    }

    // Store back the updated movements
    *osd_movements_guard = Some(osd_movements.clone());
    
    osd_movements
}


fn count_pg_states(pg_stats: &[crate::common::PgStats]) -> HashMap<String, usize> {
    let mut state_counts = HashMap::new();
    
    for pg in pg_stats {
        let state = pg.state.clone();
        *state_counts.entry(state).or_insert(0) += 1;
    }
    
    state_counts
}

fn calculate_recovery_progress_height(data: &CephPgDump) -> u16 {
    let stats_sum = &data.pg_map.pg_stats_sum.stat_sum;
    
    // Count active categories (values > 0)
    let active_categories = [
        stats_sum.num_objects_missing,
        stats_sum.num_objects_unfound,
        stats_sum.num_objects_misplaced,
        stats_sum.num_objects_degraded,
    ].iter().filter(|&&value| value > 0).count();
    
    if active_categories == 0 {
        // Just title and message: borders + title + message
        3
    } else {
        // Table height: borders + title + data rows
        // +3 for borders and title, +1 for each active category
        (3 + active_categories) as u16
    }
}

fn format_number(num: i64) -> String {
    if num.abs() >= 1_000_000 {
        format!("{:.1}M", num as f64 / 1_000_000.0)
    } else if num.abs() >= 1_000 {
        format!("{:.1}K", num as f64 / 1_000.0)
    } else {
        num.to_string()
    }
}


fn format_time(seconds: u64) -> String {
    if seconds < 60 {
        format!("{:02}s", seconds)
    } else if seconds < 3600 {
        format!("{}m{:02}s", seconds / 60, seconds % 60)
    } else if seconds < 86400 {
        format!("{}h{:02}m{:02}s", seconds / 3600, (seconds % 3600) / 60, seconds % 60)
    } else {
        format!("{}d{:02}h{:02}m", seconds / 86400, (seconds % 86400) / 3600, (seconds % 3600) / 60)
    }
}

fn format_bytes_per_second(bytes_per_second: f64) -> String {
    let abs_rate = bytes_per_second.abs();
    
    if abs_rate < 1024.0 {
        format!("{:.0}B/s", bytes_per_second)
    } else if abs_rate < 1024.0 * 1024.0 {
        format!("{:.1}KB/s", bytes_per_second / 1024.0)
    } else if abs_rate < 1024.0 * 1024.0 * 1024.0 {
        format!("{:.1}MB/s", bytes_per_second / (1024.0 * 1024.0))
    } else {
        format!("{:.1}GB/s", bytes_per_second / (1024.0 * 1024.0 * 1024.0))
    }
}


fn render_osd_data_movement_table(f: &mut Frame, area: Rect, osd_movements: HashMap<u32, OsdDataMovement>, _use_colors: bool) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("OSD Data Movement")
        .title_style(Style::default().add_modifier(Modifier::BOLD));

    // Filter to only show OSDs with actual data movement and sort by missing objects (descending)
    let mut sorted_osds: Vec<u32> = osd_movements.keys()
        .filter(|&&osd_id| {
            let movement = osd_movements.get(&osd_id).unwrap();
            movement.missing_objects > 0 || movement.excess_objects > 0
        })
        .cloned()
        .collect();
    sorted_osds.sort_by(|&a, &b| {
        let movement_a = osd_movements.get(&a).unwrap();
        let movement_b = osd_movements.get(&b).unwrap();
        // Sort by total missing objects (active + waiting), then by total excess objects, then by OSD ID
        let total_missing_a = movement_a.missing_objects_active + movement_a.missing_objects_waiting;
        let total_missing_b = movement_b.missing_objects_active + movement_b.missing_objects_waiting;
        let total_excess_a = movement_a.excess_objects_active + movement_a.excess_objects_waiting;
        let total_excess_b = movement_b.excess_objects_active + movement_b.excess_objects_waiting;
        
        total_missing_b.cmp(&total_missing_a)
            .then_with(|| total_excess_b.cmp(&total_excess_a))
            .then_with(|| a.cmp(&b))
    });

    let header = Row::new([
        Cell::from(Text::from(vec![Line::from("OSD")])).style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from(Text::from(vec![Line::from("Missing"), Line::from("Waiting")])).style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from(Text::from(vec![Line::from("Missing"), Line::from("Active")])).style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from(Text::from(vec![Line::from("Missing"), Line::from("Rate/s")])).style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from(Text::from(vec![Line::from("Missing"), Line::from("ETA")])).style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from(Text::from(vec![Line::from("Excess"), Line::from("Waiting")])).style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from(Text::from(vec![Line::from("Excess"), Line::from("Active")])).style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from(Text::from(vec![Line::from("Excess"), Line::from("Rate/s")])).style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from(Text::from(vec![Line::from("Excess"), Line::from("ETA")])).style(Style::default().add_modifier(Modifier::BOLD)),
    ])
    .height(2);

    let rows = sorted_osds.into_iter().map(|osd_id| {
        let movement = osd_movements.get(&osd_id).unwrap();
        let missing_active = movement.missing_objects_active;
        let missing_waiting = movement.missing_objects_waiting;
        let excess_active = movement.excess_objects_active;
        let excess_waiting = movement.excess_objects_waiting;
        let time_in = movement.incoming_predicted_time_secs.map_or("N/A".to_string(), |t| format_time(t));
        let time_out = movement.outgoing_predicted_time_secs.map_or("N/A".to_string(), |t| format_time(t));
        let rate_in = movement.incoming_rate.map_or("N/A".to_string(), |r| format!("{:.1}", r));
        let rate_out = movement.outgoing_rate.map_or("N/A".to_string(), |r| format!("{:.1}", r));
        
        let cells = vec![
            Cell::from(osd_id.to_string()),
            Cell::from(format!("{:>6}", format_number(missing_waiting))),
            if missing_active > 0 {
                Cell::from(format!("{:>6}", format_number(missing_active))).style(Style::default().add_modifier(Modifier::BOLD))
            } else {
                Cell::from(format!("{:>6}", format_number(missing_active)))
            },
            Cell::from(format!("{:>7}", rate_in)),
            Cell::from(format!("{:>9}", time_in)),
            Cell::from(format!("{:>6}", format_number(excess_waiting))),
            if excess_active > 0 {
                Cell::from(format!("{:>6}", format_number(excess_active))).style(Style::default().add_modifier(Modifier::BOLD))
            } else {
                Cell::from(format!("{:>6}", format_number(excess_active)))
            },
            Cell::from(format!("{:>7}", rate_out)),
            Cell::from(format!("{:>9}", time_out)),
        ];
        Row::new(cells)
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(5),   // OSD
            Constraint::Length(8),   // Missing Waiting
            Constraint::Length(8),   // Missing Active
            Constraint::Length(8),   // Missing Rate/s
            Constraint::Length(11),  // Missing ETA
            Constraint::Length(8),   // Excess Waiting
            Constraint::Length(8),   // Excess Active
            Constraint::Length(8),   // Excess Rate/s
            Constraint::Length(11),  // Excess ETA
        ],
    )
    .header(header)
    .block(block)
    .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .highlight_symbol("» ");

    f.render_widget(table, area);
}

fn calculate_inconsistent_pg_progress(
    current_data: &CephPgDump,
    _last_data: Option<&CephPgDump>,
    interval: u64,
) -> HashMap<String, InconsistentPgProgress> {
    use std::sync::Mutex;
    use std::collections::HashMap;
    
    static INCONSISTENT_PG_PROGRESS: Mutex<Option<HashMap<String, InconsistentPgProgress>>> = Mutex::new(None);
    const HISTORY_SIZE: usize = 20; // Keep 20 data points for rate calculation
    
    let mut pg_progress_guard = INCONSISTENT_PG_PROGRESS.lock().unwrap();
    let mut pg_progress = pg_progress_guard.take().unwrap_or_else(HashMap::new);
    
    // Find PGs with inconsistent state
    for pg_stat in &current_data.pg_map.pg_stats {
        if pg_stat.state.contains("inconsistent") {
            let pgid = pg_stat.pgid.clone();
            let objects_scrubbed = pg_stat.objects_scrubbed.unwrap_or(0);
            
            let entry = pg_progress.entry(pgid.clone()).or_insert_with(|| InconsistentPgProgress {
                pgid: pgid.clone(),
                num_objects: pg_stat.stat_sum.num_object_copies,
                primary_osd: pg_stat.up_primary,
                up_osds: pg_stat.up.clone(),
                state: pg_stat.state.clone(),
                objects_scrubbed,
                scrubbed_history: Vec::new(),
                scrub_rate: None,
                eta_seconds: None,
            });
            
            // Update current data
            entry.num_objects = pg_stat.stat_sum.num_object_copies;
            entry.primary_osd = pg_stat.up_primary;
            entry.up_osds = pg_stat.up.clone();
            entry.state = pg_stat.state.clone();
            entry.objects_scrubbed = objects_scrubbed;
            
            // Add to history
            entry.scrubbed_history.push(objects_scrubbed);
            if entry.scrubbed_history.len() > HISTORY_SIZE {
                entry.scrubbed_history.remove(0);
            }
            
            // Calculate rate and ETA if we have enough history
            if entry.scrubbed_history.len() >= 3 {
                let oldest_scrubbed = entry.scrubbed_history[0];
                let current_scrubbed = objects_scrubbed;
                let time_elapsed = (entry.scrubbed_history.len() - 1) as f64 * interval as f64;
                
                if current_scrubbed > oldest_scrubbed && time_elapsed > 0.0 {
                    let rate = (current_scrubbed - oldest_scrubbed) as f64 / time_elapsed;
                    entry.scrub_rate = Some(rate);
                    
                    // Calculate ETA using actual object copies
                    let estimated_total_scrubs = entry.num_objects as u64;
                    if rate > 0.0 && current_scrubbed < estimated_total_scrubs {
                        let remaining_scrubs = estimated_total_scrubs - current_scrubbed;
                        let eta_seconds = (remaining_scrubs as f64 / rate) as u64;
                        entry.eta_seconds = Some(eta_seconds);
                    }
                }
            }
        }
    }
    
    // Remove PGs that are no longer inconsistent
    let current_inconsistent_pgs: HashSet<String> = current_data.pg_map.pg_stats
        .iter()
        .filter(|pg| pg.state.contains("inconsistent"))
        .map(|pg| pg.pgid.clone())
        .collect();
    
    pg_progress.retain(|pgid, _| current_inconsistent_pgs.contains(pgid));
    
    // Store back the updated progress
    *pg_progress_guard = Some(pg_progress.clone());
    
    pg_progress
}

fn render_inconsistent_pgs_table(
    f: &mut Frame,
    area: Rect,
    inconsistent_pgs: HashMap<String, InconsistentPgProgress>,
    use_colors: bool,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Inconsistent PGs")
        .title_style(Style::default().add_modifier(Modifier::BOLD));
    
    if inconsistent_pgs.is_empty() {
        let text = Paragraph::new("No inconsistent PGs found")
            .block(block)
            .style(Style::default())
            .wrap(Wrap { trim: true });
        f.render_widget(text, area);
        return;
    }
    
    // Sort PGs by PGID for consistent display
    let mut sorted_pgs: Vec<_> = inconsistent_pgs.values().collect();
    sorted_pgs.sort_by(|a, b| a.pgid.cmp(&b.pgid));
    
    let header = Row::new([
        Cell::from("PG ID").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("Objects").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("OSDs").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("Scrubbed").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("Rate/sec").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("ETA").style(Style::default().add_modifier(Modifier::BOLD)),
    ])
    .height(1);
    
    let rows = sorted_pgs.into_iter().map(|pg| {
        let rate_str = pg.scrub_rate.map_or("N/A".to_string(), |r| format!("{:.1}", r));
        let eta_str = pg.eta_seconds.map_or("N/A".to_string(), |t| format_time(t));
        let scrub_progress = if pg.num_objects > 0 {
            let percentage = (pg.objects_scrubbed as f64 / pg.num_objects as f64) * 100.0;
            format!("{:.1}%", percentage)
        } else {
            "0.0%".to_string()
        };
        
        let row_style = if use_colors {
            if pg.state.contains("repair") {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::Yellow)
            }
        } else {
            Style::default()
        };
        
        // Create OSD list with primary in bold
        let osd_list = {
            let mut spans = Vec::new();
            for (i, &osd) in pg.up_osds.iter().enumerate() {
                if i > 0 {
                    spans.push(ratatui::text::Span::raw(","));
                }
                if osd == pg.primary_osd {
                    spans.push(ratatui::text::Span::styled(
                        osd.to_string(),
                        Style::default().add_modifier(Modifier::BOLD)
                    ));
                } else {
                    spans.push(ratatui::text::Span::raw(osd.to_string()));
                }
            }
            Text::from(vec![Line::from(spans)])
        };
        
        Row::new([
            Cell::from(pg.pgid.clone()),
            Cell::from(format_number(pg.num_objects)),
            Cell::from(osd_list),
            Cell::from(scrub_progress),
            Cell::from(rate_str),
            Cell::from(eta_str),
        ]).style(row_style)
    });
    
    let table = Table::new(
        rows,
        [
            Constraint::Length(8),   // PG ID
            Constraint::Length(8),   // Objects
            Constraint::Length(12),  // Primary OSD
            Constraint::Length(12),  // Scrubbed
            Constraint::Length(10),  // Rate/sec
            Constraint::Length(12),  // ETA
        ],
    )
    .header(header)
    .block(block);
    
    f.render_widget(table, area);
}
