use crate::common::CephPgDump;
use crate::monitor::state::{MonitorState, RecoveryData};
use crate::monitor::data::formatter::*;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Row, Table, Wrap},
    Frame,
};

pub fn render_recovery_progress(
    f: &mut Frame,
    area: Rect,
    data: &CephPgDump,
    state: &mut MonitorState,
    interval: u64,
) {
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
    let mut rows = Vec::new();
    
    for (category, current_objects) in active_categories {
        // Calculate estimated bytes for this category
        let estimated_bytes = (current_objects as f64 * avg_object_size) as i64;
        
        // Add current data to history
        let current_data = RecoveryData {
            objects: current_objects,
            bytes: estimated_bytes,
        };
        state.add_recovery_data(category, current_data, HISTORY_SIZE);
        
        // Calculate rates (objects per second and bytes per second)
        let (object_rate, data_rate) = if let Some(history) = state.get_recovery_history(category) {
            if history.len() >= 2 {
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
            let rate = object_rate.abs();
            format!("{rate:.1}/s")
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