<h1 align="center">$\color{red}{WARNNING!!}$</h1><br>
<p>read this before cloning it. this is in a devlopment stage and it need sudo permission on your computer to run,so if your a vibe-code or cuck-coder and dont know what your doing dont use it </p>

<h1 align="center">|| pasteplus ||</h1>
  
<p align="center"> <b>A fast, native-feeling clipboard manager for Linux</b><br/> Minimal. Smart. Always within reach. </p> <p align="center"> <img src="https://img.shields.io/badge/platform-linux-blue?style=flat-square" /> <img src="https://img.shields.io/badge/built%20with-tauri-orange?style=flat-square" /> <img src="https://img.shields.io/badge/license-MIT-green?style=flat-square" /> <img src="https://img.shields.io/badge/status-MVP-yellow?style=flat-square" /> </p>
✨ Overview

ClipFlow is a lightweight clipboard manager designed for Linux users who want speed, simplicity, and a native experience.

It runs quietly in the background, tracks your clipboard history, and lets you instantly recall anything with a global shortcut.
🚀 Features
🧠 Background Daemon
Seamlessly monitors clipboard changes using wl-paste (Wayland) or xclip (X11)
🧹 Smart Deduplication
Eliminates consecutive duplicate entries to keep history clean
⚡ Global Shortcut
Press Ctrl + Shift + V to access your clipboard anytime
📋 Instant Paste
Click an item → instantly paste into the focused application
💾 Persistent Storage
Stores history locally via SQLite
~/.local/share/clipflow/history.db

🛠️ Tech Stack
Frontend: HTML, CSS, JavaScript
Backend: Rust (via Tauri)
Clipboard Tools: wl-clipboard, xclip
Storage: SQLite

⚙️ System Requirements

Install the required dependencies:

Wayland
wl-clipboard
X11
xclip
Auto-Paste Support
Debian/Ubuntu → libxdo-dev
Arch → libxdo
📦 Getting Started
1. Install Node Dependencies
npm install
2. Install Linux Dependencies (Ubuntu/Debian)
sudo apt update
sudo apt install \
  libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libxdo-dev \
  wl-clipboard \
  xclip
3. Run in Development
npm run tauri dev
4. Build for Production
npm run tauri build
🎮 Usage
Ctrl + Shift + V → Open clipboard history
Click an item → Paste into active window
System Tray → Open history or quit
🗺️ Roadmap
Phase 1 (MVP) ✅
Clipboard history tracking
Global shortcut popup
SQLite persistence
Deduplication
Phase 2
Pin items 📌
Search & filtering 🔍
Configurable history limit
Better UI/UX animations
Phase 3
Cross-platform support (Windows)
Cloud sync (optional)
Plugin system
Encrypted clipboard entries
🤝 Contributing

Contributions are welcome—whether it's fixing bugs, improving UI, or suggesting features.

How to Contribute
Fork the repository

Create a new branch

git checkout -b feature/your-feature-name
Make your changes

Commit your work

git commit -m "Add: your feature description"

Push to your branch

git push origin feature/your-feature-name
Open a Pull Request
Contribution Guidelines
Keep code clean and readable
Follow existing project structure
Write meaningful commit messages
Test before submitting PRs
🐛 Issues & Feedback

Found a bug or have an idea?

Open an issue
Describe the problem clearly
Include steps to reproduce
📄 License

This project is licensed under the MIT License.

💡 Vision

ClipFlow aims to become the go-to clipboard manager for developers and power users, balancing performance with simplicity—no bloat, just flow.
