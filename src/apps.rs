use std::process::Command;
pub fn inventory() -> (String, String) {
    #[cfg(windows)] {
        let script = "$roots=@('HKLM:\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\*','HKLM:\\Software\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\*','HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\*'); $items=Get-ItemProperty $roots -ErrorAction SilentlyContinue | Where-Object { $_.DisplayName } | Sort-Object DisplayName -Unique; 'Installed applications: '+$items.Count; $items | Select-Object @{N='Application';E={$_.DisplayName}},@{N='Version';E={$_.DisplayVersion}},@{N='Publisher';E={$_.Publisher}} | Format-Table -AutoSize | Out-String -Width 220";
        return match Command::new("powershell").args(["-NoProfile","-NonInteractive","-Command",script]).output() {
            Ok(o) if o.status.success() => { let s=String::from_utf8_lossy(&o.stdout).trim().to_string(); let count=s.lines().next().unwrap_or("—").to_string(); (count,s) },
            Ok(o) => ("Unavailable".into(),String::from_utf8_lossy(&o.stderr).to_string()),
            Err(e) => ("Unavailable".into(),format!("PowerShell could not start: {e}")),
        };
    }
    #[cfg(not(windows))] { ("—".into(),"Available on Windows only".into()) }
}
