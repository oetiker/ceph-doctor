use crate::common::CephPgDump;
use chrono::{DateTime, Utc};
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render_header(f: &mut Frame, area: Rect, data: &CephPgDump, interval: u64) {
    let now = Utc::now();
    let timestamp = DateTime::parse_from_rfc3339(&data.pg_map.stamp)
        .unwrap_or_else(|_| now.into())
        .format("%Y-%m-%d %H:%M:%S UTC");

    let title = format!("CEPH DOCTOR - Cluster Monitor ({interval}s interval)");
    let subtitle = format!("Last Update: {timestamp}");

    let header_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(title)
        .title_style(Style::default().add_modifier(Modifier::BOLD));

    let header_text = Paragraph::new(Text::from(subtitle))
        .block(header_block)
        .style(Style::default())
        .wrap(Wrap { trim: true });

    f.render_widget(header_text, area);
}
