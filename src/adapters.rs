use std::process::Command;

pub fn inventory() -> (String, String) {
    #[cfg(windows)] {
        let script = "$items=Get-NetAdapter -IncludeHidden -ErrorAction SilentlyContinue | Sort-Object Name; 'Network adapters: '+@($items).Count; $items | Select-Object Name,InterfaceDescription,Status,MacAddress,LinkSpeed,MediaType,Virtual | Format-Table -AutoSize | Out-String -Width 200; 'IP configuration is shown separately in the Network page.'";
        return match Command::new("powershell").args(["-NoProfile","-NonInteractive","-Command",script]).output() {
            Ok(o) if o.status.success() => { let s=crate::output::bounded(&o.stdout).trim().to_string(); let n=s.lines().next().unwrap_or("—").to_string(); (n,s) },
            Ok(o) => ("Unavailable".into(),String::from_utf8_lossy(&o.stderr).to_string()),
            Err(e) => ("Unavailable".into(),format!("PowerShell could not start: {e}")),
        };
    }
    #[cfg(not(windows))] { ("—".into(),"Network adapter inventory is available on Windows only.".into()) }
}
