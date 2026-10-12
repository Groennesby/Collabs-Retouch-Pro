#!/usr/bin/env bash
# ==============================================================================
# COLLABS RETOUCH PRO — Linux Desktop Installer & Launcher
# Installs application launcher, desktop shortcut, icons, and binary symlink.
# Created by Dave Groennesby (Collabs Software)
# ==============================================================================

set -e

APP_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$APP_DIR"
chmod +x "$0" 2>/dev/null || true

ICON_SRC="$APP_DIR/icon.png"
if [ ! -f "$ICON_SRC" ]; then
    ICON_SRC="$APP_DIR/splashscreens/Collabs_Retouch_Pro_RP_Icon.png"
fi

echo "=== COLLABS RETOUCH PRO — LINUX INSTALLER ==="

# 1. Search for working Python with Tkinter
WORKING_PYTHON=""

check_python() {
    local py="$1"
    if command -v "$py" >/dev/null 2>&1; then
        if "$py" -c "import tkinter, tkinter.ttk; print('OK')" >/dev/null 2>&1; then
            WORKING_PYTHON="$(command -v "$py")"
            return 0
        fi
    fi
    return 1
}

if ! check_python python3; then
    if ! check_python python; then
        if [ -n "$CONDA_PREFIX" ]; then
            check_python "$CONDA_PREFIX/bin/python3" || true
        fi
    fi
fi

if [ -z "$WORKING_PYTHON" ]; then
    echo "[!] Tkinter not found in your Python environment."
    echo "[*] For Debian/Ubuntu: sudo apt install python3-tk"
    echo "[*] For Fedora/RHEL:   sudo dnf install python3-tkinter"
    echo "[*] For Arch:          sudo pacman -S tk"
    echo "[*] For Conda:         conda install -c conda-forge tk"
    exit 1
fi

echo "[✓] Using Python: $WORKING_PYTHON"

# Ensure Pillow is installed
"$WORKING_PYTHON" -m pip install --quiet pillow || true

echo "[*] Setting up desktop integration..."
mkdir -p "$HOME/.local/share/applications"
mkdir -p "$HOME/.local/share/icons/hicolor/512x512/apps"
mkdir -p "$HOME/Desktop"
mkdir -p "$HOME/bin"

if [ -f "$ICON_SRC" ]; then
    cp -f "$ICON_SRC" "$HOME/.local/share/icons/hicolor/512x512/apps/collabs-retouch-pro.png"
fi

DESKTOP_ENTRY="$HOME/.local/share/applications/collabs-retouch-pro.desktop"

cat <<EOF > "$DESKTOP_ENTRY"
[Desktop Entry]
Version=1.0
Type=Application
Name=Collabs Retouch Pro
GenericName=AI Portrait & RAW Studio
Comment=Collabs Retouch Pro Studio
Exec=$WORKING_PYTHON "$APP_DIR/launcher.py"
Icon=$HOME/.local/share/icons/hicolor/512x512/apps/collabs-retouch-pro.png
Terminal=false
Categories=Graphics;Photography;RasterGraphics;2DGraphics;
StartupNotify=true
EOF

chmod +x "$DESKTOP_ENTRY"

if [ -d "$HOME/Desktop" ]; then
    cp -f "$DESKTOP_ENTRY" "$HOME/Desktop/collabs-retouch-pro.desktop"
    chmod +x "$HOME/Desktop/collabs-retouch-pro.desktop"
fi

# Helper symlink / script in ~/bin
cat <<EOF > "$HOME/bin/collabs-retouch-pro"
#!/usr/bin/env bash
exec "$WORKING_PYTHON" "$APP_DIR/launcher.py" "\$@"
EOF
chmod +x "$HOME/bin/collabs-retouch-pro"

echo "=== INSTALLATION COMPLETE ==="
echo "[*] Launching Collabs Retouch Pro..."
exec "$WORKING_PYTHON" "$APP_DIR/launcher.py"
