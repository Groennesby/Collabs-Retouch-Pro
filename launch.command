#!/bin/bash
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$DIR"
echo "Starting Collabs Retouch Pro (macOS Native App)..."
python3 src/app.py
