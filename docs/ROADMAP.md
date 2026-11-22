# Roadmap

## Active WPF application features
- [x] System overview (Windows version/build, CPU, RAM, system drive)
- [x] Service inventory with filtering, risk labels, startup-mode changes and journal
- [x] Service start/stop with confirmation and state-transition wait
- [x] Critical service protections and reviewed Low-risk disable allowlist
- [x] Process inventory with PID, memory, CPU time and executable path
- [x] Process filtering and confirmed termination with PID/name re-check
- [x] Startup inventory and filtering for Run registry entries and user Startup folder
- [x] Confirmed startup entry removal; local backup for user Startup-folder files and journaled registry removal
- [x] Local text report export
- [x] Background inventory scans to keep UI responsive
- [x] WPF application is built and published as the Windows artifact in CI

## Next functional development
- [ ] Startup entry restore workflow (restore backup file and re-create registry values)
- [ ] Service dependency view and service-state recovery
- [ ] Process detail view (publisher/signature, command line, parent PID, network connections where supported)
- [ ] Disk and storage cleanup with explicit preview, selection, backup/recycle-bin safety and confirmation
- [ ] Installed-app uninstall integration with explicit confirmation and official uninstall command only
- [ ] Network connection/adapter diagnostics with actionable but safe repair workflows
- [ ] Module error states, per-module timestamps and progress/cancellation
- [ ] Persian localization and RTL
- [ ] Windows 10/11 manual test matrix and signed release installer

## Retained Rust + Slint read-only preview
- [x] Dashboard and OS/CPU/RAM/process snapshot
- [x] Read-only system inventory modules and bounded output
- [x] Worker-thread scans and local report export

## Safety policy
- No auto-elevation, silent changes, registry cleaning, driver deletion, or undocumented debloating.
- Defender, Firewall, Windows Update, UAC, and core Windows services must never be weakened automatically.
- Every system-changing operation needs a preview, explicit confirmation, durable journal, post-change verification, and a recovery path.
