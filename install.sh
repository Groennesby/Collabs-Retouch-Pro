#!/usr/bin/env bash
# Retouch Pro installer for macOS and Linux — downloads the newest release from GitHub.
#   curl -fsSL https://raw.githubusercontent.com/Groennesby/Collabs-Retouch-Pro/main/install.sh | bash
set -euo pipefail

REPO="Groennesby/Collabs-Retouch-Pro"
API="https://api.github.com/repos/$REPO/releases/latest"

urls="$(curl -fsSL "$API" | grep -o '"browser_download_url": *"[^"]*"' | cut -d'"' -f4 || true)"
if [ -z "$urls" ]; then
  echo "No release found yet. Check https://github.com/$REPO/releases (the build may still be running)." >&2
  exit 1
fi
pick() { echo "$urls" | grep -iE "$1" | head -n1 || true; }

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
os="$(uname -s)"

case "$os" in
  Darwin)
    url="$(pick '\.dmg$')"
    [ -n "$url" ] || { echo "No .dmg in the latest release." >&2; exit 1; }
    echo "Downloading $url"
    curl -fL --progress-bar "$url" -o "$tmp/RetouchPro.dmg"
    mnt="$(hdiutil attach -nobrowse -readonly "$tmp/RetouchPro.dmg" | grep -o '/Volumes/.*' | head -n1)"
    app="$(find "$mnt" -maxdepth 1 -name '*.app' | head -n1)"
    echo "Installing to /Applications ..."
    rm -rf "/Applications/$(basename "$app")"
    cp -R "$app" /Applications/
    hdiutil detach -quiet "$mnt" || true
    xattr -dr com.apple.quarantine "/Applications/$(basename "$app")" 2>/dev/null || true
    echo "Done. Open 'Retouch Pro' from Applications."
    ;;
  Linux)
    url="$(pick '\.AppImage$')"
    if [ -n "$url" ]; then
      mkdir -p "$HOME/.local/bin"
      echo "Downloading $url"
      curl -fL --progress-bar "$url" -o "$HOME/.local/bin/retouch-pro"
      chmod +x "$HOME/.local/bin/retouch-pro"
      echo "Done. Run: retouch-pro   (make sure ~/.local/bin is in your PATH)"
    else
      url="$(pick '\.deb$')"
      [ -n "$url" ] || { echo "No Linux package in the latest release." >&2; exit 1; }
      curl -fL --progress-bar "$url" -o "$tmp/retouch-pro.deb"
      sudo dpkg -i "$tmp/retouch-pro.deb" || sudo apt-get install -f -y
      echo "Done. Start 'Retouch Pro' from your applications menu."
    fi
    ;;
  *)
    echo "Unsupported system: $os. On Windows use install.ps1 (see README)." >&2
    exit 1
    ;;
esac
