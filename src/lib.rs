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
    if let Ok(mut exe_path) = std::env::current_exe() {
        exe_path.pop(); // Removes the executable's filename, leaving the directory
        exe_path
    } else {
        // Fallback if the executable's path cannot be determined
        log::warn!("Could not determine the executable's path. Using the current working directory for lunitool_dir.");
        std::env::current_dir().unwrap_or_else(|_| {
            log::error!("Could not determine either the executable's path or the current working directory. Using '.' as a fallback for lunitool_dir.");
            PathBuf::from(".")
        })
    }
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