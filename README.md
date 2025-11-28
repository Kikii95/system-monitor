# ⚡ System Monitor

Ultra-lightweight system performance monitor TUI written in Rust.

![Version](https://img.shields.io/badge/version-0.1.0-green)
![Rust](https://img.shields.io/badge/rust-1.75%2B-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

## Features

- 🚀 **Ultra-lightweight**: < 0.5% CPU, < 10MB RAM
- 🎨 **Multiple themes**: Hacker (green), Matrix, Minimal, Cyberpunk
- 📊 **Real-time metrics**: CPU, Memory, GPU, Network, Disk, Processes
- 📈 **60-second history**: Sparkline graphs for all metrics
- ⚙️ **Configurable**: TOML config file + CLI arguments
- 🖥️ **Cross-platform**: Linux, Windows, macOS

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
```

## Keybindings

| Key | Action |
|-----|--------|
| `Q` | Quit |
| `T` | Cycle theme |
| `R` | Force refresh |
| `H` | Toggle help |

## Configuration

Create `~/.config/system-monitor/config.toml`:

```toml
refresh_rate = 1.0
theme = "hacker"
gpu_enabled = true
compact_mode = false
show_graphs = true
graph_history = 60
```

## Themes

- **hacker**: Green Matrix-style (default)
- **matrix**: Darker green
- **minimal**: Monochrome
- **cyberpunk**: Red/Cyan

## Performance Targets

| Metric | Target |
|--------|--------|
| CPU Usage | < 0.5% |
| RAM Usage | < 10 MB |
| Binary Size | < 5 MB |
| Startup | < 200ms |

## License

MIT
