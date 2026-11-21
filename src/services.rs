use std::process::Command;

pub fn inventory() -> (String, String) {
    #[cfg(windows)]
    {
        let script = "Get-CimInstance Win32_Service | Select-Object DisplayName,Name,State,StartMode,ProcessId | Sort-Object DisplayName | Format-Table -AutoSize | Out-String -Width 220";
        let output = Command::new("powershell").args(["-NoProfile", "-NonInteractive", "-Command", script]).output();
        match output {
            Ok(out) if out.status.success() => {
                let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
                let count_script = "(Get-CimInstance Win32_Service).Count";
                let count = Command::new("powershell").args(["-NoProfile", "-NonInteractive", "-Command", count_script]).output().map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_else(|_| "?".into());
                (count, if text.is_empty() { "No service information returned.".into() } else { text })
            }
            Ok(out) => ("?".into(), format!("PowerShell returned an error: {}", String::from_utf8_lossy(&out.stderr))),
            Err(e) => ("?".into(), format!("Unable to launch PowerShell: {}", e)),
        }
    }
    #[cfg(not(windows))]
    { ("—".into(), "Windows service inventory is only available on Windows.".into()) }
}
