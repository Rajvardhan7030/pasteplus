> [!WARNING]
>Urgent info that needs immediate user attention to avoid problems.
> 
>read this before cloning it. this is in a devlopment stage and it need libraries that might not be compatible for  your distro ,so if your a vibe-code or cuck-coder and dont know what your doing dont use it

<h1 align="center">|| pasteplus ||</h1>
  
<p align="center"> <b>A fast, native-feeling clipboard manager for Linux</b><br/> Minimal. Smart. Always within reach. </p> <p align="center"> <img src="https://img.shields.io/badge/platform-linux-blue?style=flat-square" /> <img src="https://img.shields.io/badge/built%20with-tauri-orange?style=flat-square" /> <img src="https://img.shields.io/badge/license-MIT-green?style=flat-square" /> <img src="https://img.shields.io/badge/status-MVP-yellow?style=flat-square" /> </p>

## ✨ Overview ##

ClipFlow is a lightweight clipboard manager designed for Linux users who want speed, simplicity, and a native experience.

It runs quietly in the background, tracks your clipboard history, and lets you instantly recall anything with a global shortcut.

## 🚀 Features ##
-<b>🧠 Background Daemon:</b>
Seamlessly monitors clipboard changes using wl-paste (Wayland) or xclip (X11)

-<b>🧹 Smart Deduplication:</b>
Eliminates consecutive duplicate entries to keep history clean

-<b>⚡ Global Shortcut:</b>
Press Ctrl + Shift + V to access your clipboard anytime

-<b>📋 Instant Paste:</b>
Click an item → instantly paste into the focused application

-<b>💾 Persistent Storage:</b>
Stores history locally via SQLite

```bash 
 ~/.local/share/clipflow/history.db
```

### 🛠️ Tech Stack ###
-Frontend: HTML, CSS, JavaScript

-Backend: Rust (via Tauri)

-Clipboard Tools: wl-clipboard, xclip

-Storage: SQLite

### ⚙️ System Requirements ###

<b>Install the required dependencies: </b>

-Wayland<br>
-wl-clipboard<br>
-X11<br>
-xclip<br>
-Auto-Paste Support<br>
>Debian/Ubuntu → libxdo-dev<br>
>Arch → libxdo<br>

### 📦 Getting Started ###
1. Install Node Dependencies
```bash
npm install
```
2. Install Linux Dependencies (Ubuntu/Debian):
```bash

```
3. Run in Development
```bash
npm run tauri dev
```
4. Build for Production
```bash 
npm run tauri build
```
### 🎮 Usage ###
-Ctrl + Shift + V → Open clipboard history

-Click an item → Paste into active window

-System Tray → Open history or quit

### 🗺️ Roadmap ###
<b>Phase 1: (MVP) ✅ </b><br>
-Clipboard history tracking

-Global shortcut popup

-SQLite persistence

-Deduplication

<b>Phase 2:Pin items 📌</b>

-Search & filtering 🔍

-Configurable history limit

-Better UI/UX animations

<b>Phase 3:</b><br>
-Cross-platform support (Windows)

-Cloud sync (optional)

-Plugin system

-Encrypted clipboard entries

### 🤝 Contributing ###

>Contributions are welcome—whether it's fixing bugs, improving UI, or suggesting features.

<b>How to Contribute</b>

>Fork the repository

>Create a new branch

>git checkout -b feature/your-feature-name

<b>Make your changes</b>

>Commit your work
```bash
git commit -m "Add: your feature description"
```
<b>Push to your branch</b>
```bash
git push origin feature/your-feature-name
```
<b>Open a Pull Request</b>

<b>Contribution Guidelines</b>

<p>Keep code clean and readable
Follow existing project structure
Write meaningful commit messages
Test before submitting PRs</p>

### 🐛 Issues & Feedback ###

>Found a bug or have an idea?

<b>Open an issue</b><br>
Describe the problem clearly
Include steps to reproduce

### 📄 License ###

This project is licensed under the MIT License.

### 💡 Vision ###

ClipFlow aims to become the go-to clipboard manager for developers and power users, balancing performance with simplicity—no bloat, just flow.
