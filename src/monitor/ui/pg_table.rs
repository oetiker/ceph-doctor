use crate::common::CephPgDump;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Row, Table},
    Frame,
};
use std::collections::HashMap;

pub fn render_pg_states(f: &mut Frame, area: Rect, data: &CephPgDump, use_colors: bool) {
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
                    "active+clean" => Color::Green,
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
        [ratatui::layout::Constraint::Length(8), ratatui::layout::Constraint::Min(0)]
    )
    .block(block)
    .header(
        Row::new(vec!["Count", "State"])
            .style(Style::default().add_modifier(Modifier::BOLD))
    );
    
    f.render_widget(table, area);
}

fn count_pg_states(pg_stats: &[crate::common::PgStats]) -> HashMap<String, usize> {
    let mut state_counts = HashMap::new();
    
    for pg in pg_stats {
        let state = pg.state.clone();
        *state_counts.entry(state).or_insert(0) += 1;
    }
    
    state_counts
}