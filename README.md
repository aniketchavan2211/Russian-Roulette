# Russian Roulette (TUI)

![Squid Game Revolver at Head Point](https://github.com/aniketchavan2211/Russian-Roulette/blob/98790119582c0bc06a4f94cfeac5a4f1b3efcf59/assets/images/Squid%20Game%20-%20Revolver%20at%20head%20point.jpg)

## About the Game
<p align="center">
    <img src="https://github.com/aniketchavan2211/Russian-Roulette/blob/98790119582c0bc06a4f94cfeac5a4f1b3efcf59/assets/images/Rosyjska%20Ruletka.png" alt="Rússkaya rulétka" width="300" height="200">
</p>

**Russian roulette**(***Rússkaya rulétka***) is a potentially lethal game of chance in
which a player places a ***single round*** in a ***revolver***,
spins the cylinder, places the muzzle against the head or body
(their opponent's or their own),
and pulls the trigger. If the loaded chamber aligns with the barrel after
cocking (with a single-action revolver),
or is in the position which will be rotated to the barrel upon pulling the
trigger (double-action),
the weapon fires.

[READ MORE the Game at Wikipedia](https://en.wikipedia.org/wiki/Russian_roulette#Origin)

A safe, cross-platform **terminal-based Russian Roulette game** written in Rust.
The game uses a text user interface (TUI) with real-time input and suspense-based
gameplay mechanics.

⚠️ This is a **simulation only**.
No system commands, files, or destructive actions are performed.

---

## Features

- Terminal UI (TUI) using `ratatui`
- Real-time key input (no Enter required)
- Six-chamber revolver simulation
- Time-based suspense delay on trigger pull
- Survival victory after six safe rounds
- Safe exit at any time

---

## Controls

| Key | Action |
|----|-------|
| `S` | Spin cylinder |
| `F` | Fire |
| `Q` | Quit |

---

## Supported Platforms

- Linux (x86_64)
- Windows (x86_64)
- MacOS (ARM64)
- Android (Termux, aarch64)

---

## Roadmap

- [x] Core gameplay logic
- [x] Terminal UI
- [x] GitHub Actions CI (Linux & Windows)
- [x] macOS build
- [x] Android CI build
- [X] Integrity checks (SHA-256)
- [x] Static project page (GitHub Pages)

---

## License

MIT License

## Inspiration

[![ I made the DUMBEST game of all time! - you should never play it ]](https://www.youtube.com/shorts/oXSekoOazhA)
