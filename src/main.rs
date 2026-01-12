use rand::{rng, seq::SliceRandom};
use std::{
    io,
    thread,
    time::Duration,
};

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
const SUSPENSE_DELAY_MS: u64 = 600; //  suspense timing

#[derive(Clone, Copy)]
enum Chamber {
    Empty,
    Bullet,
}

enum GameState {
    Playing,
    Waiting,
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
        terminal.draw(|f| draw_ui(f, rounds, &message, message_color))?;

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
                        // ── Step 1: show trigger pulled
                        message = "Trigger Pulled !!!".to_string();
                        message_color = Color::Yellow;

                        // draw THIS frame
                        terminal.draw(|f| {
                            draw_ui(f, rounds, &message, message_color)
                        })?;

                        // ── Step 2: suspense delay
                        thread::sleep(Duration::from_millis(SUSPENSE_DELAY_MS));

                        // ── Step 3: reveal outcome
                        match chambers[0] {
                            Chamber::Empty => {
                                rounds += 1;

                                if rounds >= MAX_ROUNDS {
                                    state = GameState::Won;
                                    message = "🎉 You survived all rounds! Press any key."
                                        .to_string();
                                    message_color = Color::Green;
                                } else {
                                    state = GameState::Waiting;
                                    message =
                                        "😅 No shot fired. You are safe. Press any key."
                                            .to_string();
                                    message_color = Color::Green;
                                }
                            }
                            Chamber::Bullet => {
                                state = GameState::Dead;
                                message =
                                    "💥 BANG! You are dead. Press any key to exit."
                                        .to_string();
                                message_color = Color::Red;
                            }
                        }
                    }

                    _ => {}
                },

                GameState::Waiting => {
                    chambers = spin();
                    state = GameState::Playing;
                    message = "Next round. Press [S] to spin the cylinder.".to_string();
                    message_color = Color::Cyan;
                }

                GameState::Dead | GameState::Won => {
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

/* ─── UI ─── */

fn draw_ui(
    f: &mut ratatui::Frame,
    rounds: usize,
    message: &str,
    color: Color,
) {
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

    let msg = Paragraph::new(message)
        .alignment(Alignment::Center)
        .style(Style::default().fg(color))
        .block(Block::default().borders(Borders::ALL).title("Message"));

    f.render_widget(title, layout[0]);
    f.render_widget(menu, layout[1]);
    f.render_widget(info, layout[2]);
    f.render_widget(msg, layout[3]);
}

/* ─── Helper ─── */

fn spin() -> Vec<Chamber> {
    let mut chambers = vec![Chamber::Empty; CHAMBERS - 1];
    chambers.push(Chamber::Bullet);
    chambers.shuffle(&mut rng());
    chambers
}