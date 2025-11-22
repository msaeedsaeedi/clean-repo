#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}  clean-repo Installation Script${NC}"
echo -e "${GREEN}========================================${NC}"
echo

# Check if running as root
if [ "$EUID" -eq 0 ]; then
    echo -e "${RED}Error: Please do not run this script as root or with sudo${NC}"
    echo "The script will ask for sudo password when needed."
    exit 1
fi

# Check if running on a Debian-based system
if ! command -v apt-get &> /dev/null; then
    echo -e "${RED}Error: This script requires a Debian-based system (apt-get not found)${NC}"
    exit 1
fi

echo -e "${YELLOW}Step 1/3:${NC} Installing GPG public key..."
curl -fsSL https://repo.msaeedsaeedi.com/clean-repo/key.asc | \
    sudo gpg --dearmor -o /usr/share/keyrings/clean-repo-archive-keyring.gpg

echo -e "${GREEN}✓${NC} GPG key installed"
echo

echo -e "${YELLOW}Step 2/3:${NC} Adding APT repository..."
echo "deb [arch=amd64 signed-by=/usr/share/keyrings/clean-repo-archive-keyring.gpg] https://repo.msaeedsaeedi.com/clean-repo stable main" | \
    sudo tee /etc/apt/sources.list.d/clean-repo.list > /dev/null

echo -e "${GREEN}✓${NC} Repository added"
echo

echo -e "${YELLOW}Step 3/3:${NC} Installing clean-repo..."
sudo apt-get update -qq
sudo apt-get install -y clean-repo

echo
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}  Installation Complete! 🎉${NC}"
echo -e "${GREEN}========================================${NC}"
echo
echo "You can now use clean-repo:"
echo "  $ clean-repo --help"
echo "  $ man clean-repo"
echo
