use std::process::Command;
pub fn inventory() -> (String, String) {
    #[cfg(windows)] {
        let script = "$items=Get-ComputerRestorePoint -ErrorAction SilentlyContinue | Sort-Object CreationTime -Descending; 'Available restore points: '+@($items).Count; $items | Select-Object SequenceNumber,Description,CreationTime,RestorePointType,EventType | Format-Table -AutoSize | Out-String -Width 200; 'This view is read-only; restore points are not created or removed.'";
        return match Command::new("powershell").args(["-NoProfile","-NonInteractive","-Command",script]).output() {
            Ok(o) if o.status.success() => { let s=String::from_utf8_lossy(&o.stdout).trim().to_string(); let n=s.lines().next().unwrap_or("—").to_string(); (n,s) },
            Ok(o) => ("Unavailable".into(),String::from_utf8_lossy(&o.stderr).to_string()),
            Err(e) => ("Unavailable".into(),format!("PowerShell could not start: {e}")),
        };
    }
    #[cfg(not(windows))] { ("—".into(),"Available on Windows only".into()) }
}
