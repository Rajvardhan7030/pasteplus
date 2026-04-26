<h1 align="center">|| PastePlus ||</h1>
  
<p align="center"> <b>A fast, native-feeling clipboard manager for Linux</b><br/> Minimal. Smart. Always within reach. </p> 
<p align="center"> 
<img src="https://img.shields.io/badge/platform-linux-blue?style=flat-square" /> 
<img src="https://img.shields.io/badge/built%20with-tauri-orange?style=flat-square" /> 
<img src="https://img.shields.io/badge/license-MIT-green?style=flat-square" /> 
<img src="https://img.shields.io/badge/status-MVP-yellow?style=flat-square" /> 
</p>

✨ Overview

**PastePlus** is a lightweight clipboard manager designed for Linux users who want speed, simplicity, and a native experience. It runs quietly in the background, tracks your clipboard history precisely, and lets you instantly recall anything with a global shortcut.

🚀 Features
- 🧠 **Background Daemon**: Seamlessly monitors clipboard changes using Wayland or X11.
- 🧹 **Smart Deduplication**: Eliminates consecutive duplicate entries while preserving exact formatting, leading spaces, and newlines.
- ⚡ **Global Shortcut**: Press `Ctrl + Shift + V` to access your clipboard anytime.
- 📌 **Pinning**: Keep important snippets at the top of your history.
- 📋 **Instant Paste**: Click an item to instantly paste it into the focused application.
- 💾 **Persistent Storage**: Stores up to 100 history items locally via SQLite (~/.local/share/pasteplus/history.db).

🛠️ Tech Stack
- Frontend: HTML, CSS, JavaScript (React)
- Backend: Rust (via Tauri 2.0)
- Clipboard Tools: wl-clipboard (Wayland), xclip/xdotool (X11)
- Storage: SQLite

### 🛠️ Tech Stack ###
-Frontend: HTML, CSS, JavaScript

Install the required dependencies for your environment:

### Wayland
- `wl-clipboard`
- `wtype` (for auto-paste)

### X11
- `xclip`
- `xdotool` (for auto-paste)

📦 Getting Started
1. **Install Linux Dependencies (Ubuntu/Debian)**
   ```bash
   sudo apt update
   sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev wl-clipboard xclip xdotool wtype
   ```
2. **Install Node Dependencies**
   ```bash
   npm install
   ```
3. **Run in Development**
   ```bash
   npm run tauri dev
   ```
4. **Build for Production**
   ```bash
   npm run tauri build
   ```

🎮 Usage
- **Ctrl + Shift + V** → Open clipboard history popup.
- **Click an item** → Copies to clipboard and pastes into the active window.
- **Pin icon (📌)** → Keep an item at the top and prevent it from being deleted.
- **Trash icon (🗑️)** → Remove an item from history.
- **System Tray** → Right-click the icon to Open history or Quit.

📄 License
This project is licensed under the MIT License.

💡 Vision
PastePlus aims to become the go-to clipboard manager for developers and power users, balancing performance with simplicity—no bloat, just flow.
