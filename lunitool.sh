#!/bin/bash
# lunitool - Linux Universal Tool
# Hauptskript zur Steuerung der Umgebung mit dialog

# Verzeichnisstruktur
LUNITOOL_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CORE_DIR="${LUNITOOL_DIR}/core"
TOOLS_DIR="${LUNITOOL_DIR}/tools"
SCRIPTS_DIR="${LUNITOOL_DIR}/scripts"
CONFIG_DIR="${LUNITOOL_DIR}/configs"
RESOURCES_DIR="${LUNITOOL_DIR}/resources"
LANG_DIR="${LUNITOOL_DIR}/core/languages"

# Lade gemeinsame Funktionen
source "${CORE_DIR}/common.sh"

# Standardsprache
CURRENT_LANG="de"
KEYBOARD="de"

# Prüfe, ob notwendige Verzeichnisse existieren
if [ ! -d "$LANG_DIR" ]; then
    mkdir -p "$LANG_DIR"
fi

# Lade Sprachdatei
load_language() {
    local lang_file="${LANG_DIR}/${CURRENT_LANG}.sh"
    
    if [ -f "$lang_file" ]; then
        source "$lang_file"
    else
        log_error "Sprachdatei $lang_file nicht gefunden!"
        exit 1
    fi
}

# Sprachauswahl-Dialog
select_language() {
    local options=("de" "Deutsch (de_DE)" "en" "English (en_US)")
    
    # Menü über UI-Bibliothek anzeigen
    local selection=$(ui_show_menu "$LANG_LANGUAGE_SELECT" "${options[@]}")
    
    # Prüfen, ob Benutzer abgebrochen hat
    if [ $? -eq 0 ]; then
        CURRENT_LANG="$selection"
        
        # Sprachdatei laden
        load_language
    fi
}

# Tastaturlayout-Dialog
select_keyboard() {
    local options=("de" "Deutsch (de)" "us" "US-English (us)")
    
    # Menü über UI-Bibliothek anzeigen
    local selection=$(ui_show_menu "$LANG_KEYBOARD_SELECT" "${options[@]}")
    
    # Prüfen, ob Benutzer abgebrochen hat
    if [ $? -eq 0 ]; then
        KEYBOARD="$selection"
        
        # Tastaturlayout anwenden
        if command -v loadkeys &>/dev/null; then
            loadkeys "$KEYBOARD"
        elif command -v setxkbmap &>/dev/null; then
            setxkbmap "$KEYBOARD"
        fi
    fi
}

# Hauptmenü
main_menu() {
    local options=(
        "install" "\Z4$LANG_INSTALL\Zn - $LANG_INSTALL_DESC" 
        "backup" "\Z4$LANG_BACKUP\Zn - $LANG_BACKUP_DESC" 
        "keys" "\Z4$LANG_KEYS\Zn - $LANG_KEYS_DESC"
    )
    
    # Menü über UI-Bibliothek anzeigen
    local selection=$(ui_show_main_menu "$LANG_MAIN_MENU" "${options[@]}")
    
    # Rückgabewert prüfen
    if [ $? -ne 0 ]; then
        # Zurück zur Sprachauswahl und dann wieder zum Hauptmenü
        select_language
        select_keyboard
        main_menu
        return
    fi
    
    # Je nach Auswahl das entsprechende Modul laden
    case $selection in
        "install")
            # System-Installation
            log_info "Starte Installations-Modul..."
            source "${CORE_DIR}/setup.sh"
            start_installation
            ;;
        "backup")
            # Backup & Restore
            log_info "Starte Backup-Modul..."
            source "${TOOLS_DIR}/backup.sh"
            start_backup
            ;;
        "keys")
            # Schlüssel-Konfiguration
            log_info "Starte Schlüsselkonfiguration..."
            source "${TOOLS_DIR}/usb_creator.sh"
            start_key_config
            ;;
    esac
}

# Hauptfunktion zum Starten des Tools
start_lunitool() {
    # UI initialisieren
    ui_init
    
    # Sprachdatei laden
    load_language
    
    # ESC-Taste abfangen
    trap 'if ui_confirm_exit "$LANG_EXIT_CONFIRM" "$LANG_YES" "$LANG_NO"; then ui_cleanup; exit 0; fi' SIGINT SIGTERM
    
    # Grundeinstellungen
    select_language
    select_keyboard
    
    # Hauptmenü anzeigen
    main_menu
    
    # Bereinigen und beenden
    ui_cleanup
}

# Starte das Programm
start_lunitool