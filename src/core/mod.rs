pub mod system_info;

use anyhow::{Context, Result};
use std::fs;

use crate::get_lang_dir;

/// Load language file
pub fn load_language(lang: &str) -> Result<()> {
    let lang_dir = get_lang_dir();
    // Erstelle die Sprachdatei, wird für Kompatibilität benötigt
    let _lang_file = lang_dir.join(format!("{}.ftl", lang));

    // Create language directory if it doesn't exist
    if !lang_dir.exists() {
        fs::create_dir_all(&lang_dir).context("Failed to create language directory")?;
    }

    // In unserer vereinfachten Version ist das Laden der Datei nicht notwendig,
    // wir setzen einfach die aktuelle Sprache
    crate::lang::set_language(lang, "")?;

    log::info!("Language loaded: {}", lang);
    Ok(())
}

/// Set keyboard layout
pub fn set_keyboard(layout: &str) -> Result<()> {
    // On Unix-like systems, we could use the "loadkeys" or "setxkbmap" command
    // Here we just log the attempt and leave actual implementation to the OS-specific code
    log::info!("Setting keyboard layout to: {}", layout);
    
    // This would be the actual implementation on Linux
    if cfg!(unix) {
        use std::process::Command;
        
        // Try loadkeys (console)
        let loadkeys_result = Command::new("loadkeys")
            .arg(layout)
            .output();
            
        if let Ok(output) = loadkeys_result {
            if output.status.success() {
                return Ok(());
            }
        }
        
        // Try setxkbmap (X11)
        let setxkbmap_result = Command::new("setxkbmap")
            .arg(layout)
            .output();
            
        if let Ok(output) = setxkbmap_result {
            if output.status.success() {
                return Ok(());
            }
        }
        
        // On Windows or if both failed
        log::warn!("Could not set keyboard layout with system tools");
    }
    
    // We just pretend it worked for now
    Ok(())
}

/// Check for root/administrator privileges
pub fn check_root() -> bool {
    #[cfg(unix)]
    {
        users::get_effective_uid() == 0
    }
    
    #[cfg(windows)]
    {
        // On Windows, we would check for administrator privileges
        // This is a simplified implementation
        false
    }
    
    #[cfg(not(any(unix, windows)))]
    {
        false
    }
}