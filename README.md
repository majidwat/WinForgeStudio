# WinForge Studio

WinForge Studio is a lightweight Windows 10/11 system-inspection utility built with Rust and Slint.

## Current preview

The active Rust + Slint application includes read-only views for:
- System overview, CPU, RAM, uptime and top memory-consuming processes
- Windows services and startup registry entries
- Network configuration and adapters; physical disks and volumes
- Installed applications, signed device drivers and scheduled tasks
- Recent System event warnings/errors, installed hotfixes and firewall profiles/rules
- Existing restore points and environment variables (secret-like values redacted)
- Defender/UAC status, battery information, Windows licensing and current boot entry
- Available power schemes and Plug and Play devices reporting problem codes

All inventory collection runs on a worker thread so the Slint UI can remain responsive. Refresh requests are serialized; a second scan is ignored while one is already running. Command output is bounded before being displayed.

The legacy WPF source under `src/WinForgeStudio` is retained for reference; the active preview build is Rust + Slint.

## Build

On Windows 10/11 with the stable Rust toolchain:

```powershell
cargo test --all-targets
cargo build --release
```

Run locally with `cargo run`. GitHub Actions runs tests and the release build on `windows-latest`.

## Safety

The preview is inspection-only. It does not disable services, alter startup entries, modify firewall rules, change boot configuration, remove applications, or modify Windows security settings. Results may be partial depending on Windows edition, permissions, hardware, or available PowerShell modules. The app never elevates itself automatically.

Future system-changing operations require a preview, explicit confirmation, risk explanation, durable journal, post-change verification and recovery path. Defender, Firewall, Windows Update, UAC and core Windows services must never be silently weakened.

See [AGENTS.md](AGENTS.md), [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md), [docs/SECURITY_MODEL.md](docs/SECURITY_MODEL.md), and [docs/BUILD.md](docs/BUILD.md).
