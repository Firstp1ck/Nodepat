# Nodepat

A minimalistic text editor built with Rust and egui. Cross-platform and lightweight.

## Features

- Simple text editing with word wrap
- Undo/Redo support
- Find and Replace functionality
- Font size adjustment (Ctrl + Scroll)
- Light/Dark mode toggle
- Recent files list
- Multiple encoding support (UTF-8, UTF-16 LE/BE, ANSI)

## Installation

### Linux

You can install Nodepat without cloning the repository using the installation script:

```bash
curl -fsSL https://raw.githubusercontent.com/Firstp1ck/Nodepat/main/install-linux.sh | bash
```

### Windows

Run the installation script in PowerShell:

```powershell
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/Firstp1ck/Nodepat/main/install-windows.ps1" -OutFile "install-windows.ps1"
.\install-windows.ps1
```

Or run directly:

```powershell
Invoke-Expression (Invoke-WebRequest -Uri "https://raw.githubusercontent.com/Firstp1ck/Nodepat/main/install-windows.ps1" -UseBasicParsing).Content
```

**Note:** The installation scripts download pre-built binaries from GitHub releases. Desktop file and icon installation is optional and will be skipped if the files are not available.

## Uninstallation

The installation script automatically downloads an uninstall script for easy removal.

### Linux

If you installed via the installer, simply run:

```bash
uninstall-nodepat.sh
```

### Windows

If you installed via the installer, run:

```powershell
& "$env:LOCALAPPDATA\Nodepat\uninstall-Nodepat.ps1"
```

The uninstall scripts will remove:
- The binary executable
- Desktop entry/shortcut
- Icon file
- PATH entries (if added by the installer)
- The uninstall script itself

## Platform Support

- Linux (tested on Arch Linux)
- Windows
- macOS (should work, not tested)

## License

This project is open source and available for use.

