#!/bin/bash
# Gemeinsame Funktionen für lunitool
# Enthält Logging, UI-Funktionen und allgemeine Hilfsfunktionen

# Globale Variablen
LUNITOOL_VERSION="0.1.0"
LOG_FILE="/var/log/lunitool.log"
DEBUG_MODE=false

# Dialog-Konfiguration
DIALOG_BACKTITLE="LUNITOOL"
DIALOG_HEIGHT=20
DIALOG_WIDTH=65

# Dialog-Farben und Stil
DIALOG_COLORS="--colors"
DIALOG_STYLE="--cr-wrap"

#######################
# Logging-Funktionen #
#######################

# Log-Datei initialisieren
init_log() {
    # Erstelle Log-Verzeichnis, falls nicht vorhanden
    if [ ! -d "$(dirname "$LOG_FILE")" ]; then
        mkdir -p "$(dirname "$LOG_FILE")"
    fi
    
    # Prüfe Schreibberechtigung
    if [ ! -w "$(dirname "$LOG_FILE")" ]; then
        LOG_FILE="/tmp/lunitool.log"
        echo "Warnung: Keine Schreibberechtigung für Standard-Log. Verwende $LOG_FILE" >&2
    fi
    
    # Log-Datei initialisieren
    echo "=== LUNITOOL Log gestartet $(date) ===" > "$LOG_FILE"
    echo "Version: $LUNITOOL_VERSION" >> "$LOG_FILE"
    echo "Benutzer: $(whoami)" >> "$LOG_FILE"
    echo "Hostname: $(hostname)" >> "$LOG_FILE"
    echo "===========================" >> "$LOG_FILE"
}

# Logging-Funktion
log() {
    local level="$1"
    local message="$2"
    local timestamp=$(date "+%Y-%m-%d %H:%M:%S")
    
    # Log ins Terminal, wenn Debug-Modus aktiviert
    if [ "$DEBUG_MODE" = true ]; then
        case "$level" in
            "INFO")  echo -e "\033[0;32m[INFO]\033[0m $message" ;;
            "WARN")  echo -e "\033[0;33m[WARN]\033[0m $message" ;;
            "ERROR") echo -e "\033[0;31m[ERROR]\033[0m $message" >&2 ;;
            *)       echo "[$level] $message" ;;
        esac
    fi
    
    # In Log-Datei schreiben
    echo "[$timestamp] [$level] $message" >> "$LOG_FILE"
}

# Verschiedene Log-Level
log_info() {
    log "INFO" "$1"
}

log_warn() {
    log "WARN" "$1"
}

log_error() {
    log "ERROR" "$1"
}

log_debug() {
    if [ "$DEBUG_MODE" = true ]; then
        log "DEBUG" "$1"
    fi
}

########################
# UI-Funktionen (Dialog) #
########################

# Prüfen, ob Dialog installiert ist
ui_check_dependency() {
    if ! command -v dialog &> /dev/null; then
        echo "Dialog ist nicht installiert. Versuche zu installieren..."
        if command -v apt &> /dev/null; then
            apt update && apt install -y dialog
        elif command -v dnf &> /dev/null; then
            dnf install -y dialog
        elif command -v pacman &> /dev/null; then
            pacman -S --noconfirm dialog
        else
            echo "Konnte dialog nicht installieren. Bitte manuell installieren."
            exit 1
        fi
    fi
}

# Dialog-Design initialisieren
ui_init() {
    # Abhängigkeiten prüfen
    ui_check_dependency
    
    # Logging initialisieren
    init_log
    
    # Dialog-Konfiguration erstellen
    local dialogrc="${CONFIG_DIR}/dialogrc"
    
    if [ ! -f "$dialogrc" ]; then
        mkdir -p "${CONFIG_DIR}"
        cat > "$dialogrc" << EOL
# Dialog-Konfigurationsdatei für lunitool
# Hintergrund: Schwarz, Auswahl: Grün, Text: Weiß/Grau

# Bildschirm
screen_color = (BLACK,BLACK,OFF)
shadow_color = (BLACK,BLACK,OFF)
dialog_color = (WHITE,BLACK,OFF)
title_color = (GREEN,BLACK,ON)
border_color = (WHITE,BLACK,OFF)
button_active_color = (WHITE,GREEN,ON)
button_inactive_color = (WHITE,BLACK,OFF)
button_key_active_color = (WHITE,GREEN,ON)
button_key_inactive_color = (RED,BLACK,OFF)
button_label_active_color = (WHITE,GREEN,ON)
button_label_inactive_color = (WHITE,BLACK,ON)
inputbox_color = (BLACK,WHITE,OFF)
inputbox_border_color = (BLACK,WHITE,OFF)
searchbox_color = (BLACK,WHITE,OFF)
searchbox_title_color = (BLUE,WHITE,ON)
searchbox_border_color = (WHITE,WHITE,OFF)
position_indicator_color = (BLUE,WHITE,ON)
menubox_color = (WHITE,BLACK,OFF)
menubox_border_color = (WHITE,BLACK,OFF)
item_color = (WHITE,BLACK,OFF)
item_selected_color = (WHITE,GREEN,ON)
tag_color = (YELLOW,BLACK,ON)
tag_selected_color = (YELLOW,GREEN,ON)
tag_key_color = (RED,BLACK,OFF)
tag_key_selected_color = (RED,GREEN,ON)
check_color = (WHITE,BLACK,OFF)
check_selected_color = (WHITE,GREEN,ON)
uarrow_color = (GREEN,BLACK,ON)
darrow_color = (GREEN,BLACK,ON)
itemhelp_color = (WHITE,BLACK,OFF)
form_active_text_color = (WHITE,BLUE,ON)
form_text_color = (WHITE,CYAN,ON)
form_item_readonly_color = (CYAN,WHITE,ON)
EOL
    fi
    
    # Dialog-Konfiguration anwenden
    export DIALOGRC="$dialogrc"
    
    log_info "UI initialisiert"
}

# Dialog-Menü anzeigen
ui_show_menu() {
    local title="$1"
    shift
    local options=("$@")
    
    log_debug "Zeige Menü: $title"
    
    # Dialog anzeigen und Ergebnis speichern
    local selection=$(dialog --clear --backtitle "$DIALOG_BACKTITLE" \
                            --title "$title" \
                            --ok-label "Auswählen" --cancel-label "Zurück" \
                            $DIALOG_COLORS $DIALOG_STYLE \
                            --menu "$title" $DIALOG_HEIGHT $DIALOG_WIDTH 10 \
                            "${options[@]}" \
                            2>&1 >/dev/tty)
    
    local ret=$?
    log_debug "Menü-Auswahl: $selection (Return: $ret)"
    
    echo "$selection"
    return $ret
}

# Hauptmenü mit spezieller Formatierung
ui_show_main_menu() {
    local title="$1"
    shift
    local options=("$@")
    
    log_debug "Zeige Hauptmenü"
    
    # Dialog anzeigen und Ergebnis speichern
    local selection=$(dialog --clear --backtitle "$DIALOG_BACKTITLE" \
                            --title "$title" \
                            --ok-label "Auswählen" --cancel-label "Zurück" \
                            $DIALOG_COLORS $DIALOG_STYLE \
                            --menu "$title" $DIALOG_HEIGHT $DIALOG_WIDTH 10 \
                            "${options[@]}" \
                            2>&1 >/dev/tty)
    
    local ret=$?
    log_debug "Hauptmenü-Auswahl: $selection (Return: $ret)"
    
    echo "$selection"
    return $ret
}

# Beenden-Bestätigung
ui_confirm_exit() {
    local message="$1"
    local yes_label="$2"
    local no_label="$3"
    
    log_debug "Zeige Beenden-Bestätigung"
    
    dialog --clear --backtitle "$DIALOG_BACKTITLE" \
           --title "$message" \
           --yes-label "$yes_label" --no-label "$no_label" \
           $DIALOG_COLORS $DIALOG_STYLE \
           --yesno "$message" 7 50 \
           2>&1 >/dev/tty
           
    local ret=$?
    log_debug "Beenden-Bestätigung: $ret (0=Ja, 1=Nein)"
    
    return $ret
}

# Meldung anzeigen
ui_show_message() {
    local title="$1"
    local message="$2"
    
    log_debug "Zeige Meldung: $title"
    
    dialog --clear --backtitle "$DIALOG_BACKTITLE" \
           --title "$title" \
           $DIALOG_COLORS $DIALOG_STYLE \
           --msgbox "$message" 10 60 \
           2>&1 >/dev/tty
}

# Fortschrittsanzeige
ui_show_progress() {
    local title="$1"
    local message="$2"
    local percent="$3"
    
    echo "$percent" | dialog --clear --backtitle "$DIALOG_BACKTITLE" \
                            --title "$title" \
                            $DIALOG_COLORS $DIALOG_STYLE \
                            --gauge "$message" 10 70 0 \
                            2>&1 >/dev/tty
}

# Bereinigen bei Programmende
ui_cleanup() {
    log_info "UI-Bereinigung und Programmende"
    clear
}

##########################
# Allgemeine Funktionen #
##########################

# Überprüft, ob das Skript mit Root-Rechten ausgeführt wird
check_root() {
    if [ "$(id -u)" -ne 0 ]; then
        log_warn "Skript wird ohne Root-Rechte ausgeführt"
        ui_show_message "Hinweis" "Dieses Programm benötigt Root-Rechte für volle Funktionalität.\nMöglicherweise sind einige Funktionen eingeschränkt."
        return 1
    fi
    
    log_info "Skript wird mit Root-Rechten ausgeführt"
    return 0
}

# Systeminformationen sammeln
collect_system_info() {
    local info=$(cat /etc/os-release 2>/dev/null)
    local kernel=$(uname -r)
    local arch=$(uname -m)
    
    log_info "System: $info"
    log_info "Kernel: $kernel"
    log_info "Architektur: $arch"
    
    # Speicherplatz
    log_info "Speicherplatz: $(df -h / | grep / | awk '{print $4}') verfügbar"
    
    # RAM
    log_info "RAM: $(free -h | grep Mem | awk '{print $4}') verfügbar"
}

# Versionskontrolle
check_version() {
    log_info "Prüfe auf Updates..."
    # Hier könnte Code stehen, um nach Updates zu prüfen
    return 0
}

# Exportiere wichtige Funktionen
export -f log_info
export -f log_warn
export -f log_error
export -f ui_show_menu
export -f ui_show_message
export -f ui_show_progress