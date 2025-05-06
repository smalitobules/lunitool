#!/bin/bash
# common.sh - Gemeinsame Funktionen für lunitool
# Enthält allgemeine Hilfsfunktionen und Logging

# Globale Variablen
LUNITOOL_VERSION="0.1.0"
LOG_FILE="/var/log/lunitool.log"
DEBUG_MODE=false

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

##########################
# Allgemeine Funktionen #
##########################

# Überprüft, ob das Skript mit Root-Rechten ausgeführt wird
check_root() {
    if [ "$(id -u)" -ne 0 ]; then
        log_warn "Skript wird ohne Root-Rechte ausgeführt"
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

# Erkennt, ob wir in einer Live-Umgebung laufen
is_live_environment() {
    if [ -d "/run/live" ] || [ -f "/run/initramfs/live" ] || grep -q "boot=live" /proc/cmdline; then
        log_info "Live-Umgebung erkannt"
        return 0
    fi
    log_info "Kein Live-System erkannt"
    return 1
}

# Paketmanager ermitteln
detect_package_manager() {
    if command -v apt &> /dev/null; then
        echo "apt"
    elif command -v dnf &> /dev/null; then
        echo "dnf"
    elif command -v pacman &> /dev/null; then
        echo "pacman"
    elif command -v zypper &> /dev/null; then
        echo "zypper"
    else
        echo "unknown"
    fi
}

# Paket installieren
install_package() {
    local package="$1"
    local pm=$(detect_package_manager)
    
    log_info "Installiere Paket: $package mit $pm"
    
    case "$pm" in
        "apt")
            apt-get update && apt-get install -y "$package"
            ;;
        "dnf")
            dnf install -y "$package"
            ;;
        "pacman")
            pacman -S --noconfirm "$package"
            ;;
        "zypper")
            zypper install -y "$package"
            ;;
        *)
            log_error "Kein unterstützter Paketmanager gefunden"
            return 1
            ;;
    esac
    
    return $?
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
export -f log_debug
export -f check_root
export -f install_package