use rand::{rng, seq::SliceRandom};

use crate::config::{CHAMBERS, MAX_ROUNDS};

#[derive(Clone, Copy)]
pub enum Chamber {
    Empty,
    Bullet,
}

pub enum GameState {
    Playing,
    Waiting,
    Dead,
    Won,
}

pub struct Game {
    pub rounds: usize,
    pub chambers: Vec<Chamber>,
    pub state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Self {
            rounds: 0,
            chambers: spin(),
            state: GameState::Playing,
        }
    }

    pub fn spin_cylinder(&mut self) {
        self.chambers = spin();
    }

    pub fn is_won(&self) -> bool {
        self.rounds >= MAX_ROUNDS
    }

    pub fn current_chamber(&self) -> Chamber {
        self.chambers[0]
    }

    pub fn advance_round(&mut self) {
        self.rounds += 1;

        if self.is_won() {
            self.state = GameState::Won;
        } else {
            self.state = GameState::Waiting;
        }
    }

    pub fn kill(&mut self) {
        self.state = GameState::Dead;
    }

    pub fn reset_for_next_round(&mut self) {
        self.spin_cylinder();
        self.state = GameState::Playing;
    }
}

fn spin() -> Vec<Chamber> {
    let mut chambers = vec![Chamber::Empty; CHAMBERS - 1];
    chambers.push(Chamber::Bullet);
    chambers.shuffle(&mut rng());
    chambers
}
