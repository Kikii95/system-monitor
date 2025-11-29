use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_refresh_rate")]
    pub refresh_rate: f64,

    #[serde(default = "default_theme")]
    pub theme: String,

    #[serde(default = "default_true")]
    pub gpu_enabled: bool,

    #[serde(default)]
    pub compact_mode: bool,

    #[serde(default = "default_true")]
    pub show_graphs: bool,

    #[serde(default = "default_graph_history")]
    pub graph_history: usize,
}

fn default_refresh_rate() -> f64 {
    1.0
}

fn default_theme() -> String {
    "hacker".to_string()
}

fn default_true() -> bool {
    true
}

fn default_graph_history() -> usize {
    60
}

impl Default for Config {
    fn default() -> Self {
        Self {
            refresh_rate: default_refresh_rate(),
            theme: default_theme(),
            gpu_enabled: true,
            compact_mode: false,
            show_graphs: true,
            graph_history: default_graph_history(),
        }
    }
}

impl Config {
    /// Load config from file or use defaults
    pub fn load(path: Option<&str>) -> Result<Self> {
        // Try explicit path first
        if let Some(p) = path {
            if Path::new(p).exists() {
                let content = std::fs::read_to_string(p)?;
                return Ok(toml::from_str(&content)?);
            }
        }

        // Try default config locations
        if let Some(config_dir) = dirs::config_dir() {
            let default_path = config_dir.join("system-monitor").join("config.toml");
            if default_path.exists() {
                let content = std::fs::read_to_string(&default_path)?;
                return Ok(toml::from_str(&content)?);
            }
        }

        // Use defaults
        Ok(Self::default())
    }

    /// Builder pattern methods for CLI overrides
    pub fn with_refresh_rate(mut self, rate: f64) -> Self {
        self.refresh_rate = rate.clamp(0.25, 5.0);
        self
    }

    pub fn with_theme(mut self, theme: &str) -> Self {
        self.theme = theme.to_string();
        self
    }

    pub fn with_gpu(mut self, enabled: bool) -> Self {
        self.gpu_enabled = enabled;
        self
    }

    pub fn with_compact(mut self, compact: bool) -> Self {
        self.compact_mode = compact;
        self
    }

    /// Save config to default location
    pub fn save(&self) -> Result<()> {
        if let Some(config_dir) = dirs::config_dir() {
            let app_dir = config_dir.join("system-monitor");
            std::fs::create_dir_all(&app_dir)?;
            let config_path = app_dir.join("config.toml");
            let content = toml::to_string_pretty(self)?;
            std::fs::write(&config_path, content)?;
        }
        Ok(())
    }

    /// Get default config path
    pub fn default_path() -> Option<std::path::PathBuf> {
        dirs::config_dir().map(|d| d.join("system-monitor").join("config.toml"))
    }

    /// Create default config file if it doesn't exist
    pub fn create_default_if_missing() -> Result<bool> {
        if let Some(config_dir) = dirs::config_dir() {
            let app_dir = config_dir.join("system-monitor");
            let config_path = app_dir.join("config.toml");

            if !config_path.exists() {
                std::fs::create_dir_all(&app_dir)?;
                let default_config = Self::default();
                let content = format!(
                    "# System Monitor Configuration\n\
                    # Location: ~/.config/system-monitor/config.toml\n\n\
                    {}\n",
                    toml::to_string_pretty(&default_config)?
                );
                std::fs::write(&config_path, content)?;
                return Ok(true);
            }
        }
        Ok(false)
    }
}
