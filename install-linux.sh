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
REPO_OWNER="Firstp1ck"  # Update this with your GitHub username
REPO_NAME="Nodepat"  # Update this with your repository name

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
IS_PRERELEASE=false

# Fetch release info with error checking
TEMP_RESPONSE=$(mktemp)
HTTP_CODE=""
if command -v curl &> /dev/null; then
    HTTP_CODE=$(curl -s -o "$TEMP_RESPONSE" -w "%{http_code}" "$RELEASE_API_URL")
    RELEASE_INFO=$(cat "$TEMP_RESPONSE")
elif command -v wget &> /dev/null; then
    wget -qO "$TEMP_RESPONSE" "$RELEASE_API_URL" 2>&1
    RELEASE_INFO=$(cat "$TEMP_RESPONSE")
fi
rm -f "$TEMP_RESPONSE"

# Check if API call was successful (check for error messages in response)
if echo "$RELEASE_INFO" | grep -q '"message"'; then
    ERROR_MSG=$(echo "$RELEASE_INFO" | grep '"message"' | head -1 | sed 's/.*"message":\s*"\([^"]*\)".*/\1/')
    STATUS_CODE=$(echo "$RELEASE_INFO" | grep '"status"' | head -1 | sed 's/.*"status":\s*\([0-9]*\).*/\1/' || echo "")
    
    # If 404, try to get latest prerelease instead
    if [ "$HTTP_CODE" = "404" ] || [ "$STATUS_CODE" = "404" ]; then
        echo -e "${YELLOW}No stable release found, checking for prereleases...${NC}"
        RELEASE_API_URL="https://api.github.com/repos/$REPO_OWNER/$REPO_NAME/releases"
        
        TEMP_RESPONSE=$(mktemp)
        if command -v curl &> /dev/null; then
            HTTP_CODE=$(curl -s -o "$TEMP_RESPONSE" -w "%{http_code}" "$RELEASE_API_URL")
            RELEASES_LIST=$(cat "$TEMP_RESPONSE")
        elif command -v wget &> /dev/null; then
            wget -qO "$TEMP_RESPONSE" "$RELEASE_API_URL" 2>&1
            RELEASES_LIST=$(cat "$TEMP_RESPONSE")
        fi
        rm -f "$TEMP_RESPONSE"
        
        # Extract the first release (latest, including prereleases)
        # Releases are ordered by creation date, newest first
        if echo "$RELEASES_LIST" | grep -q '"tag_name"'; then
            # Check if the first release is a prerelease
            FIRST_PRERELEASE=$(echo "$RELEASES_LIST" | grep -A 10 '"tag_name"' | grep '"prerelease"' | head -1 | sed 's/.*"prerelease":\s*\([^,}]*\).*/\1/' | tr -d ' ')
            
            # Use the full releases list - extraction will use head -1 to get first release
            RELEASE_INFO="$RELEASES_LIST"
            
            if [ "$FIRST_PRERELEASE" = "true" ]; then
                IS_PRERELEASE=true
            else
                IS_PRERELEASE=false
            fi
        else
            echo -e "${RED}Error: Failed to fetch release information!${NC}"
            if [ -n "$HTTP_CODE" ] && [ "$HTTP_CODE" != "200" ]; then
                echo "HTTP Status Code: $HTTP_CODE"
            elif [ -n "$STATUS_CODE" ]; then
                echo "HTTP Status Code: $STATUS_CODE"
            fi
            if [ -n "$ERROR_MSG" ]; then
                echo "GitHub API Error: $ERROR_MSG"
            fi
            echo ""
            echo "Possible reasons:"
            echo "  1. Repository '$REPO_OWNER/$REPO_NAME' does not exist"
            echo "  2. Repository has no releases yet (including prereleases)"
            echo "  3. Network connectivity issues"
            echo ""
            echo "Please check that REPO_OWNER and REPO_NAME are correct in the script."
            exit 1
        fi
    else
        echo -e "${RED}Error: Failed to fetch release information!${NC}"
        if [ -n "$HTTP_CODE" ] && [ "$HTTP_CODE" != "200" ]; then
            echo "HTTP Status Code: $HTTP_CODE"
        elif [ -n "$STATUS_CODE" ]; then
            echo "HTTP Status Code: $STATUS_CODE"
        fi
        if [ -n "$ERROR_MSG" ]; then
            echo "GitHub API Error: $ERROR_MSG"
        fi
        echo ""
        echo "Possible reasons:"
        echo "  1. Repository '$REPO_OWNER/$REPO_NAME' does not exist"
        echo "  2. Repository has no releases yet"
        echo "  3. Network connectivity issues"
        echo ""
        echo "Please check that REPO_OWNER and REPO_NAME are correct in the script."
        exit 1
    fi
fi

# Extract version and download URL (using more portable grep)
LATEST_VERSION=$(echo "$RELEASE_INFO" | grep '"tag_name"' | head -1 | sed 's/.*"tag_name":\s*"\([^"]*\)".*/\1/')

if [ -z "$LATEST_VERSION" ]; then
    echo -e "${RED}Error: Failed to parse release information!${NC}"
    echo "API Response:"
    echo "$RELEASE_INFO" | head -10
    exit 1
fi

if [ "$IS_PRERELEASE" = true ]; then
    echo -e "${YELLOW}Latest version (prerelease): $LATEST_VERSION${NC}"
else
    echo -e "${GREEN}Latest version: $LATEST_VERSION${NC}"
fi

# Find the Linux binary asset (exclude .exe files, using portable sed)
# When using releases list, ensure we get assets from the first release only
if [ "$IS_PRERELEASE" = true ]; then
    # Extract first release object's assets section to ensure we get the right download URL
    # Find assets array that comes after the first tag_name
    DOWNLOAD_URL=$(echo "$RELEASE_INFO" | grep -A 200 '"tag_name"' | grep -A 200 '"assets"' | grep '"browser_download_url"' | grep -v "\.exe" | grep -i "nodepat" | head -1 | sed 's/.*"browser_download_url":\s*"\([^"]*\)".*/\1/')
else
    DOWNLOAD_URL=$(echo "$RELEASE_INFO" | grep '"browser_download_url"' | grep -v "\.exe" | grep -i "nodepat" | head -1 | sed 's/.*"browser_download_url":\s*"\([^"]*\)".*/\1/')
fi

if [ -z "$DOWNLOAD_URL" ]; then
    echo -e "${RED}Error: Linux binary not found in latest release!${NC}"
    echo "Available assets:"
    echo "$RELEASE_INFO" | grep '"name"' | head -5 | sed 's/.*"name":\s*"\([^"]*\)".*/\1/'
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
