param (
    [string]$Version = "latest"
)

$ErrorActionPreference = "Stop"

# -------------------------------------------------------------------------
# Formatting & Colors
# -------------------------------------------------------------------------
function Write-Info { param([string]$Msg) Write-Host $Msg -ForegroundColor Gray }
function Write-InfoBold { param([string]$Msg) Write-Host $Msg -ForegroundColor White }
function Write-Success { param([string]$Msg) Write-Host $Msg -ForegroundColor Green }
function Write-ErrorMsg { param([string]$Msg) Write-Host "error: $Msg" -ForegroundColor Red }

# -------------------------------------------------------------------------
# Configuration
# -------------------------------------------------------------------------
$RepoUrl = "https://github.com/dkomeza/tiny-git-helper"
$InstallDir = "$HOME\.tgh"
$BinDir = "$InstallDir\bin"
$ExeName = "tgh.exe"
$ExePath = "$BinDir\$ExeName"

# -------------------------------------------------------------------------
# Architecture Detection
# -------------------------------------------------------------------------
# Map Windows architecture variables to Rust target triples
$Arch = $env:PROCESSOR_ARCHITECTURE
$Target = ""

if ($Arch -eq "AMD64") {
    $Target = "x86_64-pc-windows-msvc"
} elseif ($Arch -eq "ARM64") {
    $Target = "aarch64-pc-windows-msvc"
} else {
    Write-ErrorMsg "Unsupported Windows architecture: $Arch"
    exit 1
}

# -------------------------------------------------------------------------
# URL Construction
# -------------------------------------------------------------------------
if ($Version -eq "latest") {
    $DownloadUrl = "$RepoUrl/releases/latest/download/tgh-$Target.zip"
} else {
    $DownloadUrl = "$RepoUrl/releases/download/$Version/tgh-$Target.zip"
}

# -------------------------------------------------------------------------
# Installation Logic
# -------------------------------------------------------------------------
Write-InfoBold "Installing tgh ($Version) for $Target..."

# Create directory
if (-not (Test-Path -Path $BinDir)) {
    New-Item -ItemType Directory -Path $BinDir | Out-Null
}

$ZipPath = "$BinDir\tgh_download.zip"

Write-Host ""
Write-Info "Downloading from $DownloadUrl"

try {
    # .NET WebClient is often faster/cleaner than Invoke-WebRequest for scripts
    $WebClient = New-Object System.Net.WebClient
    $WebClient.DownloadFile($DownloadUrl, $ZipPath)
} catch {
    Write-Host ""
    Write-ErrorMsg "Download failed. Please check version '$Version' exists or your internet connection."
    exit 1
}

Write-Host ""
Write-Info "Extracting..."

# Extract (Force overwrites existing files)
Expand-Archive -Path $ZipPath -DestinationPath $BinDir -Force

# Cleanup
Remove-Item -Path $ZipPath -Force

# -------------------------------------------------------------------------
# Path Configuration
# -------------------------------------------------------------------------
$UserPath = [Environment]::GetEnvironmentVariable("Path", [EnvironmentVariableTarget]::User)
$PathEntries = $UserPath -split ";"

if ($PathEntries -notcontains $BinDir) {
    Write-Info "Adding $BinDir to user PATH..."
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$BinDir", [EnvironmentVariableTarget]::User)
    $PathUpdated = $true
} else {
    $PathUpdated = $false
}

# -------------------------------------------------------------------------
# Success Message
# -------------------------------------------------------------------------
Write-Host ""
Write-Success "tgh ($Version) was installed successfully to $ExePath"
Write-Host ""

Write-Info "To get started, run:"
Write-Host ""

if ($PathUpdated) {
    Write-InfoBold "  Refreshenv"
    Write-Info "  (or restart your terminal to update the PATH)"
}

Write-InfoBold "  tgh --help"
Write-Host ""
