use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render_error(f: &mut Frame, area: Rect, error: &str, use_colors: bool) {
    let error_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("ERROR")
        .title_style(Style::default().add_modifier(Modifier::BOLD))
        .border_style(if use_colors {
            Style::default().fg(Color::Red)
        } else {
            Style::default()
        });

    let error_text = Paragraph::new(Text::from(error))
        .block(error_block)
        .style(if use_colors {
            Style::default().fg(Color::Red)
        } else {
            Style::default()
        })
        .wrap(Wrap { trim: true });

    f.render_widget(error_text, area);
}
