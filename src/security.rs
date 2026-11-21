use std::process::Command;

pub fn inventory() -> (String, String) {
    #[cfg(windows)] {
        let script = "$mp=Get-MpComputerStatus -ErrorAction SilentlyContinue; $fw=Get-NetFirewallProfile -ErrorAction SilentlyContinue; $uac=(Get-ItemProperty 'HKLM:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System' -Name EnableLUA -ErrorAction SilentlyContinue).EnableLUA; 'WINDOWS SECURITY OVERVIEW'; if ($mp) { 'Microsoft Defender Antivirus'; '  Antivirus enabled: '+$mp.AntivirusEnabled; '  Real-time protection: '+$mp.RealTimeProtectionEnabled; '  Behavior monitoring: '+$mp.BehaviorMonitorEnabled; '  Signature age (days): '+$mp.AntivirusSignatureAge } else { 'Microsoft Defender status unavailable (permissions or platform limitation).' }; 'Firewall profiles'; if ($fw) { $fw | Select-Object Name,Enabled,DefaultInboundAction,DefaultOutboundAction | Format-Table -AutoSize | Out-String -Width 160 } else { 'Firewall status unavailable.' }; 'User Account Control'; '  EnableLUA: '+$uac; 'Read-only status. No security settings were changed.'";
        return match Command::new("powershell").args(["-NoProfile","-NonInteractive","-Command",script]).output() {
            Ok(o) if o.status.success() => { let s=crate::output::bounded(&o.stdout).trim().to_string(); ("Security checks".into(),s) },
            Ok(o) => ("Partially unavailable".into(),String::from_utf8_lossy(&o.stderr).to_string()),
            Err(e) => ("Unavailable".into(),format!("PowerShell could not start: {e}")),
        };
    }
    #[cfg(not(windows))] { ("—".into(),"Security overview is available on Windows only.".into()) }
}
