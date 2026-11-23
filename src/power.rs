use std::process::Command;

pub fn inventory() -> (String, String) {
    #[cfg(windows)] {
        return match Command::new("powercfg").args(["/list"]).output() {
            Ok(o) if o.status.success() => {
                let s=crate::output::bounded(&o.stdout).trim().to_string();
                let count=s.lines().filter(|line| line.contains("GUID:")).count().to_string();
                ("Power schemes: ".to_owned()+&count,s)
            },
            Ok(o) => ("Unavailable".into(),crate::output::bounded(&o.stderr)),
            Err(e) => ("Unavailable".into(),format!("Unable to run powercfg: {e}")),
        };
    }
    #[cfg(not(windows))] { ("—".into(),"Power plan inventory is available on Windows only.".into()) }
}

#[cfg(windows)]
pub fn activate_scheme(guid: &str) -> Result<String, String> {
    let guid = guid.trim().to_lowercase();
    let valid = guid.len() == 36 && guid.chars().enumerate().all(|(i,c)| {
        if [8,13,18,23].contains(&i) { c == '-' } else { c.is_ascii_hexdigit() }
    });
    if !valid { return Err("Invalid power-scheme GUID.".into()); }
    let list = Command::new("powercfg").arg("/list").output().map_err(|e| e.to_string())?;
    if !list.status.success() { return Err("Could not list installed power schemes.".into()); }
    let listed = String::from_utf8_lossy(&list.stdout).to_lowercase();
    if !listed.contains(&guid) { return Err("That power scheme is not installed on this PC.".into()); }
    let profile=std::env::var_os("USERPROFILE").ok_or("User profile unavailable")?;
    let dir=std::path::PathBuf::from(profile).join("Documents").join("WinForge Studio").join("Journal");
    std::fs::create_dir_all(&dir).map_err(|e|e.to_string())?;
    use std::io::Write;
    let stamp=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d|d.as_secs()).unwrap_or(0);
    let mut log=std::fs::OpenOptions::new().create(true).append(true).open(dir.join("actions.log")).map_err(|e|e.to_string())?;
    writeln!(log,"{stamp} | REQUEST power-scheme guid={guid}")?;
    let out=Command::new("powercfg").args(["/setactive",&guid]).output().map_err(|e|e.to_string())?;
    if !out.status.success() { writeln!(log,"{stamp} | FAILED power-scheme guid={guid}")?; return Err(crate::output::bounded(&out.stderr)); }
    writeln!(log,"{stamp} | SUCCESS power-scheme guid={guid}")?;
    Ok(format!("Power scheme activated: {guid}. Review your power behavior; this is reversible through this panel."))
}
#[cfg(not(windows))]
pub fn activate_scheme(_: &str)->Result<String,String>{Err("Power-scheme changes are Windows-only.".into())}
