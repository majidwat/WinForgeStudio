# WinForge Studio

WinForge Studio is a lightweight Windows 10/11 system-inspection utility built with Rust and Slint.

## Current preview

The Rust + Slint application provides read-only views for system/CPU/memory/processes; services and startup entries; network configuration and adapters; physical disks and volumes; installed applications, signed drivers and scheduled tasks; System event warnings/errors; installed hotfixes; firewall profiles/rules; restore points; environment variables with secret-like values redacted; Defender/UAC status; battery details; Windows licensing; and the current boot entry.

The legacy WPF source under `src/WinForgeStudio` is retained for reference. The active preview build is the Rust + Slint application.

## Build

On Windows 10/11 with the stable Rust toolchain:

```powershell
cargo test --all-targets
cargo build --release
```

Or run the app with `cargo run`. GitHub Actions runs tests and the release build on `windows-latest`.

## Safety

This preview is inspection-only. It does not disable services, alter startup entries, modify firewall rules, change boot configuration, remove applications, or modify Windows security settings. Some queries can return partial data depending on Windows edition, permissions, hardware, or available PowerShell modules. The application does not elevate itself automatically.

Command output is capped before display to prevent unusually large inventories from overwhelming the interface. Future system-changing operations require preview, explicit confirmation, a risk explanation, a durable journal, post-change verification, and a recovery path. Defender, Firewall, Windows Update, UAC, and core Windows services must never be silently weakened.

See [AGENTS.md](AGENTS.md), [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md), [docs/SECURITY_MODEL.md](docs/SECURITY_MODEL.md), and [docs/BUILD.md](docs/BUILD.md).
