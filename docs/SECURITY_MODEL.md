# Security model

## Active Rust + Slint preview

The active preview is inspection-only. All inventory queries run locally; the app does not transmit system inventories or collect telemetry. It does not change Windows configuration, disable services, modify startup entries, alter firewall rules, change boot configuration, or modify security settings. It never auto-elevates. Secret-like environment values are redacted and hardware serial numbers are not collected.

Some queries may return partial data depending on Windows edition, permissions, hardware, and installed PowerShell modules. These limitations should be shown as unavailable/partial results, not treated as proof that a feature is disabled.

## Legacy source

The retained WPF source under `src/WinForgeStudio` includes older service-management functionality. It is not built by the current Rust workflow. Do not treat it as equivalent to the read-only preview.

## Future mutations

Any future mutating operation must have a reviewed allow-list, clear before/after preview, explicit confirmation, permission checks, durable journal, post-change validation, and a documented recovery path. Never target security-critical services for disabling. Do not run downloaded scripts or elevate implicitly.
