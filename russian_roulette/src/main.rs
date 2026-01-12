use rand::{rng, seq::SliceRandom};
use std::io;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

const CHAMBERS: usize = 6;
const MAX_ROUNDS: usize = 6;

#[derive(Clone, Copy)]
enum Chamber {
    Empty,
    Bullet,
}

enum GameState {
    Playing,
    Waiting, // waiting for "next"
    Dead,
    Won,
}

fn main() -> Result<(), io::Error> {
    /* ─── Terminal setup ─── */
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    /* ─── Game state ─── */
    let mut rounds = 0;
    let mut chambers = spin();
    let mut state = GameState::Playing;

    let mut message = "Press [S] to spin the cylinder.".to_string();
    let mut message_color = Color::Cyan;

    /* ─── Main loop ─── */
    loop {
        terminal.draw(|f| {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(3), // title
                    Constraint::Length(5), // menu
                    Constraint::Length(3), // info
                    Constraint::Min(1),    // message
                ])
                .split(f.size());

            let title = Paragraph::new("🔫 RUSSIAN ROULETTE 🔫")
                .alignment(Alignment::Center)
                .style(
                    Style::default()
                        .fg(Color::Red)
                        .add_modifier(Modifier::BOLD),
                )
                .block(Block::default().borders(Borders::ALL));

            let menu = Paragraph::new(
                "[ S ] Spin Cylinder    [ F ] Fire    [ Q ] Quit",
            )
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL).title("Menu"));

            let info = Paragraph::new(format!(
                "Round {}/{}",
                rounds, MAX_ROUNDS
            ))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Status"));

            let msg = Paragraph::new(message.as_str())
                .alignment(Alignment::Center)
                .style(Style::default().fg(message_color))
                .block(Block::default().borders(Borders::ALL).title("Message"));

            f.render_widget(title, layout[0]);
            f.render_widget(menu, layout[1]);
            f.render_widget(info, layout[2]);
            f.render_widget(msg, layout[3]);
        })?;

        if let Event::Key(key) = event::read()? {
            match state {
                GameState::Playing => match key.code {
                    KeyCode::Char('q') => break,

                    KeyCode::Char('s') => {
                        chambers = spin();
                        message = "Cylinder spun. Press [F] to pull the trigger.".to_string();
                        message_color = Color::Cyan;
                    }

                    KeyCode::Char('f') => {
                        match chambers[0] {
                            Chamber::Empty => {
                                rounds += 1;

                                if rounds >= MAX_ROUNDS {
                                    state = GameState::Won;
                                    message = "🎉 You survived all rounds! Press any key.".to_string();
                                    message_color = Color::Yellow;
                                } else {
                                    state = GameState::Waiting;
                                    message =
                                        "😅 Click… safe. Press any key for next round.".to_string();
                                    message_color = Color::Green;
                                }
                            }
                            Chamber::Bullet => {
                                state = GameState::Dead;
                                message =
                                    "💥 BANG! You are dead. Press any key to exit.".to_string();
                                message_color = Color::Red;
                            }
                        }
                    }

                    _ => {}
                },

                GameState::Waiting => {
                    // any key continues to next round
                    chambers = spin();
                    state = GameState::Playing;
                    message = "Next round. Press [S] to spin the cylinder.".to_string();
                    message_color = Color::Cyan;
                }

                GameState::Dead | GameState::Won => {
                    // wait for key, then exit
                    break;
                }
            }
        }
    }

    /* ─── Cleanup ─── */
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

/* ─── Helper ─── */

fn spin() -> Vec<Chamber> {
    let mut chambers = vec![Chamber::Empty; CHAMBERS - 1];
    chambers.push(Chamber::Bullet);
    chambers.shuffle(&mut rng());
    chambers
}
