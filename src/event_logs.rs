use std::process::Command;
pub fn inventory() -> (String, String) {
    #[cfg(windows)] {
        let script = "$items=Get-WinEvent -FilterHashtable @{LogName='System'; Level=1,2,3} -MaxEvents 80 -ErrorAction SilentlyContinue; 'Recent System warnings/errors/critical: '+@($items).Count; $items | Select-Object -First 80 TimeCreated,Id,LevelDisplayName,ProviderName,Message | Format-List | Out-String -Width 220";
        return match Command::new("powershell").args(["-NoProfile","-NonInteractive","-Command",script]).output() {
            Ok(o) if o.status.success() => { let s=crate::output::bounded(&o.stdout).trim().to_string(); let n=s.lines().next().unwrap_or("—").to_string(); (n,s) },
            Ok(o) => ("Unavailable".into(),String::from_utf8_lossy(&o.stderr).to_string()),
            Err(e) => ("Unavailable".into(),format!("PowerShell could not start: {e}")),
        };
    }
    #[cfg(not(windows))] { ("—".into(),"Available on Windows only".into()) }
}
