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

# Prozentuale Dialog-Konfiguration
get_dialog_dimensions() {
    local term_width=$(tput cols)
    local term_height=$(tput lines)
    
    # Prozentuale Berechnung mit Minimalwerten
    DIALOG_WIDTH=$(( term_width * 90 / 100 ))
    DIALOG_HEIGHT=$(( term_height * 80 / 100 ))
    
    # Minimalwerte setzen
    [ "$DIALOG_WIDTH" -lt 80 ] && DIALOG_WIDTH=80
    [ "$DIALOG_HEIGHT" -lt 25 ] && DIALOG_HEIGHT=25
    
    # Maximale Werte
    [ "$DIALOG_WIDTH" -gt 120 ] && DIALOG_WIDTH=120
    [ "$DIALOG_HEIGHT" -gt 40 ] && DIALOG_HEIGHT=40
    
    # Menu-Höhe berechnen (60% der Dialog-Höhe)
    MENU_HEIGHT=$(( DIALOG_HEIGHT * 60 / 100 ))
}

# Dialog-Konfiguration
DIALOG_BACKTITLE="LUNITOOL"
get_dialog_dimensions

# Dialog-Optionen und Stil
DIALOG_COMMON_OPTS="--colors --no-shadow --begin 2 2"
DIALOG_BUTTON_OK="--ok-label \"Auswählen\""
DIALOG_BUTTON_CANCEL="--cancel-label \"Zurück\""

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
# Dunkles Hintergrundthema mit Grün als Akzentfarbe

# Bildschirm
screen_color = (BLACK,BLACK,OFF)
dialog_color = (WHITE,BLACK,OFF)
title_color = (GREEN,BLACK,ON)
border_color = (WHITE,BLACK,OFF)
shadow_color = (BLACK,BLACK,OFF)

# Buttons und Menü
button_active_color = (BLACK,GREEN,ON)
button_inactive_color = (WHITE,BLACK,OFF)
button_key_active_color = (BLACK,GREEN,ON)
button_key_inactive_color = (RED,BLACK,OFF)
button_label_active_color = (BLACK,GREEN,ON)
button_label_inactive_color = (WHITE,BLACK,ON)

# Menüelemente
menubox_color = (WHITE,BLACK,OFF)
menubox_border_color = (WHITE,BLACK,OFF)
item_color = (WHITE,BLACK,OFF)
item_selected_color = (BLACK,GREEN,ON)
tag_color = (GREEN,BLACK,ON)
tag_selected_color = (BLACK,GREEN,ON)
tag_key_color = (GREEN,BLACK,ON)
tag_key_selected_color = (BLACK,GREEN,ON)

# Formulare und Eingabe
inputbox_color = (BLACK,WHITE,OFF)
inputbox_border_color = (BLACK,WHITE,OFF)
searchbox_color = (BLACK,WHITE,OFF)
searchbox_title_color = (BLUE,WHITE,ON)
searchbox_border_color = (WHITE,WHITE,OFF)
position_indicator_color = (BLUE,WHITE,ON)
form_active_text_color = (WHITE,BLUE,ON)
form_text_color = (WHITE,CYAN,ON)
form_item_readonly_color = (CYAN,WHITE,ON)

# Navigation
check_color = (WHITE,BLACK,OFF)
check_selected_color = (BLACK,GREEN,ON)
uarrow_color = (GREEN,BLACK,ON)
darrow_color = (GREEN,BLACK,ON)
itemhelp_color = (WHITE,BLACK,OFF)
EOL
    fi
    
    # Dialog-Konfiguration anwenden
    export DIALOGRC="$dialogrc"
    
    # Dialog-Optionen export
    export DIALOG_OPTS="$DIALOG_COMMON_OPTS"
    
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
    get_dialog_dimensions
    
    # Dialog anzeigen und Ergebnis speichern
    local selection=$(dialog --clear --backtitle "$DIALOG_BACKTITLE" \
                            --title "$title" \
                            --ok-label "Auswählen" --cancel-label "Zurück" \
                            --cr-wrap --center \
                            --menu "" $DIALOG_HEIGHT $DIALOG_WIDTH $MENU_HEIGHT \
                            "${options[@]}" \
                            2>&1 >/dev/tty)
    
    local ret=$?
    log_debug "Menü-Auswahl: $selection (Return: $ret)"
    
    # ESC-Taste abfangen
    if [ $ret -eq 255 ]; then
        # ESC wurde gedrückt - Beenden-Dialog anzeigen
        if ui_confirm_exit "Möchtest du das Programm beenden?" "Ja" "Nein"; then
            clear
            exit 0
        else
            # Zurück zum Menü
            ui_show_menu "$title" "${options[@]}"
            return $?
        fi
    fi
    
    echo "$selection"
    return $ret
}

# Hauptmenü mit spezieller Formatierung (Card-Style)
ui_show_main_menu() {
    local title="$1"
    shift
    local options=("$@")
    
    log_debug "Zeige Hauptmenü: $title"
    get_dialog_dimensions
    
    # Erstelle ein kombiniertes Menü mit radiolist zur besseren Darstellung
    # Die Radiobuttons simulieren die Karten des TUI
    local selection=$(dialog --clear --backtitle "$DIALOG_BACKTITLE" \
                            --title "$title" \
                            --ok-label "Auswählen" --cancel-label "Zurück" \
                            --cr-wrap --center \
                            --radiolist "$title" $DIALOG_HEIGHT $DIALOG_WIDTH $MENU_HEIGHT \
                            "${options[@]}" \
                            2>&1 >/dev/tty)
    
    local ret=$?
    log_debug "Hauptmenü-Auswahl: $selection (Return: $ret)"
    
    # ESC-Taste abfangen
    if [ $ret -eq 255 ]; then
        # ESC wurde gedrückt - Beenden-Dialog anzeigen
        if ui_confirm_exit "Möchtest du das Programm beenden?" "Ja" "Nein"; then
            clear
            exit 0
        else
            # Zurück zum Menü
            ui_show_main_menu "$title" "${options[@]}"
            return $?
        fi
    fi
    
    echo "$selection"
    return $ret
}

# Beenden-Bestätigung
ui_confirm_exit() {
    local message="$1"
    local yes_label="$2"
    local no_label="$3"
    
    log_debug "Zeige Beenden-Bestätigung"
    get_dialog_dimensions
    
    # Box-Größe anpassen
    local box_width=$(( DIALOG_WIDTH / 2 ))
    local box_height=$(( DIALOG_HEIGHT / 4 ))
    
    dialog --clear --backtitle "$DIALOG_BACKTITLE" \
           --title "$message" \
           --yes-label "$yes_label" --no-label "$no_label" \
           --cr-wrap --center \
           --yesno "" $box_height $box_width \
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
    get_dialog_dimensions
    
    dialog --clear --backtitle "$DIALOG_BACKTITLE" \
           --title "$title" \
           --ok-label "OK" \
           --cr-wrap --center \
           --msgbox "$message" $(( DIALOG_HEIGHT / 2 )) $(( DIALOG_WIDTH / 2 )) \
           2>&1 >/dev/tty
}

# Warnung anzeigen
ui_show_warning() {
    local title="$1"
    local message="$2"
    
    log_debug "Zeige Warnung: $title"
    get_dialog_dimensions
    
    dialog --clear --backtitle "$DIALOG_BACKTITLE" \
           --title "$title" \
           --ok-label "OK" \
           --cr-wrap --center \
           --colors \
           --msgbox "\Z1[Hinweis]\Zn\n\n$message" $(( DIALOG_HEIGHT / 2 )) $(( DIALOG_WIDTH / 2 )) \
           2>&1 >/dev/tty
}

# Fehler anzeigen
ui_show_error() {
    local title="$1"
    local message="$2"
    
    log_error "UI-Fehler: $message"
    get_dialog_dimensions
    
    dialog --clear --backtitle "$DIALOG_BACKTITLE" \
           --title "$title" \
           --ok-label "OK" \
           --cr-wrap --center \
           --colors \
           --msgbox "\Z1[Fehler]\Zn\n\n$message" $(( DIALOG_HEIGHT / 2 )) $(( DIALOG_WIDTH / 2 )) \
           2>&1 >/dev/tty
}

# Fortschrittsanzeige
ui_show_progress() {
    local title="$1"
    local message="$2"
    local percent="$3"
    
    get_dialog_dimensions
    
    echo "$percent" | dialog --clear --backtitle "$DIALOG_BACKTITLE" \
                            --title "$title" \
                            --cr-wrap --center \
                            --gauge "$message" $(( DIALOG_HEIGHT / 2 )) $(( DIALOG_WIDTH * 70 / 100 )) 0 \
                            2>&1 >/dev/tty
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
export -f ui_show_progress
export -f ui_confirm_exit