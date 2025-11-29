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
        self.config.refresh_rate = (self.config.refresh_rate + 0.5).min(10.0);
    }

    /// Decrease refresh rate (faster updates)
    pub fn decrease_refresh_rate(&mut self) {
        self.config.refresh_rate = (self.config.refresh_rate - 0.5).max(0.1);
    }

    /// Save current config to file
    pub fn save_config(&mut self) {
        match self.config.save() {
            Ok(_) => {
                self.status_message = Some("Config saved!".to_string());
            }
            Err(e) => {
                self.status_message = Some(format!("Save failed: {}", e));
            }
        }
    }

    /// Clear status message
    #[allow(dead_code)]
    pub fn clear_status(&mut self) {
        self.status_message = None;
    }
}
