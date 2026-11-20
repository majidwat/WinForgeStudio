use sysinfo::{System, SystemExt, CpuExt};

pub struct Snapshot {
    pub os_name: String,
    pub cpu_name: String,
    pub cpu_count: String,
    pub ram_total: String,
    pub ram_used: String,
    pub ram_percent: String,
    pub host_name: String,
}

pub fn snapshot() -> Snapshot {
    let mut sys = System::new_all();
    sys.refresh_all();

    let total = sys.total_memory();
    let used = sys.used_memory();
    let percent = if total > 0 {
        (used as f64 / total as f64 * 100.0).round() as u64
    } else {
        0
    };

    Snapshot {
        os_name: System::name().unwrap_or_else(|| "Windows".into()),
        cpu_name: sys.cpus().first().map(|c| c.brand().trim().to_string()).unwrap_or_else(|| "Unknown CPU".into()),
        cpu_count: sys.cpus().len().to_string(),
        ram_total: format_bytes(total),
        ram_used: format_bytes(used),
        ram_percent: format!("{percent}%"),
        host_name: System::host_name().unwrap_or_else(|| "PC".into()),
    }
}

fn format_bytes(bytes: u64) -> String {
    let gib = bytes as f64 / 1024.0 / 1024.0 / 1024.0;
    format!("{gib:.1} GB")
}
