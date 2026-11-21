# Roadmap

## Implemented in the Rust + Slint preview
- [x] Dashboard and OS/CPU/RAM/process snapshot
- [x] Read-only services, startup, network, storage, apps, drivers and scheduled tasks
- [x] Event log, installed hotfixes, firewall and restore points
- [x] Environment inventory with secret-like values redacted
- [x] Defender/UAC, battery, Windows activation and boot diagnostics
- [x] Power schemes, PnP problem devices, motherboard/BIOS/graphics inventory
- [x] Worker-thread scanning, scan status and single-active-scan guard
- [x] Bounded diagnostic output and UTF-8 truncation unit tests
- [x] CI test/build workflow with downloadable Windows executable artifact

## Remaining before a usable 1.0 release
- [ ] Per-module progress, cancellation and process timeouts
- [ ] Structured PowerShell output and searchable/filterable data grids
- [ ] Complete Persian/English localization and RTL layout
- [ ] Module-level error states and last-scan timestamps
- [ ] Safe process details, filtering and process-tree view
- [ ] Service dependency analysis and reviewed safety catalog
- [ ] Backup/restore workflow and recovery validation before destructive features
- [ ] Windows 10/11 UI smoke tests
- [ ] Signed installer/portable release and upgrade/uninstall documentation

## Explicitly excluded from automatic optimization
- Disabling Defender, Firewall, Windows Update, UAC, or core Windows services
- Registry cleaning, driver deletion, or undocumented debloating scripts
