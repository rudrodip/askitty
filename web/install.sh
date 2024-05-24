#!/bin/bash

ASKITTY_REPO="rudrodip/askitty"
INSTALL_DIR="/usr/local/bin"
BIN_NAME="askitty"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check if curl is installed
if ! command -v curl &> /dev/null; then
    echo -e "${RED}Error: curl is not installed. Please install curl before running this script.${NC}"
    exit 1
fi

# Check if running on Windows
if [ -n "$WINDIR" ]; then
    echo -e "${RED}Error: This script is intended for macOS. Please use macOS to run it.${NC}"
    exit 1
fi

# Get the latest release URL
LATEST_RELEASE_URL="https://api.github.com/repos/$ASKITTY_REPO/releases/latest"
DOWNLOAD_URL=$(curl -s $LATEST_RELEASE_URL | grep "browser_download_url.*$BIN_NAME" | cut -d '"' -f 4)

# Check if the download URL was retrieved
if [ -z "$DOWNLOAD_URL" ]; then
    echo -e "${RED}Error: Could not find the download URL for the latest release.${NC}"
    exit 1
fi

# Create a temporary directory for the download
TEMP_DIR=$(mktemp -d)
TEMP_FILE="$TEMP_DIR/$BIN_NAME"

# Download the binary with progress bar
echo -e "${BLUE}Downloading $BIN_NAME from $DOWNLOAD_URL...${NC}"
curl -# -fSL "$DOWNLOAD_URL" -o "$TEMP_FILE"

# Check if the download was successful
if [ $? -ne 0 ]; then
    echo -e "${RED}Error downloading the $BIN_NAME binary.${NC}"
    rm -f "$TEMP_FILE"  # Remove the binary if an error occurred
    rmdir "$TEMP_DIR"
    exit 1
fi

# Make it executable
chmod +x "$TEMP_FILE"

# Move it to the installation directory
sudo mv "$TEMP_FILE" "$INSTALL_DIR/"
if [ $? -eq 0 ]; then
    echo -e "${GREEN}$BIN_NAME installed successfully in $INSTALL_DIR${NC}"
    echo -e "${BLUE}\nRun '$BIN_NAME' in terminal\n${NC}"
else
    echo -e "${RED}Error moving $BIN_NAME to $INSTALL_DIR. Please check permissions.${NC}"
    rm -f "$TEMP_FILE"  # Remove the binary if the move operation failed
    rmdir "$TEMP_DIR"
    exit 1
fi

# Clean up temporary directory
rmdir "$TEMP_DIR"
