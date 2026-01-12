use rand::{rng, seq::SliceRandom};
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

const CHAMBERS: usize = 6;

#[derive(Clone, Copy, Debug)]
enum Chamber {
    Empty,
    Bullet,
}

fn main() {
    banner();

    let mut rounds_survived = 0;

    loop {
        wait_for_trigger(rounds_survived + 1);

        let mut chambers = spin_cylinder();
        suspense();

        match chambers[0] {
            Chamber::Empty => {
                rounds_survived += 1;
                click(rounds_survived);
            }
            Chamber::Bullet => {
                boom();
                game_over(rounds_survived);
                break;
            }
        }
    }
}

/* ───────────────────────── GAME MECHANICS ───────────────────────── */

fn spin_cylinder() -> Vec<Chamber> {
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
    println!();
}

fn wait_for_trigger(round: usize) {
    print!("Round {round} — press ENTER to spin & pull trigger ");
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut String::new());
}

fn suspense() {
    print!("Spinning cylinder");
    let _ = io::stdout().flush();

    for _ in 0..3 {
        thread::sleep(Duration::from_millis(500));
        print!(".");
        let _ = io::stdout().flush();
    }
    println!();
}

fn click(round: usize) {
    println!("Pulling Trigger!!!");
    println!("😅  You survived round {round}.");
    println!();
}

fn boom() {
    println!("⁍ SHOT FIRED ⁍");
    println!("💥  B A N G  💥");
    thread::sleep(Duration::from_secs(1));
}

fn game_over(rounds: usize) {
    println!();
    println!("☠️  GAME OVER");
    println!("Rounds survived: {rounds}");
    // fake_consequence();
}

/* ───────────────────────── SAFE PRANK ───────────────────────── */

// Function is inactice
fn fake_consequence() {
    println!();
    println!("System anomaly detected.");
    println!("Initiating containment protocol…");

    for i in (0..=100).step_by(20) {
        println!("Stabilizing… {i}%");
        thread::sleep(Duration::from_millis(400));
    }

    println!("System stable.");
    println!("No damage done. Breathe 😁");
}
