# Windows manual smoke-test checklist

Test the current WPF release artifact on Windows 10/11.

## Launch and scanning
- [ ] Launch `WinForgeStudio.exe` as a standard user; confirm no auto-elevation.
- [ ] Confirm the system scan runs and the window remains responsive.
- [ ] Press Refresh repeatedly; only one scan should run at a time.
- [ ] Confirm overview updates after scan.

## Functional controls
- [ ] Services: search by display name or service name.
- [ ] Services: select a service and try Start; confirm; verify final state or clear error.
- [ ] Services: select a service and try Stop; critical services must be blocked; confirm for other services.
- [ ] Services: Set Manual/Automatic; confirm and verify registry value. Disable must be limited to reviewed Low-risk entries.
- [ ] Processes: search by name, PID, or path; confirm results update immediately.
- [ ] Processes: end a noncritical test process; confirm dialog, process stops, list refreshes.
- [ ] Startup: search name, source, or command.
- [ ] Startup: remove a test HKCU Run entry; confirm it is removed and recorded in journal.
- [ ] Startup: remove a test item from user Startup folder; confirm a local backup is created before deletion.
- [ ] Export report: choose destination, export and inspect text contents.

## Safety and errors
- [ ] Run without elevation and confirm permission-denied actions produce understandable errors, not crashes.
- [ ] Verify no auto-elevation, remote script, telemetry or background service.
- [ ] Confirm protected critical service disable/stop paths are blocked.
- [ ] Confirm startup-file backup and local journal exist before removal.
- [ ] Confirm no system changes occur on scan alone.

## Close and repeat
- [ ] Close/reopen and rescan.
- [ ] Record any failing action and Windows version.
