#!/bin/bash
set -e

REPO_URL="https://repo.msaeedsaeedi.com/clean-repo"
APT_LIST_FILE="/etc/apt/sources.list.d/clean-repo.list"

echo "Installing clean-repo..."

# Check for APT
if ! command -v apt >/dev/null 2>&1; then
    echo "Error: APT not found. This script requires Debian/Ubuntu."
    exit 1
fi

# Add repository and install
echo "deb [trusted=yes] $REPO_URL/ ./" | sudo tee "$APT_LIST_FILE" > /dev/null
sudo apt update
sudo apt install -y clean-repo

echo "✓ clean-repo installed successfully!"
