#!/bin/bash
set -e

REPO_URL="https://msaeedsaeedi.github.io/clean-repo"
APT_LIST_FILE="/etc/apt/sources.list.d/clean-repo.list"

echo "Installing clean-repo via APT repository..."

# Check if we're on a supported system
if ! command -v apt >/dev/null 2>&1; then
    echo "Error: This installation method requires APT (Debian/Ubuntu)"
    echo "Please check README for other installation options."
    exit 1
fi

# Add repository
echo "Adding clean-repo APT repository..."
echo "deb [trusted=yes] $REPO_URL/apt-repo/ ./" | sudo tee "$APT_LIST_FILE" > /dev/null

# Update package lists and install
echo "Installing clean-repo..."
sudo apt update
sudo apt install clean-repo -y

echo ""
echo "🎉 clean-repo installed successfully!"
echo ""
echo "To update in the future, run:"
echo "  sudo apt update && sudo apt upgrade"
echo ""
echo "To uninstall, run:"
echo "  sudo apt remove clean-repo"
echo "  sudo rm $APT_LIST_FILE"