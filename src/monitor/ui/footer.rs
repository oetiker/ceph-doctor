use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render_footer(f: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Controls")
        .title_style(Style::default().add_modifier(Modifier::BOLD));

    let text = Paragraph::new(Text::from("Press 'q', 'Esc', or 'Ctrl+C' to exit"))
        .block(block)
        .style(Style::default())
        .wrap(Wrap { trim: true });

    f.render_widget(text, area);
}
