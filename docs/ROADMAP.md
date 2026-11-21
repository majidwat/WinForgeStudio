# Roadmap

## Implemented in the Rust + Slint preview
- [x] Dashboard shell and live OS/CPU/RAM/process snapshot
- [x] Read-only service and startup inventory
- [x] Network configuration, adapters, physical disks and volumes
- [x] Installed applications, signed drivers and scheduled tasks
- [x] Event log, installed hotfix, firewall and restore-point inventory
- [x] Environment inventory with secret-like values redacted
- [x] Defender/UAC status, battery, activation and boot-entry diagnostics
- [x] Power scheme inventory and Plug and Play device problem-code inventory
- [x] Worker-thread scanning with a single active scan guard and UI status
- [x] Bounded diagnostic output and UTF-8 truncation unit tests
- [x] Windows CI tests and release build workflow

## Remaining before a usable 1.0 release
- [ ] Per-module progress, cancellation and timeouts for external commands
- [ ] Replace formatted PowerShell tables with structured data and searchable grids
- [ ] Complete Persian/English localization and RTL layout
- [ ] Add module-level error states and last-scan timestamps
- [ ] Add safe process details and filtering
- [ ] Add service dependency analysis and a reviewed safety catalog
- [ ] Add backup/restore workflow and recovery validation before destructive features
- [ ] Add UI smoke tests on Windows 10 and Windows 11
- [ ] Package a signed installer/portable release and document upgrade/uninstall

## Explicitly excluded from automatic optimization
- Disabling Defender, Firewall, Windows Update, UAC, or core Windows services
- Registry cleaning, driver deletion, or undocumented debloating scripts
