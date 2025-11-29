# ⚡ System Monitor

A blazingly fast, ultra-lightweight system performance monitor for your terminal.

![Version](https://img.shields.io/badge/version-1.1.1-green)
![Rust](https://img.shields.io/badge/rust-1.75%2B-orange)
![License](https://img.shields.io/badge/license-MIT-blue)
![Platform](https://img.shields.io/badge/platform-linux%20%7C%20windows%20%7C%20wsl2-lightgrey)

```
╔══════════════════════════════════════════════════════════════════╗
║  SYSTEM MONITOR                                     v1.1.1       ║
╠══════════════════════════════════════════════════════════════════╣
║  CPU [████████████░░░░░░░░] 58.3%    MEM [██████░░░░░░░░░░] 38.2% ║
║  ▁▂▃▅▆▇█▇▆▅▄▃▂▁▂▃▄▅▆▇█▇▆▅▄▃▂▁     ▁▁▂▂▃▃▄▄▅▅▆▆▇▇████▇▇▆▆▅▅▄▄▃▃▂▂ ║
╠══════════════════════════════════════════════════════════════════╣
║  GPU: NVIDIA GeForce RTX 4070 Ti     51°C    8W/285W    0%       ║
║  NET: ↓ 1.2 MB/s  ↑ 256 KB/s         DISK: R 45 MB/s  W 12 MB/s  ║
╚══════════════════════════════════════════════════════════════════╝
```

## Why?

I wanted a **dead-simple**, **keyboard-driven** system monitor that:

- Uses almost **zero resources** (< 0.5% CPU, < 5MB RAM)
- Looks good in any terminal with **customizable themes**
- Provides **real-time metrics** with historical sparklines
- Works on **Linux, Windows, and WSL2** out of the box
- Is a **single binary** with no dependencies

Inspired by `htop`, `btop`, and `nvtop` — but leaner and Rust-powered.

---

## Features

- **Ultra-lightweight** — 0.0% CPU usage, ~4MB RAM, 1.7MB binary
- **9 Beautiful themes** — From Matrix green to Tokyo Night purple
- **NVIDIA GPU monitoring** — Usage, VRAM, temperature, power draw
- **60-second sparklines** — Visual history for all metrics
- **Persistent config** — Save your preferences with a single keypress
- **Cross-platform** — Native binaries for Linux and Windows

---

## Installation

### Pre-built Binaries (Recommended)

Download the latest release for your platform:

**Linux:**
```bash
curl -LO https://github.com/Kikii95/system-monitor/releases/latest/download/system-monitor-linux-x86_64.tar.gz
tar -xzf system-monitor-linux-x86_64.tar.gz
sudo mv system-monitor /usr/local/bin/
```

**Windows:**
1. Download `system-monitor-windows-x86_64.zip` from [Releases](https://github.com/Kikii95/system-monitor/releases)
2. Extract and run `system-monitor.exe`
3. For best results, use [Windows Terminal](https://aka.ms/terminal)

### Build from Source

Requires Rust 1.75 or later.

```bash
git clone https://github.com/Kikii95/system-monitor.git
cd system-monitor
cargo build --release
./target/release/system-monitor
```

### Cargo Install

```bash
cargo install --git https://github.com/Kikii95/system-monitor.git
```

---

## Usage

```bash
# Launch with defaults (1s refresh, hacker theme)
system-monitor

# Custom refresh rate (in seconds)
system-monitor --refresh 0.5

# Choose a theme
system-monitor --theme dracula

# Disable GPU monitoring
system-monitor --no-gpu

# Create config file with current defaults
system-monitor --init-config

# Check system info without launching UI
system-monitor --check
```

---

## Keybindings

| Key | Action |
|-----|--------|
| `Q` / `Esc` | Quit |
| `T` | Cycle through themes |
| `R` | Force refresh |
| `H` / `F1` / `?` | Toggle help overlay |
| `+` / `=` | Faster refresh (0.25s → 0.5s → 1s → ...) |
| `-` / `_` | Slower refresh |
| `S` | Save current settings to config file |

---

## Themes

Press `T` to cycle through 9 built-in themes:

| Theme | Style |
|-------|-------|
| `hacker` | Classic Matrix green (default) |
| `matrix` | Darker terminal green |
| `minimal` | Clean monochrome |
| `cyberpunk` | Neon red & cyan |
| `dracula` | Purple & pink |
| `nord` | Arctic blue |
| `gruvbox` | Warm retro orange |
| `tokyo` | Tokyo Night blue/purple |
| `ocean` | Deep sea blue |

---

## Configuration

Config is stored at:
- **Linux/WSL:** `~/.config/system-monitor/config.toml`
- **Windows:** `%APPDATA%\system-monitor\config.toml`

Generate a default config:
```bash
system-monitor --init-config
```

Example config:
```toml
refresh_rate = 1.0
theme = "dracula"
gpu_enabled = true
compact_mode = false
show_graphs = true
graph_history = 60
```

> **Tip:** Press `S` while running to save your current settings (theme, refresh rate, etc.)

---

## Tech Stack

| Component | Technology |
|-----------|------------|
| Language | Rust 1.75+ |
| TUI Framework | [ratatui](https://github.com/ratatui-org/ratatui) |
| Terminal Backend | [crossterm](https://github.com/crossterm-rs/crossterm) |
| System Metrics | [sysinfo](https://github.com/GuillaumeGomez/sysinfo) |
| NVIDIA GPU | [nvml-wrapper](https://github.com/Cldfire/nvml-wrapper) |
| CLI Parsing | [clap](https://github.com/clap-rs/clap) |
| Config | [toml](https://github.com/toml-rs/toml) + [serde](https://github.com/serde-rs/serde) |

---

## Performance

Measured on an AMD Ryzen 9 / RTX 4070 Ti system:

| Metric | Value |
|--------|-------|
| CPU Usage | **0.0%** |
| Memory | **~4 MB** |
| Binary Size | **1.7 MB** |
| Startup Time | **< 100ms** |

---

## WSL2 + NVIDIA GPU

If GPU monitoring shows "No NVIDIA GPU detected" on WSL2:

```bash
# Create symlink for NVIDIA library
sudo ln -sf /usr/lib/wsl/lib/libnvidia-ml.so.1 /usr/lib/wsl/lib/libnvidia-ml.so

# Add to library path permanently
echo '/usr/lib/wsl/lib' | sudo tee /etc/ld.so.conf.d/wsl-nvidia.conf
sudo ldconfig
```

---

## Contributing

Contributions are welcome! Feel free to:

1. Fork the repo
2. Create a feature branch (`git checkout -b feature/cool-stuff`)
3. Commit your changes (`git commit -m 'Add cool stuff'`)
4. Push to the branch (`git push origin feature/cool-stuff`)
5. Open a Pull Request

---

## License

MIT License — see [LICENSE](LICENSE) for details.

---

Made with Rust and caffeine.
