use sysinfo::System;

pub struct Snapshot {
    pub os_name: String, pub os_version: String, pub hostname: String, pub uptime: String,
    pub cpu_name: String, pub cpu_count: String, pub ram_total: String, pub ram_used: String,
    pub ram_percent: String, pub process_count: String, pub top_processes: String,
}
pub fn snapshot() -> Snapshot {
    let mut system = System::new_all(); system.refresh_all();
    let total = system.total_memory(); let used = system.used_memory();
    let percent = if total > 0 { (used as f64 / total as f64 * 100.0).round() as u64 } else { 0 };
    let uptime_seconds = System::uptime();
    let uptime = format!("{}d {}h {}m", uptime_seconds / 86_400, (uptime_seconds % 86_400) / 3_600, (uptime_seconds % 3_600) / 60);
    let mut processes: Vec<(String, String, u64, f32)> = system.processes().values()
        .map(|p| (p.pid().to_string(), p.name().to_string(), p.memory(), p.cpu_usage()))
        .collect();
    processes.sort_by(|a,b| b.2.cmp(&a.2));
    let process_count = processes.len().to_string();
    let top_processes = format!("{:<8} {:<30} {:>10} {:>8}\\n{}", "PID", "PROCESS", "RAM (MB)", "CPU %", processes.iter().take(20)
        .map(|(pid, name, memory, cpu)| format!("{:<8} {:<30} {:>10.1} {:>7.1}", pid, name.chars().take(29).collect::<String>(), *memory as f64 / 1024.0, cpu))
        .collect::<Vec<_>>().join("\\n"));
    Snapshot {
        os_name: System::name().unwrap_or_else(|| "Windows".into()), os_version: System::os_version().unwrap_or_else(|| "Unknown".into()),
        hostname: System::host_name().unwrap_or_else(|| "Unknown".into()), uptime,
        cpu_name: system.cpus().first().map(|c| c.brand().trim().to_string()).unwrap_or_else(|| "Unknown CPU".into()),
        cpu_count: system.cpus().len().to_string(), ram_total: format_gib(total), ram_used: format_gib(used),
        ram_percent: format!("{}%", percent), process_count, top_processes,
    }
}
fn format_gib(bytes: u64) -> String { format!("{:.1} GB", bytes as f64 / 1024.0 / 1024.0 / 1024.0) }
