# Build and test (Rust/Slint only)

Requirements: Windows 10/11 and stable Rust toolchain.

```powershell
cargo test --all-targets
cargo build --release
.\package.ps1
```

The native executable is `target/release/winforge-studio.exe`. The portable ZIP is `target/release/WinForgeStudio-portable.zip`. CI publishes the native Rust/Slint app as `WinForgeStudio-windows` and `WinForgeStudio-portable`, retained for 14 days.

**No WPF, C#, XAML, or .NET runtime is part of the product.** All UI must be Slint and all functionality must be implemented in Rust.
