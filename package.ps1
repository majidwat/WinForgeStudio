$ErrorActionPreference = "Stop"
$root = $PSScriptRoot
$exe = Join-Path $root "target\release\winforge-studio.exe"
if (!(Test-Path $exe)) { throw "Release executable missing. Run cargo build --release first." }
$stage = Join-Path $root "target\release\WinForgeStudio-portable"
$zip = Join-Path $root "target\release\WinForgeStudio-portable.zip"
if (Test-Path $stage) { Remove-Item $stage -Recurse -Force }
New-Item -ItemType Directory -Path $stage | Out-Null
Copy-Item $exe $stage
Copy-Item (Join-Path $root "README.md") $stage
Copy-Item (Join-Path $root "SECURITY.md") $stage
Copy-Item (Join-Path $root "docs\BUILD.md") $stage
if (Test-Path $zip) { Remove-Item $zip -Force }
Compress-Archive -Path (Join-Path $stage "*") -DestinationPath $zip -CompressionLevel Optimal
Write-Host "Portable package created: $zip" -ForegroundColor Green
