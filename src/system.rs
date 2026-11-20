use sysinfo::{System, SystemExt, CpuExt};
pub struct Snapshot { pub os_name:String,pub cpu_name:String,pub cpu_count:String,pub ram_total:String,pub ram_used:String,pub ram_percent:String }
pub fn snapshot()->Snapshot { let mut s=System::new_all(); s.refresh_all(); let total=s.total_memory(); let used=s.used_memory(); let pct=if total>0 {(used as f64/total as f64*100.0).round() as u64} else {0}; Snapshot{os_name:System::name().unwrap_or_else(||"Windows".into()),cpu_name:s.cpus().first().map(|c|c.brand().trim().to_string()).unwrap_or_else(||"Unknown CPU".into()),cpu_count:s.cpus().len().to_string(),ram_total:fmt(total),ram_used:fmt(used),ram_percent:format!("{pct}%")} }
fn fmt(b:u64)->String { format!("{:.1} GB",b as f64/1024.0/1024.0/1024.0) }
