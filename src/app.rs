use std::time::Instant;
use anyhow::Result;

use crate::collectors::Collectors;
use crate::config::Config;
use crate::themes::{Theme, THEMES};

/// Application state
pub struct App {
    pub config: Config,
    pub collectors: Collectors,
    pub theme: Theme,
    pub theme_index: usize,
    pub should_quit: bool,
    pub show_help: bool,
    pub status_message: Option<String>,
    status_message_time: Option<Instant>,
}

impl App {
    pub fn new(config: Config) -> Result<Self> {
        let theme_index = THEMES
            .iter()
            .position(|t| t.name == config.theme)
            .unwrap_or(0);

        let collectors = Collectors::new(&config)?;

        Ok(Self {
            config,
            collectors,
            theme: THEMES[theme_index].clone(),
            theme_index,
            should_quit: false,
            show_help: false,
            status_message: None,
            status_message_time: None,
        })
    }

    /// Update all collectors
    pub fn update(&mut self) -> Result<()> {
        self.collectors.update()
    }

    /// Force immediate refresh
    pub fn force_refresh(&mut self) {
        let _ = self.update();
    }

    /// Cycle through available themes
    pub fn cycle_theme(&mut self) {
        self.theme_index = (self.theme_index + 1) % THEMES.len();
        self.theme = THEMES[self.theme_index].clone();
        self.config.theme = self.theme.name.to_string();
    }

    /// Toggle help overlay
    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    /// Increase refresh rate (slower updates)
    pub fn increase_refresh_rate(&mut self) {
        // Use predefined steps for clean values
        const STEPS: &[f64] = &[0.25, 0.5, 1.0, 1.5, 2.0, 3.0, 5.0, 10.0];
        let current = self.config.refresh_rate;
        for &step in STEPS {
            if step > current + 0.01 {
                self.config.refresh_rate = step;
                return;
            }
        }
        self.config.refresh_rate = 10.0;
    }

    /// Decrease refresh rate (faster updates)
    pub fn decrease_refresh_rate(&mut self) {
        // Use predefined steps for clean values
        const STEPS: &[f64] = &[0.25, 0.5, 1.0, 1.5, 2.0, 3.0, 5.0, 10.0];
        let current = self.config.refresh_rate;
        for &step in STEPS.iter().rev() {
            if step < current - 0.01 {
                self.config.refresh_rate = step;
                return;
            }
        }
        self.config.refresh_rate = 0.25;
    }

    /// Save current config to file
    pub fn save_config(&mut self) {
        match self.config.save() {
            Ok(_) => {
                if let Some(path) = Config::default_path() {
                    self.status_message = Some(format!("✓ Saved to {}", path.display()));
                } else {
                    self.status_message = Some("✓ Config saved!".to_string());
                }
            }
            Err(e) => {
                self.status_message = Some(format!("✗ Save failed: {}", e));
            }
        }
        self.status_message_time = Some(Instant::now());
    }

    /// Clear status message if it's been shown long enough (3 seconds)
    pub fn clear_expired_status(&mut self) {
        if let Some(time) = self.status_message_time {
            if time.elapsed().as_secs() >= 3 {
                self.status_message = None;
                self.status_message_time = None;
            }
        }
    }
}
