# WinForge Studio

Lightweight native Windows system inspection and maintenance app built **only with Rust and Slint**. WPF, C#, and XAML are not used.

## Build
Install stable Rust and run:

```powershell
cargo test --all-targets
cargo build --release
.\package.ps1
```

The executable is `target/release/winforge-studio.exe`; portable ZIP is `target/release/WinForgeStudio-portable.zip`. The Windows build workflow is manual-only (`workflow_dispatch`) while implementation is in progress; it uploads both artifacts when explicitly run.

## Current implementation
- Dashboard, system information, process snapshot
- Read-only inventory pages for services, startup, network, storage, apps, drivers, tasks, event logs, updates, firewall, restore points, environment (with secret redaction), security, battery, activation, adapters, boot, power, device health, hardware, privacy, and cleanup preview
- Background inventory scanning and local report export
- Process termination with explicit confirmation, exact PID/image-name identity checks, protected-process denylist, and action journal
- Service start/stop and Manual/Automatic startup-mode controls with core-service protection, confirmation, state verification, and action journal
- Current-user Startup Run/RunOnce value and Startup-folder file removal with backup-before-delete

This is a native Rust/Slint application. All pages are rendered in Slint; Windows inventory uses Rust modules and Windows-native/command-based interfaces. Not all inventory modules expose modification controls yet.

## Safety
Scan/report functions are read-only. No automatic elevation, remote upload, or system changes. Destructive/optimization features are not enabled until they have explicit confirmation, journaling, and recovery paths.
