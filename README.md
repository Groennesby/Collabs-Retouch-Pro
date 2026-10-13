# Retouch Pro

Professional AI portrait & RAW photo editing — free, offline, no credits.
**Made by Dave Groennesby · Collabs Software** (dave.groennesby@hotmail.com)

## Installers (macOS · Windows · Linux)

Native installers must be built on their own operating system. The workflow in
`.github/workflows/release.yml` does that for you on GitHub for all three:

1. Create a (free) GitHub repository and push this folder to it.
2. Create a tag and push it:
   ```bash
   git tag v1.0.0 && git push origin v1.0.0
   ```
   (or run the workflow manually from the **Actions** tab → *Build Retouch Pro installers* → *Run workflow*)
3. After ~10–15 minutes the **Releases** page contains:
   | System  | Files |
   |---------|-------|
   | macOS   | `Retouch Pro_1.0.0_universal.dmg` (Apple Silicon + Intel) |
   | Windows | `Retouch Pro_1.0.0_x64-setup.exe` and `.msi` |
   | Linux   | `.AppImage`, `.deb`, `.rpm` |

The workflow first runs the slider tests (`cargo test`) so a broken build never ships.

## Local development

```bash
npm install
npm run tauri dev      # run the app
npm run tauri build    # installer for the OS you are on
cargo test --manifest-path src-tauri/Cargo.toml -- --nocapture   # slider tests
```

## First launch on an unsigned build

* **macOS:** right-click the app → *Open* (once), or *System Settings → Privacy & Security → Open Anyway*.
* **Windows:** SmartScreen → *More info* → *Run anyway*.

Removing these warnings requires paid code-signing certificates (Apple Developer / Windows).
