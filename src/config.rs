use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

use crate::get_config_dir;

/// Application configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub current_lang: String,
    pub keyboard: String,
    pub debug_mode: bool,
    pub log_file: String,
    pub ui: UiConfig,
}

/// UI-specific configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct UiConfig {
    pub theme: String,
    pub auto_size: bool,
}

impl Default for Config {
    fn default() -> Self {
        let current_working_dir = std::env::current_dir()
            .unwrap_or_else(|_| {
                // Simple fallback if CWD cannot be determined.
                PathBuf::from(".")
            });

        Self {
            current_lang: "de".to_string(),
            keyboard: "de".to_string(),
            debug_mode: true,
            log_file: current_working_dir.join("lunitool.log").to_string_lossy().into_owned(),
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

        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).context("Failed to create config directory")?;
        }

        if !config_path.exists() {
            let default_config = Config::default();
            default_config.save()?;
            return Ok(default_config);
        }

        let config_str = fs::read_to_string(&config_path).context("Failed to read config file")?;
        serde_yaml::from_str(&config_str).context("Failed to parse config file")
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let config_dir = get_config_dir();
        let config_path = config_dir.join("config.yaml");

        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).context("Failed to create config directory")?;
        }

        let config_str = serde_yaml::to_string(self).context("Failed to serialize config")?;
        fs::write(&config_path, config_str).context("Failed to write config file")?;

        Ok(())
    }
}