use std::process::Command;

pub fn inventory() -> String {
    #[cfg(windows)]
    {
        let script = "$d=Get-PhysicalDisk | Select FriendlyName,MediaType,BusType,HealthStatus,OperationalStatus,Size; $v=Get-Volume | Select DriveLetter,FileSystemLabel,FileSystem,HealthStatus,Size,SizeRemaining; 'PHYSICAL DISKS'; $d | Format-Table -AutoSize | Out-String -Width 200; 'VOLUMES'; $v | Format-Table -AutoSize | Out-String -Width 200";
        match Command::new("powershell").args(["-NoProfile", "-NonInteractive", "-Command", script]).output() {
            Ok(out) if out.status.success() => crate::output::bounded(&out.stdout),
            Ok(out) => format!("Storage query failed: {}", String::from_utf8_lossy(&out.stderr)),
            Err(e) => format!("Unable to launch PowerShell: {}", e),
        }
    }
    #[cfg(not(windows))]
    { "Storage inventory is only available on Windows.".into() }
}
