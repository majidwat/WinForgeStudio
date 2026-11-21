use sysinfo::System;

pub struct Snapshot {
    pub os_name: String,
    pub cpu_name: String,
    pub cpu_count: String,
    pub ram_total: String,
    pub ram_used: String,
    pub ram_percent: String,
}

pub fn snapshot() -> Snapshot {
    let mut system = System::new_all();
    system.refresh_all();

    let total = system.total_memory();
    let used = system.used_memory();
    let percent = if total > 0 {
        (used as f64 / total as f64 * 100.0).round() as u64
    } else {
        0
    };

    Snapshot {
        os_name: System::name().unwrap_or_else(|| "Windows".into()),
        cpu_name: system.cpus().first().map(|cpu| cpu.brand().trim().to_string()).unwrap_or_else(|| "Unknown CPU".into()),
        cpu_count: system.cpus().len().to_string(),
        ram_total: format_gib(total),
        ram_used: format_gib(used),
        ram_percent: format!("{}%", percent),
    }
}


fn format_gib(bytes: u64) -> String {
    format!("{:.1} GB", bytes as f64 / 1024.0 / 1024.0 / 1024.0)
}
