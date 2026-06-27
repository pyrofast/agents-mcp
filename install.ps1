#!/usr/bin/env pwsh
$ErrorActionPreference = "Stop"
$Repo = "pyrofast/mcplink"

Write-Host "==> mcplink installer" -ForegroundColor Cyan
Write-Host ""

# Detect architecture
$Arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else {
  Write-Host "Unsupported architecture: only x86_64 is supported" -ForegroundColor Red
  exit 1
}
$Target = "${Arch}-pc-windows-msvc"
$Binary = "mcplink-${Target}.exe"

# Fetch latest release tag
Write-Host "==> Fetching latest release..." -ForegroundColor Cyan
$ApiUrl = "https://api.github.com/repos/$Repo/releases/latest"
$Tag = (Invoke-RestMethod -Uri $ApiUrl -Headers @{ "Accept" = "application/json" }).tag_name
Write-Host "    Latest: $Tag"

$Url = "https://github.com/$Repo/releases/download/$Tag/$Binary"

# Determine install directory
if ([Security.Principal.WindowsPrincipal]::new(
    [Security.Principal.WindowsIdentity]::GetCurrent()
  ).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
  $DestDir = "$env:ProgramFiles\mcplink"
} else {
  $DestDir = "$env:USERPROFILE\.mcplink\bin"
}

$DestPath = "$DestDir\mcplink.exe"
New-Item -ItemType Directory -Force -Path $DestDir | Out-Null

Write-Host "==> Downloading $Binary..." -ForegroundColor Cyan
Invoke-WebRequest -Uri $Url -OutFile "$DestPath" -UseBasicParsing

# Add to PATH if not already there
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$DestDir*") {
  [Environment]::SetEnvironmentVariable("Path", "$UserPath;$DestDir", "User")
  $env:Path += ";$DestDir"
  Write-Host "==> Added $DestDir to PATH" -ForegroundColor Cyan
}

Write-Host "==> Done! Run 'mcplink' to start." -ForegroundColor Cyan
