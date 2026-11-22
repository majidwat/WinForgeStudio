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
- [ ] Startup filtering and registry-value restore workflow
- [ ] Structured interactive tables (not only text output), per-module loading/error states and scan timestamps
- [ ] Storage cleanup: selectable preview, size calculation, safety checks, confirmation, recovery strategy
- [ ] Installed-app management via supported uninstall commands and confirmation
- [ ] Network diagnostics with safe repair options
- [ ] Persian localization and RTL
- [ ] Windows 10/11 smoke test, signed installer (deferred; do not build until requested)

## Safety requirements
No automatic elevation, silent destructive changes, registry cleaning, driver deletion, or undocumented debloating. Defender, Firewall, Windows Update, UAC and core services must not be weakened automatically. Every mutation must require explicit confirmation, journal before execution, verify outcome, and provide a recovery path.
