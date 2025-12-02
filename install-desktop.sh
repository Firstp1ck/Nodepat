#!/bin/bash

# Installation script for Nodepat desktop entry
# This script installs the desktop file and icon

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DESKTOP_FILE="$SCRIPT_DIR/nodepat.desktop"
ICON_FILE="$SCRIPT_DIR/icon.jpg"
APP_NAME="nodepat"

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Installing Nodepat desktop entry...${NC}"

# Check if files exist
if [ ! -f "$DESKTOP_FILE" ]; then
    echo "Error: $DESKTOP_FILE not found!"
    exit 1
fi

if [ ! -f "$ICON_FILE" ]; then
    echo -e "${YELLOW}Warning: $ICON_FILE not found. Icon will not be installed.${NC}"
    ICON_INSTALL=false
else
    ICON_INSTALL=true
fi

# Create directories if they don't exist
mkdir -p ~/.local/share/applications
mkdir -p ~/.local/share/icons

# Install icon if it exists
if [ "$ICON_INSTALL" = true ]; then
    echo "Installing icon..."
    cp "$ICON_FILE" ~/.local/share/icons/${APP_NAME}.jpg
    echo "Icon installed to ~/.local/share/icons/${APP_NAME}.jpg"
fi

# Create a temporary desktop file with absolute icon path
TEMP_DESKTOP=$(mktemp)
sed "s|Icon=nodepat|Icon=$HOME/.local/share/icons/${APP_NAME}.jpg|" "$DESKTOP_FILE" > "$TEMP_DESKTOP"

# If icon file doesn't exist, use a fallback
if [ "$ICON_INSTALL" = false ]; then
    sed -i "s|Icon=.*|Icon=text-editor|" "$TEMP_DESKTOP"
fi

# Copy desktop file
cp "$TEMP_DESKTOP" ~/.local/share/applications/${APP_NAME}.desktop
chmod +x ~/.local/share/applications/${APP_NAME}.desktop

# Cleanup
rm "$TEMP_DESKTOP"

echo -e "${GREEN}Desktop entry installed successfully!${NC}"
echo "Location: ~/.local/share/applications/${APP_NAME}.desktop"
echo ""
echo "The application should now appear in your application menu."
echo "You may need to log out and log back in for changes to take effect."

