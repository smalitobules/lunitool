use anyhow::{Context, Result};
use chrono::Local;
use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;
use std::{fs, path::Path};

/// Setup the logger
pub fn setup_logger(log_file_path: &str, debug_mode: bool) -> Result<()> {
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Blue)
        .trace(Color::White);

    let log_path = Path::new(log_file_path);

    // Attempt to create the parent directory for the log file if it doesn't exist
    if let Some(parent_dir) = log_path.parent() {
        if !parent_dir.exists() {
            if let Err(e) = fs::create_dir_all(parent_dir) {
                // Output to stderr as the logger is not yet initialized
                eprintln!("Warning: Could not create log directory '{}': {}. Logging to this file might fail.", parent_dir.display(), e);
            }
        }
    }

    let base_level = if debug_mode {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    let lunitool_level = if debug_mode {
        LevelFilter::Trace // Or keep it Debug if Trace is too verbose for lunitool's own logs even in debug_mode
    } else {
        LevelFilter::Debug // In non-debug mode, lunitool's specific logs might still be Debug
    };


    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        .level(base_level) // General log level
        .level_for("lunitool", lunitool_level) // Specific level for "lunitool" crate/target
        .chain(fern::log_file(log_path).context(format!("Failed to open log file at: {}", log_file_path))?)
        .apply()
        .context("Failed to initialize logger")?;

    log::info!("=== LUNITOOL Log started ===");
    log::info!("Version: {}", env!("CARGO_PKG_VERSION"));
    if let Ok(user) = std::env::var("USER") {
        log::info!("User: {}", user);
    }
    if let Ok(hostname) = std::env::var("HOSTNAME") {
        log::info!("Hostname: {}", hostname);
    }
    log::info!("===========================");

    Ok(())
}