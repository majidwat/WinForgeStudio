# Security Policy

The active Rust + Slint preview is read-only and performs local Windows inventory queries. It does not upload system inventories, collect telemetry, or elevate itself. The legacy WPF source is retained for reference and contains older service-management code; it is not the active Cargo build.

## Reporting

Do not publish exploit details or personal system inventories in public issues. Use a private GitHub Security Advisory when available, or contact the repository owner privately.

## Security expectations

- No secrets or telemetry collection.
- Redact secret-like environment variable values before display.
- Do not collect hardware serial numbers by default.
- No silent elevation or automatic destructive changes.
- Never weaken Defender, Firewall, Windows Update, UAC, or core Windows services as an optimization preset.
- Every future mutation must be previewed, explicitly confirmed, journaled before execution, verified after execution, and recoverable.
- Treat unknown services and packages as requiring review.
- Never execute remote scripts or binaries as part of a scan or optimization.
