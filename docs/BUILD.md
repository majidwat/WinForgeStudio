# Build and test

Requirements:
- Windows 10 or Windows 11
- Stable Rust toolchain (install via rustup)
- Windows SDK/runtime components required by Slint's selected renderer

Run from the repository root in PowerShell:

```powershell
cargo test --all-targets
cargo build --release
cargo run
```

The release executable is `target/release/winforge-studio.exe`.

GitHub Actions runs tests and a release build on `windows-latest`. On success, download the `WinForgeStudio-windows` artifact from the workflow run page. Artifacts are retained for 14 days. This is an unsigned preview binary, not an installer; Windows SmartScreen may display a warning.

The application is inspection-only. Some system queries can return partial data based on permissions, Windows edition, hardware and installed PowerShell modules. No automatic elevation is performed.
