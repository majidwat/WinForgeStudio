# Windows manual smoke-test checklist

Run this checklist on a Windows 10/11 machine after downloading the CI artifact.

## Launch and responsiveness
- [ ] Launch `winforge-studio.exe` as a standard user; confirm no UAC prompt appears.
- [ ] Confirm the window opens and shows the scanning status.
- [ ] Confirm the UI remains responsive while inventory collection runs.
- [ ] Confirm a second Refresh click does not start a parallel scan.
- [ ] Confirm the status changes to scan complete.

## Navigation and display
- [ ] Scroll the sidebar to the final entries and open every module.
- [ ] Scroll the page content on long inventory pages.
- [ ] Confirm each module shows data or a clear unavailable/partial message.
- [ ] Confirm process rows show PID, process name, RAM and CPU columns.
- [ ] Confirm startup inventory includes registry entries and Startup folders.

## Safety and privacy
- [ ] Confirm no service startup type or service state changes after a scan.
- [ ] Confirm firewall, power plans, boot configuration, privacy settings and Windows security settings remain unchanged.
- [ ] Confirm sensitive environment variable names (TOKEN, SECRET, AUTH, CERT, JWT, SESSION, PROXY, URL/URI) display as `[REDACTED]`.
- [ ] Confirm hardware serial numbers are not displayed.
- [ ] Confirm the app does not request elevation or run remote scripts.

## Close and repeat
- [ ] Close the app after a completed scan and launch it again.
- [ ] Repeat the scan and confirm it does not crash.
- [ ] Record any module that reports unavailable data; some Windows editions and hardware do not expose every inventory source.
