mod app;
mod config;
mod error;
mod collectors;
mod ui;
mod themes;

use std::io;
use anyhow::Result;
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

use crate::app::App;
use crate::config::Config;

#[derive(Parser, Debug)]
#[command(name = "system-monitor")]
#[command(author = "kiki")]
#[command(version = "1.0.0")]
#[command(about = "Ultra-lightweight system performance monitor", long_about = None)]
struct Args {
    /// Refresh rate in seconds
    #[arg(short, long, default_value_t = 1.0)]
    refresh: f64,

    /// Theme: hacker, matrix, minimal, cyberpunk, dracula, nord, gruvbox, tokyo, ocean
    #[arg(short, long, default_value = "hacker")]
    theme: String,

    /// Disable GPU monitoring
    #[arg(long)]
    no_gpu: bool,

    /// Compact mode (minimal display)
    #[arg(short, long)]
    compact: bool,

    /// Path to config file
    #[arg(short = 'C', long)]
    config: Option<String>,

    /// Create default config file and exit
    #[arg(long)]
    init_config: bool,

    /// Check system info and exit (no UI)
    #[arg(long)]
    check: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Handle --init-config
    if args.init_config {
        match Config::create_default_if_missing() {
            Ok(true) => {
                if let Some(path) = Config::default_path() {
                    println!("Created config file: {}", path.display());
                }
            }
            Ok(false) => {
                if let Some(path) = Config::default_path() {
                    println!("Config file already exists: {}", path.display());
                }
            }
            Err(e) => {
                eprintln!("Failed to create config: {}", e);
                std::process::exit(1);
            }
        }
        return Ok(());
    }

    // Handle --check
    if args.check {
        use crate::collectors::Collectors;
        let config = Config::load(args.config.as_deref())?;
        let mut collectors = Collectors::new(&config)?;
        collectors.update()?;

        println!("╔══════════════════════════════════════════╗");
        println!("║       SYSTEM MONITOR - DIAGNOSTICS       ║");
        println!("╠══════════════════════════════════════════╣");
        println!("║ CPU                                      ║");
        println!("║   Cores: {:>3} physical, {:>3} logical      ║",
            collectors.cpu.data.physical_cores,
            collectors.cpu.data.logical_cores);
        println!("║   Usage: {:>5.1}%                          ║", collectors.cpu.data.global_usage);
        println!("╠══════════════════════════════════════════╣");
        println!("║ MEMORY                                   ║");
        println!("║   Total: {:>5.1} GB                        ║", collectors.memory.data.total as f64 / 1024.0 / 1024.0 / 1024.0);
        println!("║   Used:  {:>5.1}%                          ║", collectors.memory.data.usage_percent);
        println!("╠══════════════════════════════════════════╣");
        println!("║ GPU                                      ║");
        if collectors.gpu.data.available {
            println!("║   ✅ {}  ║", format!("{:<30}", collectors.gpu.data.name));
            println!("║   Driver: {:>30} ║", collectors.gpu.data.driver_version);
        } else {
            println!("║   ❌ No NVIDIA GPU detected              ║");
        }
        println!("╠══════════════════════════════════════════╣");
        println!("║ NETWORK                                  ║");
        println!("║   Interface: {:>27} ║", collectors.network.data.interface);
        println!("╠══════════════════════════════════════════╣");
        println!("║ SYSTEM                                   ║");
        println!("║   OS: {:>34} ║", collectors.system.data.os_name);
        println!("║   Kernel: {:>30} ║", collectors.system.data.kernel_version);
        println!("╚══════════════════════════════════════════╝");
        return Ok(());
    }

    // Load config (CLI args override config file)
    let config = Config::load(args.config.as_deref())?
        .with_refresh_rate(args.refresh)
        .with_theme(&args.theme)
        .with_gpu(!args.no_gpu)
        .with_compact(args.compact);

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let mut app = App::new(config)?;
    let result = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Handle any errors from the app
    if let Err(err) = result {
        eprintln!("Error: {err:?}");
        std::process::exit(1);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    let mut last_tick = std::time::Instant::now();

    loop {
        // Draw UI
        terminal.draw(|frame| ui::render(frame, app))?;

        // Dynamic tick rate (can be changed at runtime)
        let tick_rate = std::time::Duration::from_secs_f64(app.config.refresh_rate);

        // Poll for events with timeout
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => {
                            app.should_quit = true;
                        }
                        KeyCode::Char('t') | KeyCode::Char('T') => {
                            app.cycle_theme();
                        }
                        KeyCode::Char('r') | KeyCode::Char('R') => {
                            app.force_refresh();
                        }
                        KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::Char('?') | KeyCode::F(1) => {
                            app.toggle_help();
                        }
                        KeyCode::Char('+') | KeyCode::Char('=') => {
                            app.decrease_refresh_rate();
                        }
                        KeyCode::Char('-') | KeyCode::Char('_') => {
                            app.increase_refresh_rate();
                        }
                        KeyCode::Char('s') | KeyCode::Char('S') => {
                            app.save_config();
                        }
                        KeyCode::Esc => {
                            if app.show_help {
                                app.show_help = false;
                            } else {
                                app.should_quit = true;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        // Check if we should quit
        if app.should_quit {
            return Ok(());
        }

        // Update metrics on tick
        if last_tick.elapsed() >= tick_rate {
            app.update()?;
            last_tick = std::time::Instant::now();
        }
    }
}
