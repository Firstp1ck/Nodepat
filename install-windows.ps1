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
$IsPrerelease = $false
$ReleaseInfo = $null

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
    # Check if it's a 404 error (no stable release found)
    $StatusCode = $null
    try {
        $StatusCode = $_.Exception.Response.StatusCode.value__
    } catch {
        # StatusCode might not be available in all PowerShell versions
        $StatusCode = $null
    }
    
    # Also check error message for 404
    $Is404 = ($StatusCode -eq 404) -or ($_.Exception.Message -like "*404*") -or ($_.Exception.Message -like "*Not Found*")
    
    if ($Is404) {
        Write-ColorOutput Yellow "No stable release found, checking for prereleases..."
        
        try {
            $ReleasesUrl = "https://api.github.com/repos/$RepoOwner/$RepoName/releases"
            $ReleasesList = Invoke-RestMethod -Uri $ReleasesUrl -Method Get
            
            if ($ReleasesList.Count -eq 0) {
                Write-ColorOutput Red "Error: No releases found (including prereleases)!"
                Write-Output ""
                Write-Output "Possible reasons:"
                Write-Output "  1. Repository '$RepoOwner/$RepoName' does not exist"
                Write-Output "  2. Repository has no releases yet"
                Write-Output "  3. Network connectivity issues"
                exit 1
            }
            
            # Get the first release (newest, including prereleases)
            $ReleaseInfo = $ReleasesList[0]
            $LatestVersion = $ReleaseInfo.tag_name
            $IsPrerelease = $ReleaseInfo.prerelease
            
            if ($IsPrerelease) {
                Write-ColorOutput Yellow "Latest version (prerelease): $LatestVersion"
            } else {
                Write-ColorOutput Green "Latest version: $LatestVersion"
            }
            
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
            Write-Output ""
            Write-Output "Possible reasons:"
            Write-Output "  1. Repository '$RepoOwner/$RepoName' does not exist"
            Write-Output "  2. Repository has no releases yet (including prereleases)"
            Write-Output "  3. Network connectivity issues"
            exit 1
        }
    } else {
        Write-ColorOutput Red "Error: Failed to fetch release information!"
        Write-ColorOutput Red $_.Exception.Message
        Write-Output ""
        Write-Output "Possible reasons:"
        Write-Output "  1. Repository '$RepoOwner/$RepoName' does not exist"
        Write-Output "  2. Repository has no releases yet"
        Write-Output "  3. Network connectivity issues"
        exit 1
    }
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

# Download icon (optional)
$IconPath = "$InstallDir\$AppName.ico"
$IconDownloaded = $false
try {
    Write-Output "Downloading icon..."
    $IconUrl = "https://raw.githubusercontent.com/$RepoOwner/$RepoName/main/icon.jpg"
    $IconJpgPath = "$InstallDir\$AppName.jpg"
    Invoke-WebRequest -Uri $IconUrl -OutFile $IconJpgPath -UseBasicParsing -ErrorAction SilentlyContinue
    if (Test-Path $IconJpgPath) {
        $IconPath = $IconJpgPath
        $IconDownloaded = $true
        Write-ColorOutput Green "Icon downloaded"
    }
} catch {
    Write-ColorOutput Yellow "Warning: Could not download icon (optional)"
}

# Create desktop shortcut
Write-Output "Creating desktop shortcut..."
try {
    $WshShell = New-Object -ComObject WScript.Shell
    $Shortcut = $WshShell.CreateShortcut($ShortcutPath)
    $Shortcut.TargetPath = $DownloadPath
    $Shortcut.WorkingDirectory = $InstallDir
    $Shortcut.Description = "Nodepat - A minimalistic text editor"
    if ($IconDownloaded) {
        $Shortcut.IconLocation = $IconPath
    }
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
Write-Output "Version installed: $LatestVersion"
if ($IsPrerelease) {
    Write-ColorOutput Yellow "  (This is a prerelease version)"
}
Write-Output ""
Write-Output "You can now:"
Write-Output "  1. Double-click the desktop shortcut to launch $AppName"
Write-Output "  2. Run '$AppName' from any terminal (after restart)"
Write-Output "  3. Run '$DownloadPath' directly"
Write-Output ""
Write-ColorOutput Green "Installation completed successfully!"

