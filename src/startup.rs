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
        entries.sort();
        let count = entries.len().to_string();
        (count, entries.join("\n"))
    }
}
#[cfg(windows)]
pub fn inventory() -> (String, String) { windows::inventory() }

#[cfg(not(windows))]
pub fn inventory() -> (String, String) {
    ("0".into(), "Startup registry inventory is available on Windows.".into())
}
