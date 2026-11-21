# WinForge Studio

A lightweight Windows 10/11 system-inspection utility built with Rust and Slint.

## Current preview

The active Rust + Slint application provides read-only pages for:
- System overview, CPU, RAM, uptime and top memory-consuming processes
- Windows services and startup registry entries
- Network configuration/adapters; physical disks and volumes
- Installed applications, signed drivers and scheduled tasks
- Recent System event warnings/errors, installed hotfixes and firewall profiles/rules
- Existing restore points and environment variables (secret-like values redacted)
- Defender/UAC status, battery, Windows licensing and current boot entry
- Power schemes, Plug and Play problem devices, motherboard/BIOS/graphics hardware

All inventory collection runs on a worker thread. Refresh scans are serialized, UI status is shown, and command output is capped before display.

The legacy WPF source under `src/WinForgeStudio` is retained for reference. The active preview is Rust + Slint.

## Build

On Windows with stable Rust:

```powershell
cargo test --all-targets
cargo build --release
```

GitHub Actions runs tests and the release build on `windows-latest`, then publishes the executable as a workflow artifact named `WinForgeStudio-windows` (14-day retention).

## Safety

The preview is inspection-only. It does not disable services, alter startup entries, modify firewall rules, change boot configuration, remove applications, or change Windows security settings. Results may be partial depending on Windows edition, permissions, hardware, or available PowerShell modules. The app never elevates itself automatically.

Any future system-changing operation requires preview, explicit confirmation, risk explanation, a durable journal, post-change verification and a recovery path. Defender, Firewall, Windows Update, UAC and core Windows services must never be silently weakened.

See [AGENTS.md](AGENTS.md), [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md), [docs/SECURITY_MODEL.md](docs/SECURITY_MODEL.md), and [docs/BUILD.md](docs/BUILD.md).
