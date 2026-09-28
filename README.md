# SoulsConflict

[![Download Latest Version Portable .Zip]](https://github.com/nobodyttk/SoulsConflict-Mod-Manager-and-Merger-for-Dark-Souls-Remastered/releases)

**SoulsConflict** is a powerful, portable, and user-friendly Mod Conflict Checker & Merger tool for **Dark Souls Remastered**. 
It was built from the ground up to allow players to safely combine multiple mods together, intelligently resolving conflicts in game files that would otherwise crash the game or overwrite critical changes.

Whether you're merging complex `.emevd` event scripts, `.msb` map layouts, or standard `.bnd3` (like `.param`, `.fmg`, `.talkesd`) archives, SoulsConflict handles it automatically with a seamless interface.


## 🚀 Features

- **Smart Merging Engine:** Intelligently combines `.emevd`, `.msb`, and `.bnd3` files instead of just overwriting them. Prioritizes the mod with the highest load order for direct conflicts.
- **In-Depth Conflict Diagnostics:** Analyzes your `mods/` directory and generates a clear report highlighting *Safe*, *Mergeable*, and *Critical Conflict* files.
- **1-Click Deploy & Restore:** Inject your merged mods directly into the Dark Souls Remastered game folder safely. Restore the game back to 100% Vanilla at any time with one click.
- **100% Portable:** No installation required. Drop your downloaded mods into the `mods/` folder, run the executable, and merge.
- **Seamless Co-op Integration:** Native support for launching the popular Dark Souls 1 Seamless Co-op mod (`ds1sc_launcher.exe`) directly from the UI.
- **Multi-Language Support:** The UI is available in 10 different languages (auto-detected based on your OS).

## 🛠️ Technology Stack

SoulsConflict uses a modern, lightweight, and fast architecture:

- **Backend (Rust):**
  - Uses `tao` for native window creation.
  - Uses `wry` for a lightweight, system-native WebView interface.
  - Fully custom parsers in Rust for proprietary FromSoftware file formats (BND3, EMEVD, MSB, FMG).
  - High performance, multithreaded file operations.
- **Frontend (Web Technologies):**
  - Pure **Vanilla HTML, CSS, and JavaScript**.
  - No heavy frameworks like React or Angular to ensure the executable remains extremely small and boots instantly.
  - Dynamic JavaScript-based localization engine.

## 📁 Project Structure

```
SoulsConflict/
├── src/
│   ├── main.rs          # Entry point, TCP API Server & WebView Initialization
│   ├── merger.rs        # Smart Merging Engine logic for combining files
│   ├── scanner.rs       # Diagnostic engine for detecting file collisions
│   ├── formats.rs       # Custom parsers for FromSoftware formats (BND3, EMEVD, etc.)
│   └── deployer.rs      # Handles game directory injection, backups, and launching
├── static/
│   ├── index.html       # The main User Interface layout
│   ├── app.js           # Frontend logic, API calls, and Localization dictionary
│   └── style.css        # Vanilla CSS styling, Dark Theme and Responsive layout
├── build.rs             # Rust build script (embeds application icons)
├── package_release.ps1  # PowerShell script to compile and package the release zip
└── Cargo.toml           # Rust dependencies and project configuration
```

## 🎮 How to Use

1. **Extract Mods:** Extract your downloaded Dark Souls Remastered mods into the `mods/` folder (e.g., `mods/MyMod1/chr/...`).
2. **Run SoulsConflict:** Open `souls_conflict.exe`. The app will spin up its local backend and open the WebView interface.
3. **Analyze & Merge:** Navigate to the **Diagnostic & Merge** tab. Set your mod load order and click "Merge Mods". The resulting files will be saved in the `merged/` directory.
4. **Deploy & Play:** Navigate to the **Mod Deployer** tab. Ensure your game path is correct, click "Deploy Mods to Game", and launch the game directly from the app (Supports both standard launch and Seamless Co-op).

## 🔨 Building from Source

To build SoulsConflict from source, you need to have [Rust and Cargo](https://rustup.rs/) installed.

```bash
# Clone the repository
git clone https://github.com/nobodyttk/SoulsConflict-Mod-Manager-and-Merger-for-Dark-Souls-Remastered.git
cd SoulsConflict-Mod-Manager-and-Merger-for-Dark-Souls-Remastered

# Build the project in release mode
cargo build --release

# Alternatively, package for distribution using the provided script (Windows)
.\package_release.ps1
```

## 📝 License

This project is open-source and free to use. Mods and files merged by this tool belong to their respective creators.
