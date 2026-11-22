# WinForge Studio

A lightweight Windows 10/11 system-inspection and maintenance utility.

## Active Windows application

The downloadable Windows artifacts now build the WPF application in `src/WinForgeStudio` (the application with interactive service and process controls), rather than packaging the earlier Rust/Slint inspection-only shell.

Current functional modules:
- System overview (Windows, CPU, memory, system drive)
- Services: searchable inventory and startup-mode controls (Manual, Automatic, Disabled), with risk warning and local change journal
- Processes: searchable process list with PID, RAM, CPU time and executable path; selected-process termination with confirmation and identity re-check
- Startup inventory: filter by name, command, and source
- Change journal path and safety information
- Local text report export for the active inventory

The application does not auto-elevate. Service registry changes may require administrator permissions. Ending a process can discard unsaved work. Critical Windows services are protected from disable actions.

## Build on Windows

Install .NET 8 SDK and run:

```powershell
dotnet build src/WinForgeStudio/WinForgeStudio.csproj -c Release
dotnet publish src/WinForgeStudio/WinForgeStudio.csproj -c Release -r win-x64 --self-contained true
```

GitHub Actions tests/builds the Rust inventory modules and publishes the active WPF application as `WinForgeStudio-windows` (standalone EXE) and `WinForgeStudio-portable` (ZIP with executable and docs), with 14-day artifact retention. The EXE is unsigned; it is not an installer.

## Rust inspection modules

The root Rust + Slint implementation is retained as a separate read-only inventory preview. Its modules include network, storage, apps, drivers, scheduled tasks, events, updates, firewall, restore points, environment redaction, security, battery, activation, adapters, boot, power, device health, hardware, privacy, and cleanup preview.

## Safety

No silent system changes or auto-elevation. Service changes require explicit confirmation and are journaled. The preview modules are read-only. Results may be partial depending on Windows version, permissions, hardware, or installed components.

See `SECURITY.md`, `docs/SECURITY_MODEL.md`, `docs/SMOKE_TEST.md`, and `docs/BUILD.md`.
