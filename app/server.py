#!/usr/bin/env python3
"""
Collabs Retouch Pro — Local High-Performance Studio Server & Launcher
Created by Dave Groennesby (Collabs Software)
"""

import os
import sys
import webbrowser
import threading
import time
from http.server import HTTPServer, SimpleHTTPRequestHandler

PORT = 1420
APP_DIR = os.path.dirname(os.path.abspath(__file__))

class StudioHandler(SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=APP_DIR, **kwargs)

    def end_headers(self):
        # Enable Cross-Origin Isolation for SharedArrayBuffer & WASM SIMD performance
        self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
        self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
        self.send_header('Cache-Control', 'no-cache, no-store, must-revalidate')
        super().end_headers()

def run_server(port=PORT):
    server = HTTPServer(('127.0.0.1', port), StudioHandler)
    print(f"[*] Collabs Retouch Pro Studio Server running at http://127.0.0.1:{port}/")
    return server

def main():
    server = run_server(PORT)
    if "--no-browser" not in sys.argv:
        def open_browser():
            time.sleep(0.5)
            webbrowser.open(f"http://127.0.0.1:{PORT}/index.html")
        threading.Thread(target=open_browser, daemon=True).start()

    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\n[*] Collabs Retouch Pro closed cleanly.")

if __name__ == "__main__":
    main()
