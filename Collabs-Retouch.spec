# -*- mode: python ; coding: utf-8 -*-
import os
import sys
from PyInstaller.utils.hooks import collect_data_files

block_cipher = None

# Collect Tkinter and Pillow assets
datas = [
    ('src', 'src'),
    ('Finalsplash.jpg', '.'),
]

if os.path.exists('icon.png'):
    datas.append(('icon.png', '.'))

# Collect system Tcl/Tk data files if available
try:
    datas += collect_data_files('tkinter')
except Exception:
    pass

binaries = []

# If on macOS, scan Homebrew and Frameworks for Tcl/Tk dylibs
if sys.platform == 'darwin':
    possible_tk_dirs = [
        '/opt/homebrew/opt/tcl-tk/lib',
        '/usr/local/opt/tcl-tk/lib',
        '/Library/Frameworks/Tcl.framework/Versions/Current',
        '/Library/Frameworks/Tk.framework/Versions/Current',
    ]
    for tk_dir in possible_tk_dirs:
        if os.path.isdir(tk_dir):
            for f in os.listdir(tk_dir):
                if f.endswith('.dylib') or f.endswith('.framework'):
                    binaries.append((os.path.join(tk_dir, f), '.'))

a = Analysis(
    ['launcher.py'],
    pathex=['.', 'src'],
    binaries=binaries,
    datas=datas,
    hiddenimports=[
        'tkinter',
        'tkinter.ttk',
        'tkinter.filedialog',
        'tkinter.messagebox',
        'PIL',
        'PIL.Image',
        'PIL.ImageTk',
        'PIL.ImageEnhance',
        'PIL.ImageFilter',
    ],
    hookspath=[],
    hooksconfig={},
    runtime_hooks=[],
    excludes=[],
    win_no_prefer_redirects=False,
    win_private_assemblies=False,
    cipher=block_cipher,
    noarchive=False,
)

pyz = PYZ(a.pure, a.zipped_data, cipher=block_cipher)

exe = EXE(
    pyz,
    a.scripts,
    a.binaries,
    a.zipfiles,
    a.datas,
    [],
    name='Collabs-Retouch',
    debug=False,
    bootloader_ignore_signals=False,
    strip=False,
    upx=True,
    upx_exclude=[],
    runtime_tmpdir=None,
    console=False,
    disable_windowed_traceback=False,
    argv_emulation=False,
    target_arch=None,
    codesign_identity=None,
    entitlements_file=None,
    icon='icon.icns' if os.path.exists('icon.icns') else None,
)

if sys.platform == 'darwin':
    app = BUNDLE(
        exe,
        name='Collabs-Retouch.app',
        icon='icon.icns' if os.path.exists('icon.icns') else None,
        bundle_identifier='com.collabssoftware.retouchpro',
        info_plist={
            'NSHighResolutionCapable': 'True',
            'CFBundleDisplayName': 'Collabs Retouch Pro',
            'CFBundleName': 'Collabs Retouch Pro',
        },
    )
