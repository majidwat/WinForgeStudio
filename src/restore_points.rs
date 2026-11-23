use std::process::Command;
pub fn inventory() -> (String, String) {
    #[cfg(windows)] {
        let script = "$items=Get-ComputerRestorePoint -ErrorAction SilentlyContinue | Sort-Object CreationTime -Descending; 'Available restore points: '+@($items).Count; $items | Select-Object SequenceNumber,Description,CreationTime,RestorePointType,EventType | Format-Table -AutoSize | Out-String -Width 200; 'This view is read-only; restore points are not created or removed.'";
        return match Command::new("powershell").args(["-NoProfile","-NonInteractive","-Command",script]).output() {
            Ok(o) if o.status.success() => { let s=crate::output::bounded(&o.stdout).trim().to_string(); let n=s.lines().next().unwrap_or("—").to_string(); (n,s) },
            Ok(o) => ("Unavailable".into(),String::from_utf8_lossy(&o.stderr).to_string()),
            Err(e) => ("Unavailable".into(),format!("PowerShell could not start: {e}")),
        };
    }
    #[cfg(not(windows))] { ("—".into(),"Available on Windows only".into()) }
}

#[cfg(windows)]
pub fn create_restore_point(description: &str) -> Result<String,String> {
    let label=description.trim();
    if label.is_empty() || label.len()>64 || label.chars().any(|c| !(c.is_ascii_alphanumeric() || c==' ' || c=='-' || c=='_')) { return Err("Restore point description must be 1–64 simple characters.".into()); }
    let profile=std::env::var_os("USERPROFILE").ok_or("User profile unavailable")?;
    let dir=std::path::PathBuf::from(profile).join("Documents").join("WinForge Studio").join("Journal");
    std::fs::create_dir_all(&dir).map_err(|e|e.to_string())?;
    let stamp=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|e|e.to_string())?.as_secs();
    use std::io::Write;
    let mut log=std::fs::OpenOptions::new().create(true).append(true).open(dir.join("actions.log")).map_err(|e|e.to_string())?;
    writeln!(log,"{stamp} | REQUEST restore-point description={label}")?;
    let script="param([string]$Label) try { Checkpoint-Computer -Description $Label -RestorePointType 'MODIFY_SETTINGS' -ErrorAction Stop; Write-Output 'Restore point request completed.' } catch { Write-Error $_; exit 2 }";
    let out=Command::new("powershell").args(["-NoProfile","-NonInteractive","-Command",script,label]).output().map_err(|e|format!("Restore point command failed to start: {e}"))?;
    if out.status.success(){writeln!(log,"{stamp} | SUCCESS restore-point description={label}")?;Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())}
    else {let err=String::from_utf8_lossy(&out.stderr).trim().to_string();writeln!(log,"{stamp} | FAILED restore-point description={label}")?;Err(format!("Could not create restore point. It may require Administrator, System Protection enabled, and Windows may enforce a creation interval. {err}"))}
}
#[cfg(not(windows))]
pub fn create_restore_point(_: &str)->Result<String,String>{Err("Restore point creation is Windows-only.".into())}
