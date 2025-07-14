use crate::common::OsdDataMovement;
use crate::monitor::data::formatter::*;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Cell, Row, Table},
    Frame,
};
use std::collections::HashMap;

pub fn render_osd_data_movement_table(
    f: &mut Frame,
    area: Rect,
    osd_movements: HashMap<u32, OsdDataMovement>,
    _use_colors: bool,
) {
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
        let time_in = movement.incoming_predicted_time_secs.map_or("N/A".to_string(), format_time);
        let time_out = movement.outgoing_predicted_time_secs.map_or("N/A".to_string(), format_time);
        let rate_in = movement.incoming_rate.map_or("N/A".to_string(), |r| format!("{r:.1}"));
        let rate_out = movement.outgoing_rate.map_or("N/A".to_string(), |r| format!("{r:.1}"));
        
        let cells = vec![
            Cell::from(osd_id.to_string()),
            Cell::from(format!("{:>6}", format_number(missing_waiting))),
            if missing_active > 0 {
                Cell::from(format!("{:>6}", format_number(missing_active))).style(Style::default().add_modifier(Modifier::BOLD))
            } else {
                Cell::from(format!("{:>6}", format_number(missing_active)))
            },
            Cell::from(format!("{rate_in:>7}")),
            Cell::from(format!("{time_in:>9}")),
            Cell::from(format!("{:>6}", format_number(excess_waiting))),
            if excess_active > 0 {
                Cell::from(format!("{:>6}", format_number(excess_active))).style(Style::default().add_modifier(Modifier::BOLD))
            } else {
                Cell::from(format!("{:>6}", format_number(excess_active)))
            },
            Cell::from(format!("{rate_out:>7}")),
            Cell::from(format!("{time_out:>9}")),
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

pub fn render_inconsistent_pgs_table(
    f: &mut Frame,
    area: Rect,
    inconsistent_pgs: HashMap<String, crate::common::InconsistentPgProgress>,
    use_colors: bool,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Inconsistent PGs")
        .title_style(Style::default().add_modifier(Modifier::BOLD));
    
    if inconsistent_pgs.is_empty() {
        let text = ratatui::widgets::Paragraph::new("No inconsistent PGs found")
            .block(block)
            .style(Style::default())
            .wrap(ratatui::widgets::Wrap { trim: true });
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
        let rate_str = pg.scrub_rate.map_or("N/A".to_string(), |r| format!("{r:.1}"));
        let eta_str = pg.eta_seconds.map_or("N/A".to_string(), format_time);
        let scrub_progress = if pg.num_objects > 0 {
            let percentage = (pg.objects_scrubbed as f64 / pg.num_objects as f64) * 100.0;
            format!("{percentage:.1}%")
        } else {
            "0.0%".to_string()
        };
        
        let row_style = if use_colors {
            if pg.state.contains("repair") {
                Style::default().fg(ratatui::style::Color::Red)
            } else {
                Style::default().fg(ratatui::style::Color::Yellow)
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