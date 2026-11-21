use std::process::Command;

pub fn inventory() -> (String, String) {
    #[cfg(windows)] {
        let script = "'SYSTEM'; Get-CimInstance Win32_ComputerSystem | Select-Object Manufacturer,Model,SystemType,TotalPhysicalMemory | Format-List | Out-String -Width 180; 'BASEBOARD'; Get-CimInstance Win32_BaseBoard | Select-Object Manufacturer,Product,Version | Format-List | Out-String -Width 180; 'BIOS'; Get-CimInstance Win32_BIOS | Select-Object Manufacturer,SMBIOSBIOSVersion,ReleaseDate | Format-List | Out-String -Width 180; 'GRAPHICS'; Get-CimInstance Win32_VideoController | Select-Object Name,DriverVersion,VideoModeDescription,AdapterRAM | Format-Table -AutoSize | Out-String -Width 180";
        return match Command::new("powershell").args(["-NoProfile","-NonInteractive","-Command",script]).output() {
            Ok(o) if o.status.success() => ("Hardware inventory".into(),crate::output::bounded(&o.stdout).trim().to_string()),
            Ok(o) => ("Partially unavailable".into(),crate::output::bounded(&o.stderr)),
            Err(e) => ("Unavailable".into(),format!("PowerShell could not start: {e}")),
        };
    }
    #[cfg(not(windows))] { ("—".into(),"Hardware inventory is available on Windows only.".into()) }
}
