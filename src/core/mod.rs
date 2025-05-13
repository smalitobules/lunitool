pub mod system_info;
pub mod disk_info;

pub use system_info::collect_system_info;

use anyhow::Result;

use crate::config::Config;

/// Load language file (now primarily sets the global locale via lang module)
pub fn load_language(lang: &str) -> Result<()> {
    // The logic for lang_dir, creating it, or creating a specific lang_file path
    // is no longer needed here as languages are embedded and managed by the lang module.
    
    // The primary responsibility is to tell the lang module to switch the current language.
    crate::lang::set_language(lang)?;

    log::info!("Language set via core::load_language: {}", lang);
    Ok(())
}

/// Set keyboard layout (Unix-specific)
pub fn set_keyboard(layout: &str) -> Result<()> {
    log::info!("Attempting to set keyboard layout to: {} (Unix-specific)", layout);

    #[cfg(unix)]
    {
        use std::process::Command;
        
        // Try loadkeys (console)
        let loadkeys_result = Command::new("loadkeys").arg(layout).output();
        if let Ok(output) = loadkeys_result {
            if output.status.success() {
                log::info!("Keyboard layout set to '{}' using loadkeys.", layout);
                return Ok(());
            }
        }
        
        // Try setxkbmap (X11)
        let setxkbmap_result = Command::new("setxkbmap").arg(layout).output();
        if let Ok(output) = setxkbmap_result {
            if output.status.success() {
                log::info!("Keyboard layout set to '{}' using setxkbmap.", layout);
                return Ok(());
            }
        }
        
        // If both loadkeys and setxkbmap failed on Unix
        log::warn!("Failed to set keyboard layout '{}' on Unix using system tools (loadkeys, setxkbmap).", layout);
        return Err(anyhow::anyhow!("Failed to set keyboard layout '{}' on Unix using available system tools.", layout));
    }

    #[cfg(not(unix))]
    {
        // This tool is intended for Unix-like systems only.
        // Setting keyboard layout is not supported on this platform.
        log::error!("set_keyboard called on a non-Unix system. This operation is not supported by lunitool.");
        Err(anyhow::anyhow!("set_keyboard is only supported on Unix-like systems for lunitool."))
    }
}

/// Check for root/administrator privileges
pub fn check_root() -> bool {
    #[cfg(unix)]
    {
        users::get_effective_uid() == 0
    }
    
    #[cfg(not(unix))]
    {
        false
    }
}

pub fn initialize_language(config: &Config) -> Result<()> {
    let lang = config.current_lang.as_str();
    crate::lang::set_language(lang)?;
    Ok(())
}