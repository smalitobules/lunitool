use anyhow::{Context, Result};
use std::collections::HashMap;
use std::sync::Mutex;

// HashMap für Übersetzungen
lazy_static::lazy_static! {
    static ref TRANSLATIONS: Mutex<HashMap<String, HashMap<String, String>>> = Mutex::new({
        let mut translations = HashMap::new();
        
        // Deutsche Übersetzungen
        let mut de = HashMap::new();
        de.insert("LANG_TITLE".to_string(), "Linux Universal Tool".to_string());
        de.insert("LANG_SUBTITLE".to_string(), "Zentrale Verwaltungsumgebung".to_string());
        de.insert("LANG_LANGUAGE_SELECT".to_string(), "Sprache / Language".to_string());
        de.insert("LANG_KEYBOARD_SELECT".to_string(), "Tastaturlayout".to_string());
        de.insert("LANG_NAVIGATION".to_string(), "↑/↓: Navigation   Enter: Auswählen   Backspace: Zurück   ESC: Beenden".to_string());
        de.insert("LANG_MAIN_MENU".to_string(), "Hauptmenü".to_string());
        de.insert("LANG_INSTALL".to_string(), "System-Installation".to_string());
        de.insert("LANG_INSTALL_DESC".to_string(), "Neues Linux-System einrichten und konfigurieren. Partitionierung, Bootloader-Installation und Grundeinrichtung.".to_string());
        de.insert("LANG_BACKUP".to_string(), "Sicherung & Wiederherstellung".to_string());
        de.insert("LANG_BACKUP_DESC".to_string(), "Systemsicherungen erstellen und verwalten. Daten wiederherstellen und Systemzustände sichern.".to_string());
        de.insert("LANG_KEYS".to_string(), "Schlüssel-Verwaltung".to_string());
        de.insert("LANG_KEYS_DESC".to_string(), "Kryptografische Schlüssel für Boot-USB und Systemverschlüsselung erstellen und verwalten.".to_string());
        de.insert("LANG_EXIT_CONFIRM".to_string(), "Möchtest du lunitool wirklich beenden?".to_string());
        de.insert("LANG_YES".to_string(), "Ja".to_string());
        de.insert("LANG_NO".to_string(), "Nein".to_string());
        de.insert("LANG_INVALID_SELECTION".to_string(), "Ungültige Auswahl".to_string());
        de.insert("LANG_NOT_IMPLEMENTED".to_string(), "Diese Funktion ist noch nicht implementiert und wird in einer zukünftigen Version verfügbar sein.".to_string());
        
        // Englische Übersetzungen
        let mut en = HashMap::new();
        en.insert("LANG_TITLE".to_string(), "Linux Universal Tool".to_string());
        en.insert("LANG_SUBTITLE".to_string(), "Central Management Environment".to_string());
        en.insert("LANG_LANGUAGE_SELECT".to_string(), "Language / Sprache".to_string());
        en.insert("LANG_KEYBOARD_SELECT".to_string(), "Keyboard Layout".to_string());
        en.insert("LANG_NAVIGATION".to_string(), "↑/↓: Navigation   Enter: Select   Backspace: Back   ESC: Exit".to_string());
        en.insert("LANG_MAIN_MENU".to_string(), "Main Menu".to_string());
        en.insert("LANG_INSTALL".to_string(), "System Installation".to_string());
        en.insert("LANG_INSTALL_DESC".to_string(), "Set up and configure a new Linux system. Partitioning, bootloader installation and basic configuration.".to_string());
        en.insert("LANG_BACKUP".to_string(), "Backup & Restore".to_string());
        en.insert("LANG_BACKUP_DESC".to_string(), "Create and manage system backups. Restore data and preserve system states.".to_string());
        en.insert("LANG_KEYS".to_string(), "Key Management".to_string());
        en.insert("LANG_KEYS_DESC".to_string(), "Create and manage cryptographic keys for boot USB drives and system encryption.".to_string());
        en.insert("LANG_EXIT_CONFIRM".to_string(), "Do you really want to exit lunitool?".to_string());
        en.insert("LANG_YES".to_string(), "Yes".to_string());
        en.insert("LANG_NO".to_string(), "No".to_string());
        en.insert("LANG_INVALID_SELECTION".to_string(), "Invalid selection".to_string());
        en.insert("LANG_NOT_IMPLEMENTED".to_string(), "This feature is not yet implemented and will be available in a future version.".to_string());
        
        translations.insert("de".to_string(), de);
        translations.insert("en".to_string(), en);
        
        translations
    });
    
    // Aktuelle Sprache
    static ref CURRENT_LANGUAGE: Mutex<String> = Mutex::new("de".to_string());
}

/// Set the current language
pub fn set_language(lang: &str, _content: &str) -> Result<()> {
    // Überprüfen, ob die Sprache unterstützt wird
    let available_languages = TRANSLATIONS.lock().unwrap().keys().cloned().collect::<Vec<String>>();
    
    if !available_languages.contains(&lang.to_string()) {
        return Err(anyhow::anyhow!("Unsupported language: {}", lang));
    }
    
    // Aktuelle Sprache setzen
    let mut current_lang = CURRENT_LANGUAGE.lock().unwrap();
    *current_lang = lang.to_string();
    
    log::info!("Language set to: {}", lang);
    Ok(())
}

/// Get a translated string by key
pub fn get_text(key: &str) -> String {
    // Aktuelle Sprache holen
    let current_lang = CURRENT_LANGUAGE.lock().unwrap().clone();
    
    // Übersetzungen holen
    let translations = TRANSLATIONS.lock().unwrap();
    
    // Text in der aktuellen Sprache suchen
    if let Some(lang_map) = translations.get(&current_lang) {
        if let Some(text) = lang_map.get(key) {
            return text.clone();
        }
    }
    
    // Fallback zur englischen Sprache
    if current_lang != "en" {
        if let Some(en_map) = translations.get("en") {
            if let Some(text) = en_map.get(key) {
                return text.clone();
            }
        }
    }
    
    // Wenn nichts gefunden wurde, Schlüssel zurückgeben
    key.to_string()
}

/// Create language resource file for compatibility
pub fn create_language_file(lang: &str) -> Result<()> {
    use std::fs;
    use crate::get_lang_dir;
    
    let lang_dir = get_lang_dir();
    let lang_file = lang_dir.join(format!("{}.ftl", lang));
    
    // Create language directory if it doesn't exist
    if !lang_dir.exists() {
        fs::create_dir_all(&lang_dir).context("Failed to create language directory")?;
    }
    
    // Don't overwrite existing file
    if lang_file.exists() {
        return Ok(());
    }
    
    // Erstelle leere Datei (für Kompatibilität)
    fs::write(&lang_file, "# Language file placeholder").context("Failed to write language file")?;
    log::info!("Created language file placeholder: {}", lang_file.display());
    
    Ok(())
}