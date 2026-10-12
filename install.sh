#!/bin/bash
echo "=================================================="
echo "   COLLABS RETOUCH PRO - UNIVERSAL INSTALLER"
echo "=================================================="

# Detect OS
OS="$(uname)"
echo "Detected OS: $OS"

if [ "$OS" = "Darwin" ]; then
    echo "Running macOS Installation..."
    if ! command -v git &> /dev/null; then
        echo "Error: git is required. Please install Xcode Command Line Tools: xcode-select --install"
        exit 1
    fi
    python3 -c "import urllib.request; exec(urllib.request.urlopen('https://raw.githubusercontent.com/Groennesby/Collabs-Retouch-Pro/main/install.py').read().decode())"
elif [ "$OS" = "Linux" ]; then
    echo "Running Linux Installation..."
    if ! command -v git &> /dev/null; then
        echo "Error: git is required. Install via: sudo apt install git python3-pip"
        exit 1
    fi
    python3 -c "import urllib.request; exec(urllib.request.urlopen('https://raw.githubusercontent.com/Groennesby/Collabs-Retouch-Pro/main/install.py').read().decode())"
else
    echo "Unsupported Unix shell or Windows. Please run install.py directly via python."
fi
