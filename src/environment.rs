use std::process::Command;
pub fn inventory() -> (String, String) {
    #[cfg(windows)] {
        let script = "$items=Get-ChildItem Env: | Sort-Object Name; 'Environment variables: '+@($items).Count; $items | ForEach-Object { $v=$_.Value; if ($_.Name -match '(?i)TOKEN|SECRET|PASSWORD|PASSWD|API.?KEY|CREDENTIAL|AUTH|COOKIE|PRIVATE.?KEY|CLIENT.?SECRET|SIGNING.?KEY|ACCESS.?KEY|PROXY|CERT|BEARER|JWT|SESSION|SSO|(^|_)URL($|_)|(^|_)URI($|_)') { $v='[REDACTED]' }; if ($v.Length -gt 500) { $v=$v.Substring(0,500)+'…[truncated]' }; '{0}={1}' -f $_.Name,$v }";
        return match Command::new("powershell").args(["-NoProfile","-NonInteractive","-Command",script]).output() {
            Ok(o) if o.status.success() => { let s=crate::output::bounded(&o.stdout).trim().to_string(); let n=s.lines().next().unwrap_or("—").to_string(); (n,s) },
            Ok(o) => ("Unavailable".into(),String::from_utf8_lossy(&o.stderr).to_string()),
            Err(e) => ("Unavailable".into(),format!("PowerShell could not start: {e}")),
        };
    }
    #[cfg(not(windows))] { ("—".into(),"Available on Windows only".into()) }
}
