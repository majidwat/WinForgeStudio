use std::process::Command;

#[cfg(windows)]
const INVENTORY_SCRIPT: &str = r#"$roots=@('HKLM:\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\*','HKLM:\\Software\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\*','HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\*'); $items=Get-ItemProperty $roots -ErrorAction SilentlyContinue | Where-Object { $_.DisplayName } | Sort-Object DisplayName -Unique; 'Installed applications: '+$items.Count; $items | Select-Object @{N='Application';E={$_.DisplayName}},@{N='Version';E={$_.DisplayVersion}},@{N='Publisher';E={$_.Publisher}} | Format-Table -AutoSize | Out-String -Width 220"#;

pub fn inventory() -> (String, String) {
    #[cfg(windows)] {
        return match Command::new("powershell").args(["-NoProfile", "-NonInteractive", "-Command", INVENTORY_SCRIPT]).output() {
            Ok(o) if o.status.success() => { let s = crate::output::bounded(&o.stdout).trim().to_string(); let count = s.lines().next().unwrap_or("—").to_string(); (count, s) },
            Ok(o) => ("Unavailable".into(), String::from_utf8_lossy(&o.stderr).to_string()),
            Err(e) => ("Unavailable".into(), format!("PowerShell could not start: {e}")),
        };
    }
    #[cfg(not(windows))] { ("—".into(), "Available on Windows only".into()) }
}

/// Launches the registered uninstaller for one exact DisplayName. No quiet switches are added.
#[cfg(windows)]
pub fn uninstall_exact(display_name: &str) -> Result<String, String> {
    let target = display_name.trim();
    if target.is_empty() || target.len() > 300 || target.contains('\0') { return Err("Enter an exact installed application name.".into()); }
    let script = r#"
param([string]$TargetName)
$roots=@('HKLM:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*','HKLM:\Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\*','HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*')
$matches=@(Get-ItemProperty $roots -ErrorAction SilentlyContinue | Where-Object { $_.DisplayName -and $_.DisplayName -ceq $TargetName })
if($matches.Count -ne 1){ Write-Error "Expected one exact uninstall entry; found $($matches.Count)."; exit 3 }
$app=$matches[0]; $command=[string]$app.UninstallString
if([string]::IsNullOrWhiteSpace($command)){ Write-Error "No registered uninstall command."; exit 4 }
$guid=[regex]::Match($command,'(?i)\{[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\}')
if($command -match '(?i)msiexec' -and $guid.Success){
 $p=Start-Process -FilePath "$env:SystemRoot\System32\msiexec.exe" -ArgumentList @('/x',$guid.Value) -Wait -PassThru; exit $p.ExitCode
}
if($command.StartsWith('"')){ $end=$command.IndexOf('"',1); if($end -lt 2){Write-Error "Malformed uninstall command.";exit 5}; $exe=$command.Substring(1,$end-1); $args=$command.Substring($end+1).Trim() }
else{ $parts=$command -split '\s+',2; $exe=$parts[0]; $args=if($parts.Count -gt 1){$parts[1]}else{''} }
if(!(Test-Path -LiteralPath $exe -PathType Leaf)){Write-Error "Uninstaller executable missing.";exit 6}
if($exe -match '(?i)cmd\.exe|powershell\.exe|pwsh\.exe|wscript\.exe|cscript\.exe'){Write-Error "Script-host uninstaller blocked.";exit 7}
$p=Start-Process -FilePath $exe -ArgumentList $args -Wait -PassThru; exit $p.ExitCode
"#;
    let profile=std::env::var_os("USERPROFILE").ok_or("User profile unavailable")?;
    let dir=std::path::PathBuf::from(profile).join("Documents").join("WinForge Studio").join("Journal");
    std::fs::create_dir_all(&dir).map_err(|e|e.to_string())?;
    let stamp=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|e|e.to_string())?.as_secs();
    use std::io::Write;
    let mut log=std::fs::OpenOptions::new().create(true).append(true).open(dir.join("actions.log")).map_err(|e|e.to_string())?;
    writeln!(log,"{stamp} | REQUEST app-uninstall exact-name={target}").map_err(|e|e.to_string())?;
    let output=Command::new("powershell").args(["-NoProfile","-NonInteractive","-ExecutionPolicy","Restricted","-Command",script,target]).output().map_err(|e|format!("Uninstaller could not start: {e}"))?;
    let out=String::from_utf8_lossy(&output.stdout).trim().to_string(); let err=String::from_utf8_lossy(&output.stderr).trim().to_string();
    if output.status.success(){writeln!(log,"{stamp} | SUCCESS app-uninstall exact-name={target}").map_err(|e|e.to_string())?;Ok(format!("Uninstaller completed for '{target}'. Refresh inventory to verify. {out}"))}
    else{writeln!(log,"{stamp} | FAILED app-uninstall exact-name={target} exit={:?}",output.status.code()).map_err(|e|e.to_string())?;Err(format!("Uninstaller exit {:?}. {out} {err}",output.status.code()).trim().to_string())}
}
#[cfg(not(windows))]
pub fn uninstall_exact(_: &str)->Result<String,String>{Err("Installed app uninstall is Windows-only.".into())}
