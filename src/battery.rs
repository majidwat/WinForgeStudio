use std::process::Command;

pub fn inventory() -> (String, String) {
    #[cfg(windows)] {
        let script = "$items=Get-CimInstance Win32_Battery -ErrorAction SilentlyContinue; if (!$items) { 'No battery detected. Desktop PCs commonly have no battery.' } else { $items | Select-Object Name,Status,EstimatedChargeRemaining,BatteryStatus,DesignVoltage,FullChargeCapacity,DesignCapacity | Format-List | Out-String -Width 180 }";
        return match Command::new("powershell").args(["-NoProfile","-NonInteractive","-Command",script]).output() {
            Ok(o) if o.status.success() => { let s=String::from_utf8_lossy(&o.stdout).trim().to_string(); let n=if s.starts_with("No battery") {"0 batteries".into()} else {"Battery detected".into()}; (n, if s.is_empty() {"No battery data returned.".into()} else {s}) },
            Ok(o) => ("Unavailable".into(),String::from_utf8_lossy(&o.stderr).to_string()),
            Err(e) => ("Unavailable".into(),format!("PowerShell could not start: {e}")),
        };
    }
    #[cfg(not(windows))] { ("—".into(),"Battery inventory is available on Windows only.".into()) }
}
