use std::process::Command;
pub fn inventory() -> (String, String) {
    #[cfg(windows)] {
        let script = "$p=Get-NetFirewallProfile -ErrorAction SilentlyContinue; 'Firewall profiles: '+@($p).Count; $p | Select-Object Name,Enabled,DefaultInboundAction,DefaultOutboundAction,NotifyOnListen | Format-Table -AutoSize | Out-String -Width 200; 'Enabled rules (first 100):'; Get-NetFirewallRule -Enabled True -ErrorAction SilentlyContinue | Select-Object -First 100 DisplayName,Direction,Action,Profile | Format-Table -AutoSize | Out-String -Width 200";
        return match Command::new("powershell").args(["-NoProfile","-NonInteractive","-Command",script]).output() {
            Ok(o) if o.status.success() => { let s=crate::output::bounded(&o.stdout).trim().to_string(); let n=s.lines().next().unwrap_or("—").to_string(); (n,s) },
            Ok(o) => ("Unavailable".into(),String::from_utf8_lossy(&o.stderr).to_string()),
            Err(e) => ("Unavailable".into(),format!("PowerShell could not start: {e}")),
        };
    }
    #[cfg(not(windows))] { ("—".into(),"Available on Windows only".into()) }
}
