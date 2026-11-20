# WinForge Studio — Instructions for AI Coding Agents

This file is the canonical project contract for every AI coding agent. Read it before exploring or editing code. These rules apply to Claude Code, OpenCode, GitHub Copilot, Codex, Cursor, and other agents. If an agent-specific file conflicts with this file, follow this file and report the conflict.

## 1. Product definition

WinForge Studio is a **live Windows 10/11 inspection and controlled-configuration desktop application**. The selected product scope is live management only. Do not add an ISO builder, offline-image servicing, or Windows installation workflows unless the owner explicitly changes scope.

Primary goals, in priority order:

1. Protect the user's operating system and data.
2. Make system state understandable before offering changes.
3. Keep the app responsive and resource-conscious.
4. Prefer built-in Windows APIs and documented mechanisms.
5. Keep builds deterministic, fast, and easy to reproduce.
6. Maintain a clear, testable, modular codebase.

This is not a one-click debloater. Never optimize by blindly disabling services or removing components.

## 2. Current technology baseline

- Language: C# (latest language version supported by the selected .NET SDK; currently C# 12).
- Runtime/SDK: .NET 8.
- Desktop UI: WPF.
- Target: Windows 10/11 x64.
- Project: `src/WinForgeStudio/WinForgeStudio.csproj`.
- Inventory: WMI / `System.Management`, .NET process APIs, Registry read access.
- Service configuration: Service Control Manager / Registry, only for explicitly approved operations.
- Local audit journal: JSON Lines under `%LOCALAPPDATA%\WinForgeStudio\`.

### Performance and build constraints

- Keep the dependency set small. Do not add a NuGet package when the BCL or Windows API already provides the required capability.
- Avoid large UI frameworks, browser-based shells, Electron, embedded web runtimes, background daemons, telemetry SDKs, and unnecessary abstractions.
- Prefer async I/O for operations that can block the UI. Never run WMI enumeration, disk scans, process enumeration, or service operations on the UI thread.
- Use incremental builds during development. Do not enable expensive analyzers or source generators without a demonstrated benefit.
- Do not claim a build is fast or the app is lightweight without measuring it on Windows. Record cold build, incremental build, idle RAM, startup time, and package size before/after significant changes.
- WPF is the current baseline, not a permanent requirement. Do not migrate to WinForms, WinUI, Avalonia, Rust, or C++ without a written Architecture Decision Record and explicit owner approval. Native C++ may reduce runtime overhead but increases implementation and maintenance cost; evaluate the whole lifecycle, not only executable size.

## 3. Safety contract — non-negotiable

### Default behavior

- Scanning is read-only.
- No automatic changes on launch, startup, schedule, or after a scan.
- No bulk destructive actions in the initial product.
- Every mutation must show the exact target, current value, proposed value, expected effect, risk, and rollback plan before confirmation.
- Record the old value and intended new value in the audit journal before applying a mutation.
- If journaling fails, abort the mutation.
- After a mutation, read the value back and report whether it succeeded.
- Never silently elevate. Explain when administrator privileges are required.
- Never download or execute scripts, binaries, drivers, or packages as part of an optimization.
- Never use hidden PowerShell execution. If PowerShell is needed, show the exact command and require approval.

### Protected areas

Never disable, remove, weaken, or bypass these as a generic optimization:

- Microsoft Defender / Windows Security / real-time protection
- Windows Firewall
- Windows Update and servicing stack
- User Account Control (UAC)
- BitLocker or device encryption
- SmartScreen, Secure Boot, TPM protections
- Event Log, RPC, DCOM, Plug and Play, LSASS, Winlogon, core networking, cryptographic services
- Accessibility features without explicit user choice

Do not recommend disabling a service merely because its name is unfamiliar, its current CPU usage is low, or it is stopped.

### Risk labels

Risk labels are conservative hints, not proof. A service's executable path containing the Windows directory does not by itself prove that the service is critical. Prefer explicit, reviewed service metadata and dependency information. Unknown services must be labelled **Unknown / Needs review**, never "Safe to disable".

### Rollback and backups

- A restore point is helpful but is not a complete backup and is not guaranteed to exist.
- Before changing registry-backed settings, capture the exact previous value, value kind, key path, and timestamp.
- Do not promise rollback until a tested rollback implementation exists.
- The audit journal is an audit log, not a rollback engine.
- Do not delete backups or journal entries automatically.

## 4. Architecture boundaries

Keep responsibilities separated:

- `Models/`: immutable data contracts and domain types.
- `Services/`: Windows API access, inventory, policy/risk evaluation, journaling, backup and restore.
- `ViewModels/`: presentation state and orchestration; no direct registry/WMI calls.
- `Views/` or XAML: presentation only; no system mutation logic in code-behind.
- `Infrastructure/`: logging, settings, storage, process execution abstractions.
- `Tests/`: unit tests for pure logic and integration tests requiring Windows.

Do not put WMI queries, registry writes, process launching, or service-control calls in UI code-behind. Introduce interfaces around Windows-specific operations so risk logic can be unit-tested without modifying a real machine.

## 5. Coding standards

- Use nullable reference types.
- Use explicit, descriptive names; avoid cryptic abbreviations.
- Prefer immutable records for inventory results.
- Dispose `ManagementObjectSearcher`, `ManagementObject`, `Process`, `ServiceController`, Registry keys, streams, and other disposable resources.
- Handle access-denied and process-exited races per item; do not let one inaccessible process abort a complete scan.
- Do not swallow all exceptions silently. Log a safe diagnostic and surface a useful summary to the user.
- Do not log secrets, tokens, personal file contents, or unnecessary user data.
- Keep UI responsive; use cancellation tokens for long-running scans.
- Avoid unnecessary polling. Use event-driven updates or a user-triggered refresh.
- Keep public APIs small and document non-obvious Windows behavior.

## 6. Required workflow for every task

1. Read this file and the relevant architecture/roadmap documents.
2. Inspect the existing implementation before editing. Do not assume a feature exists because it is listed in the roadmap.
3. State the planned files and behavior changes before a broad refactor.
4. Preserve existing user data and settings.
5. Implement the smallest coherent change.
6. Add or update tests, especially for risk classification, backup, restore, and policy decisions.
7. Build Release on Windows and run tests.
8. Review the diff for unsafe service names, destructive commands, privilege escalation, and accidental telemetry.
9. Update documentation and roadmap status.
10. Summarize what changed, what was tested, and any unverified behavior.

Do not mark work complete if it has not been built and tested. Distinguish "implemented", "compiled", and "tested on Windows".

## 7. Git and collaboration rules

- Default branch: `main`.
- Use short-lived feature branches for substantial work.
- Never force-push, delete branches, rewrite history, or change repository visibility without explicit owner approval.
- Never commit credentials, API keys, personal data, local machine inventories, build output, or user-specific logs.
- Keep commits focused and use conventional messages where practical.
- Do not merge a pull request until CI passes and the diff has been reviewed.

## 8. Definition of done

A change is done only when:
- behavior matches the requested scope;
- safety and rollback implications are documented;
- relevant tests pass;
- Release build succeeds on a supported Windows environment;
- no secrets or generated artifacts are committed;
- docs and roadmap reflect reality;
- final notes clearly separate verified results from assumptions.
