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

#[cfg(windows)]
pub fn cleanup_user_temp_older_than(days: u64) -> Result<String,String> {
    if days < 7 || days > 365 { return Err("Cleanup age must be between 7 and 365 days.".into()); }
    let root=std::env::var_os("TEMP").ok_or("User TEMP directory is not defined")?;
    let root=PathBuf::from(root);
    let canonical=fs::canonicalize(&root).map_err(|e|format!("Cannot resolve user TEMP: {e}"))?;
    if !canonical.is_dir() { return Err("User TEMP path is not a directory.".into()); }
    let cutoff=std::time::SystemTime::now()-std::time::Duration::from_secs(days.saturating_mul(86400));
    let mut removed_files=0u64; let mut freed=0u64; let mut failed=0u64;
    fn walk(root:&Path,dir:&Path,cutoff:std::time::SystemTime,removed:&mut u64,freed:&mut u64,failed:&mut u64){
        let entries=match fs::read_dir(dir){Ok(e)=>e,Err(_)=>{*failed+=1;return}};
        for entry in entries.flatten(){
            let p=entry.path(); let ft=match entry.file_type(){Ok(t)=>t,Err(_)=>{*failed+=1;continue}};
            if ft.is_symlink(){continue;}
            if ft.is_dir(){walk(root,&p,cutoff,removed,freed,failed); let _=fs::remove_dir(&p); continue;}
            if ft.is_file(){
                let meta=match entry.metadata(){Ok(m)=>m,Err(_)=>{*failed+=1;continue}};
                if meta.modified().map(|m|m<cutoff).unwrap_or(false){
                    if let Ok(c)=fs::canonicalize(&p){if !c.starts_with(root){*failed+=1;continue;}}
                    match fs::remove_file(&p){Ok(())=>{*removed+=1;*freed=freed.saturating_add(meta.len());},Err(_)=>*failed+=1}
                }
            }
        }
    }
    walk(&canonical,&canonical,cutoff,&mut removed_files,&mut freed,&mut failed);
    let profile=std::env::var_os("USERPROFILE").ok_or("User profile unavailable")?;
    let logdir=PathBuf::from(profile).join("Documents").join("WinForge Studio").join("Journal");
    fs::create_dir_all(&logdir).map_err(|e|e.to_string())?;
    use std::io::Write; let mut log=fs::OpenOptions::new().create(true).append(true).open(logdir.join("actions.log")).map_err(|e|e.to_string())?;
    let stamp=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d|d.as_secs()).unwrap_or(0);
    writeln!(log,"{stamp} | CLEANUP user-temp older-than-days={days} removed-files={removed_files} bytes={freed} failures={failed}").map_err(|e|e.to_string())?;
    Ok(format!("Cleanup complete. Removed {removed_files} files older than {days} days; reclaimed approximately {} bytes. Failed/inaccessible items: {failed}. Only current-user TEMP was targeted; reparse points were skipped.",freed))
}
#[cfg(not(windows))]
pub fn cleanup_user_temp_older_than(_:u64)->Result<String,String>{Err("Cleanup is Windows-only.".into())}
