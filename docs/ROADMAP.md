# Roadmap

## Implemented in the Rust + Slint preview
- [x] Dashboard shell and live OS/CPU/RAM/process snapshot
- [x] Read-only service and startup inventory
- [x] Network configuration, adapters, physical disks and volumes
- [x] Installed applications, signed drivers and scheduled tasks
- [x] Event log, installed hotfix, firewall and restore-point inventory
- [x] Environment inventory with secret-like values redacted
- [x] Defender/UAC status, battery, activation and boot-entry diagnostics
- [x] Bounded command output and automated UTF-8 truncation tests
- [x] Windows release build in GitHub Actions

## Remaining before a usable 1.0 release
- [ ] Move slow system scans off the UI thread; add cancellation and per-module progress
- [ ] Replace formatted PowerShell tables with structured data and searchable grids
- [ ] Complete Persian/English localization and RTL layout
- [ ] Add module-level error states and last-scan timestamps
- [ ] Add safe process details and filtering
- [ ] Add service dependency analysis and a reviewed safety catalog
- [ ] Add backup/restore workflow and recovery validation before any destructive feature
- [ ] Add UI smoke tests on Windows 10 and Windows 11
- [ ] Package a signed installer/portable release and document upgrade/uninstall

## Explicitly excluded from automatic optimization
- Disabling Defender, Firewall, Windows Update, UAC, or core Windows services
- Registry cleaning, driver deletion, or undocumented debloating scripts
