#!/usr/bin/env python3
"""
Collabs Retouch Pro — Master Branded Desktop Launcher
Mandatory 5-second branded splash screen before launching studio.
Created by Dave Groennesby (Collabs Software)
"""

import sys
import os
import time
import subprocess

BASE_DIR = os.path.dirname(os.path.abspath(__file__))
SPLASH_JPG = os.path.join(BASE_DIR, "splashscreens", "Collabs_Retouch_Pro_Splash_1_Editorial_Studio.jpg")
SPLASH_PNG = os.path.join(BASE_DIR, "splashscreens", "Collabs_Retouch_Pro_Splash_1_Editorial_Studio.png")
SPLASH_PATH = SPLASH_JPG if os.path.exists(SPLASH_JPG) else SPLASH_PNG
MIN_SPLASH_SECONDS = 5.0

def show_splash_and_launch():
    start_time = time.time()
    has_display = bool(os.environ.get("DISPLAY") or os.environ.get("WAYLAND_DISPLAY") or sys.platform == "darwin")

    if os.path.exists(SPLASH_PATH) and has_display:
        try:
            import tkinter as tk
            from PIL import Image, ImageTk

            root = tk.Tk()
            root.title("Collabs Retouch Pro")
            root.overrideredirect(True)
            root.attributes('-topmost', True)

            img = Image.open(SPLASH_PATH)
            sw = root.winfo_screenwidth()
            sh = root.winfo_screenheight()

            target_w = min(1100, int(sw * 0.75))
            target_h = int(target_w * (img.height / img.width))

            img_resized = img.resize((target_w, target_h), Image.Resampling.LANCZOS)
            photo = ImageTk.PhotoImage(img_resized)

            x = (sw - target_w) // 2
            y = (sh - target_h) // 2
            root.geometry(f"{target_w}x{target_h}+{x}+{y}")

            lbl = tk.Label(root, image=photo, borderwidth=0, highlightthickness=0)
            lbl.pack(fill=tk.BOTH, expand=True)

            def close_and_run():
                elapsed = time.time() - start_time
                if elapsed < MIN_SPLASH_SECONDS:
                    root.after(int((MIN_SPLASH_SECONDS - elapsed) * 1000), close_and_run)
                else:
                    root.destroy()
                    launch_engine()

            root.after(int(MIN_SPLASH_SECONDS * 1000), close_and_run)
            root.mainloop()
            return
        except Exception as e:
            print(f"[*] Tkinter splash notice ({e}), using timed fallback.")

    # Fallback delay
    remaining = MIN_SPLASH_SECONDS - (time.time() - start_time)
    if remaining > 0:
        time.sleep(remaining)
    launch_engine()

def launch_engine():
    server_script = os.path.join(BASE_DIR, "app", "server.py")
    subprocess.Popen([sys.executable, server_script])

if __name__ == "__main__":
    show_splash_and_launch()
