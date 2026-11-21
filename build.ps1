$ErrorActionPreference = "Stop"
Push-Location $PSScriptRoot
try {
    Write-Host "Rust toolchain:" -ForegroundColor Cyan
    cargo --version
    if ($LASTEXITCODE -ne 0) { throw "Rust/Cargo is required. Install the stable toolchain with rustup." }

    Write-Host "Running tests..." -ForegroundColor Cyan
    cargo test --all-targets
    if ($LASTEXITCODE -ne 0) { throw "Rust tests failed." }

    Write-Host "Building WinForge Studio (Release)..." -ForegroundColor Cyan
    cargo build --release
    if ($LASTEXITCODE -ne 0) { throw "Release build failed." }

    Write-Host "Packaging portable ZIP..." -ForegroundColor Cyan
    & (Join-Path $PSScriptRoot "package.ps1")
    if ($LASTEXITCODE -ne 0) { throw "Packaging failed." }
    Write-Host "Build and package complete." -ForegroundColor Green
}
finally {
    Pop-Location
}
