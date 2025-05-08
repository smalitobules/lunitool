#!/bin/bash

# Einfaches Skript zum Kompilieren des Lunitool-Projekts

# Farben für die Ausgabe
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}### Lunitool Compiler ###${NC}"

# Sicherstellen, dass die Verzeichnisstruktur existiert
echo -e "${YELLOW}Erstelle notwendige Verzeichnisse...${NC}"
mkdir -p src/core src/lang src/tools src/ui core/languages

# Kompiliere das Projekt
echo -e "${YELLOW}Kompiliere das Projekt...${NC}"
cargo clean
cargo build --release

# Prüfe, ob die Kompilierung erfolgreich war
if [ $? -eq 0 ]; then
    echo -e "${GREEN}Kompilierung erfolgreich abgeschlossen!${NC}"
    echo "Die ausführbare Datei befindet sich in: target/release/lunitool"
    echo ""
    echo -e "${YELLOW}Hinweise:${NC}"
    echo "1. Das Programm muss mit 'sudo' gestartet werden für volle Funktionalität."
    echo "2. Starte das Programm mit: sudo ./target/release/lunitool"
else
    echo -e "${RED}Kompilierung fehlgeschlagen. Bitte überprüfe die Fehler oben.${NC}"
    exit 1
fi

echo -e "${GREEN}Fertig!${NC}"