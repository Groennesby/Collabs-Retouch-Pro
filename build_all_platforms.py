#!/usr/bin/env python3
"""
Collabs Retouch Pro — Standalone Cross-Platform Packager
Packages pure standalone desktop distributions for Linux, macOS (Apple Silicon + Intel), and Windows.
Incorporates:
  - Official Icon: Collabs_Retouch_Pro_RP_Icon_gold_rim.png (.ico, .icns, .png)
  - Splash Screen: Finalsplash.jpg
  - Fused Lightcraft RAW + Evoto AI Portrait Retouch Studio Engine
"""

import os
import sys
import shutil
import zipfile
import plistlib

BASE_DIR = '/home/dave/Collabs GIT Software/collabs-retouch-pro'
DIST_DIR = os.path.join(BASE_DIR, 'dist')
CACHE_DIR = '/home/dave/.cache/electron-packages'
LINUX_CACHE = '/home/dave/.cache/electron/5a625dfd3df3efc3971556afe64989f9aa6ea0f4cd6e33889c1951789041498f/electron-v44.7.0-linux-x64.zip'

VERSION = 'v44.7.0'
APP_NAME = 'Collabs Retouch Pro'

def copy_app_source(target_app_dir):
    os.makedirs(target_app_dir, exist_ok=True)
    # Copy essential files
    shutil.copy(os.path.join(BASE_DIR, 'package.json'), target_app_dir)
    shutil.copy(os.path.join(BASE_DIR, 'main.js'), target_app_dir)
    shutil.copy(os.path.join(BASE_DIR, 'Finalsplash.jpg'), target_app_dir)
    shutil.copy(os.path.join(BASE_DIR, 'icon.png'), target_app_dir)
    shutil.copy(os.path.join(BASE_DIR, 'icon.ico'), target_app_dir)
    shutil.copy(os.path.join(BASE_DIR, 'icon.icns'), target_app_dir)
    
    app_folder = os.path.join(target_app_dir, 'app')
    if os.path.exists(app_folder):
        shutil.rmtree(app_folder)
    shutil.copytree(os.path.join(BASE_DIR, 'app'), app_folder)

def build_linux():
    print("Building Linux x64 Standalone...")
    out_dir = os.path.join(DIST_DIR, 'Collabs-Retouch-Pro-Linux-x64')
    if os.path.exists(out_dir):
        shutil.rmtree(out_dir)
    os.makedirs(out_dir, exist_ok=True)

    with zipfile.ZipFile(LINUX_CACHE, 'r') as z:
        z.extractall(out_dir)

    # Executable permissions & rename
    electron_bin = os.path.join(out_dir, 'electron')
    app_bin = os.path.join(out_dir, 'collabs-retouch-pro')
    if os.path.exists(electron_bin):
        os.chmod(electron_bin, 0o755)
        shutil.copy(electron_bin, app_bin)
        os.chmod(app_bin, 0o755)

    # Copy app
    copy_app_source(os.path.join(out_dir, 'resources', 'app'))

    # Create desktop entry
    desktop_file = os.path.join(out_dir, 'collabs-retouch-pro.desktop')
    with open(desktop_file, 'w') as f:
        f.write(f"""[Desktop Entry]
Name=Collabs Retouch Pro
Comment=AI Portrait & RAW Retouch Studio
Exec={os.path.abspath(app_bin)} %U
Icon={os.path.abspath(os.path.join(out_dir, 'resources', 'app', 'icon.png'))}
Terminal=false
Type=Application
Categories=Graphics;Photography;
StartupWMClass=collabs-retouch-pro
""")
    os.chmod(desktop_file, 0o755)

    # Create a quick launcher shell script
    launcher_sh = os.path.join(out_dir, 'run.sh')
    with open(launcher_sh, 'w') as f:
        f.write("""#!/usr/bin/env bash
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
"$DIR/collabs-retouch-pro" --no-sandbox "$@"
""")
    os.chmod(launcher_sh, 0o755)
    print("  -> Linux package ready at:", out_dir)

def build_windows():
    print("Building Windows x64 Standalone...")
    win_zip = os.path.join(CACHE_DIR, f'electron-{VERSION}-win32-x64.zip')
    out_dir = os.path.join(DIST_DIR, 'Collabs-Retouch-Pro-Windows-x64')
    if os.path.exists(out_dir):
        shutil.rmtree(out_dir)
    os.makedirs(out_dir, exist_ok=True)

    with zipfile.ZipFile(win_zip, 'r') as z:
        z.extractall(out_dir)

    # Rename electron.exe to Collabs Retouch Pro.exe
    electron_exe = os.path.join(out_dir, 'electron.exe')
    app_exe = os.path.join(out_dir, 'Collabs Retouch Pro.exe')
    if os.path.exists(electron_exe):
        shutil.move(electron_exe, app_exe)

    # Copy app source
    copy_app_source(os.path.join(out_dir, 'resources', 'app'))

    # Copy icon to root
    shutil.copy(os.path.join(BASE_DIR, 'icon.ico'), os.path.join(out_dir, 'icon.ico'))

    # Create run.bat
    bat_file = os.path.join(out_dir, 'Launch Collabs Retouch Pro.bat')
    with open(bat_file, 'w') as f:
        f.write('@echo off\r\nstart "" "%~dp0Collabs Retouch Pro.exe"\r\n')

    print("  -> Windows package ready at:", out_dir)

def build_mac(arch):
    print(f"Building macOS {arch} Standalone (.app)...")
    mac_zip = os.path.join(CACHE_DIR, f'electron-{VERSION}-darwin-{arch}.zip')
    out_dir = os.path.join(DIST_DIR, f'Collabs-Retouch-Pro-macOS-{arch}')
    if os.path.exists(out_dir):
        shutil.rmtree(out_dir)
    os.makedirs(out_dir, exist_ok=True)

    with zipfile.ZipFile(mac_zip, 'r') as z:
        z.extractall(out_dir)

    # Electron.app exists in out_dir
    electron_app = os.path.join(out_dir, 'Electron.app')
    final_app = os.path.join(out_dir, 'Collabs Retouch Pro.app')
    if os.path.exists(electron_app):
        shutil.move(electron_app, final_app)

    # Contents
    contents_dir = os.path.join(final_app, 'Contents')
    macos_dir = os.path.join(contents_dir, 'MacOS')
    resources_dir = os.path.join(contents_dir, 'Resources')

    # Rename executable
    electron_bin = os.path.join(macos_dir, 'Electron')
    app_bin = os.path.join(macos_dir, 'Collabs Retouch Pro')
    if os.path.exists(electron_bin):
        shutil.move(electron_bin, app_bin)
        os.chmod(app_bin, 0o755)

    # Update Info.plist
    plist_path = os.path.join(contents_dir, 'Info.plist')
    if os.path.exists(plist_path):
        try:
            with open(plist_path, 'rb') as fp:
                pl = plistlib.load(fp)
            pl['CFBundleDisplayName'] = 'Collabs Retouch Pro'
            pl['CFBundleName'] = 'Collabs Retouch Pro'
            pl['CFBundleExecutable'] = 'Collabs Retouch Pro'
            pl['CFBundleIdentifier'] = 'com.collabs.retouchpro'
            pl['CFBundleIconFile'] = 'icon.icns'
            with open(plist_path, 'wb') as fp:
                plistlib.dump(pl, fp)
        except Exception as e:
            print("  Warning: could not patch Info.plist:", e)

    # Replace icon.icns
    target_icns = os.path.join(resources_dir, 'icon.icns')
    shutil.copy(os.path.join(BASE_DIR, 'icon.icns'), target_icns)
    if os.path.exists(os.path.join(resources_dir, 'electron.icns')):
        shutil.copy(os.path.join(BASE_DIR, 'icon.icns'), os.path.join(resources_dir, 'electron.icns'))

    # Copy app source into Resources/app
    copy_app_source(os.path.join(resources_dir, 'app'))

    print(f"  -> macOS {arch} package ready at:", final_app)

def main():
    os.makedirs(DIST_DIR, exist_ok=True)
    build_linux()
    build_windows()
    build_mac('arm64')
    build_mac('x64')
    print("\nALL PLATFORM STANDALONE DESKTOP APPS BUILT SUCCESSFULLY!")

if __name__ == '__main__':
    main()