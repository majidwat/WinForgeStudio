use std::process::Command;

pub fn inventory() -> (String, String) {
    #[cfg(windows)] {
        let script = "$items=Get-CimInstance Win32_PnPEntity -Filter 'ConfigManagerErrorCode <> 0' -ErrorAction SilentlyContinue | Sort-Object Name; 'Devices reporting a problem: '+@($items).Count; if ($items) { $items | Select-Object Name,PNPClass,Manufacturer,ConfigManagerErrorCode,Status | Format-Table -AutoSize | Out-String -Width 200 } else { 'No Plug and Play device problem codes were reported.' }";
        return match Command::new("powershell").args(["-NoProfile","-NonInteractive","-Command",script]).output() {
            Ok(o) if o.status.success() => { let s=crate::output::bounded(&o.stdout).trim().to_string(); let n=s.lines().next().unwrap_or("—").to_string(); (n,s) },
            Ok(o) => ("Unavailable".into(),String::from_utf8_lossy(&o.stderr).to_string()),
            Err(e) => ("Unavailable".into(),format!("PowerShell could not start: {e}")),
        };
    }
    #[cfg(not(windows))] { ("—".into(),"Device health inventory is available on Windows only.".into()) }
}
