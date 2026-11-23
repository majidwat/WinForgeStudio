use std::{fs, path::PathBuf};

/// Reads the local action journal without modifying it.
pub fn read_history() -> String {
    #[cfg(windows)] {
        let profile = match std::env::var_os("USERPROFILE") {
            Some(p) => PathBuf::from(p),
            None => return "User profile unavailable.".into(),
        };
        let path = profile.join("Documents").join("WinForge Studio").join("Journal").join("actions.log");
        match fs::read_to_string(&path) {
            Ok(content) if content.trim().is_empty() => "Journal exists but contains no entries.".into(),
            Ok(content) => {
                let lines: Vec<&str> = content.lines().rev().take(500).collect();
                format!("Journal: {}\nShowing up to the latest 500 entries (newest first):\n{}", path.display(), lines.join("\n"))
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => "No action journal exists yet.".into(),
            Err(e) => format!("Unable to read action journal: {e}"),
        }
    }
    #[cfg(not(windows))]
    { "Action history is available on Windows only.".into() }
}
