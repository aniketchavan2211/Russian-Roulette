# Russian Roulette (TUI)

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
- Android (Termux, aarch64)

---

## Roadmap

- [x] Core gameplay logic
- [x] Terminal UI
- [x] GitHub Actions CI (Linux & Windows)
- [ ] macOS build
- [ ] Android CI build
- [ ] Integrity checks (SHA-256)
- [ ] Static project page (GitHub Pages)

---

## License

MIT License
