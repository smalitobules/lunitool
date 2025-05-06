#!/bin/bash
# UI-Bibliothek für lunitool
# Enthält alle Dialog-basierten UI-Funktionen

# Dialog-Konfiguration
DIALOG_BACKTITLE="LUNITOOL"
DIALOG_HEIGHT=20
DIALOG_WIDTH=65

# Dialog-Farben und Stil
DIALOG_COLORS="--colors"
DIALOG_STYLE="--cr-wrap"

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
}

# Dialog-Menü anzeigen
ui_show_menu() {
    local title="$1"
    shift
    local options=("$@")
    
    # Dialog anzeigen und Ergebnis speichern
    local selection=$(dialog --clear --backtitle "$DIALOG_BACKTITLE" \
                            --title "$title" \
                            --ok-label "Auswählen" --cancel-label "Zurück" \
                            $DIALOG_COLORS $DIALOG_STYLE \
                            --menu "$title" $DIALOG_HEIGHT $DIALOG_WIDTH 10 \
                            "${options[@]}" \
                            2>&1 >/dev/tty)
    
    local ret=$?
    echo "$selection"
    return $ret
}

# Hauptmenü mit spezieller Formatierung
ui_show_main_menu() {
    local title="$1"
    shift
    local options=("$@")
    
    # Dialog anzeigen und Ergebnis speichern
    local selection=$(dialog --clear --backtitle "$DIALOG_BACKTITLE" \
                            --title "$title" \
                            --ok-label "Auswählen" --cancel-label "Zurück" \
                            $DIALOG_COLORS $DIALOG_STYLE \
                            --menu "$title" $DIALOG_HEIGHT $DIALOG_WIDTH 10 \
                            "${options[@]}" \
                            2>&1 >/dev/tty)
    
    local ret=$?
    echo "$selection"
    return $ret
}

# Beenden-Bestätigung
ui_confirm_exit() {
    local message="$1"
    local yes_label="$2"
    local no_label="$3"
    
    dialog --clear --backtitle "$DIALOG_BACKTITLE" \
           --title "$message" \
           --yes-label "$yes_label" --no-label "$no_label" \
           $DIALOG_COLORS $DIALOG_STYLE \
           --yesno "$message" 7 50 \
           2>&1 >/dev/tty
           
    return $?
}

# Fehlende Module herunterladen
ui_download_module() {
    local module_name="$1"
    
    dialog --backtitle "$DIALOG_BACKTITLE" \
           --title "Download" \
           --infobox "Modul '${module_name}' wird heruntergeladen..." 5 50
    
    # Hier würde die Logik zum Herunterladen und Verifizieren des Moduls stehen
    sleep 2
    
    dialog --backtitle "$DIALOG_BACKTITLE" \
           --title "Download fehlgeschlagen" \
           --msgbox "Modul wurde nicht gefunden und konnte nicht heruntergeladen werden." 8 50
}

# Bereinigen bei Programmende
ui_cleanup() {
    clear
}