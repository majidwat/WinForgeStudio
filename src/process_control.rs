use std::{fs::OpenOptions, io::Write, path::PathBuf};
use sysinfo::{Pid, System};

fn journal_path() -> Result<PathBuf, String> {
    let profile = std::env::var_os("USERPROFILE").ok_or("User profile is unavailable")?;
    let dir = PathBuf::from(profile).join("Documents").join("WinForge Studio").join("Journal");
    std::fs::create_dir_all(&dir).map_err(|e| format!("Cannot create journal folder: {e}"))?;
    Ok(dir.join("actions.log"))
}

fn append_journal(message: &str) -> Result<(), String> {
    let path = journal_path()?;
    let mut file = OpenOptions::new().create(true).append(true).open(path).map_err(|e| format!("Cannot open action journal: {e}"))?;
    writeln!(file, "{} | {}", chrono_stamp(), message).map_err(|e| format!("Cannot write action journal: {e}"))
}

fn chrono_stamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs().to_string()).unwrap_or_else(|_| "unknown-time".into())
}

fn protected_name(name: &str) -> bool {
    matches!(name.to_ascii_lowercase().as_str(),
        "system" | "system idle process" | "registry" | "smss.exe" | "csrss.exe" |
        "wininit.exe" | "services.exe" | "lsass.exe" | "winlogon.exe" |
        "svchost.exe" | "fontdrvhost.exe" | "dwm.exe" | "explorer.exe")
}

/// Terminate only after an explicit UI confirmation. The expected image name is
/// compared against a fresh process snapshot to reduce PID-reuse mistakes.
pub fn terminate(pid_value: i32, expected_name: &str) -> Result<String, String> {
    if pid_value <= 0 { return Err("A valid positive PID is required.".into()); }
    let expected = expected_name.trim();
    if expected.is_empty() { return Err("Enter the exact process image name shown by Windows.".into()); }
    if protected_name(expected) { return Err("This process is protected by WinForge safety policy.".into()); }

    let pid = Pid::from_u32(pid_value as u32);
    let mut system = System::new_all();
    system.refresh_all();
    let process = system.process(pid).ok_or("The process no longer exists. Refresh the inventory.")?;
    let actual = process.name().to_string();
    if !actual.eq_ignore_ascii_case(expected) {
        return Err(format!("PID identity mismatch: current image is '{actual}', not '{expected}'. Refresh and verify."));
    }
    if protected_name(&actual) { return Err("This process is protected by WinForge safety policy.".into()); }

    append_journal(&format!("REQUEST terminate pid={pid_value} image={actual}"))?;
    system.refresh_all();
    let process = system.process(pid).ok_or("The process exited before termination.")?;
    if !process.name().eq_ignore_ascii_case(expected) {
        return Err("PID identity changed before termination; action cancelled.".into());
    }
    if !process.kill() { return Err("Windows refused to terminate this process. Try a permitted administrator session if appropriate.".into()); }
    system.refresh_all();
    if system.process(pid).is_some() {
        append_journal(&format!("VERIFY pending pid={pid_value} image={actual}"))?;
        return Err("Termination was requested, but the process is still present. Check permissions and refresh.");
    }
    append_journal(&format!("SUCCESS terminate pid={pid_value} image={actual}"))?;
    Ok(format!("Terminated {actual} (PID {pid_value})."))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn protects_windows_core_processes_case_insensitively() {
        assert!(protected_name("LSASS.EXE"));
        assert!(protected_name("System"));
    }
    #[test]
    fn allows_ordinary_application_names() {
        assert!(!protected_name("notepad.exe"));
    }
    #[test]
    fn rejects_invalid_pid_and_missing_name() {
        assert!(terminate(0, "notepad.exe").is_err());
        assert!(terminate(123, " ").is_err());
    }
}
