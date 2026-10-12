import os

app_name = "Collabs Retouch Pro"
app_dir = f"/home/dave/Applications/{app_name}.app"
contents_dir = os.path.join(app_dir, "Contents")
macos_dir = os.path.join(contents_dir, "MacOS")
resources_dir = os.path.join(contents_dir, "Resources")

os.makedirs(macos_dir, exist_ok=True)
os.makedirs(resources_dir, exist_ok=True)

# 1. Info.plist
plist_content = f"""<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>runner</string>
    <key>CFBundleIdentifier</key>
    <string>com.collabs.retouchpro</string>
    <key>CFBundleName</key>
    <string>{app_name}</string>
    <key>CFBundleDisplayName</key>
    <string>{app_name}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
</dict>
</plist>
"""
with open(os.path.join(contents_dir, "Info.plist"), "w") as f:
    f.write(plist_content)

# 2. Runner script
runner_content = """#!/bin/bash
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
APP_ROOT="$DIR/../.."
cd "$APP_ROOT"
python3 src/app.py
"""
runner_path = os.path.join(macos_dir, "runner")
with open(runner_path, "w") as f:
    f.write(runner_content)

os.chmod(runner_path, 0o755)
os.system(f"xattr -cr '{app_dir}' 2>/dev/null || true")
os.system(f"codesign --force --deep --sign - '{app_dir}' 2>/dev/null || true")
print(f"macOS .app bundle successfully created at {app_dir}")
