# Nodepat Windows Uninstallation Script
# Removes the binary, desktop shortcut, icon, and PATH entries

$ErrorActionPreference = "Stop"

# Colors for output
function Write-ColorOutput($ForegroundColor) {
    $fc = $host.UI.RawUI.ForegroundColor
    $host.UI.RawUI.ForegroundColor = $ForegroundColor
    if ($args) {
        Write-Output $args
    }
    $host.UI.RawUI.ForegroundColor = $fc
}

Write-ColorOutput Green "========================================"
Write-ColorOutput Green "  Nodepat Uninstallation Script"
Write-ColorOutput Green "========================================"
Write-Output ""

# Configuration
$AppName = "Nodepat"
$BinaryName = "Nodepat.exe"
$InstallDir = "$env:LOCALAPPDATA\$AppName"
$DesktopPath = [Environment]::GetFolderPath("Desktop")
$ShortcutPath = "$DesktopPath\$AppName.lnk"
$UninstallScriptPath = "$InstallDir\uninstall-$AppName.ps1"

# Track what was removed
$RemovedCount = 0

# Remove binary
$BinaryPath = "$InstallDir\$BinaryName"
if (Test-Path $BinaryPath) {
    Write-Output "Removing binary..."
    Remove-Item -Path $BinaryPath -Force -ErrorAction SilentlyContinue
    Write-ColorOutput Green "Removed: $BinaryPath"
    $RemovedCount++
} else {
    Write-ColorOutput Yellow "Binary not found: $BinaryPath"
}

# Remove icon
$IconPath = "$InstallDir\$AppName.jpg"
if (Test-Path $IconPath) {
    Write-Output "Removing icon..."
    Remove-Item -Path $IconPath -Force -ErrorAction SilentlyContinue
    Write-ColorOutput Green "Removed: $IconPath"
    $RemovedCount++
} else {
    Write-ColorOutput Yellow "Icon not found: $IconPath"
}

# Remove desktop shortcut
if (Test-Path $ShortcutPath) {
    Write-Output "Removing desktop shortcut..."
    Remove-Item -Path $ShortcutPath -Force -ErrorAction SilentlyContinue
    Write-ColorOutput Green "Removed: $ShortcutPath"
    $RemovedCount++
} else {
    Write-ColorOutput Yellow "Desktop shortcut not found: $ShortcutPath"
}

# Remove uninstall script itself (before checking if directory is empty)
if (Test-Path $UninstallScriptPath) {
    Write-Output "Removing uninstall script..."
    Remove-Item -Path $UninstallScriptPath -Force -ErrorAction SilentlyContinue
    Write-ColorOutput Green "Removed: $UninstallScriptPath"
    $RemovedCount++
}

# Remove installation directory if empty
if (Test-Path $InstallDir) {
    $Items = Get-ChildItem -Path $InstallDir -ErrorAction SilentlyContinue
    if ($Items.Count -eq 0) {
        Write-Output "Removing empty installation directory..."
        Remove-Item -Path $InstallDir -Force -ErrorAction SilentlyContinue
        Write-ColorOutput Green "Removed: $InstallDir"
    } else {
        Write-ColorOutput Yellow "Installation directory not empty, keeping: $InstallDir"
    }
}

# Remove from PATH
Write-Output "Checking for PATH entries..."
$PathRemoved = $false
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")

if ($UserPath -like "*$InstallDir*") {
    # Remove the installation directory from PATH
    $NewPath = ($UserPath -split ';' | Where-Object { $_ -ne $InstallDir }) -join ';'
    [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
    Write-ColorOutput Green "Removed $InstallDir from user PATH"
    $PathRemoved = $true
} else {
    Write-ColorOutput Yellow "No PATH entries found to remove"
}

Write-Output ""
Write-ColorOutput Green "========================================"
if ($RemovedCount -gt 0) {
    Write-ColorOutput Green "  Uninstallation Complete!"
    Write-ColorOutput Green "========================================"
    Write-Output ""
    Write-Output "Removed $RemovedCount item(s):"
    if (-not (Test-Path $BinaryPath)) { Write-Output "  ✓ Binary" }
    if (-not (Test-Path $ShortcutPath)) { Write-Output "  ✓ Desktop shortcut" }
    if (-not (Test-Path $IconPath)) { Write-Output "  ✓ Icon" }
    if ($PathRemoved) { Write-Output "  ✓ PATH entry" }
    if (-not (Test-Path $UninstallScriptPath)) { Write-Output "  ✓ Uninstall script" }
    Write-Output ""
    Write-Output "Note: You may need to restart your terminal for PATH changes to take effect."
} else {
    Write-ColorOutput Yellow "  Nothing to uninstall"
    Write-ColorOutput Green "========================================"
    Write-Output ""
    Write-Output "No Nodepat installation found."
}
Write-Output ""

