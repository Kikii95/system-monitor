# ⚡ System Monitor

Ultra-lightweight system performance monitor TUI written in Rust.

![Version](https://img.shields.io/badge/version-1.1.1-green)
![Rust](https://img.shields.io/badge/rust-1.75%2B-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

## Features

- 🚀 **Ultra-lightweight**: < 0.5% CPU, < 10MB RAM
- 🎨 **4 Themes**: Hacker (green), Matrix, Minimal, Cyberpunk
- 📊 **Real-time metrics**: CPU, Memory, GPU (NVIDIA), Network, Disk, Processes
- 📈 **60-second history**: Sparkline graphs for all metrics
- ⚙️ **Persistent config**: TOML config file (~/.config/system-monitor/)
- ⌨️ **Full keybindings**: Theme switching, refresh control, help overlay
- 🖥️ **Cross-platform**: Linux, WSL2, Windows

## Installation

```bash
# From source
cargo install --path .

# Or build manually
cargo build --release
./target/release/system-monitor
```

## Usage

```bash
# Default (1s refresh, hacker theme)
system-monitor

# Custom refresh rate
system-monitor --refresh 0.5

# Different theme
system-monitor --theme matrix

# Disable GPU monitoring
system-monitor --no-gpu

# Compact mode
system-monitor --compact

# Custom config file
system-monitor --config /path/to/config.toml

# Create default config file
system-monitor --init-config
```

## Keybindings

| Key | Action |
|-----|--------|
| `Q` / `Esc` | Quit |
| `T` | Cycle theme |
| `R` | Force refresh |
| `H` / `F1` / `?` | Toggle help overlay |
| `+` / `=` | Decrease refresh rate (faster) |
| `-` / `_` | Increase refresh rate (slower) |
| `S` | Save current config to file |

## Configuration

Config file location: `~/.config/system-monitor/config.toml`

```bash
# Create default config file
system-monitor --init-config
```

```toml
refresh_rate = 1.0
theme = "hacker"
gpu_enabled = true
compact_mode = false
show_graphs = true
graph_history = 60
```

> **Note**: CLI arguments override config file values. Press `S` at runtime to save current settings.

## Themes

| Theme | Colors | Vibe |
|-------|--------|------|
| **hacker** | Green | Matrix-style (default) |
| **matrix** | Dark green | Classic terminal |
| **minimal** | Monochrome | Clean & simple |
| **cyberpunk** | Red/Cyan | Neon nights |
| **dracula** | Purple/Pink | Vampire approved |
| **nord** | Arctic blue | Scandinavian cool |
| **gruvbox** | Warm orange | Retro vibes |
| **tokyo** | Blue/Purple | Tokyo Night |
| **ocean** | Deep blue | Under the sea |

## Performance Targets

| Metric | Target |
|--------|--------|
| CPU Usage | < 0.5% |
| RAM Usage | < 10 MB |
| Binary Size | < 5 MB |
| Startup | < 200ms |

## License

MIT
