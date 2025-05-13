use anyhow::{Context, Result};
use lunitool_lib::{
    app::App,
    config::Config,
    core::load_language,
    logger::setup_logger,
    ui::tui::setup_terminal,
};
use std::{process};

fn main() -> Result<()> {
    // Load configuration first to get log path and debug_mode
    let config = Config::load().unwrap_or_else(|err| {
        // Cannot use logger here as it's not set up yet.
        // eprintln is a reasonable fallback for critical config load failure.
        eprintln!("Error: Failed to load configuration: {}. Exiting.", err);
        process::exit(1);
    });

    // Setup logger using details from config
    setup_logger(&config.log_file, config.debug_mode).context("Failed to setup logger")?;
    
    log::info!("Starting lunitool v{}", env!("CARGO_PKG_VERSION"));
    // Config is already loaded, so no need to log its loading again here, 
    // but we can log that we are proceeding with the loaded config.
    log::debug!("Configuration loaded: {:?}", config);

    load_language(&config.current_lang).unwrap_or_else(|err| {
        log::error!("Failed to load language files: {}", err);
        process::exit(1);
    });

    let terminal = setup_terminal().context("Failed to setup terminal")?;

    let mut app = App::new(config, terminal);

    let res = app.run();

    app.restore_terminal()?;

    if let Err(err) = res {
        log::error!("Application error: {}", err);
        return Err(err);
    }

    log::info!("Exiting lunitool cleanly");
    Ok(())
}