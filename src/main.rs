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
#[command(version = "0.1.0")]
#[command(about = "Ultra-lightweight system performance monitor", long_about = None)]
struct Args {
    /// Refresh rate in seconds
    #[arg(short, long, default_value_t = 1.0)]
    refresh: f64,

    /// Theme to use (hacker, matrix, minimal, cyberpunk)
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
}

fn main() -> Result<()> {
    let args = Args::parse();

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
    let tick_rate = std::time::Duration::from_secs_f64(app.config.refresh_rate);
    let mut last_tick = std::time::Instant::now();

    loop {
        // Draw UI
        terminal.draw(|frame| ui::render(frame, app))?;

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
                        KeyCode::Char('h') | KeyCode::Char('H') => {
                            app.toggle_help();
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
