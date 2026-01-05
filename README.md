# file-manager-rs

**A cross-platform TUI file manager written in Rust**, built with a clean core/UI separation, an event–effect architecture, and worker-based filesystem operations.

This project focuses on **software engineering quality** rather than just functionality, aiming to showcase how to design a maintainable, testable, and scalable terminal application in Rust.

---

## ✨ Features

- 📁 Directory navigation (enter / go up / selection)
- 🖥️ Modern terminal UI built with **ratatui**
- ⌨️ Keyboard-driven workflow (vim-like bindings)
- 🔄 On-demand directory refresh
- 👻 Toggle hidden files
- 📊 Sorting by name, modification date, or size
- 🗑️ Safe delete with confirmation
- 🚀 Open files and directories using the OS default application
- 🧩 Cross-platform support (Linux, macOS, Windows)

---

## 🏗 Architecture

The project is structured as a **Rust workspace** with clear separation of responsibilities:

```
crates/
  fm_core       # Domain logic, state, actions, reducers
  fm_tui        # Terminal UI rendering and input handling
  fm_platform   # OS-specific integrations (open file, etc.)
  fm_cli        # Application entry point and event loop
```

### Core Design Principles

- **Core/UI decoupling**  
  The core logic is completely independent from the TUI layer, enabling easier testing and future UI swaps.

- **Event–Effect Architecture**  
  User input produces **Actions**, which are reduced into **Effects**.  
  Effects are executed by worker threads and emit **Events** that update the application state.

- **Non-blocking UI**  
  Filesystem operations run in worker threads, keeping the TUI responsive at all times.

---

## ⌨️ Key Bindings

| Key | Action |
|---|---|
| `j` / `↓` | Move selection down |
| `k` / `↑` | Move selection up |
| `Enter` | Enter directory |
| `h` / `Backspace` | Go to parent directory |
| `.` | Toggle hidden files |
| `1` | Sort by name |
| `2` | Sort by modification date |
| `3` | Sort by size |
| `d` | Delete selected entry |
| `y` / `n` | Confirm / cancel delete |
| `o` | Open file or directory |
| `r` | Refresh directory |
| `?` | Toggle help |
| `q` | Quit |

---

## 🚀 Getting Started

### Requirements
- Rust (stable)
- A terminal that supports ANSI escape codes

### Run

```bash
cargo run -p fm_cli
```

---

## 🎯 Project Goals

This project is intentionally designed to highlight:

- Clean architecture in Rust
- Event-driven state management
- Safe and explicit filesystem operations
- Cross-platform terminal application design
- Senior-level code organization suitable for real-world tools

---

## 🛣 Roadmap

- [ ] File operations with progress bars (copy / move)
- [ ] Background filesystem watcher
- [ ] Undo / redo for destructive operations
- [ ] Search and filtering
- [ ] Configuration file (themes, keybindings)
- [ ] Benchmarks and stress tests

---

## 📜 License

MIT License
