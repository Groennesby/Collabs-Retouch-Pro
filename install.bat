@echo off
TITLE Collabs Retouch Pro Installer
echo ==================================================
echo    COLLABS RETOUCH PRO - WINDOWS INSTALLER
echo ==================================================
python --version >nul 2>&1
if %errorlevel% neq 0 (
    echo Python is required but not found. Please install Python from python.org.
    pause
    exit /b 1
)
python -m pip install --upgrade pip pillow
git --version >nul 2>&1
if %errorlevel% neq 0 (
    echo Git is required. Please install Git for Windows.
    pause
    exit /b 1
)
python -c "import urllib.request; exec(urllib.request.urlopen('https://raw.githubusercontent.com/Groennesby/Collabs-Retouch-Pro/main/install.py').read().decode())"
echo Installation complete!
pause
