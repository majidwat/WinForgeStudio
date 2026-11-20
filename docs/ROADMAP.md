# Roadmap

Roadmap items are plans, not claims that features already exist. Update status only after implementation and verification.

## 0.1 — Initial live-management prototype

- [x] System inventory baseline
- [x] Process inventory baseline
- [x] Read-only startup inventory
- [x] Windows service inventory baseline
- [x] Explicit service startup-mode change with confirmation
- [x] Append-only JSONL audit journal
- [ ] Verify Release build on Windows 10/11
- [ ] Add automated test project
- [ ] Move blocking inventory work off the UI thread
- [ ] Validate admin-required actions and error reporting

## 0.2 — Safe inventory and localization

- [ ] Complete Persian translation and RTL layout
- [ ] Service dependency graph (dependent services and dependencies)
- [ ] Signed publisher and executable-path metadata where available
- [ ] Exportable HTML/JSON/CSV inventory report
- [ ] Reviewed risk policy catalog; unknown must never mean safe
- [ ] Async scans, progress, cancellation, and UI virtualization

## 0.3 — Startup management

- [ ] Enable/disable startup entries with exact before/after backup
- [ ] Support Run keys, Startup folder shortcuts, and documented startup sources
- [ ] Per-entry restore and audit
- [ ] No automatic removal

## 0.4 — Application/package management

- [ ] Inventory installed Win32 applications
- [ ] Inventory Appx/MSIX packages and provisioned packages
- [ ] Explain per-user vs all-user impact
- [ ] Safe uninstall workflows with vendor uninstallers / supported Windows APIs
- [ ] Never remove protected inbox components by default

## 0.5 — Backup and restore center

- [ ] Versioned backup format
- [ ] Restore registry values and startup configuration
- [ ] Verify restore result
- [ ] Recovery guidance when an operation cannot be reversed
- [ ] Journal integrity and export

## 0.6 — Performance lab

- [ ] Baseline CPU, memory, disk, and startup measurements
- [ ] Before/after comparison
- [ ] User-selectable reversible settings only
- [ ] No placebo tweaks or undocumented registry hacks
- [ ] Explain trade-offs and reboot requirements

## 1.0 — Stable release

- [ ] Automated tests and Windows CI
- [ ] Signed installer and update strategy
- [ ] Accessibility and keyboard navigation
- [ ] Full bilingual UX
- [ ] Privacy/security review
- [ ] Compatibility matrix for supported Windows builds
- [ ] Reproducible release process and release notes
