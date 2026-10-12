@echo off
TITLE Collabs Retouch Pro Installer
echo ==================================================
echo    COLLABS RETOUCH PRO - WINDOWS INSTALLER
echo ==================================================

set WORKING_PYTHON=

where python >nul 2>&1
if %errorlevel% equ 0 (
    python -c "import tkinter, tkinter.ttk; print('OK')" >nul 2>&1
    if %errorlevel% equ 0 (
        set WORKING_PYTHON=python
    )
)

if "%WORKING_PYTHON%"=="" (
    where py >nul 2>&1
    if %errorlevel% equ 0 (
        py -3 -c "import tkinter, tkinter.ttk; print('OK')" >nul 2>&1
        if %errorlevel% equ 0 (
            set WORKING_PYTHON=py -3
        )
    )
)

if "%WORKING_PYTHON%"=="" (
    echo [ERROR] No working Python installation with Tkinter found!
    echo Please install Python 3.10+ from https://www.python.org/downloads/
    echo Make sure to check "tcl/tk and IDLE" during installation.
    pause
    exit /b 1
)

echo [✓] Using Python: %WORKING_PYTHON%
echo [*] Checking required dependencies (Pillow)...
%WORKING_PYTHON% -m pip install --upgrade pip pillow

echo [*] Launching Collabs Retouch Pro...
%WORKING_PYTHON% launcher.py
pause
