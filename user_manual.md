# PastePlus User Manual

Welcome to **PastePlus**, your lightweight, native clipboard manager for Linux. This guide covers installation, daily usage, and common troubleshooting steps.

---

## 1. Installation Guide

PastePlus is built with **Tauri 2.0** and **React**. To compile and run it from source on Linux (Ubuntu/Debian), follow these steps:

### Prerequisites
1. **Rust:** Install via [rustup.rs](https://rustup.rs/):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. **Node.js:** Recommended version 18 or higher.
3. **System Dependencies:** PastePlus requires several system libraries for the GUI and clipboard interaction.
   
   **For Ubuntu/Debian:**
   ```bash
   sudo apt update
   sudo apt install -y libwebkit2gtk-4.1-dev build-essential \
     curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev \
     wl-clipboard xclip xdotool wtype
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
Press the global hotkey **`Ctrl + Shift + V`** at any time to summon the PastePlus history window.

### Selecting & Auto-Pasting
1. Scroll through the list to find your desired snippet.
2. **Click an item** to select it.
3. PastePlus will:
   - Update your system clipboard with the exact text (preserving all formatting and spaces).
   - Show a **"Copied!"** visual indicator.
   - Automatically hide the window.
   - **Auto-Paste:** After a brief delay (to allow the target window to focus), it will automatically paste the text into your active application.

### Managing History
- **Pinning (📌):** Click the pin icon to keep an item at the top of your list. Pinned items are never automatically deleted.
- **Deleting (🗑️):** Click the trash icon to remove an item from history permanently.
- **History Limit:** PastePlus keeps the last **100** items. Pinned items do not count towards this limit.

### System Tray
The app runs in the background. Use the system tray icon to:
- **Open:** Manually show the history window.
- **Quit:** Completely exit the application.

---

## 3. Troubleshooting

### Auto-Paste not working
- **X11 Users:** Ensure `xdotool` is installed. If `xdotool` fails, PastePlus will attempt a fallback method, but having it installed is recommended.
- **Wayland Users:** Ensure `wtype` is installed. 
- **Focus Issues:** If the text doesn't paste automatically, simply press `Ctrl + V` manually. The text is already copied to your clipboard.

### History not updating
- Ensure either `wl-clipboard` (Wayland) or `xclip` (X11) is installed. These tools are required for PastePlus to "see" your clipboard.

### Formatting is lost
- PastePlus is designed to preserve exact text content. If you find formatting is missing, please ensure the source application is copying standard text format.

---

**Thank you for using PastePlus!**
