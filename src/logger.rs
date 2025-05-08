use anyhow::{Context, Result};
use chrono::Local;
use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;
use std::{fs, io, path::Path};

/// Setup the logger
pub fn setup_logger() -> Result<()> {
    // Configure colors for log levels
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Blue)
        .trace(Color::White);

    // Create log directory if it doesn't exist
    let log_dir = Path::new("/var/log");
    if !log_dir.exists() {
        if let Err(e) = fs::create_dir_all(log_dir) {
            eprintln!("Warning: Could not create log directory: {}", e);
            // Fallback to temporary directory
            let tmp_log = Path::new("/tmp/lunitool.log");
            eprintln!("Using temporary log file: {}", tmp_log.display());
        }
    }

    // Determine log file path
    let log_file = if log_dir.exists() && fs::metadata(log_dir).map(|m| m.permissions().readonly()).unwrap_or(true) == false {
        log_dir.join("lunitool.log")
    } else {
        Path::new("/tmp/lunitool.log").to_path_buf()
    };

    // Setup logger
    fern::Dispatch::new()
        // Format logs
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        // Set log levels
        .level(LevelFilter::Info)
        .level_for("lunitool", LevelFilter::Debug)
        // Output to log file only, NOT to stdout to keep UI clean
        .chain(fern::log_file(log_file).context("Failed to open log file")?)
        // Apply configuration
        .apply()
        .context("Failed to initialize logger")?;

    // Log startup message
    log::info!("=== LUNITOOL Log started ===");
    log::info!("Version: 0.1.0");
    if let Ok(user) = std::env::var("USER") {
        log::info!("User: {}", user);
    }
    if let Ok(hostname) = std::env::var("HOSTNAME") {
        log::info!("Hostname: {}", hostname);
    }
    log::info!("===========================");

    Ok(())
}