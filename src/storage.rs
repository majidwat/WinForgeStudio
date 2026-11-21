use std::process::Command;

pub fn inventory() -> String {
    #[cfg(windows)]
    {
        let script = "$d=Get-PhysicalDisk | Select FriendlyName,MediaType,BusType,HealthStatus,OperationalStatus,Size; $v=Get-Volume | Where-Object { $_.Size -gt 0 } | Select DriveLetter,FileSystemLabel,FileSystem,HealthStatus,@{N='SizeGB';E={[math]::Round($_.Size/1GB,2)}},@{N='FreeGB';E={[math]::Round($_.SizeRemaining/1GB,2)}},@{N='UsedPercent';E={[math]::Round(100*(1-$_.SizeRemaining/$_.Size),1)}}; 'PHYSICAL DISKS'; $d | Format-Table -AutoSize | Out-String -Width 200; 'VOLUMES'; $v | Format-Table -AutoSize | Out-String -Width 200";
        match Command::new("powershell").args(["-NoProfile", "-NonInteractive", "-Command", script]).output() {
            Ok(out) if out.status.success() => crate::output::bounded(&out.stdout),
            Ok(out) => format!("Storage query failed: {}", String::from_utf8_lossy(&out.stderr)),
            Err(e) => format!("Unable to launch PowerShell: {}", e),
        }
    }
    #[cfg(not(windows))]
    { "Storage inventory is only available on Windows.".into() }
}
