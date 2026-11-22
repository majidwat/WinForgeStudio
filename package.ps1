$ErrorActionPreference = "Stop"
$root = $PSScriptRoot
$project = Join-Path $root "src\WinForgeStudio\WinForgeStudio.csproj"
$publish = Join-Path $root "target\release\WinForgeStudio-Windows"
$stage = Join-Path $root "target\release\WinForgeStudio-portable"
$zip = Join-Path $root "target\release\WinForgeStudio-portable.zip"

if (Test-Path $publish) { Remove-Item $publish -Recurse -Force }
dotnet publish $project -c Release -r win-x64 --self-contained true /p:PublishSingleFile=true /p:IncludeNativeLibrariesForSelfExtract=true /p:EnableCompressionInSingleFile=true -o $publish
$exe = Join-Path $publish "WinForgeStudio.exe"
if (!(Test-Path $exe)) { throw "WPF publish failed: WinForgeStudio.exe was not produced." }
if (Test-Path $stage) { Remove-Item $stage -Recurse -Force }
New-Item -ItemType Directory -Path $stage | Out-Null
Copy-Item $publish\* $stage -Recurse
Copy-Item (Join-Path $root "README.md") $stage
Copy-Item (Join-Path $root "SECURITY.md") $stage
Copy-Item (Join-Path $root "docs\BUILD.md") $stage
if (Test-Path $zip) { Remove-Item $zip -Force }
Compress-Archive -Path (Join-Path $stage "*") -DestinationPath $zip -CompressionLevel Optimal
Write-Host "WPF portable package created: $zip" -ForegroundColor Green
