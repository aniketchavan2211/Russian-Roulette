
use rand::{rng, seq::SliceRandom};
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

const CHAMBERS: usize = 6;

#[derive(Debug, Clone, Copy)]
enum Chamber {
    Empty,
    Bullet,
}

fn main() {
    banner();

    let mut chambers = load_chambers();
    let mut position = 0;

    loop {
        if position >= CHAMBERS {
            victory();
            break;
        }

        wait_for_trigger(position + 1);

        suspense();

        match chambers[position] {
            Chamber::Empty => {
                click(position + 1);
                position += 1;
            }
            Chamber::Bullet => {
                boom();
                game_over();
                break;
            }
        }
    }
}

/* ───────────────────────── CORE GAME LOGIC ───────────────────────── */

fn load_chambers() -> Vec<Chamber> {
    let mut chambers = vec![Chamber::Empty; CHAMBERS - 1];
    chambers.push(Chamber::Bullet);
    chambers.shuffle(&mut rng());
    chambers
}

/* ───────────────────────── PRESENTATION ───────────────────────── */

fn banner() {
    println!("══════════════════════════════════════════");
    println!("🔫  R U S S I A N   R O U L E T T E  🔫");
    println!("══════════════════════════════════════════");
    println!("• 6 chambers");
    println!("• 1 bullet");
    println!("• Survive them all… or don’t 😈");
    println!();
}

fn wait_for_trigger(round: usize) {
    print!("Round {round} — press ENTER to pull the trigger ");
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut String::new());
}

fn suspense() {
    print!("Spinning cylinder");
    let _ = io::stdout().flush();

    for _ in 0..3 {
        thread::sleep(Duration::from_millis(600));
        print!(".");
        let _ = io::stdout().flush();
    }
    println!();
}

fn click(round: usize) {
    println!("Trigger Pulled! But 😅  Chamber {round} was empty.");
    println!();
}

fn boom() {
    println!("Shot Fired ---> 💥  B A N G  💥");
    thread::sleep(Duration::from_secs(1));
}

fn game_over() {
    println!();
    println!("☠️  GAME OVER");
    println!("The chamber was loaded.");
    fake_consequence();
}

fn victory() {
    println!("🎉  YOU SURVIVED 🎉");
    println!("All chambers were empty.");
    println!("Luck favors the reckless.");
}

/* ───────────────────────── SAFE PRANK CONSEQUENCE ───────────────────────── */

fn fake_consequence() {
    println!();
    println!("System integrity compromised.");
    println!("Initiating emergency protocol…");

    for i in (0..=100).step_by(25) {
        println!("Processing… {i}%");
        thread::sleep(Duration::from_millis(500));
    }

    println!("Rollback successful.");
    println!("No damage done. Relax 😁");
}
