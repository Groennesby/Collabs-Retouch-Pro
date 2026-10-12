#!/usr/bin/env bash
# ==============================================================================
# COLLABS RETOUCH PRO — Linux Desktop Installer
# Installs application launcher, desktop shortcut, icons, and binary symlink.
# Created by Dave Groennesby (Collabs Software)
# ==============================================================================

set -e

APP_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [ -f "$APP_DIR/icon.png" ]; then
    ICON_SRC="$APP_DIR/icon.png"
elif [ -f "$APP_DIR/splashscreens/Collabs_Retouch_Pro_RP_Icon.png" ]; then
    ICON_SRC="$APP_DIR/splashscreens/Collabs_Retouch_Pro_RP_Icon.png"
else
    ICON_SRC="$APP_DIR/app/icon.png"
fi
LAUNCHER="$APP_DIR/launch_retouch_pro.py"

echo "=== COLLABS RETOUCH PRO — LINUX INSTALLER ==="
echo "[*] Setting up directories..."

mkdir -p "$HOME/.local/share/applications"
mkdir -p "$HOME/.local/share/icons/hicolor/512x512/apps"
mkdir -p "$HOME/Desktop"
mkdir -p "$HOME/bin"

echo "[*] Installing branded RP icon..."
cp -f "$ICON_SRC" "$HOME/.local/share/icons/hicolor/512x512/apps/collabs-retouch-pro.png"

DESKTOP_ENTRY="$HOME/.local/share/applications/collabs-retouch-pro.desktop"

echo "[*] Creating desktop application entry..."
cat <<EOF > "$DESKTOP_ENTRY"
[Desktop Entry]
Version=1.0
Type=Application
Name=Collabs Retouch Pro
GenericName=AI Portrait & RAW Studio
Comment=What if Lightroom's sliders married Evoto's AI and had Luminar as a baby? You'd get Collabs Retouch Pro. And it's FREE.
Exec=python3 "$LAUNCHER"
Icon=$HOME/.local/share/icons/hicolor/512x512/apps/collabs-retouch-pro.png
Terminal=false
Categories=Graphics;Photography;RasterGraphics;2DGraphics;
StartupNotify=true
EOF

chmod +x "$DESKTOP_ENTRY"

# Also copy to desktop if Desktop directory exists
if [ -d "$HOME/Desktop" ]; then
    cp -f "$DESKTOP_ENTRY" "$HOME/Desktop/collabs-retouch-pro.desktop"
    chmod +x "$HOME/Desktop/collabs-retouch-pro.desktop"
    gio set "$HOME/Desktop/collabs-retouch-pro.desktop" metadata::trusted true 2>/dev/null || true
fi

# Create CLI helper in ~/bin
cat <<EOF > "$HOME/bin/collabs-retouch-pro"
#!/usr/bin/env bash
exec python3 "$LAUNCHER" "\$@"
EOF
chmod +x "$HOME/bin/collabs-retouch-pro"

echo "=== INSTALLATION SUCCESSFUL ==="
echo "[✓] Collabs Retouch Pro is installed and available in your application menu and desktop!"
