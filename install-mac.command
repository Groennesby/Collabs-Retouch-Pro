#!/bin/zsh
# ==============================================================================
# COLLABS RETOUCH PRO — macOS Native Application Installer & Launcher
# Works dynamically across Apple Silicon (/opt/homebrew) & Intel (/usr/local)
# Handles Conda, Homebrew, and Python.org setups without crashing
# Created by Dave Groennesby (Collabs Software)
# ==============================================================================

set -e

DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$DIR"

chmod +x "$0" 2>/dev/null || true

echo "=== COLLABS RETOUCH PRO — MACOS LAUNCHER & INSTALLER ==="

# 1. Search for a working Python with Tkinter
WORKING_PYTHON=""

check_python() {
    local py="$1"
    if [ -x "$py" ]; then
        if "$py" -c "import tkinter; import tkinter.ttk; print('OK')" >/dev/null 2>&1; then
            WORKING_PYTHON="$py"
            return 0
        fi
    fi
    return 1
}

# Check active Python first
if command -v python3 >/dev/null 2>&1; then
    check_python "$(which python3)" || true
fi

# Check CONDA_PREFIX
if [ -z "$WORKING_PYTHON" ] && [ -n "$CONDA_PREFIX" ]; then
    check_python "$CONDA_PREFIX/bin/python3" || true
fi

# Check Apple Silicon Homebrew
if [ -z "$WORKING_PYTHON" ]; then
    for py in /opt/homebrew/bin/python3.12 /opt/homebrew/bin/python3.11 /opt/homebrew/bin/python3.10 /opt/homebrew/bin/python3; do
        if check_python "$py"; then break; fi
    done
fi

# Check Intel Homebrew
if [ -z "$WORKING_PYTHON" ]; then
    for py in /usr/local/bin/python3.12 /usr/local/bin/python3.11 /usr/local/bin/python3.10 /usr/local/bin/python3; do
        if check_python "$py"; then break; fi
    done
fi

# Check Official Python.org frameworks
if [ -z "$WORKING_PYTHON" ]; then
    for py in /Library/Frameworks/Python.framework/Versions/*/bin/python3; do
        if check_python "$py"; then break; fi
    done
fi

# 2. If no working Python found, prompt user and assist
if [ -z "$WORKING_PYTHON" ]; then
    echo "[!] Tkinter not found in your default Python environment."
    echo "[*] Attempting auto-fix via Homebrew or Conda..."
    
    if command -v brew >/dev/null 2>&1; then
        echo "[*] Homebrew detected. Installing tcl-tk and python-tk..."
        brew install tcl-tk python-tk@3.12 || brew install python-tk@3.11 || true
        for py in /opt/homebrew/bin/python3.12 /usr/local/bin/python3.12 /opt/homebrew/bin/python3; do
            if check_python "$py"; then break; fi
        done
    elif command -v conda >/dev/null 2>&1; then
        echo "[*] Conda detected. Installing tk from conda-forge..."
        conda install -y -c conda-forge tk || true
        check_python "$(which python3)" || true
    fi
fi

# If STILL not found, show native AppleScript dialog
if [ -z "$WORKING_PYTHON" ]; then
    osascript -e 'display dialog "Collabs Retouch Pro requires Python with Tcl/Tk support.\n\nRun in Terminal:\nbrew install tcl-tk python-tk@3.12\n\nOr download official Python from python.org" with title "Collabs Retouch Pro" buttons {"OK"} default button "OK" with icon stop'
    exit 1
fi

echo "[✓] Using Python: $WORKING_PYTHON"

# Ensure Pillow is installed in the working Python environment
"$WORKING_PYTHON" -m pip install --quiet pillow || true

# 3. Create or Update Native .app Bundle in ~/Applications or /Applications
TARGET_DIR="/Applications"
if [ ! -w "$TARGET_DIR" ]; then
    TARGET_DIR="$HOME/Applications"
    mkdir -p "$TARGET_DIR"
fi

APP_BUNDLE="$TARGET_DIR/Collabs Retouch Pro.app"
mkdir -p "$APP_BUNDLE/Contents/MacOS"
mkdir -p "$APP_BUNDLE/Contents/Resources"

# Info.plist
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

# Copy Icon
if [ -f "$DIR/icon.icns" ]; then
    cp -f "$DIR/icon.icns" "$APP_BUNDLE/Contents/Resources/icon.icns"
fi

# Copy Payload
cp -r "$DIR/src" "$APP_BUNDLE/Contents/Resources/"
cp -f "$DIR/launcher.py" "$APP_BUNDLE/Contents/Resources/"

# Launcher script inside .app
cat <<EOF > "$APP_BUNDLE/Contents/MacOS/launch"
#!/bin/zsh
RESOURCES_DIR="\$(cd "\$(dirname "\$0")/../Resources" && pwd)"
export PYTHONPATH="\$RESOURCES_DIR/src:\$RESOURCES_DIR"
exec "$WORKING_PYTHON" "\$RESOURCES_DIR/launcher.py" "\$@"
EOF
chmod +x "$APP_BUNDLE/Contents/MacOS/launch"

echo "[✓] Collabs Retouch Pro successfully installed at: $APP_BUNDLE"
echo "[*] Launching Collabs Retouch Pro..."
exec "$WORKING_PYTHON" "$DIR/launcher.py"
