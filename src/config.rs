use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;

use crate::get_config_dir;

/// Application configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Current language
    pub current_lang: String,
    /// Keyboard layout
    pub keyboard: String,
    /// Debug mode
    pub debug_mode: bool,
    /// Log file path
    pub log_file: String,
    /// UI-specific settings
    pub ui: UiConfig,
}

/// UI-specific configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct UiConfig {
    /// Color theme
    pub theme: String,
    /// Terminal dimensions
    pub auto_size: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            current_lang: "de".to_string(),
            keyboard: "de".to_string(),
            debug_mode: true,
            log_file: "/var/log/lunitool.log".to_string(),
            ui: UiConfig {
                theme: "default".to_string(),
                auto_size: true,
            },
        }
    }
}

impl Config {
    /// Load configuration from file
    pub fn load() -> Result<Self> {
        let config_dir = get_config_dir();
        let config_path = config_dir.join("config.yaml");

        // Create config directory if it doesn't exist
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).context("Failed to create config directory")?;
        }

        // If config file doesn't exist, create default
        if !config_path.exists() {
            let default_config = Config::default();
            default_config.save()?;
            return Ok(default_config);
        }

        // Load config from file
        let config_str = fs::read_to_string(&config_path).context("Failed to read config file")?;
        serde_yaml::from_str(&config_str).context("Failed to parse config file")
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let config_dir = get_config_dir();
        let config_path = config_dir.join("config.yaml");

        // Create config directory if it doesn't exist
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).context("Failed to create config directory")?;
        }

        // Serialize and save config
        let config_str = serde_yaml::to_string(self).context("Failed to serialize config")?;
        fs::write(&config_path, config_str).context("Failed to write config file")?;

        Ok(())
    }
}