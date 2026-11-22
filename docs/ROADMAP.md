# Roadmap (Rust + Slint only)

## Native application
- [x] Slint UI navigation and dashboard
- [x] Rust system snapshot and process inventory
- [x] Read-only Windows inventory modules
- [x] Background scan and local report export
- [x] Portable ZIP packaging and Windows CI artifacts

## Functional migration / development
- [ ] Migrate interactive process actions into Rust with Slint-selected PID, confirmation and identity checks
- [ ] Service filtering, start/stop, startup-type control, risk policy, journal and recovery in Rust/Slint
- [ ] Startup entry filtering, removal, backup and restore in Rust/Slint
- [ ] Structured interactive tables (not only text output), per-module loading/error states and scan timestamps
- [ ] Storage cleanup: selectable preview, size calculation, safety checks, confirmation, recovery strategy
- [ ] Installed-app management via supported uninstall commands and confirmation
- [ ] Network diagnostics with safe repair options
- [ ] Persian localization and RTL
- [ ] Windows 10/11 smoke test, signed installer

## Safety requirements
No automatic elevation, silent destructive changes, registry cleaning, driver deletion, or undocumented debloating. Defender, Firewall, Windows Update, UAC and core services must not be weakened automatically. Every mutation must require explicit confirmation, journal before execution, verify outcome, and provide a recovery path.
