#!/bin/bash

# Installation script for Nodepat
# Downloads the latest release, installs the binary, desktop file, and icon

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DESKTOP_FILE="$SCRIPT_DIR/nodepat.desktop"
ICON_FILE="$SCRIPT_DIR/icon.jpg"
APP_NAME="Nodepat"
BIN_NAME="Nodepat"

# Configuration - Update these with your repository information
REPO_OWNER="YOUR_USERNAME"  # Update this with your GitHub username
REPO_NAME="Notepad"  # Update this with your repository name

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}  Nodepat Installation Script${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""

# Detect architecture
ARCH=$(uname -m)
case $ARCH in
    x86_64)
        ARCH_NAME="x86_64"
        ;;
    aarch64|arm64)
        ARCH_NAME="aarch64"
        ;;
    *)
        echo -e "${RED}Error: Unsupported architecture: $ARCH${NC}"
        echo "Supported architectures: x86_64, aarch64"
        exit 1
        ;;
esac

echo -e "${BLUE}Detected architecture: $ARCH_NAME${NC}"
echo ""

# Check for required tools
if ! command -v curl &> /dev/null && ! command -v wget &> /dev/null; then
    echo -e "${RED}Error: curl or wget is required but not installed!${NC}"
    exit 1
fi

# Function to download file
download_file() {
    local url=$1
    local output=$2
    
    if command -v curl &> /dev/null; then
        curl -L -o "$output" "$url"
    elif command -v wget &> /dev/null; then
        wget -O "$output" "$url"
    fi
}

# Get latest release information
echo -e "${GREEN}Fetching latest release information...${NC}"
RELEASE_API_URL="https://api.github.com/repos/$REPO_OWNER/$REPO_NAME/releases/latest"

if command -v curl &> /dev/null; then
    RELEASE_INFO=$(curl -s "$RELEASE_API_URL")
elif command -v wget &> /dev/null; then
    RELEASE_INFO=$(wget -qO- "$RELEASE_API_URL")
fi

# Extract version and download URL
LATEST_VERSION=$(echo "$RELEASE_INFO" | grep -oP '"tag_name":\s*"\K[^"]+' | head -1)

if [ -z "$LATEST_VERSION" ]; then
    echo -e "${RED}Error: Failed to fetch release information!${NC}"
    echo "Please check that REPO_OWNER and REPO_NAME are correct in the script."
    exit 1
fi

echo -e "${GREEN}Latest version: $LATEST_VERSION${NC}"

# Find the Linux binary asset (exclude .exe files)
DOWNLOAD_URL=$(echo "$RELEASE_INFO" | grep -oP '"browser_download_url":\s*"\K[^"]*Nodepat[^"]*' | grep -v "\.exe" | head -1)

if [ -z "$DOWNLOAD_URL" ]; then
    echo -e "${RED}Error: Linux binary not found in latest release!${NC}"
    echo "Available assets:"
    echo "$RELEASE_INFO" | grep -oP '"name":\s*"\K[^"]+' | head -5
    exit 1
fi

echo -e "${BLUE}Download URL: $DOWNLOAD_URL${NC}"
echo ""

# Create directories if they don't exist
echo "Creating directories..."
mkdir -p ~/.local/bin
mkdir -p ~/.local/share/applications
mkdir -p ~/.local/share/icons

# Download binary
TEMP_BINARY=$(mktemp)
echo -e "${GREEN}Downloading $BIN_NAME...${NC}"
download_file "$DOWNLOAD_URL" "$TEMP_BINARY"

# Make binary executable
chmod +x "$TEMP_BINARY"

# Install binary
echo "Installing binary..."
INSTALL_PATH="$HOME/.local/bin/$BIN_NAME"
mv "$TEMP_BINARY" "$INSTALL_PATH"
echo -e "Binary installed to ${GREEN}$INSTALL_PATH${NC}"
echo ""

# Check if files exist
if [ ! -f "$DESKTOP_FILE" ]; then
    echo -e "${YELLOW}Warning: $DESKTOP_FILE not found. Desktop entry will not be installed.${NC}"
    DESKTOP_INSTALL=false
else
    DESKTOP_INSTALL=true
fi

if [ ! -f "$ICON_FILE" ]; then
    echo -e "${YELLOW}Warning: $ICON_FILE not found. Icon will not be installed.${NC}"
    ICON_INSTALL=false
else
    ICON_INSTALL=true
fi

# Install icon if it exists
if [ "$ICON_INSTALL" = true ]; then
    echo "Installing icon..."
    cp "$ICON_FILE" ~/.local/share/icons/${APP_NAME}.jpg
    echo -e "Icon installed to ${GREEN}~/.local/share/icons/${APP_NAME}.jpg${NC}"
fi

# Install desktop file if it exists
if [ "$DESKTOP_INSTALL" = true ]; then
    echo "Installing desktop entry..."
    
    # Create a temporary desktop file with correct paths
    TEMP_DESKTOP=$(mktemp)
    
    # Update Exec path and Icon path
    sed -e "s|Exec=nodepat|Exec=$INSTALL_PATH|" \
        -e "s|Exec=Nodepat|Exec=$INSTALL_PATH|" \
        -e "s|Icon=nodepat|Icon=$HOME/.local/share/icons/${APP_NAME}.jpg|" \
        "$DESKTOP_FILE" > "$TEMP_DESKTOP"
    
    # If icon file doesn't exist, use a fallback
    if [ "$ICON_INSTALL" = false ]; then
        sed -i "s|Icon=.*|Icon=text-editor|" "$TEMP_DESKTOP"
    fi
    
    # Copy desktop file
    cp "$TEMP_DESKTOP" ~/.local/share/applications/${APP_NAME}.desktop
    chmod +x ~/.local/share/applications/${APP_NAME}.desktop
    
    # Cleanup
    rm "$TEMP_DESKTOP"
    
    echo -e "Desktop entry installed to ${GREEN}~/.local/share/applications/${APP_NAME}.desktop${NC}"
fi

# Add to PATH if not already present
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    echo ""
    echo -e "${YELLOW}Adding ~/.local/bin to PATH...${NC}"
    
    # Detect shell and add to appropriate config file
    if [ -n "$ZSH_VERSION" ]; then
        CONFIG_FILE="$HOME/.zshrc"
    elif [ -n "$BASH_VERSION" ]; then
        CONFIG_FILE="$HOME/.bashrc"
    else
        CONFIG_FILE="$HOME/.profile"
    fi
    
    if ! grep -q "$HOME/.local/bin" "$CONFIG_FILE" 2>/dev/null; then
        echo "" >> "$CONFIG_FILE"
        echo "# Added by Nodepat installer" >> "$CONFIG_FILE"
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$CONFIG_FILE"
        echo -e "${GREEN}Added PATH entry to $CONFIG_FILE${NC}"
        echo -e "${YELLOW}Run 'source $CONFIG_FILE' or restart your terminal for PATH changes to take effect.${NC}"
    fi
fi

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}  Installation Complete!${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo "Installed files:"
echo "  - Binary: $INSTALL_PATH"
if [ "$ICON_INSTALL" = true ]; then
    echo "  - Icon: ~/.local/share/icons/${APP_NAME}.jpg"
fi
if [ "$DESKTOP_INSTALL" = true ]; then
    echo "  - Desktop entry: ~/.local/share/applications/${APP_NAME}.desktop"
fi
echo ""
echo "Version installed: $LATEST_VERSION"
echo ""
echo "You can now:"
echo "  1. Run '$BIN_NAME' from any terminal (after restart or 'source ~/.bashrc')"
echo "  2. Launch from your application menu"
echo "  3. Run '$INSTALL_PATH' directly"
echo ""
echo -e "${GREEN}Installation completed successfully!${NC}"
