# Roadmap (Rust + Slint only)

## Native application
- [x] Slint UI navigation and dashboard
- [x] Rust system snapshot and process inventory
- [x] Read-only Windows inventory modules
- [x] Background scan and local report export
- [x] Portable ZIP packaging and Windows CI artifacts

## Functional migration / development
- [x] Process termination in Rust/Slint with explicit confirmation, PID/image identity checks, protected-process denylist and journal
- [x] Service start/stop and Manual/Automatic startup mode controls with core-service denylist, explicit confirmation, verification and journal
- [ ] Service filtering and full risk classification/recovery UI
- [x] Current-user Run/RunOnce and Startup-folder removal with backup-before-delete
- [x] Startup filtering and restore for registry values/Startup files with backup validation and overwrite protection
- [ ] Structured interactive tables (not only text output), per-module loading/error states and scan timestamps
- [ ] Storage cleanup: per-category selectable preview, size calculation, stronger recovery and UI report (guarded stale user-temp cleanup implemented)
- [x] Installed-app filtering and exact-name interactive uninstall launch (MSI or registered uninstaller; script-host commands blocked)
- [x] Confirmed DNS cache flush with action journal
- [x] Installed power scheme activation with validation, confirmation and journal
- [x] Read-only DISM CheckHealth and SFC verify-only integrity checks (async UI)
- [x] Non-invasive ping and DNS network diagnostics; safe repair options remain
- [ ] Persian localization and RTL
- [x] System Restore point creation action with confirmation and journal\n- [ ] Windows 10/11 smoke test, signed installer (deferred; do not build until requested)

## Safety requirements
No automatic elevation, silent destructive changes, registry cleaning, driver deletion, or undocumented debloating. Defender, Firewall, Windows Update, UAC and core services must not be weakened automatically. Every mutation must require explicit confirmation, journal before execution, verify outcome, and provide a recovery path.
