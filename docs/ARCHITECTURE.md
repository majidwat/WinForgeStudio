# Architecture

## Product boundary

WinForge Studio manages a **running Windows installation**. The project is not an ISO remastering tool. It must not mount or service offline WIM/ESD images unless the product owner explicitly approves a scope change.

## Current stack

| Concern | Current choice |
|---|---|
| Language | C# 12 |
| Framework | .NET 8 |
| Desktop UI | WPF |
| Target OS | Windows 10/11 x64 |
| System inventory | WMI through System.Management |
| Processes | System.Diagnostics.Process |
| Startup inventory | Registry Run keys and Startup folder |
| Service inventory | Win32_Service WMI class |
| Service startup configuration | Windows service registry configuration, guarded |
| Audit | Append-only JSONL in LocalAppData |

## Layering

```text
WPF Views (XAML)
       |
       v
ViewModels (presentation state / commands)
       |
       v
Application services / policies
       |
       +------ System inventory adapters (WMI, Process, Registry)
       +------ Service Control Manager adapter
       +------ Journal / Backup / Restore
       |
       v
Windows 10/11 APIs
```

The UI must not call WMI, Registry, ServiceController, or Process APIs directly. Keep Windows access behind interfaces so tests can substitute fake implementations.

## Resource strategy

- No always-running Windows service.
- No telemetry or background polling by default.
- Load data on demand and cancel long-running scans.
- Page or virtualize large tables.
- Avoid repeatedly enumerating every process/service on every navigation event.
- Cache only short-lived inventory and make refresh explicit.
- Keep NuGet dependencies minimal.
- Measure memory and startup on a real Windows test machine.

## Language/UI decision

C# with WPF is the current implementation baseline. It enables fast iteration, strong Windows API access, and a single language across UI and system logic. WPF is not the absolute lightest native UI option. If measurements show unacceptable idle memory or startup cost, compare a small WinForms prototype before considering a C++ rewrite. Do not migrate without an ADR containing measured trade-offs and owner approval.

## Current implementation caveats

The 0.1 baseline is an early prototype:
- Some system inventory runs synchronously and must be moved off the UI thread before adding more expensive scans.
- Risk scoring is heuristic and must be replaced with a reviewed policy catalog plus dependency-aware analysis.
- Startup inventory is read-only.
- Service startup changes are not yet backed by a tested rollback workflow.
- The first language selector does not yet provide complete Persian RTL translation.
- A journal entry is not proof that a mutation succeeded; verify state after each change.
- CPU time shown for processes is cumulative, not instantaneous CPU utilization.

## Proposed module boundaries

```text
src/WinForgeStudio/
  Models/
  Views/
  ViewModels/
  Services/
    Inventory/
    Services/
    Startup/
    Packages/
    Privacy/
    Performance/
    Journal/
    Backup/
  Infrastructure/
  Policies/
  Resources/
Tests/
  WinForgeStudio.UnitTests/
  WinForgeStudio.IntegrationTests/
```

Adopt this structure incrementally; do not perform a large folder-only refactor without adding value.

## Measurement protocol

Before and after performance-sensitive changes, record:
- clean Release build duration;
- incremental Release build duration;
- application cold-start time;
- idle working set after 60 seconds;
- peak working set during a full inventory scan;
- published folder size and single-file size;
- CPU usage during idle and scan.

Use the same Windows version, hardware, build configuration, and measurement procedure. Never invent benchmark numbers.
