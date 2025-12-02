# Nodepat Windows Installation Script
# Downloads the latest release and creates a desktop shortcut

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
Write-ColorOutput Green "  Nodepat Installation Script"
Write-ColorOutput Green "========================================"
Write-Output ""

# Configuration
$RepoOwner = "Firstp1ck"  # Update this with your GitHub username
$RepoName = "Nodepat"  # Update this with your repository name
$AppName = "Nodepat"
$BinaryName = "Nodepat.exe"
$InstallDir = "$env:LOCALAPPDATA\$AppName"
$DesktopPath = [Environment]::GetFolderPath("Desktop")
$ShortcutPath = "$DesktopPath\$AppName.lnk"

# Check if running as administrator (not required for user installation)
Write-Output "Installing for current user..."
Write-Output ""

# Create installation directory
Write-Output "Creating installation directory..."
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
Write-ColorOutput Green "Installation directory: $InstallDir"
Write-Output ""

# Get latest release information
Write-Output "Fetching latest release information..."
try {
    $ReleaseUrl = "https://api.github.com/repos/$RepoOwner/$RepoName/releases/latest"
    $ReleaseInfo = Invoke-RestMethod -Uri $ReleaseUrl -Method Get
    
    $LatestVersion = $ReleaseInfo.tag_name
    Write-ColorOutput Green "Latest version: $LatestVersion"
    
    # Find the Windows binary asset
    $WindowsAsset = $ReleaseInfo.assets | Where-Object { $_.name -eq $BinaryName }
    
    if (-not $WindowsAsset) {
        Write-ColorOutput Red "Error: Windows binary ($BinaryName) not found in latest release!"
        Write-Output "Available assets:"
        $ReleaseInfo.assets | ForEach-Object { Write-Output "  - $($_.name)" }
        exit 1
    }
    
    $DownloadUrl = $WindowsAsset.browser_download_url
    $DownloadPath = "$InstallDir\$BinaryName"
    
    Write-Output "Download URL: $DownloadUrl"
    Write-Output ""
} catch {
    Write-ColorOutput Red "Error: Failed to fetch release information!"
    Write-ColorOutput Red $_.Exception.Message
    exit 1
}

# Download the binary
Write-Output "Downloading $BinaryName..."
try {
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $DownloadPath -UseBasicParsing
    Write-ColorOutput Green "Download complete!"
    Write-Output ""
} catch {
    Write-ColorOutput Red "Error: Failed to download binary!"
    Write-ColorOutput Red $_.Exception.Message
    exit 1
}

# Verify the file exists
if (-not (Test-Path $DownloadPath)) {
    Write-ColorOutput Red "Error: Downloaded file not found!"
    exit 1
}

# Create desktop shortcut
Write-Output "Creating desktop shortcut..."
try {
    $WshShell = New-Object -ComObject WScript.Shell
    $Shortcut = $WshShell.CreateShortcut($ShortcutPath)
    $Shortcut.TargetPath = $DownloadPath
    $Shortcut.WorkingDirectory = $InstallDir
    $Shortcut.Description = "Nodepat - A minimalistic text editor"
    $Shortcut.Save()
    Write-ColorOutput Green "Desktop shortcut created!"
    Write-Output ""
} catch {
    Write-ColorOutput Yellow "Warning: Failed to create desktop shortcut!"
    Write-ColorOutput Yellow $_.Exception.Message
    Write-Output ""
}

# Add to PATH (optional - user PATH)
Write-Output "Adding to user PATH..."
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
    Write-ColorOutput Green "Added $InstallDir to user PATH"
    Write-Output "Note: You may need to restart your terminal for PATH changes to take effect."
    Write-Output ""
} else {
    Write-Output "Already in PATH"
    Write-Output ""
}

# Summary
Write-ColorOutput Green "========================================"
Write-ColorOutput Green "  Installation Complete!"
Write-ColorOutput Green "========================================"
Write-Output ""
Write-Output "Installed files:"
Write-Output "  - Binary: $DownloadPath"
Write-Output "  - Desktop shortcut: $ShortcutPath"
Write-Output ""
Write-Output "You can now:"
Write-Output "  1. Double-click the desktop shortcut to launch $AppName"
Write-Output "  2. Run '$AppName' from any terminal (after restart)"
Write-Output "  3. Run '$DownloadPath' directly"
Write-Output ""
Write-ColorOutput Green "Installation completed successfully!"

