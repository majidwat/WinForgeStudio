use std::process::Command;
pub fn inventory() -> (String, String) {
    #[cfg(windows)] {
        let script = "$items=Get-HotFix -ErrorAction SilentlyContinue | Sort-Object InstalledOn -Descending; 'Installed hotfixes: '+@($items).Count; $rebootKeys=@('HKLM:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Component Based Servicing\\RebootPending','HKLM:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\WindowsUpdate\\Auto Update\\RebootRequired'); $pending=@($rebootKeys | Where-Object { Test-Path $_ }).Count -gt 0; $rename=(Get-ItemProperty 'HKLM:\\SYSTEM\\CurrentControlSet\\Control\\Session Manager' -Name PendingFileRenameOperations -ErrorAction SilentlyContinue).PendingFileRenameOperations; if($rename){$pending=$true}; 'Pending reboot indicators: '+$pending; 'Update-related services'; Get-Service -Name wuauserv,bits,cryptsvc -ErrorAction SilentlyContinue | Select-Object Name,Status,StartType | Format-Table -AutoSize | Out-String -Width 160; 'Recent installed hotfixes'; $items | Select-Object -First 100 HotFixID,Description,InstalledBy,InstalledOn | Format-Table -AutoSize | Out-String -Width 220";
        return match Command::new("powershell").args(["-NoProfile","-NonInteractive","-Command",script]).output() {
            Ok(o) if o.status.success() => { let s=crate::output::bounded(&o.stdout).trim().to_string(); let n=s.lines().next().unwrap_or("—").to_string(); (n,s) },
            Ok(o) => ("Unavailable".into(),String::from_utf8_lossy(&o.stderr).to_string()),
            Err(e) => ("Unavailable".into(),format!("PowerShell could not start: {e}")),
        };
    }
    #[cfg(not(windows))] { ("—".into(),"Available on Windows only".into()) }
}
