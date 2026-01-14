use std::{io, thread, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{Terminal, backend::CrosstermBackend, style::Color};

use crate::{
    config::SUSPENSE_DELAY_MS,
    game::{Chamber, Game, GameState},
    ui::draw_ui,
};

pub struct App;

impl App {
    pub fn run() -> Result<(), io::Error> {
        /* ─── Terminal setup ─── */
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        /* ─── Game setup ─── */
        let mut game = Game::new();

        let mut message = "Press [S] to spin the cylinder.".to_string();
        let mut message_color = Color::Cyan;

        /* ─── Main loop ─── */
        loop {
            terminal.draw(|f| draw_ui(f, game.rounds, &message, message_color))?;

            if let Event::Key(key) = event::read()? {
                match game.state {
                    GameState::Playing => match key.code {
                        KeyCode::Char('q') => break,

                        KeyCode::Char('s') => {
                            game.spin_cylinder();
                            message = "Cylinder spun. Press [F] to pull the trigger.".to_string();
                            message_color = Color::Cyan;
                        }

                        KeyCode::Char('f') => {
                            // Step 1: show trigger pulled
                            message = "Trigger Pulled !!!".to_string();
                            message_color = Color::Yellow;

                            // draw this frame immediately
                            terminal.draw(|f| draw_ui(f, game.rounds, &message, message_color))?;

                            // Step 2: suspense delay
                            thread::sleep(Duration::from_millis(SUSPENSE_DELAY_MS));

                            // Step 3: reveal outcome
                            match game.current_chamber() {
                                Chamber::Empty => {
                                    game.advance_round();

                                    if matches!(game.state, GameState::Won) {
                                        message = "🎉 You survived all rounds! Press any key."
                                            .to_string();
                                        message_color = Color::Green;
                                    } else {
                                        message = "😅 No shot fired. You are safe. Press any key."
                                            .to_string();
                                        message_color = Color::Green;
                                    }
                                }

                                Chamber::Bullet => {
                                    game.kill();
                                    message =
                                        "💥 BANG! You are dead. Press any key to exit.".to_string();
                                    message_color = Color::Red;
                                }
                            }
                        }

                        _ => {}
                    },

                    GameState::Waiting => {
                        // Any key continues to next round
                        game.reset_for_next_round();
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
}
