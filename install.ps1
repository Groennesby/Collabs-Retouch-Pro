# Retouch Pro installer for Windows — downloads the newest release from GitHub.
#   irm https://raw.githubusercontent.com/Groennesby/Collabs-Retouch-Pro/main/install.ps1 | iex
$ErrorActionPreference = 'Stop'
$repo = 'Groennesby/Collabs-Retouch-Pro'
$rel = Invoke-RestMethod "https://api.github.com/repos/$repo/releases/latest"
$asset = $rel.assets | Where-Object { $_.name -match 'x64-setup\.exe$' } | Select-Object -First 1
if (-not $asset) { $asset = $rel.assets | Where-Object { $_.name -match '\.msi$' } | Select-Object -First 1 }
if (-not $asset) { throw "No Windows installer in the latest release. See https://github.com/$repo/releases" }
$file = Join-Path $env:TEMP $asset.name
Write-Host "Downloading $($asset.name) ..."
Invoke-WebRequest $asset.browser_download_url -OutFile $file
Write-Host "Starting installer ..."
Start-Process $file -Wait
