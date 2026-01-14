use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::config::MAX_ROUNDS;

pub fn draw_ui(f: &mut ratatui::Frame, rounds: usize, message: &str, color: Color) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(f.size());

    let title = Paragraph::new("🔫 RUSSIAN ROULETTE 🔫")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));

    let menu = Paragraph::new("[ S ] Spin Cylinder    [ F ] Fire    [ Q ] Quit")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title("Menu"));

    let info = Paragraph::new(format!("Round {}/{}", rounds, MAX_ROUNDS))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Status"));

    let msg = Paragraph::new(message)
        .alignment(Alignment::Center)
        .style(Style::default().fg(color))
        .block(Block::default().borders(Borders::ALL).title("Message"));

    f.render_widget(title, layout[0]);
    f.render_widget(menu, layout[1]);
    f.render_widget(info, layout[2]);
    f.render_widget(msg, layout[3]);
}
