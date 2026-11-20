# WinForge Studio

**Live Windows inspection and controlled optimization.**

WinForge Studio is a Windows desktop utility for inspecting a running Windows 10/11 system and making carefully confirmed configuration changes. The initial release focuses on live-system management; ISO creation is explicitly out of scope.

## Technology

- C# 12 / .NET 8
- WPF desktop UI
- Windows Management Instrumentation (WMI) for system and service inventory
- Windows Registry and Service Control Manager for narrowly scoped service configuration
- JSON Lines local change journal

WPF was selected for the first implementation because it supports a modern desktop UI while keeping the codebase in one managed language. It is not the absolute minimum-memory option; a native C++/Win32 application could be leaner but would increase implementation and maintenance complexity. Do not change the UI framework or language without an explicit architecture decision.

## Current status

This repository starts at **0.1.0**. It contains a working-source baseline, not a production-ready optimizer. The code has not yet been built and tested on a Windows machine in this development environment.

Included:
- System inventory (Windows version/build, CPU, RAM, system drive)
- Process inventory (PID, working-set memory, cumulative CPU time, executable path when accessible)
- Startup inventory (Run keys and current-user Startup folder; read-only)
- Windows service inventory with conservative risk hints
- Explicit service startup-mode changes (Automatic / Manual / Disabled), confirmation dialog, and local audit record
- Dark desktop shell with English/Persian labels in the initial UI

Not included yet:
- Full Persian RTL localization
- Automatic debloating or bulk service changes
- One-click rollback
- Startup item modification
- Appx/MSIX package removal
- ISO builder (not part of the selected live-management scope)
- Performance tuning presets

## Build on Windows

Requirements: Windows 10/11 x64 and .NET 8 SDK.

```powershell
.
.\build.ps1
```

Or:

```powershell
dotnet restore .\src\WinForgeStudio\WinForgeStudio.csproj
dotnet build .\src\WinForgeStudio\WinForgeStudio.csproj -c Release
```

Run:

```powershell
dotnet run --project .\src\WinForgeStudio\WinForgeStudio.csproj
```

Publish a self-contained single-file x64 build:

```powershell
dotnet publish .\src\WinForgeStudio\WinForgeStudio.csproj -c Release -r win-x64 --self-contained true -p:PublishSingleFile=true -p:EnableCompressionInSingleFile=true -o .\publish
```

The app starts without elevation. Some inventory details may be unavailable to a standard user. Service configuration changes require an elevated process; WinForge does not silently elevate itself.

## Safety principles

1. Inventory and scans are read-only.
2. No automatic optimization or bulk destructive actions.
3. Every supported mutation requires explicit user confirmation.
4. Critical Windows services must never be disabled by a generic heuristic.
5. Record the prior value and intended new value before applying a change.
6. Prefer reversible Windows APIs and documented configuration mechanisms.
7. Never disable Defender, Windows Update, Firewall, UAC, or security controls as a default optimization.
8. Never execute commands, scripts, or downloaded payloads without showing the exact action and receiving approval.

Create a restore point and a full backup before modifying Windows configuration. Risk labels are guidance, not proof of safety.

## Repository guide for AI agents

Read [`AGENTS.md`](AGENTS.md) before making changes. It defines architecture boundaries, coding conventions, safety rules, build/test expectations, and the required workflow for all AI coding agents. See [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) for design details and [`docs/ROADMAP.md`](docs/ROADMAP.md) for planned milestones.

## License

MIT for original WinForge Studio source code. Windows, .NET, and third-party components remain subject to their respective licenses.
