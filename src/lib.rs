pub mod app;
pub mod config;
pub mod core;
pub mod ui;
pub mod tools;
pub mod logger;
pub mod lang;
pub mod error;

use std::path::PathBuf;

// Global paths
pub fn get_lunitool_dir() -> PathBuf {
    // This should be updated to use a suitable path for the application
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

pub fn get_core_dir() -> PathBuf {
    get_lunitool_dir().join("core")
}

pub fn get_tools_dir() -> PathBuf {
    get_lunitool_dir().join("tools")
}

pub fn get_scripts_dir() -> PathBuf {
    get_lunitool_dir().join("scripts")
}

pub fn get_config_dir() -> PathBuf {
    get_lunitool_dir().join("configs")
}

pub fn get_resources_dir() -> PathBuf {
    get_lunitool_dir().join("resources")
}

pub fn get_lang_dir() -> PathBuf {
    get_lunitool_dir().join("core").join("languages")
}