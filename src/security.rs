use std::process::Command;

pub fn inventory() -> (String, String) {
    #[cfg(windows)] {
        let script = "$mp=Get-MpComputerStatus -ErrorAction SilentlyContinue; $fw=Get-NetFirewallProfile -ErrorAction SilentlyContinue; $uac=(Get-ItemProperty 'HKLM:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System' -Name EnableLUA -ErrorAction SilentlyContinue).EnableLUA; 'WINDOWS SECURITY OVERVIEW'; if ($mp) { 'Microsoft Defender Antivirus'; '  Antivirus enabled: '+$mp.AntivirusEnabled; '  Real-time protection: '+$mp.RealTimeProtectionEnabled; '  Behavior monitoring: '+$mp.BehaviorMonitorEnabled; '  Signature age (days): '+$mp.AntivirusSignatureAge } else { 'Microsoft Defender status unavailable (permissions or platform limitation).' }; 'Firewall profiles'; if ($fw) { $fw | Select-Object Name,Enabled,DefaultInboundAction,DefaultOutboundAction | Format-Table -AutoSize | Out-String -Width 160 } else { 'Firewall status unavailable.' }; 'User Account Control'; '  EnableLUA: '+$uac; 'Read-only status. No security settings were changed.'";
        return match Command::new("powershell").args(["-NoProfile","-NonInteractive","-Command",script]).output() {
            Ok(o) if o.status.success() => { let s=crate::output::bounded(&o.stdout).trim().to_string(); ("Security checks".into(),security_recommendations(s)) },
            Ok(o) => ("Partially unavailable".into(),String::from_utf8_lossy(&o.stderr).to_string()),
            Err(e) => ("Unavailable".into(),format!("PowerShell could not start: {e}")),
        };
    }
    #[cfg(not(windows))] { ("—".into(),"Security overview is available on Windows only.".into()) }
}

fn security_recommendations(report: String) -> String {
    let lower = report.to_lowercase();
    let mut notes = vec!["Recommendations (based on available indicators):".to_string()];
    if lower.contains("real-time protection: false") { notes.push("• Review Microsoft Defender real-time protection in Windows Security.".into()); }
    else if lower.contains("real-time protection: true") { notes.push("• Defender real-time protection appears enabled.".into()); }
    else { notes.push("• Defender real-time protection status is unknown; verify directly in Windows Security.".into()); }
    if lower.contains("firewall profiles") {
        if lower.contains("false") { notes.push("• Review any firewall profile reported disabled; avoid disabling firewall profiles.".into()); }
        else { notes.push("• Confirm firewall profile status for every active network.".into()); }
    }
    if lower.contains("enablelua: 0") || lower.contains("enablelua: false") { notes.push("• UAC indicator suggests it may be disabled; review User Account Control settings.".into()); }
    else { notes.push("• Keep User Account Control enabled and at its recommended level.".into()); }
    notes.push("No settings were modified.".into());
    format!("{}\\n\\n{}",report,notes.join("\\n"))
}
