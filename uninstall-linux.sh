#!/bin/bash

# Uninstallation script for Nodepat
# Removes the binary, desktop file, icon, and PATH entries

set -e

APP_NAME="Nodepat"
BIN_NAME="Nodepat"

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}  Nodepat Uninstallation Script${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""

# Installation paths
INSTALL_PATH="$HOME/.local/bin/$BIN_NAME"
DESKTOP_FILE="$HOME/.local/share/applications/${APP_NAME}.desktop"
ICON_FILE="$HOME/.local/share/icons/${APP_NAME}.jpg"
UNINSTALL_SCRIPT_PATH="$HOME/.local/bin/uninstall-${APP_NAME,,}.sh"

# Track what was removed
REMOVED_COUNT=0

# Remove binary
if [ -f "$INSTALL_PATH" ]; then
    echo -e "${BLUE}Removing binary...${NC}"
    rm -f "$INSTALL_PATH"
    echo -e "${GREEN}Removed: $INSTALL_PATH${NC}"
    REMOVED_COUNT=$((REMOVED_COUNT + 1))
else
    echo -e "${YELLOW}Binary not found: $INSTALL_PATH${NC}"
fi

# Remove desktop entry
if [ -f "$DESKTOP_FILE" ]; then
    echo -e "${BLUE}Removing desktop entry...${NC}"
    rm -f "$DESKTOP_FILE"
    echo -e "${GREEN}Removed: $DESKTOP_FILE${NC}"
    REMOVED_COUNT=$((REMOVED_COUNT + 1))
    
    # Update desktop database
    if command -v update-desktop-database &> /dev/null; then
        update-desktop-database ~/.local/share/applications 2>/dev/null || true
    fi
else
    echo -e "${YELLOW}Desktop entry not found: $DESKTOP_FILE${NC}"
fi

# Remove icon
if [ -f "$ICON_FILE" ]; then
    echo -e "${BLUE}Removing icon...${NC}"
    rm -f "$ICON_FILE"
    echo -e "${GREEN}Removed: $ICON_FILE${NC}"
    REMOVED_COUNT=$((REMOVED_COUNT + 1))
else
    echo -e "${YELLOW}Icon not found: $ICON_FILE${NC}"
fi

# Remove PATH entries from shell config files
echo -e "${BLUE}Checking for PATH entries...${NC}"
PATH_REMOVED=false

# Function to remove PATH entry from a config file
remove_path_entry() {
    local config_file=$1
    if [ -f "$config_file" ]; then
        if grep -q "# Added by Nodepat installer" "$config_file" 2>/dev/null; then
            # Remove the PATH entry added by Nodepat installer
            # Remove the comment line, the export line, and any empty line after
            sed -i '/# Added by Nodepat installer/d' "$config_file"
            sed -i '/export PATH="\$HOME\/\.local\/bin:\$PATH"/d' "$config_file"
            # Remove empty lines that might be left
            sed -i '/^$/N;/^\n$/d' "$config_file"
            echo -e "${GREEN}Removed PATH entry from: $config_file${NC}"
            PATH_REMOVED=true
        fi
    fi
}

# Detect shell and remove from appropriate config file
if [ -n "$ZSH_VERSION" ]; then
    remove_path_entry "$HOME/.zshrc"
elif [ -n "$BASH_VERSION" ]; then
    remove_path_entry "$HOME/.bashrc"
fi

# Also check .profile (common fallback)
remove_path_entry "$HOME/.profile"

if [ "$PATH_REMOVED" = false ]; then
    echo -e "${YELLOW}No PATH entries found to remove${NC}"
fi

# Remove uninstall script itself
if [ -f "$UNINSTALL_SCRIPT_PATH" ]; then
    echo -e "${BLUE}Removing uninstall script...${NC}"
    rm -f "$UNINSTALL_SCRIPT_PATH"
    echo -e "${GREEN}Removed: $UNINSTALL_SCRIPT_PATH${NC}"
    REMOVED_COUNT=$((REMOVED_COUNT + 1))
fi

echo ""
echo -e "${GREEN}========================================${NC}"
if [ $REMOVED_COUNT -gt 0 ]; then
    echo -e "${GREEN}  Uninstallation Complete!${NC}"
    echo -e "${GREEN}========================================${NC}"
    echo ""
    echo "Removed $REMOVED_COUNT item(s):"
    [ -f "$INSTALL_PATH" ] || echo "  ✓ Binary"
    [ -f "$DESKTOP_FILE" ] || echo "  ✓ Desktop entry"
    [ -f "$ICON_FILE" ] || echo "  ✓ Icon"
    [ "$PATH_REMOVED" = true ] && echo "  ✓ PATH entry"
    [ -f "$UNINSTALL_SCRIPT_PATH" ] || echo "  ✓ Uninstall script"
    echo ""
    echo "Note: If you modified your shell config files, you may need to restart your terminal"
    echo "      or run 'source ~/.bashrc' (or ~/.zshrc) for PATH changes to take effect."
else
    echo -e "${YELLOW}  Nothing to uninstall${NC}"
    echo -e "${GREEN}========================================${NC}"
    echo ""
    echo "No Nodepat installation found."
fi
echo ""

