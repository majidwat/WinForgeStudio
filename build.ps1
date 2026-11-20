$ErrorActionPreference = "Stop"
$project = Join-Path $PSScriptRoot "src\WinForgeStudio\WinForgeStudio.csproj"

Write-Host "Using .NET SDK:" -ForegroundColor Cyan
dotnet --version
if ($LASTEXITCODE -ne 0) { throw ".NET 8 SDK is required." }

dotnet restore $project
if ($LASTEXITCODE -ne 0) { throw "Restore failed." }

dotnet build $project -c Release --no-restore
if ($LASTEXITCODE -ne 0) { throw "Build failed." }

Write-Host "Build completed." -ForegroundColor Green
