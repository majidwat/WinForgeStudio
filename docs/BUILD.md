# Build and test

Requirements: Windows 10/11 and the stable Rust toolchain installed with rustup.

From PowerShell at the repository root:

```powershell
cargo test --all-targets
cargo build --release
cargo run
```

Or run `.uild.ps1`, which executes the test suite and builds the Rust release binary. The executable is `target/release/winforge-studio.exe`.

GitHub Actions performs the same tests and release build on `windows-latest`. On success, download the `WinForgeStudio-windows` artifact from the workflow run page. The artifact is retained for 14 days. It is an unsigned preview executable, not an installer; SmartScreen may display a warning.

The app does not auto-elevate. Some Windows inventory queries may return partial results based on permissions, Windows edition, hardware, or installed PowerShell modules.
