use std::fs;
use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    // Create language directory
    let out_dir = env::var("OUT_DIR").unwrap();
    let lang_dir = Path::new(&out_dir).join("languages");
    
    fs::create_dir_all(&lang_dir).unwrap();
    
    // Create default language files
    create_language_file(&lang_dir, "de");
    create_language_file(&lang_dir, "en");
}

fn create_language_file(lang_dir: &Path, lang: &str) {
    let lang_file = lang_dir.join(format!("{}.ftl", lang));
    
    // Don't overwrite existing file
    if lang_file.exists() {
        return;
    }
    
    // Create language file with default content
    let content = match lang {
        "de" => r#"
# German language file for lunitool
LANG_TITLE = Linux Universal Tool
LANG_SUBTITLE = Zentrale Verwaltungsumgebung

LANG_LANGUAGE_SELECT = Sprache / Language
LANG_KEYBOARD_SELECT = Tastaturlayout

LANG_NAVIGATION = ↑/↓: Navigation   Enter: Auswählen   Backspace: Zurück   ESC: Beenden

LANG_MAIN_MENU = Hauptmenü
LANG_INSTALL = System-Installation
LANG_INSTALL_DESC = Neues Linux-System einrichten
LANG_BACKUP = Sicherung & Wiederherstellung
LANG_BACKUP_DESC = Sichern und Wiederherstellen
LANG_KEYS = Schlüssel-Verwaltung
LANG_KEYS_DESC = Boot-USB und Authentifizierung

LANG_EXIT_CONFIRM = Möchtest du lunitool wirklich beenden?
LANG_YES = Ja
LANG_NO = Nein
LANG_INVALID_SELECTION = Ungültige Auswahl
"#,
        "en" => r#"
# English language file for lunitool
LANG_TITLE = Linux Universal Tool
LANG_SUBTITLE = Central Management Environment

LANG_LANGUAGE_SELECT = Language / Sprache
LANG_KEYBOARD_SELECT = Keyboard Layout

LANG_NAVIGATION = ↑/↓: Navigation   Enter: Select   Backspace: Back   ESC: Exit

LANG_MAIN_MENU = Main Menu
LANG_INSTALL = System Installation
LANG_INSTALL_DESC = Set up a new Linux system
LANG_BACKUP = Backup & Restore
LANG_BACKUP_DESC = Backup and recovery tools
LANG_KEYS = Key Management
LANG_KEYS_DESC = Boot-USB and authentication

LANG_EXIT_CONFIRM = Do you really want to exit lunitool?
LANG_YES = Yes
LANG_NO = No
LANG_INVALID_SELECTION = Invalid selection
"#,
        _ => "",
    };
    
    fs::write(&lang_file, content).unwrap();
    println!("cargo:warning=Created language file: {}", lang_file.display());
}