use std::process::Command;

pub fn inventory() -> (String, String) {
    #[cfg(windows)] {
        let script = "$checks=@(@{Label='Telemetry policy (AllowTelemetry)';Path='HKLM:\\SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection';Name='AllowTelemetry'},@{Label='Advertising ID';Path='HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\AdvertisingInfo';Name='Enabled'},@{Label='Location app consent';Path='HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\CapabilityAccessManager\\ConsentStore\\location';Name='Value'},@{Label='Camera app consent';Path='HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\CapabilityAccessManager\\ConsentStore\\webcam';Name='Value'},@{Label='Microphone app consent';Path='HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\CapabilityAccessManager\\ConsentStore\\microphone';Name='Value'}); foreach($c in $checks){$v=Get-ItemPropertyValue -Path $c.Path -Name $c.Name -ErrorAction SilentlyContinue; if($null -eq $v){$v='Not configured / unavailable'}; '{0}: {1}' -f $c.Label,$v}; 'These are registry policy/consent indicators, not a complete privacy audit.'";
        return match Command::new("powershell").args(["-NoProfile","-NonInteractive","-Command",script]).output() {
            Ok(o) if o.status.success() => ("Privacy indicators".into(),crate::output::bounded(&o.stdout).trim().to_string()),
            Ok(o) => ("Unavailable".into(),crate::output::bounded(&o.stderr)),
            Err(e) => ("Unavailable".into(),format!("PowerShell could not start: {e}")),
        };
    }
    #[cfg(not(windows))] { ("—".into(),"Privacy indicators are available on Windows only.".into()) }
}
