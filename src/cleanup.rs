use std::fs;
use std::path::{Path, PathBuf};

const MAX_DEPTH: usize = 4;
const MAX_ENTRIES: usize = 10_000;

#[derive(Default)]
struct Stats { files: u64, bytes: u64, visited: usize, inaccessible: u64, capped: bool }

fn scan_dir(path: &Path, depth: usize, stats: &mut Stats) {
    if depth > MAX_DEPTH || stats.visited >= MAX_ENTRIES { stats.capped = true; return; }
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => { stats.inaccessible += 1; return; }
    };
    for entry in entries {
        if stats.visited >= MAX_ENTRIES { stats.capped = true; break; }
        stats.visited += 1;
        let entry = match entry { Ok(e) => e, Err(_) => { stats.inaccessible += 1; continue; } };
        let kind = match entry.file_type() { Ok(t) => t, Err(_) => { stats.inaccessible += 1; continue; } };
        if kind.is_symlink() { continue; }
        if kind.is_dir() {
            if depth < MAX_DEPTH { scan_dir(&entry.path(), depth + 1, stats); } else { stats.capped = true; }
        } else if kind.is_file() {
            if let Ok(metadata) = entry.metadata() { stats.files += 1; stats.bytes = stats.bytes.saturating_add(metadata.len()); }
            else { stats.inaccessible += 1; }
        }
    }
}

fn roots() -> Vec<(String, PathBuf)> {
    let mut roots = Vec::new();
    if let Some(temp) = std::env::var_os("TEMP") { roots.push(("User temporary folder".into(), PathBuf::from(temp))); }
    if let Some(windir) = std::env::var_os("WINDIR") { roots.push(("Windows temporary folder".into(), PathBuf::from(windir).join("Temp"))); }
    roots
}

fn format_bytes(bytes: u64) -> String {
    if bytes >= 1024 * 1024 * 1024 { format!("{:.2} GB", bytes as f64 / 1024.0 / 1024.0 / 1024.0) }
    else if bytes >= 1024 * 1024 { format!("{:.1} MB", bytes as f64 / 1024.0 / 1024.0) }
    else { format!("{:.1} KB", bytes as f64 / 1024.0) }
}

pub fn inventory() -> (String, String) {
    #[cfg(windows)] {
        let mut all = Stats::default();
        let mut lines = Vec::new();
        for (label, path) in roots() {
            let before = all.bytes;
            let files_before = all.files;
            scan_dir(&path, 0, &mut all);
            lines.push(format!("{}\n  Path: {}\n  Files: {}\n  Estimated size: {}", label, path.display(), all.files - files_before, format_bytes(all.bytes - before)));
        }
        if lines.is_empty() { return ("Unavailable".into(), "Temporary folder paths were not available.".into()); }
        lines.push(format!("Total files: {}\nEstimated reclaimable space: {}\nAccess denied/unavailable items: {}", all.files, format_bytes(all.bytes), all.inaccessible));
        if all.capped { lines.push(format!("Scan capped at {} entries and depth {} for responsiveness; estimate may be incomplete.", MAX_ENTRIES, MAX_DEPTH)); }
        lines.push("Preview only: no files were deleted. Temporary files may be in use; review before cleanup.".into());
        return ("Temporary-file preview".into(), lines.join("\n\n"));
    }
    #[cfg(not(windows))]
    { ("—".into(), "Temporary-folder preview is available on Windows only.".into()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn byte_format_is_human_readable() {
        assert_eq!(format_bytes(1024 * 1024), "1.0 MB");
        assert_eq!(format_bytes(1024 * 1024 * 1024), "1.00 GB");
    }
    #[test]
    fn stats_saturate_on_addition() {
        let mut stats = Stats { bytes: u64::MAX - 2, ..Stats::default() };
        stats.bytes = stats.bytes.saturating_add(10);
        assert_eq!(stats.bytes, u64::MAX);
    }
}
