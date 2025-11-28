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
    }

    /// Toggle help overlay
    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }
}
