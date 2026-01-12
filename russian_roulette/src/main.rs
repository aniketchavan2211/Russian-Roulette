use rand::{rng, seq::SliceRandom};
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

const CHAMBERS: usize = 6;
const MAX_ROUNDS: usize = 6;

#[derive(Clone, Copy, Debug)]
enum Chamber {
    Empty,
    Bullet,
}

fn main() {
    banner();

    let mut rounds_played = 0;
    let mut chambers = spin_cylinder();

    loop {
        if rounds_played >= MAX_ROUNDS {
            victory();
            break;
        }

        print_prompt(rounds_played + 1);

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Input error. Exiting.");
            break;
        }

        match input.trim().to_lowercase().as_str() {
            "s" | "spin" => {
                chambers = spin_cylinder();
                println!("🔄 Cylinder spined.");
            }

            "f" | "fire" => {
                suspense();

                match chambers[0] {
                    Chamber::Empty => {
                        rounds_played += 1;
                        click(rounds_played);
                    }
                    Chamber::Bullet => {
                        boom();
                        game_over(rounds_played);
                        break;
                    }
                }
            }

            "q" | "e" | "quit" | "exit" => {
                println!("Exiting game. Cowardice is a valid survival strategy 😈");
                break;
            }

            _ => {
                println!("Unknown command. Use: [s]pin, [f]ire, [q]uit");
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
    println!("Rules:");
    println!("• 6 chambers, 1 bullet");
    println!("• Max 6 rounds");
    println!("• Spin as much as you want");
    println!("• Fire when ready");
    println!("• Quit anytime with q / e / Ctrl+C");
    println!();
}

fn print_prompt(round: usize) {
    println!("──────────────────────────────────────────");
    println!("Round {round}/{MAX_ROUNDS}");
    print!("Choose action [s]pin | [f]ire | [q]uit → ");
    let _ = io::stdout().flush();
}

fn suspense() {
    print!("Pulling trigger");
    let _ = io::stdout().flush();

    for _ in 0..3 {
        thread::sleep(Duration::from_millis(500));
        print!(".");
        let _ = io::stdout().flush();
    }
    println!();
}

fn click(round: usize) {
    println!("Trigger Pulled !!!");
    println!(" 😅  You survived round {round}.");
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
}

fn victory() {
    println!();
    println!("🎉 CONGRATULATIONS 🎉");
    println!("You survived all {MAX_ROUNDS} rounds.");
    println!("Luck, courage, or both.");
}
