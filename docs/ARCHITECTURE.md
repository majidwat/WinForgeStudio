# Architecture

The active preview is a Rust + Slint desktop application.

- `ui/app.slint`: navigation, page layout, view properties and refresh interaction.
- `src/main.rs`: scan orchestration and UI-thread updates.
- `src/system.rs`: OS, CPU, RAM, uptime and process snapshot using sysinfo.
- Other `src/*.rs` modules: isolated, read-only Windows inventory queries.
- `src/output.rs`: bounded UTF-8-safe command output and unit tests.

Refresh starts a worker thread. Inventory collection happens off the UI thread; the completed snapshot is transferred back with Slint's event-loop callback before properties are updated. Only one full scan may run at a time. The Dashboard provides snapshot cards for OS, CPU, memory, process count, hostname and uptime. Settings is informational and does not change Windows settings.

External PowerShell/Windows commands use non-interactive, no-profile invocation where applicable. Results are informational and can be incomplete if the command, module, hardware, or permission is unavailable. The application does not auto-elevate.

System mutations are not part of the current Rust preview. Any future mutation must pass a safety gate and require explicit approval, a before/after journal, verification, and recovery support.
