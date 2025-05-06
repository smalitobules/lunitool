#!/bin/bash
# lunitool - Linux Universal Tool
# Hauptskript zur Steuerung der Umgebung

# Verzeichnisstruktur definieren
LUNITOOL_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CORE_DIR="${LUNITOOL_DIR}/core"
TOOLS_DIR="${LUNITOOL_DIR}/tools"
SCRIPTS_DIR="${LUNITOOL_DIR}/scripts"
CONFIG_DIR="${LUNITOOL_DIR}/configs"
RESOURCES_DIR="${LUNITOOL_DIR}/resources"
LANG_DIR="${CORE_DIR}/languages"

# Standardwerte
CURRENT_LANG="de"
KEYBOARD="de"

# Pfade in Umgebungsvariablen exportieren, damit sie in eingebundenen Skripten verfügbar sind
export LUNITOOL_DIR CORE_DIR TOOLS_DIR SCRIPTS_DIR CONFIG_DIR RESOURCES_DIR LANG_DIR

# Prüfen und erstellen der erforderlichen Verzeichnisse
mkdir -p "$CORE_DIR" "$TOOLS_DIR" "$SCRIPTS_DIR" "$CONFIG_DIR" "$RESOURCES_DIR" "$LANG_DIR"

# Grundlegende Funktionen einbinden
source "${CORE_DIR}/common.sh"
source "${CORE_DIR}/ui.sh"

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
    if [ $? -eq 0 ] && [ -n "$selection" ]; then
        CURRENT_LANG="$selection"
        
        # Sprachdatei laden
        load_language
        
        log_info "Sprache auf $CURRENT_LANG geändert"
        return 0
    fi
    
    return 1
}

# Tastaturlayout-Dialog
select_keyboard() {
    local options=("de" "Deutsch (de)" "us" "US-English (us)")
    
    # Menü über UI-Bibliothek anzeigen
    local selection=$(ui_show_menu "$LANG_KEYBOARD_SELECT" "${options[@]}")
    
    # Prüfen, ob Benutzer abgebrochen hat
    if [ $? -eq 0 ] && [ -n "$selection" ]; then
        KEYBOARD="$selection"
        
        # Tastaturlayout anwenden
        if command -v loadkeys &>/dev/null; then
            loadkeys "$KEYBOARD"
        elif command -v setxkbmap &>/dev/null; then
            setxkbmap "$KEYBOARD"
        fi
        
        log_info "Tastaturlayout auf $KEYBOARD geändert"
        return 0
    fi
    
    return 1
}

# Hauptmenü
main_menu() {
    # Optionen für das Hauptmenü - mit besserer Card-Style-Formatierung
    local options=(
        "install" "ON" "\Z4System-Installation\Zn\nNeues Linux-System einrichten" 
        "backup" "OFF" "\Z4Backup & Restore\Zn\nSichern und wiederherstellen" 
        "keys" "OFF" "\Z4Schlüssel-Konfiguration\Zn\nBoot-USB und Authentifizierung"
    )
    
    local running=true
    
    while $running; do
        # Menü über UI-Bibliothek anzeigen
        local selection=$(ui_show_main_menu "$LANG_MAIN_MENU" "${options[@]}")
        local ret=$?
        
        # Rückgabewert prüfen
        if [ $ret -ne 0 ]; then
            # Zurück zur Sprachauswahl und Tastaturauswahl
            log_debug "Zurück zur Grundkonfiguration"
            select_language
            select_keyboard
            continue
        fi
        
        # Je nach Auswahl das entsprechende Modul laden
        case $selection in
            "install")
                # System-Installation
                log_info "Starte Installations-Modul..."
                if [ -f "${CORE_DIR}/setup.sh" ]; then
                    source "${CORE_DIR}/setup.sh"
                    start_installation
                else
                    ui_show_message "Information" "Das Installations-Modul ist noch nicht verfügbar.\n\nEs wird in einer zukünftigen Version implementiert."
                fi
                ;;
            "backup")
                # Backup & Restore
                log_info "Starte Backup-Modul..."
                if [ -f "${TOOLS_DIR}/backup.sh" ]; then
                    source "${TOOLS_DIR}/backup.sh"
                    start_backup
                else
                    ui_show_message "Information" "Das Backup-Modul ist noch nicht verfügbar.\n\nEs wird in einer zukünftigen Version implementiert."
                fi
                ;;
            "keys")
                # Schlüssel-Konfiguration
                log_info "Starte Schlüsselkonfiguration..."
                if [ -f "${TOOLS_DIR}/usb_creator.sh" ]; then
                    source "${TOOLS_DIR}/usb_creator.sh"
                    start_key_config
                else
                    ui_show_message "Information" "Das USB-Schlüssel-Modul ist noch nicht verfügbar.\n\nEs wird in einer zukünftigen Version implementiert."
                fi
                ;;
            *)
                # Unbekannte Auswahl
                log_error "Unbekannte Hauptmenü-Auswahl: $selection"
                ui_show_message "Fehler" "Ungültige Auswahl"
                ;;
        esac
    done
}

# Signalbehandlung für Abbruch (STRG+C)
setup_signal_handling() {
    trap 'handle_exit' SIGINT SIGTERM EXIT
}

# Behandlung von Programmende und Abbruch
handle_exit() {
    # Entferne den Signal-Handler, um endlose Schleifen zu vermeiden
    trap - SIGINT SIGTERM EXIT
    
    # Frage nur nach Bestätigung, wenn nicht durch EXIT ausgelöst
    if [ "$1" != "EXIT" ]; then
        if ui_confirm_exit "$LANG_EXIT_CONFIRM" "$LANG_YES" "$LANG_NO"; then
            log_info "Programm wird auf Benutzeranfrage beendet"
            cleanup_and_exit
        else
            # Stelle den Signal-Handler wieder her
            setup_signal_handling
            return 0
        fi
    else
        # Normales Programmende
        cleanup_and_exit
    fi
}

# Aufräumen und Beenden
cleanup_and_exit() {
    log_info "Bereinige und beende Programm"
    ui_cleanup
    exit 0
}

# Hauptfunktion zum Starten des Tools
start_lunitool() {
    # Initialisiere Logging
    init_log
    log_info "Starte lunitool v0.1.0"
    
    # Überprüfe Root-Rechte
    if ! check_root; then
        log_warn "Einige Funktionen könnten ohne Root-Rechte eingeschränkt sein"
    fi
    
    # Sammle Systeminformationen
    collect_system_info
    
    # UI initialisieren
    if ! ui_init; then
        log_error "UI-Initialisierung fehlgeschlagen"
        echo "Fehler: Konnte UI nicht initialisieren. Prüfe, ob 'dialog' installiert ist."
        exit 1
    fi
    
    # Signalbehandlung einrichten
    setup_signal_handling
    
    # Sprachdatei laden
    load_language
    
    # Grundeinstellungen
    select_language
    select_keyboard
    
    # Hauptmenü anzeigen
    main_menu
    
    # Bereinigen und beenden (wird normalerweise nicht erreicht)
    cleanup_and_exit
}

# Starte das Programm
start_lunitool