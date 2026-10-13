<p align="center">
  <img src="docs/logo.png" alt="Retouch Pro" width="140" />
</p>

<h1 align="center">Retouch Pro</h1>

<p align="center">
  <b>Professional AI Portrait &amp; RAW Photo Editing</b><br/>
  Free · Offline · No credits · No subscription
</p>

<p align="center">
  <img src="docs/splash.jpg" alt="Retouch Pro splash screen" width="760" />
</p>

---

## ⬇️ Download

<p align="center">
  <a href="https://github.com/Groennesby/Collabs-Retouch-Pro/releases/latest"><b>➡️ Go to the latest release (all downloads)</b></a>
</p>

| System | Download this file | Notes |
|--------|--------------------|-------|
| 🪟 **Windows** 10 / 11 (64-bit) | **`Retouch Pro_…_x64-setup.exe`** (or the `.msi`) | Double-click to install. |
| 🍎 **macOS** 11 or newer | **`Retouch Pro_…_universal.dmg`** | One file for **Apple Silicon (M1–M4) and Intel**. Open it, drag *Retouch Pro* to *Applications*. |
| 🐧 **Linux** | **`.AppImage`** (any distro) · **`.deb`** (Ubuntu/Debian) · **`.rpm`** (Fedora) | AppImage: `chmod +x` and run. |

> **First launch (the app is not code-signed):**
> macOS — right-click the app → **Open** (once). Windows — SmartScreen → **More info → Run anyway**.

### Or install from the terminal (one command)

**macOS / Linux**
```bash
curl -fsSL https://raw.githubusercontent.com/Groennesby/Collabs-Retouch-Pro/main/install.sh | bash
```

**Windows** (PowerShell)
```powershell
irm https://raw.githubusercontent.com/Groennesby/Collabs-Retouch-Pro/main/install.ps1 | iex
```

These download the newest installer from the [Releases](https://github.com/Groennesby/Collabs-Retouch-Pro/releases) page for your system.
If the Releases page is still empty, the installers are still being built — check the
[Actions tab](https://github.com/Groennesby/Collabs-Retouch-Pro/actions) (takes about 15 minutes after a new version tag).

## ✨ Features

- 148 Lightroom-style sliders: tone, white balance, HSL, curves, colour grading, black &amp; white mixer, sharpening, lens corrections, grain, vignette
- **AI Denoise** that works on both **RAW and JPG**
- AI portrait retouching: skin, eyes, teeth, makeup, hair, face &amp; body reshape (478-point face mesh)
- Split before/after view, filmstrip, batch-friendly workflow
- 100 % offline — your photos never leave your computer

## 🛠 For developers

```bash
npm install
npm run tauri dev       # run the app
npm run tauri build     # installer for the OS you are on
cargo test --manifest-path src-tauri/Cargo.toml -- --nocapture   # slider tests
```

Installers for all three systems are built automatically by GitHub Actions
(`.github/workflows/release.yml`) when a version tag is pushed:

```bash
git tag v1.0.1 && git push origin v1.0.1
```

---

<p align="center">
  Made by <b>Dave Groennesby</b> · <b>Collabs Software</b><br/>
  © 2026 Collabs Software. All rights reserved.
</p>
