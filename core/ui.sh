#!/bin/bash
# ui.sh - UI-Bibliothek für lunitool
# Enthält alle UI-Funktionen basierend auf Dialog

# Lade Abhängigkeiten
if [ -z "$LUNITOOL_DIR" ]; then
    echo "Fehler: LUNITOOL_DIR nicht definiert"
    exit 1
fi

# Stelle sicher, dass common.sh geladen ist
if ! type log_info >/dev/null 2>&1; then
    source "${LUNITOOL_DIR}/core/common.sh"
fi

# Dialog-Konfiguration
DIALOG_BACKTITLE="LUNITOOL"
DIALOG_HEIGHT=20
DIALOG_WIDTH=65

# Dialog-Farben und Stil
DIALOG_COLORS="--colors"
DIALOG_STYLE="--cr-wrap"

########################
# UI-Initialisierung   #
########################

# Prüfen, ob Dialog installiert ist
ui_check_dependency() {
    if ! command -v dialog &> /dev/null; then
        log_warn "Dialog ist nicht installiert. Versuche zu installieren..."
        install_package "dialog"
        
        if ! command -v dialog &> /dev/null; then
            log_error "Konnte dialog nicht installieren. UI-Funktionen sind nicht verfügbar."
            return 1
        fi
    fi
    
    return 0
}

# Dialog-Design initialisieren
ui_init() {
    # Abhängigkeiten prüfen
    ui_check_dependency || return 1
    
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
    
    log_info "UI-System initialisiert"
    return 0
}

########################
# UI-Grundfunktionen   #
########################

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

########################
# UI-Dialoge           #
########################

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

# Warnung anzeigen
ui_show_warning() {
    local title="$1"
    local message="$2"
    
    log_debug "Zeige Warnung: $title"
    
    dialog --clear --backtitle "$DIALOG_BACKTITLE" \
           --title "$title" \
           $DIALOG_COLORS $DIALOG_STYLE \
           --colors \
           --msgbox "\Z1[WARNUNG]\Zn $message" 10 60 \
           2>&1 >/dev/tty
}

# Fehler anzeigen
ui_show_error() {
    local title="$1"
    local message="$2"
    
    log_error "UI-Fehler: $message"
    
    dialog --clear --backtitle "$DIALOG_BACKTITLE" \
           --title "$title" \
           $DIALOG_COLORS $DIALOG_STYLE \
           --colors \
           --msgbox "\Z1[FEHLER]\Zn $message" 10 60 \
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

# Eingabefeld anzeigen
ui_get_input() {
    local title="$1"
    local message="$2"
    local default="$3"
    
    log_debug "Zeige Eingabefeld: $title"
    
    local input=$(dialog --clear --backtitle "$DIALOG_BACKTITLE" \
                        --title "$title" \
                        $DIALOG_COLORS $DIALOG_STYLE \
                        --inputbox "$message" 10 60 "$default" \
                        2>&1 >/dev/tty)
    
    local ret=$?
    log_debug "Eingabe: $input (Return: $ret)"
    
    echo "$input"
    return $ret
}

# Passwort-Eingabefeld anzeigen
ui_get_password() {
    local title="$1"
    local message="$2"
    
    log_debug "Zeige Passwort-Eingabefeld: $title"
    
    local password=$(dialog --clear --backtitle "$DIALOG_BACKTITLE" \
                          --title "$title" \
                          $DIALOG_COLORS $DIALOG_STYLE \
                          --passwordbox "$message" 10 60 \
                          2>&1 >/dev/tty)
    
    local ret=$?
    log_debug "Passwort eingegeben (Return: $ret)"
    
    echo "$password"
    return $ret
}

# Checkliste anzeigen
ui_show_checklist() {
    local title="$1"
    local message="$2"
    shift 2
    local options=("$@")
    
    log_debug "Zeige Checkliste: $title"
    
    local selections=$(dialog --clear --backtitle "$DIALOG_BACKTITLE" \
                             --title "$title" \
                             $DIALOG_COLORS $DIALOG_STYLE \
                             --checklist "$message" 20 70 10 \
                             "${options[@]}" \
                             2>&1 >/dev/tty)
    
    local ret=$?
    log_debug "Checklisten-Auswahl: $selections (Return: $ret)"
    
    echo "$selections"
    return $ret
}

# Dateiauswahl anzeigen
ui_select_file() {
    local title="$1"
    local path="${2:-$HOME}"
    
    log_debug "Zeige Dateiauswahl: $title (Pfad: $path)"
    
    local file=$(dialog --clear --backtitle "$DIALOG_BACKTITLE" \
                      --title "$title" \
                      $DIALOG_COLORS $DIALOG_STYLE \
                      --fselect "$path/" 14 70 \
                      2>&1 >/dev/tty)
    
    local ret=$?
    log_debug "Dateiauswahl: $file (Return: $ret)"
    
    echo "$file"
    return $ret
}

# Kalender anzeigen
ui_select_date() {
    local title="$1"
    local date_format="${2:-%Y-%m-%d}"
    local default_date="${3:-$(date +%Y-%m-%d)}"
    
    log_debug "Zeige Kalender: $title"
    
    local selected_date=$(dialog --clear --backtitle "$DIALOG_BACKTITLE" \
                               --title "$title" \
                               $DIALOG_COLORS $DIALOG_STYLE \
                               --calendar "Wähle ein Datum:" 0 0 \
                               "$(echo "$default_date" | cut -d'-' -f3)" \
                               "$(echo "$default_date" | cut -d'-' -f2)" \
                               "$(echo "$default_date" | cut -d'-' -f1)" \
                               2>&1 >/dev/tty)
    
    local ret=$?
    log_debug "Datumswahl: $selected_date (Return: $ret)"
    
    # Formatieren des Datums
    if [ $ret -eq 0 ]; then
        selected_date=$(date -d "$(echo $selected_date | sed 's/\//\-/g')" +"$date_format" 2>/dev/null)
    fi
    
    echo "$selected_date"
    return $ret
}

# Text-Editor anzeigen
ui_text_editor() {
    local title="$1"
    local file="$2"
    
    log_debug "Zeige Text-Editor: $title (Datei: $file)"
    
    dialog --clear --backtitle "$DIALOG_BACKTITLE" \
           --title "$title" \
           $DIALOG_COLORS $DIALOG_STYLE \
           --editbox "$file" 20 80 \
           2>/tmp/dialog_edit_$$.tmp >/dev/tty
    
    local ret=$?
    
    if [ $ret -eq 0 ]; then
        cat /tmp/dialog_edit_$$.tmp > "$file"
        log_debug "Text-Editor: Änderungen gespeichert"
    else
        log_debug "Text-Editor: Abgebrochen"
    fi
    
    rm -f /tmp/dialog_edit_$$.tmp
    return $ret
}

# Info-Box anzeigen (ohne Bestätigung)
ui_show_info() {
    local title="$1"
    local message="$2"
    local timeout="${3:-0}"  # 0 = kein Timeout
    
    log_debug "Zeige Info-Box: $title (Timeout: $timeout)"
    
    if [ "$timeout" -gt 0 ]; then
        dialog --clear --backtitle "$DIALOG_BACKTITLE" \
               --title "$title" \
               $DIALOG_COLORS $DIALOG_STYLE \
               --timeout "$timeout" \
               --infobox "$message" 8 60 \
               2>&1 >/dev/tty
    else
        dialog --clear --backtitle "$DIALOG_BACKTITLE" \
               --title "$title" \
               $DIALOG_COLORS $DIALOG_STYLE \
               --infobox "$message" 8 60 \
               2>&1 >/dev/tty
    fi
}

# Bereinigen bei Programmende
ui_cleanup() {
    log_info "UI-Bereinigung"
    clear
}

# Exportiere wichtige Funktionen
export -f ui_show_menu
export -f ui_show_main_menu
export -f ui_show_message
export -f ui_show_warning
export -f ui_show_error
export -f ui_get_input
export -f ui_show_progress
export -f ui_confirm_exit