#[cfg(windows)]
mod windows {
    use winreg::{enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE}, RegKey};
    use winreg::enums::KEY_READ;

    pub fn inventory() -> (String, String) {
        let mut entries: Vec<String> = Vec::new();
        for (root, label) in [(HKEY_CURRENT_USER, "Current user"), (HKEY_LOCAL_MACHINE, "All users")] {
            let key = RegKey::predef(root);
            for path in [
                "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                "Software\\Microsoft\\Windows\\CurrentVersion\\RunOnce",
                "Software\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Run",
                "Software\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\RunOnce",
            ] {
                if let Ok(run_key) = key.open_subkey_with_flags(path, KEY_READ) {
                    for (name, value) in run_key.enum_values().flatten() {
                        let decoded = String::from_utf16_lossy(
                            &value.bytes.chunks_exact(2).map(|b| u16::from_le_bytes([b[0], b[1]])).collect::<Vec<_>>()
                        );
                        let cleaned = decoded.trim_end_matches('\0').replace('\n', " ");
                        entries.push(format!("{} | {} | {} | {}", label, path.rsplit('\\').next().unwrap_or(path), name, cleaned));
                    }
                }
            }
        }
        for (folder, label) in [
            (std::env::var("APPDATA").ok().map(|p| std::path::PathBuf::from(p).join("Microsoft\\Windows\\Start Menu\\Programs\\Startup")), "Current user Startup folder"),
            (std::env::var("ProgramData").ok().map(|p| std::path::PathBuf::from(p).join("Microsoft\\Windows\\Start Menu\\Programs\\StartUp")), "All users Startup folder"),
        ] {
            if let Some(folder) = folder {
                if let Ok(items) = std::fs::read_dir(folder) {
                    for item in items.flatten() {
                        let path = item.path();
                        entries.push(format!("{} | {} | {}", label, path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_else(|| "unknown".into()), path.display()));
                    }
                }
            }
        }
        entries.sort();
        let count = entries.len().to_string();
        (count, crate::output::bounded(entries.join("\n").as_bytes()))
    }
}
#[cfg(windows)]
pub fn inventory() -> (String, String) { windows::inventory() }

#[cfg(not(windows))]
pub fn inventory() -> (String, String) {
    ("0".into(), "Startup registry inventory is available on Windows.".into())
}

#[cfg(windows)]
pub fn remove_current_user_entry(name: &str) -> Result<String, String> {
    use winreg::{enums::{HKEY_CURRENT_USER, KEY_READ, KEY_WRITE}, RegKey};
    use std::{fs::OpenOptions, io::Write, path::PathBuf};
    let name=name.trim();
    if name.is_empty() || name.len()>255 || name.chars().any(|c| c=='\\' || c=='/' || c=='\0') { return Err("Invalid startup value name.".into()); }
    let root=RegKey::predef(HKEY_CURRENT_USER);
    let mut found=None;
    for sub in ["Software\\Microsoft\\Windows\\CurrentVersion\\Run","Software\\Microsoft\\Windows\\CurrentVersion\\RunOnce","Software\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Run","Software\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\RunOnce"] {
        if let Ok(key)=root.open_subkey_with_flags(sub,KEY_READ|KEY_WRITE) {
            if let Ok(value)=key.get_raw_value(name) { found=Some((sub.to_string(),key,value)); break; }
        }
    }
    let (sub,key,value)=found.ok_or("No matching current-user Run/RunOnce value was found.")?;
    let profile=std::env::var_os("USERPROFILE").ok_or("User profile unavailable")?;
    let backup=PathBuf::from(profile).join("Documents").join("WinForge Studio").join("Backups");
    std::fs::create_dir_all(&backup).map_err(|e|e.to_string())?;
    let stamp=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|e|e.to_string())?.as_secs();
    let safe=name.chars().map(|c|if c.is_ascii_alphanumeric()||c=='-'||c=='_' {c} else {'_'}).collect::<String>();
    let path=backup.join(format!("startup-registry-{safe}-{stamp}.txt"));
    let bytes=value.bytes.iter().map(|b|format!("{b:02X}")).collect::<String>();
    let mut file=OpenOptions::new().create_new(true).write(true).open(&path).map_err(|e|format!("Could not create backup: {e}"))?;
    writeln!(file,"Key: HKCU\\{sub}\nName: {name}\nType: {:?}\nDataHex: {bytes}",value.vtype).map_err(|e|e.to_string())?;
    key.delete_value(name).map_err(|e|format!("Backup saved but registry removal failed: {e}"))?;
    if key.get_raw_value(name).is_ok(){return Err(format!("Backup saved at {}; removal could not be verified.",path.display()));}
    Ok(format!("Removed HKCU startup value '{name}'. Backup: {}",path.display()))
}

#[cfg(windows)]
pub fn remove_current_user_startup_file(name: &str) -> Result<String, String> {
    use std::path::{Path,PathBuf};
    let name=name.trim();
    if name.is_empty() || name.chars().any(|c| c=='\\' || c=='/' || c=='\0') || name=="." || name==".." {return Err("Enter a file name only, not a path.".into());}
    let appdata=std::env::var_os("APPDATA").ok_or("APPDATA is unavailable")?;
    let folder=PathBuf::from(appdata).join("Microsoft\\Windows\\Start Menu\\Programs\\Startup");
    let source=folder.join(name);
    if !source.is_file(){return Err("No matching file exists in the current-user Startup folder.".into());}
    let profile=std::env::var_os("USERPROFILE").ok_or("User profile unavailable")?;
    let backup=PathBuf::from(profile).join("Documents").join("WinForge Studio").join("Backups").join("StartupFiles");
    std::fs::create_dir_all(&backup).map_err(|e|e.to_string())?;
    let mut destination=backup.join(name);
    if destination.exists(){let stamp=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|e|e.to_string())?.as_secs();destination=backup.join(format!("{stamp}-{}",name));}
    std::fs::copy(&source,&destination).map_err(|e|format!("Could not back up startup file: {e}"))?;
    if !Path::new(&destination).is_file(){return Err("Backup verification failed; original was not removed.".into());}
    std::fs::remove_file(&source).map_err(|e|format!("Backup exists at {}; removal failed: {e}",destination.display()))?;
    if source.exists(){return Err(format!("Backup exists at {}; removal could not be verified.",destination.display()));}
    Ok(format!("Removed startup file '{name}'. Backup: {}",destination.display()))
}

#[cfg(not(windows))]
pub fn remove_current_user_entry(_: &str)->Result<String,String>{Err("Available only on Windows.".into())}
#[cfg(not(windows))]
pub fn remove_current_user_startup_file(_: &str)->Result<String,String>{Err("Available only on Windows.".into())}

#[cfg(windows)]
pub fn restore_current_user_startup_file(backup_name:&str)->Result<String,String>{
    use std::path::PathBuf;
    let name=backup_name.trim();
    if name.is_empty() || name.chars().any(|c|c=='\\'||c=='/'||c=='\0') || name=="." || name==".." {return Err("Enter a backup file name only.".into());}
    let profile=std::env::var_os("USERPROFILE").ok_or("User profile unavailable")?;
    let backup=PathBuf::from(profile).join("Documents").join("WinForge Studio").join("Backups").join("StartupFiles").join(name);
    if !backup.is_file(){return Err("Backup file was not found in WinForge StartupFiles backup folder.".into());}
    let appdata=std::env::var_os("APPDATA").ok_or("APPDATA is unavailable")?;
    let folder=PathBuf::from(appdata).join("Microsoft\\Windows\\Start Menu\\Programs\\Startup");
    std::fs::create_dir_all(&folder).map_err(|e|e.to_string())?;
    let destination=folder.join(name);
    if destination.exists(){return Err("A Startup item with this name already exists; restore cancelled to avoid overwrite.".into());}
    std::fs::copy(&backup,&destination).map_err(|e|format!("Restore failed: {e}"))?;
    if !destination.is_file(){return Err("Restore could not be verified.".into());}
    Ok(format!("Restored '{name}' to the current-user Startup folder."))
}
#[cfg(not(windows))]
pub fn restore_current_user_startup_file(_: &str)->Result<String,String>{Err("Available only on Windows.".into())}
