import sys
import os
import subprocess
import shutil

def find_working_python():
    """
    Check various Python candidates to find one that has a working tkinter.
    """
    candidates = []
    
    # 1. Current sys.executable
    if sys.executable:
        candidates.append(sys.executable)
        
    # 2. Check CONDA_PREFIX if in conda
    conda_prefix = os.environ.get('CONDA_PREFIX')
    if conda_prefix:
        candidates.append(os.path.join(conda_prefix, 'bin', 'python3'))
        candidates.append(os.path.join(conda_prefix, 'bin', 'python'))
        candidates.append(os.path.join(conda_prefix, 'python.exe'))

    # 3. macOS standard locations
    if sys.platform == 'darwin':
        candidates.extend([
            '/opt/homebrew/bin/python3',
            '/opt/homebrew/bin/python3.12',
            '/opt/homebrew/bin/python3.11',
            '/opt/homebrew/bin/python3.10',
            '/usr/local/bin/python3',
            '/usr/local/bin/python3.12',
            '/usr/local/bin/python3.11',
            '/Library/Frameworks/Python.framework/Versions/3.12/bin/python3',
            '/Library/Frameworks/Python.framework/Versions/3.11/bin/python3',
            '/Library/Frameworks/Python.framework/Versions/3.10/bin/python3',
            '/System/Library/Frameworks/Python.framework/Versions/Current/bin/python3',
            '/usr/bin/python3'
        ])
        
    # 4. Standard path binaries
    for cmd in ['python3', 'python']:
        p = shutil.which(cmd)
        if p and p not in candidates:
            candidates.append(p)
            
    # Test each candidate for working tkinter
    checked = set()
    for py in candidates:
        if not py or py in checked:
            continue
        checked.add(py)
        if not (os.path.isfile(py) or shutil.which(py)):
            continue
            
        try:
            res = subprocess.run(
                [py, "-c", "import tkinter; import tkinter.ttk; print('OK')"],
                capture_output=True,
                text=True,
                timeout=5
            )
            if res.returncode == 0 and "OK" in res.stdout:
                return py
        except Exception:
            pass
            
    return None

def show_native_error(title, message):
    """
    Display a native OS dialog without relying on Tkinter.
    """
    if sys.platform == 'darwin':
        apple_script = f'''
        display dialog "{message}" with title "{title}" buttons {{"Copy Command & Open Guide", "OK"}} default button "OK" with icon stop
        '''
        try:
            proc = subprocess.run(['osascript', '-e', apple_script], capture_output=True, text=True)
            if "Copy Command" in proc.stdout:
                import webbrowser
                webbrowser.open("https://www.python.org/downloads/macos/")
        except Exception:
            print(f"\n[{title}]\n{message}\n")
    elif sys.platform == 'win32':
        try:
            import ctypes
            ctypes.windll.user32.MessageBoxW(0, message, title, 0x10)
        except Exception:
            print(f"\n[{title}]\n{message}\n")
    else:
        # Linux zenity or kdialog fallback
        if shutil.which('zenity'):
            subprocess.run(['zenity', '--error', f'--title={title}', f'--text={message}'])
        elif shutil.which('kdialog'):
            subprocess.run(['kdialog', '--error', message, f'--title={title}'])
        else:
            print(f"\n[{title}]\n{message}\n")

def main():
    base_dir = os.path.dirname(os.path.abspath(__file__))
    app_dir = os.path.join(base_dir, 'app')

    # 1. First priority: High-Performance Studio Engine via pywebview
    try:
        import webview
        sys.path.insert(0, app_dir)
        import server as studio_server
        srv = studio_server.run_server(1420)
        import threading
        t = threading.Thread(target=srv.serve_forever, daemon=True)
        t.start()

        window = webview.create_window(
            'Collabs Retouch Pro — AI Portrait & RAW Studio',
            'http://127.0.0.1:1420/index.html',
            width=1440,
            height=920,
            min_size=(1024, 700),
            background_color='#0E0E10'
        )
        webview.start()
        return
    except ImportError:
        pass
    except Exception as e:
        print(f"[*] Webview note: {e}")

    # 2. Second priority: Standard Tkinter Native fallback
    app_script = os.path.join(base_dir, 'src', 'app.py')
    try:
        import tkinter
        target_python = sys.executable
    except ImportError:
        target_python = find_working_python()

    if not target_python:
        err_msg = (
            "Collabs Retouch Pro requires GUI support.\n\n"
            "Quick Fix for macOS:\n"
            "• pip install pywebview\n"
            "• or: brew install tcl-tk python-tk@3.12"
        )
        show_native_error("Collabs Retouch Pro - Setup", err_msg)
        sys.exit(1)

    os.environ['PYTHONPATH'] = os.path.join(base_dir, 'src')
    cmd = [target_python, app_script] + sys.argv[1:]
    os.execv(target_python, cmd)

if __name__ == '__main__':
    main()
