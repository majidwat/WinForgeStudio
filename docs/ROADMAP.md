# Roadmap

## Implemented in the Rust + Slint preview
- [x] Dashboard and OS/CPU/RAM/process snapshot with PID, RAM and CPU columns
- [x] Read-only services, registry startup entries and user/common Startup folders
- [x] Network configuration/adapters, storage, installed apps, signed drivers and scheduled tasks
- [x] Event log, installed hotfixes, firewall and restore-point inventory
- [x] Environment inventory with secret-like values redacted
- [x] Defender/UAC, battery, activation, boot and privacy indicators
- [x] Power schemes, PnP problem devices, motherboard/BIOS/graphics inventory
- [x] Worker-thread scanning, single-scan guard, scan status and scrollable navigation/content
- [x] Bounded diagnostic output and UTF-8 truncation unit tests
- [x] CI tests, Windows release build and downloadable executable artifact

## Remaining before a usable 1.0 release
- [ ] Per-module progress, cancellation and external-process timeouts
- [ ] Structured PowerShell output and searchable/filterable data grids
- [ ] Complete Persian/English localization and RTL layout\n- [x] Dashboard overview cards for CPU, memory, OS, process count and uptime\n- [x] Settings page documenting privacy, safety and scan behavior\n- [x] Dashboard quick health review using RAM, Defender, device problem codes and pending-restart indicators (informational only)
- [ ] Module-level error states and last-scan timestamps
- [ ] Safe process details, filtering and process-tree view
- [ ] Service dependency analysis and reviewed safety catalog
- [ ] Backup/restore workflow and recovery validation before destructive features
- [ ] Windows 10/11 UI smoke tests
- [ ] Signed installer/portable release and upgrade/uninstall documentation

## Explicitly excluded from automatic optimization
- Disabling Defender, Firewall, Windows Update, UAC, or core Windows services
- Registry cleaning, driver deletion, or undocumented debloating scripts
