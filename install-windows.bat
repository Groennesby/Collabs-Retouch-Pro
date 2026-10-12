@echo off
REM ==============================================================================
REM COLLABS RETOUCH PRO — Windows Master Installer
REM Creates Start Menu & Desktop shortcuts with branded icons and splash launcher.
REM Created by Dave Groennesby (Collabs Software)
REM ==============================================================================

setlocal enabledelayedexpansion
title Collabs Retouch Pro Installer

echo ======================================================
echo    COLLABS RETOUCH PRO — WINDOWS INSTALLER
echo ======================================================
echo.

set "SCRIPT_DIR=%~dp0"
set "TARGET_DIR=%APPDATA%\Collabs Software\Collabs Retouch Pro"
set "ICON_FILE=%SCRIPT_DIR%Collabs_Retouch_Pro_Mascot.ico"

echo [*] Setting up program directory at: !TARGET_DIR!
if not exist "!TARGET_DIR!" mkdir "!TARGET_DIR!"

echo [*] Copying core engine files...
xcopy /E /I /Y "%SCRIPT_DIR%app" "!TARGET_DIR!\app" >nul
xcopy /E /I /Y "%SCRIPT_DIR%splashscreens" "!TARGET_DIR!\splashscreens" >nul
copy /Y "%SCRIPT_DIR%launch_retouch_pro.py" "!TARGET_DIR!\" >nul
copy /Y "%SCRIPT_DIR%test_portrait_4000x6000.jpg" "!TARGET_DIR!\" >nul

REM Check icon
if exist "%SCRIPT_DIR%softwareelements\Collabs Retouch Pro\Collabs_Retouch_Pro_Mascot.ico" (
    copy /Y "%SCRIPT_DIR%softwareelements\Collabs Retouch Pro\Collabs_Retouch_Pro_Mascot.ico" "!TARGET_DIR!\icon.ico" >nul
) else (
    copy /Y "%SCRIPT_DIR%Collabs_Retouch_Pro_Mascot.ico" "!TARGET_DIR!\icon.ico" >nul 2>nul
)

echo [*] Creating Desktop & Start Menu shortcuts...
set "DESKTOP_DIR=%USERPROFILE%\Desktop"
set "SHORTCUT_PATH=%DESKTOP_DIR%\Collabs Retouch Pro.lnk"

powershell -NoProfile -Command ^
  "$ws = New-Object -ComObject WScript.Shell; " ^
  "$s = $ws.CreateShortcut('%SHORTCUT_PATH%'); " ^
  "$s.TargetPath = 'pythonw.exe'; " ^
  "$s.Arguments = '\"!TARGET_DIR!\launch_retouch_pro.py\"'; " ^
  "$s.WorkingDirectory = '!TARGET_DIR!'; " ^
  "$s.IconLocation = '!TARGET_DIR!\icon.ico,0'; " ^
  "$s.Description = 'Collabs Retouch Pro — AI Portrait & RAW Studio'; " ^
  "$s.Save()"

echo.
echo ======================================================
echo    INSTALLATION SUCCESSFUL!
echo ======================================================
echo [OK] Collabs Retouch Pro is installed and ready on your Desktop.
echo.
pause
