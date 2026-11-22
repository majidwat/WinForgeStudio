use std::{fs::OpenOptions, io::Write, path::PathBuf, process::Command, thread, time::Duration};

fn journal(message: &str) -> Result<(), String> {
    let profile=std::env::var_os("USERPROFILE").ok_or("User profile unavailable")?;
    let dir=PathBuf::from(profile).join("Documents").join("WinForge Studio").join("Journal");
    std::fs::create_dir_all(&dir).map_err(|e|e.to_string())?;
    let mut f=OpenOptions::new().create(true).append(true).open(dir.join("actions.log")).map_err(|e|e.to_string())?;
    writeln!(f,"{} | {}",SystemTime::now().duration_since(UNIX_EPOCH).map(|d|d.as_secs()).unwrap_or(0),message).map_err(|e|e.to_string())
}
use std::time::{SystemTime,UNIX_EPOCH};

fn validate_name(name:&str)->Result<String,String>{
    let n=name.trim();
    if n.is_empty() || n.len()>128 || !n.chars().all(|c|c.is_ascii_alphanumeric()||c=='_'||c=='-') { return Err("Enter a valid Windows service name (letters, numbers, _ or -).".into()); }
    let lower=n.to_ascii_lowercase();
    let protected=["rpcss","dcomlaunch","plugplay","eventlog","winmgmt","cryptsvc","bits","wuauserv","mpssvc","windefend","samss","lanmanworkstation","lanmanserver","schedule","power","profsvc","gpsvc","lsass","trustedinstaller"];
    if protected.contains(&lower.as_str()) { return Err(format!("Service '{}' is protected by WinForge safety policy.", n)); }
    let low_risk=["spooler","wsearch","sysmain","fax","wersvc"];
    if !low_risk.contains(&lower.as_str()) { return Err(format!("Service '{}' is not in WinForge's conservative low-risk allowlist; no change was made.", n)); }
    Ok(n.to_string())
}

fn query_state(name:&str)->Result<String,String>{
    let out=Command::new("sc.exe").args(["query",name]).output().map_err(|e|format!("Cannot query service: {e}"))?;
    if !out.status.success(){return Err(String::from_utf8_lossy(&out.stderr).to_string());}
    let text=String::from_utf8_lossy(&out.stdout);
    for line in text.lines(){let t=line.trim();if t.starts_with("STATE"){return Ok(t.to_string());}}
    Err("Windows did not return a service state.".into())
}

/// Starts/stops a service only after a UI confirmation; core Windows services are blocked.
pub fn set_running(name:&str,start:bool)->Result<String,String>{
    let name=validate_name(name)?;
    let before=query_state(&name)?;
    let verb=if start{"start"}else{"stop"};
    journal(&format!("REQUEST service-{verb} name={name} before={before}"))?;
    let out=Command::new("sc.exe").args([verb,&name]).output().map_err(|e|format!("Cannot execute service action: {e}"))?;
    if !out.status.success(){return Err(format!("Windows rejected service {verb}: {}",String::from_utf8_lossy(&out.stdout)));}
    let wanted=if start{"RUNNING"}else{"STOPPED"};
    for _ in 0..15{
        thread::sleep(Duration::from_millis(400));
        if let Ok(state)=query_state(&name){if state.contains(wanted){journal(&format!("SUCCESS service-{verb} name={name} state={state}"))?;return Ok(format!("Service {name} is {wanted}."));}}
    }
    let final_state=query_state(&name).unwrap_or_else(|e|e);
    journal(&format!("VERIFY service-{verb} name={name} state={final_state}"))?;
    Err(format!("Service command was accepted but {wanted} was not verified. Current state: {final_state}"))
}

#[cfg(test)]
mod tests{
 use super::*;
 #[test] fn blocks_core_services(){assert!(validate_name("WinDefend").is_err());assert!(validate_name("RpcSs").is_err());}
 #[test] fn validates_service_names(){assert!(validate_name("Spooler").is_ok());assert!(validate_name("LanmanWorkstation").is_err());assert!(validate_name("bad name").is_err());assert!(validate_name("x;shutdown").is_err());}
}

/// Change only between Manual and Automatic. Disabling services is intentionally unsupported.
pub fn set_start_mode(name:&str, mode:&str)->Result<String,String>{
    let name=validate_name(name)?;
    let mode=match mode.to_ascii_lowercase().as_str(){"manual"=>"demand","automatic"=>"auto",_=>return Err("Only Manual or Automatic startup modes are supported.".into())};
    let before=query_state(&name)?;
    journal(&format!("REQUEST service-start-mode name={name} mode={mode} before={before}"))?;
    let out=Command::new("sc.exe").args(["config",&name,"start=",mode]).output().map_err(|e|e.to_string())?;
    if !out.status.success(){return Err(format!("Windows rejected startup-mode change: {}",String::from_utf8_lossy(&out.stdout)));}
    let verify=Command::new("sc.exe").args(["qc",&name]).output().map_err(|e|e.to_string())?;
    let text=String::from_utf8_lossy(&verify.stdout);
    let expected=if mode=="auto"{"AUTO_START"}else{"DEMAND_START"};
    if !text.contains(expected){journal(&format!("VERIFY service-start-mode name={name} expected={expected} failed"))?;return Err("Startup mode change was not verified by Windows.".into());}
    journal(&format!("SUCCESS service-start-mode name={name} mode={mode}"))?;
    Ok(format!("Service {name} startup mode set to {}.",if mode=="auto"{"Automatic"}else{"Manual"}))
}
