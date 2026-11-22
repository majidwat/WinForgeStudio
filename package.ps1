$ErrorActionPreference = "Stop"
$root = $PSScriptRoot
$exe = Join-Path $root "target\release\winforge-studio.exe"
$stage = Join-Path $root "target\release\WinForgeStudio-portable"
$zip = Join-Path $root "target\release\WinForgeStudio-portable.zip"
if (!(Test-Path $exe)) { throw "Rust release executable not found. Run cargo build --release first." }
if (Test-Path $stage) { Remove-Item $stage -Recurse -Force }
New-Item -ItemType Directory -Path $stage | Out-Null
Copy-Item $exe (Join-Path $stage "WinForgeStudio.exe")
foreach ($file in @("README.md","SECURITY.md","docs\BUILD.md","docs\SECURITY_MODEL.md")) {
  Copy-Item (Join-Path $root $file) $stage
}
if (Test-Path $zip) { Remove-Item $zip -Force }
Compress-Archive -Path (Join-Path $stage "*") -DestinationPath $zip -CompressionLevel Optimal
Write-Host "Rust/Slint portable package created: $zip" -ForegroundColor Green
