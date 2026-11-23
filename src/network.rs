use std::process::Command;

pub fn inventory() -> String {
    #[cfg(windows)]
    {
        match Command::new("ipconfig").arg("/all").output() {
            Ok(out) if out.status.success() => crate::output::bounded(&out.stdout),
            Ok(out) => format!("ipconfig failed: {}", String::from_utf8_lossy(&out.stderr)),
            Err(e) => format!("Unable to run ipconfig: {}", e),
        }
    }
    #[cfg(not(windows))]
    { "Network inventory is only available on Windows.".into() }
}

#[cfg(windows)]
pub fn flush_dns_cache() -> Result<String, String> {
    use std::{fs::OpenOptions, io::Write, path::PathBuf};
    let profile=std::env::var_os("USERPROFILE").ok_or("User profile unavailable")?;
    let dir=PathBuf::from(profile).join("Documents").join("WinForge Studio").join("Journal");
    std::fs::create_dir_all(&dir).map_err(|e|e.to_string())?;
    let mut journal=OpenOptions::new().create(true).append(true).open(dir.join("actions.log")).map_err(|e|e.to_string())?;
    let stamp=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d|d.as_secs()).unwrap_or(0);
    writeln!(journal,"{stamp} | REQUEST network-flush-dns").map_err(|e|e.to_string())?;
    let out=Command::new("ipconfig").arg("/flushdns").output().map_err(|e|format!("Could not run ipconfig: {e}"))?;
    let message=String::from_utf8_lossy(&out.stdout).trim().to_string();
    if !out.status.success(){return Err(format!("DNS cache flush failed: {}",String::from_utf8_lossy(&out.stderr)));}
    writeln!(journal,"{stamp} | SUCCESS network-flush-dns").map_err(|e|e.to_string())?;
    Ok(if message.is_empty(){"DNS cache flush completed successfully.".into()}else{message})
}
#[cfg(not(windows))]
pub fn flush_dns_cache()->Result<String,String>{Err("DNS cache flush is available on Windows only.".into())}

#[cfg(windows)]
pub fn diagnostics() -> String {
    let checks = [
        ("Gateway reachability", Command::new("ping").args(["-n","2","-w","1200","1.1.1.1"]).output()),
        ("DNS resolution", Command::new("nslookup").arg("example.com").output()),
    ];
    let mut report=String::from("Network diagnostics (non-invasive)\\n");
    for (label, result) in checks {
        report.push_str(&format!("\\n== {label} ==\\n"));
        match result { Ok(out) => { report.push_str(&crate::output::bounded(&out.stdout)); if !out.status.success(){ report.push_str(&crate::output::bounded(&out.stderr)); } }, Err(e)=>report.push_str(&format!("Could not run check: {e}")) }
    }
    report
}
#[cfg(not(windows))]
pub fn diagnostics()->String { "Network diagnostics are available on Windows only.".into() }
