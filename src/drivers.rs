use std::process::Command;
pub fn inventory() -> (String, String) {
    #[cfg(windows)] {
        let script = "$items=Get-CimInstance Win32_PnPSignedDriver -ErrorAction SilentlyContinue | Sort-Object DeviceName; 'Signed device drivers: '+$items.Count; $items | Select-Object DeviceName,DriverVersion,DriverDate,Manufacturer,DeviceClass | Format-Table -AutoSize | Out-String -Width 220";
        return match Command::new("powershell").args(["-NoProfile","-NonInteractive","-Command",script]).output() {
            Ok(o) if o.status.success() => { let s=String::from_utf8_lossy(&o.stdout).trim().to_string(); let count=s.lines().next().unwrap_or("—").to_string(); (count,s) },
            Ok(o) => ("Unavailable".into(),String::from_utf8_lossy(&o.stderr).to_string()),
            Err(e) => ("Unavailable".into(),format!("PowerShell could not start: {e}")),
        };
    }
    #[cfg(not(windows))] { ("—".into(),"Available on Windows only".into()) }
}
