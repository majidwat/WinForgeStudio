# WinForge Studio

A lightweight Windows 10/11 system-inspection utility built with Rust and Slint.

## Current preview

The active Rust + Slint application provides read-only views for:
- System overview, CPU, RAM, uptime and top 20 memory-consuming processes (PID, RAM and CPU usage)
- Windows services and startup registry entries plus current-user/all-users Startup folders
- Network configuration/adapters; physical disks and volumes
- Installed applications, signed device drivers and scheduled tasks
- Recent System event warnings/errors, installed hotfixes and firewall profiles/rules
- Existing restore points and environment variables (secret-like values redacted)
- Defender/UAC status, battery, Windows licensing and current boot entry
- Power schemes, Plug and Play problem devices, motherboard/BIOS/graphics hardware
- Windows privacy policy and app-consent indicators

Inventory collection runs on a worker thread. Refresh scans are serialized, the sidebar and module content are scrollable, and command output is capped before display.

The legacy WPF source under `src/WinForgeStudio` is retained for reference. The active preview is Rust + Slint.

## Build

On Windows with stable Rust:

```powershell
cargo test --all-targets
cargo build --release
```

GitHub Actions runs tests and the release build on `windows-latest`, then publishes the executable as `WinForgeStudio-windows` (14-day artifact retention). This is an unsigned preview binary, not an installer.

## Safety

The preview is inspection-only. It does not disable services, alter startup entries, modify firewall rules, change boot configuration, remove applications, or change Windows security settings. Results may be partial depending on Windows edition, permissions, hardware, or available PowerShell modules. The app never elevates itself automatically.

Future system-changing operations require preview, explicit confirmation, risk explanation, a durable journal, post-change verification and a recovery path. Defender, Firewall, Windows Update, UAC and core Windows services must never be silently weakened.

See [AGENTS.md](AGENTS.md), [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md), [docs/SECURITY_MODEL.md](docs/SECURITY_MODEL.md), and [docs/BUILD.md](docs/BUILD.md).
