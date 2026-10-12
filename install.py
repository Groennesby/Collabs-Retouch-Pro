#!/usr/bin/env python3
"""
Collabs Retouch Pro - Universal One-Line Installer
Supports macOS, Linux, and Windows (via PowerShell / WSL or Python).
"""

import os
import sys
import platform
import subprocess
import urllib.request
import zipfile
import tarfile
import tempfile

REPO_URL = "https://github.com/Groennesby/Collabs-Retouch-Pro.git"
ZIP_URL = "https://github.com/Groennesby/Collabs-Retouch-Pro/archive/refs/heads/main.zip"

def print_banner():
    print("=" * 60)
    print("       COLLABS RETOUCH PRO - UNIVERSAL INSTALLER")
    print("=" * 60)

def install_macos():
    print("[*] Installing Collabs Retouch Pro for macOS...")
    # Make sure python3 and pip/pillow are installed
    os.system("python3 -m pip install --upgrade pip pillow")
    
    # Clone or download repo to ~/Applications/Collabs Retouch Pro
    app_dir = os.path.expanduser("~/Applications/Collabs Retouch Pro")
    os.makedirs(app_dir, exist_ok=True)
    
    if os.path.exists(os.path.join(app_dir, ".git")):
        os.system(f"cd '{app_dir}' && git pull")
    else:
        os.system(f"git clone {REPO_URL} '{app_dir}' || true")
        
    # Build macOS .app bundle
    make_script = os.path.join(app_dir, "make_macos_app.py")
    if os.path.exists(make_script):
        os.system(f"python3 '{make_script}'")
    
    print("[+] macOS installation complete! You can find 'Collabs Retouch Pro.app' in ~/Applications/")

def install_linux():
    print("[*] Installing Collabs Retouch Pro for Linux...")
    os.system("python3 -m pip install --upgrade pip pillow")
    
    app_dir = os.path.expanduser("~/Applications/Collabs Retouch Pro")
    os.makedirs(app_dir, exist_ok=True)
    
    if os.path.exists(os.path.join(app_dir, ".git")):
        os.system(f"cd '{app_dir}' && git pull")
    else:
        os.system(f"git clone {REPO_URL} '{app_dir}' || true")
        
    # Make launch.sh executable
    launch_sh = os.path.join(app_dir, "launch.sh")
    if os.path.exists(launch_sh):
        os.chmod(launch_sh, 0o755)
        
    print(f"[+] Linux installation complete! Run via: {launch_sh}")

def install_windows():
    print("[*] Installing Collabs Retouch Pro for Windows...")
    app_dir = os.path.expanduser("~/AppData/Local/CollabsRetouchPro")
    os.makedirs(app_dir, exist_ok=True)
    
    os.system(f"git clone {REPO_URL} \"{app_dir}\" 2>nul || cd /d \"{app_dir}\" && git pull")
    os.system("python -m pip install pillow")
    print(f"[+] Windows installation complete! Launch via batch file in {app_dir}")

def main():
    print_banner()
    system = platform.system()
    print(f"Detected OS: {system}")
    
    if system == "Darwin":
        install_macos()
    elif system == "Linux":
        install_linux()
    elif system == "Windows" or "CYGWIN" in system or "MINGW" in system:
        install_windows()
    else:
        print(f"[-] Unsupported OS: {system}. Attempting generic python install...")
        install_linux()

if __name__ == "__main__":
    main()
