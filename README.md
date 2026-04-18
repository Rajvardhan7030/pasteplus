# PastePlus (Phase 1 MVP)

PastePlus is a lightweight, native-feeling clipboard manager for Linux. It stays in the background, keeping track of your text history so you can recall it instantly with a global hotkey.

## Features
- **Silent Background Daemon:** Watches your clipboard using `wl-paste` (Wayland) or `xclip` (X11).
- **Intelligent Deduplication:** Won't clutter your history with identical consecutive copies.
- **Global Popup:** Press `Ctrl+Shift+V` to summon the history list at any time.
- **Instant Paste:** Select an item to automatically paste it into your focused application.
- **SQLite Storage:** History is persisted locally at `~/.local/share/pasteplus/history.db`.

## System Requirements
You'll need these installed on your system for PastePlus to work properly:

- **Wayland users:** `wl-clipboard`
- **X11 users:** `xclip`
- **Auto-Paste support:** `libxdo-dev` (Debian/Ubuntu) or `libxdo` (Arch)

## Getting Started
1. **Install dependencies:** `npm install`
   **linux-dependancy:**
   for ubuntu debian
   ```bash
   sudo apt install libsoup2.4-dev libwebkit2gtk-4.0-dev build-essential
     curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
     wl-clipboard xclip libxdo-dev
   ```
2. **Development:** `npm run tauri dev`
3. **Build:** `npm run tauri build`

## Usage
- **Ctrl+Shift+V**: Open the clipboard history popup.
- **Click an item**: Paste it into the last active window.
- **System Tray**: Use the icon to manually open the history or quit the app.
