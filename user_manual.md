# PastePlus User Manual

Welcome to **PastePlus**, your lightweight, native clipboard manager for Linux. This guide covers installation, daily usage, and common troubleshooting steps.

---

## 1. Installation Guide

PastePlus is built with **Tauri** and **React**. To compile and run it from source on Linux (Ubuntu/Debian), follow these steps:

### Prerequisites
1. **Rust:** Install via [rustup.rs](https://rustup.rs/):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. **Node.js:** Recommended version 18 or higher.
3. **System Dependencies:** PastePlus requires several system libraries for the GUI and clipboard interaction.
   ```bash
   sudo apt update
   sudo apt install -y libsoup2.4-dev libwebkit2gtk-4.0-dev build-essential \
     curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev \
     wl-clipboard xclip libxdo-dev
   ```

### Setup & Build
1. **Clone the repository** and navigate to the project folder.
2. **Install frontend dependencies:**
   ```bash
   npm install
   ```
3. **Run in Development Mode:**
   ```bash
   npm run tauri dev
   ```
4. **Build a Production Release:**
   ```bash
   npm run tauri build
   ```
   The binary will be located in `src-tauri/target/release/pasteplus`.

---

## 2. How to Use PastePlus

### Bringing up the History
Press the global hotkey **`Ctrl + Shift + V`** at any time to summon the PastePlus history window. It will appear at your current mouse position or centered on your screen depending on your window manager settings.

### Selecting & Auto-Pasting
1. Scroll through the list using your mouse.
2. **Click an item** to select it.
3. PastePlus will automatically:
   - Update your system clipboard with the selected text.
   - Hide the window.
   - Simulate a `Ctrl + V` command to paste the text into your active application.

### Managing History
- **Pinning (📌):** Hover over an item and click the pin icon. Pinned items are never automatically deleted by the cleanup logic.
- **Deleting (🗑️):** Hover over an item and click the trash icon to remove it from history permanently.
- **Limit:** By default, PastePlus keeps the last 50 unpinned items.

### System Tray
The app runs in the background. Use the system tray icon (usually in the top or bottom corner of your desktop) to:
- **Open:** Manually show the history window.
- **Quit:** Completely exit the application.

---

## 3. Troubleshooting

### Compilation Error: `libsoup-2.4` not found
This is the most common error on modern Linux distributions (like Ubuntu 24.04).
- **Cause:** Tauri 1.x defaults to `libsoup-2.4`, but newer systems may only have `libsoup-3.0`.
- **Fix:** Ensure you have installed the compatibility package: `sudo apt install libsoup2.4-dev`.

### Auto-Paste not working
- **X11 Users:** Ensure `libxdo-dev` (or `xdotool`) is installed.
- **Wayland Users:** Auto-paste relies on `wl-copy` and simulated key events. Due to Wayland's security model, some applications (especially those running via XWayland) may block simulated paste events. If auto-paste fails, the content is still copied to your clipboard—just manually press `Ctrl + V`.

### History not updating
- Ensure either `wl-clipboard` (Wayland) or `xclip` (X11) is installed on your system. PastePlus uses these tools to monitor and update the clipboard.

---

**Thank you for using PastePlus!**
