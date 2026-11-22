# Build and test

Requirements: Windows 10/11, .NET 8 SDK, and stable Rust toolchain (the Rust inventory preview is still retained).

## Build active Windows application

From PowerShell at repository root:

```powershell
dotnet build src/WinForgeStudio/WinForgeStudio.csproj -c Release
.package.ps1
```

The packaging script publishes a self-contained, single-file WPF app for `win-x64` and creates `target/release/WinForgeStudio-portable.zip`. The Windows executable is `target/release/WinForgeStudio-Windows/WinForgeStudio.exe`. No .NET runtime install should be needed on the target PC.

GitHub Actions additionally runs the Rust module tests and builds the retained Rust/Slint preview, then publishes the **active WPF app** as `WinForgeStudio-windows` (EXE) and `WinForgeStudio-portable` (ZIP with docs), each retained for 14 days. Download the artifacts from the specific workflow run's Artifacts section. The EXE is unsigned and may trigger SmartScreen.

## Rust inspection preview

```powershell
cargo test --all-targets
cargo build --release
cargo run
```

The root Rust + Slint app remains a read-only inspection preview and is not the active WPF artifact.

The WPF app does not auto-elevate. Service operations and inventory visibility may be limited by the current Windows account permissions.
