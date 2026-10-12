#!/usr/bin/env bash
# ==============================================================================
# COLLABS RETOUCH PRO — macOS Native Application Installer
# Builds and installs native Collabs Retouch Pro.app with splashscreen & icons.
# Created by Dave Groennesby (Collabs Software)
# ==============================================================================

set -e

DIR="$(cd "$(dirname "$0")" && pwd)"
TARGET_DIR="/Applications"

if [ ! -w "$TARGET_DIR" ]; then
    TARGET_DIR="$HOME/Applications"
    mkdir -p "$TARGET_DIR"
fi

APP_BUNDLE="$TARGET_DIR/Collabs Retouch Pro.app"
echo "=== COLLABS RETOUCH PRO — MACOS INSTALLER ==="

# 1. Install dependencies
echo "[*] Checking Python and required libraries (Pillow)..."
python3 -m pip install --quiet pillow || python -m pip install --quiet pillow || true

echo "[*] Creating macOS Application bundle at: $APP_BUNDLE"

rm -rf "$APP_BUNDLE"
mkdir -p "$APP_BUNDLE/Contents/MacOS"
mkdir -p "$APP_BUNDLE/Contents/Resources"

# 1. Info.plist
cat <<EOF > "$APP_BUNDLE/Contents/Info.plist"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>English</string>
    <key>CFBundleDisplayName</key>
    <string>Collabs Retouch Pro</string>
    <key>CFBundleExecutable</key>
    <string>launch</string>
    <key>CFBundleIdentifier</key>
    <string>com.collabssoftware.retouchpro</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>Collabs Retouch Pro</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    <key>CFBundleIconFile</key>
    <string>icon.icns</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
EOF

# 2. Copy Icons & Splash
if [ -f "$DIR/icon.icns" ]; then
    cp -f "$DIR/icon.icns" "$APP_BUNDLE/Contents/Resources/icon.icns"
elif [ -f "$DIR/softwareelements/Collabs Retouch Pro/Collabs_Retouch_Pro_Mascot.icns" ]; then
    cp -f "$DIR/softwareelements/Collabs Retouch Pro/Collabs_Retouch_Pro_Mascot.icns" "$APP_BUNDLE/Contents/Resources/icon.icns"
fi

if [ -f "$DIR/Finalsplash.jpg" ]; then
    cp -f "$DIR/Finalsplash.jpg" "$APP_BUNDLE/Contents/Resources/splash.jpg"
elif [ -f "$DIR/splashscreens/Finalsplash.jpg" ]; then
    cp -f "$DIR/splashscreens/Finalsplash.jpg" "$APP_BUNDLE/Contents/Resources/splash.jpg"
fi

# 3. Copy Application Payload
cp -r "$DIR/app" "$APP_BUNDLE/Contents/Resources/"
cp -r "$DIR/src" "$APP_BUNDLE/Contents/Resources/"
cp -f "$DIR/test_portrait_4000x6000.jpg" "$APP_BUNDLE/Contents/Resources/" 2>/dev/null || true

# 4. Create MacOS/launch runner
cat <<'EOF' > "$APP_BUNDLE/Contents/MacOS/launch"
#!/usr/bin/env bash
RESOURCES_DIR="$(cd "$(dirname "$0")/../Resources" && pwd)"
export PYTHONPATH="$RESOURCES_DIR"

if command -v python3 >/dev/null 2>&1; then
    python3 "$RESOURCES_DIR/src/app.py"
elif command -v python >/dev/null 2>&1; then
    python "$RESOURCES_DIR/src/app.py"
else
    echo "Error: Python not found"
fi
EOF
chmod +x "$APP_BUNDLE/Contents/MacOS/launch"

echo "=== INSTALLATION COMPLETE ==="
echo "[✓] Collabs Retouch Pro.app has been installed to $APP_BUNDLE"
echo "[*] Launching Collabs Retouch Pro..."
open "$APP_BUNDLE" 2>/dev/null || python3 "$APP_BUNDLE/Contents/Resources/src/app.py"
