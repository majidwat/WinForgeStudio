use std::process::Command;

pub fn inventory() -> (String, String) {
    #[cfg(windows)]
    {
        let script = "$items=Get-CimInstance Win32_Service | Sort-Object DisplayName; 'Windows services: '+@($items).Count; $items | Select-Object DisplayName,Name,State,StartMode,ProcessId,@{Name='WinForgePolicy';Expression={if ($_.Name -in @('Spooler','WSearch','SysMain','Fax','WerSvc')) {'Low-risk allowlist'} else {'Read-only / changes blocked'}}} | Format-Table -AutoSize | Out-String -Width 260";
        match Command::new("powershell").args(["-NoProfile", "-NonInteractive", "-Command", script]).output() {
            Ok(out) if out.status.success() => {
                let text = crate::output::bounded(&out.stdout).trim().to_string();
                let count = text.lines().next().unwrap_or("—").to_string();
                (count, if text.is_empty() { "No service information returned.".into() } else { text })
            }
            Ok(out) => ("Unavailable".into(), format!("PowerShell returned an error: {}", crate::output::bounded(&out.stderr))),
            Err(e) => ("Unavailable".into(), format!("Unable to launch PowerShell: {e}")),
        }
    }
    #[cfg(not(windows))]
    { ("—".into(), "Windows service inventory is only available on Windows.".into()) }
}
