param(
    [string]$Repo = "DanKaufmanDev/QwikBootCLI",
    [string]$InstallDir = "$env:USERPROFILE\AppData\Local\bin"
)

$ErrorActionPreference = "Stop"

Write-Host "Installing QwikBoot CLI..."

if (!(Test-Path $InstallDir)) {
    New-Item - ItemType Directory -Force -Path $InstallDir | Out-Null
}

$arch = if ([System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture -eq "x64") {
    "x86_64"
} else {
    "x86"
}

$target = "${arch}-pc-windows-gnu"

$release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"

if (!$release) {
    Write-Error "Failed to fetch release information"
    exit 1
}

$asset = $release.assets | Where-Object { $_.browser_download_url -like "*$target*" }

if (!$asset) {
    Write-Error "No asset found for target: $target"
    exit 1
}

$url = $asset.browser_download_url
$zipPath = $env:TEMP\qwikboot.zip

Write-Host "Downloading from $url"
Invoke-WebRequest -Uri $url -OutFile $zipPath

$extractPath  = "$env:TEMP\qwikboot"
if (Test-Path $extractPath) { Remove-Item -Recurse -Force $extractPath }
Expand-Archive -Path $zipPath -DestinationPath $extractPath

Move-Item -Force "$extractPath\qwikboot.exe" "$InstallDir\qwikboot.exe"

if (-not ($env:PATH -split ';' | ForEach-Object { $_.Trim() | Where-Object { $_ -eq $InstallDir }})) {
    setx PATH "$env:PATH;$InstallDir" | Out-Null
}

Write-Host "QwikBoot CLI installed successfully!"
Write-Host "Run 'qwikboot --version' to verify installation."