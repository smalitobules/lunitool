use anyhow::{Context, Result};
use lunitool_lib::{
    app::App,
    config::Config,
    core::load_language,
    logger::setup_logger,
    ui::tui::setup_terminal,
};
use std::{io, process};

fn main() -> Result<()> {
    // Setup logger
    setup_logger().context("Failed to setup logger")?;
    log::info!("Starting lunitool v0.1.0");

    // Load configuration
    let config = Config::load().unwrap_or_else(|err| {
        log::error!("Failed to load configuration: {}", err);
        process::exit(1);
    });

    // Load language files
    load_language(&config.current_lang).unwrap_or_else(|err| {
        log::error!("Failed to load language files: {}", err);
        process::exit(1);
    });

    // Setup terminal
    let terminal = setup_terminal().context("Failed to setup terminal")?;

    // Create app
    let mut app = App::new(config, terminal);

    // Run app
    let res = app.run();

    // Restore terminal
    app.restore_terminal()?;

    // Handle result from app
    if let Err(err) = res {
        log::error!("Application error: {}", err);
        return Err(err);
    }

    log::info!("Exiting lunitool cleanly");
    Ok(())
}