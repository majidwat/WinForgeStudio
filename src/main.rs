mod cleanup;
mod privacy;
mod hardware;
mod device_health;
mod power;
mod output;
mod boot;
mod adapters;
mod activation;
mod security;
mod battery;
mod system;
mod startup;
mod services;
mod network;
mod storage;
mod apps;
mod drivers;
mod tasks;
mod event_logs;
mod updates;
mod firewall;
mod restore_points;
mod environment;
mod process_control;

use std::sync::atomic::{AtomicBool, Ordering};

slint::include_modules!();

static SCAN_RUNNING: AtomicBool = AtomicBool::new(false);

struct InventorySnapshot {
    system: system::Snapshot,
    startup: (String, String),
    services: (String, String),
    network: String,
    storage: String,
    apps: (String, String),
    drivers: (String, String),
    tasks: (String, String),
    events: (String, String),
    updates: (String, String),
    firewall: (String, String),
    restore: (String, String),
    environment: (String, String),
    battery: (String, String),
    security: (String, String),
    activation: (String, String),
    adapters: (String, String),
    boot: (String, String),
    power: (String, String),
    device_health: (String, String),
    hardware: (String, String),
    privacy: (String, String),
    health: String,
    cleanup: (String, String),
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;
    let weak = ui.as_weak();
    refresh_async(weak.clone());
    ui.on_refresh_requested(move || refresh_async(weak.clone()));
    let action_weak = ui.as_weak();
    ui.on_terminate_process_requested(move |pid, expected_name| {
        let status = match process_control::terminate(pid, &expected_name) { Ok(message) => message, Err(error) => format!("Action blocked/failed: {error}") };
        if let Some(window) = action_weak.upgrade() { window.set_scan_status(status.into()); }
        refresh_async(action_weak.clone());
    });
    let export_weak = ui.as_weak();
    ui.on_export_requested(move || { if let Some(window) = export_weak.upgrade() { export_report(window.get_report_content().to_string(), export_weak.clone()); } });
    ui.run()
}

fn refresh_async(weak: slint::Weak<MainWindow>) {
    if SCAN_RUNNING.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
        return;
    }
    if let Some(ui) = weak.upgrade() {
        ui.set_scan_status("Scanning Windows…".into());
    }
    std::thread::spawn(move || {
        let snapshot = collect_snapshot();
        let result = slint::invoke_from_event_loop(move || {
            if let Some(ui) = weak.upgrade() {
                apply_snapshot(&ui, snapshot);
                ui.set_scan_status("Scan complete · read-only".into());
            }
            SCAN_RUNNING.store(false, Ordering::Release);
        });
        if result.is_err() {
            SCAN_RUNNING.store(false, Ordering::Release);
        }
    });
}

fn collect_snapshot() -> InventorySnapshot {
    let security = security::inventory();
    let device_health = device_health::inventory();
    let updates = updates::inventory();
    let system = system::snapshot();
    let health = health_summary(&system, &security, &device_health, &updates);
    InventorySnapshot {
        system,
        startup: startup::inventory(),
        services: services::inventory(),
        network: network::inventory(),
        storage: storage::inventory(),
        apps: apps::inventory(),
        drivers: drivers::inventory(),
        tasks: tasks::inventory(),
        events: event_logs::inventory(),
        updates,
        firewall: firewall::inventory(),
        restore: restore_points::inventory(),
        environment: environment::inventory(),
        battery: battery::inventory(),
        security,
        activation: activation::inventory(),
        adapters: adapters::inventory(),
        boot: boot::inventory(),
        power: power::inventory(),
        device_health,
        hardware: hardware::inventory(),
        privacy: privacy::inventory(),
        health,
        cleanup: cleanup::inventory(),
    }
}

fn health_summary(system: &system::Snapshot, security: &(String, String), devices: &(String, String), updates: &(String, String)) -> String {
    let mut notes = Vec::new();
    if let Some(percent) = system.ram_percent.trim_end_matches('%').parse::<u64>().ok() {
        if percent >= 90 { notes.push(format!("High memory usage: {}. Review heavy apps on the Processes page.", system.ram_percent)); }
        else if percent >= 75 { notes.push(format!("Memory usage is elevated: {}.", system.ram_percent)); }
        else { notes.push(format!("Memory usage looks moderate: {}.", system.ram_percent)); }
    } else { notes.push("Memory usage could not be assessed.".into()); }

    let sec = security.1.to_lowercase();
    if sec.contains("real-time protection: false") { notes.push("Defender real-time protection reports disabled. Review Windows Security; no changes were made.".into()); }
    else if sec.contains("real-time protection: true") { notes.push("Defender real-time protection reports enabled.".into()); }
    else { notes.push("Defender real-time protection status was not confirmed.".into()); }

    let device_text = devices.1.to_lowercase();
    if device_text.contains("devices reporting a problem: 0") || device_text.contains("no plug and play device problem codes") {
        notes.push("No Plug and Play problem codes were reported.".into());
    } else if device_text.contains("devices reporting a problem:") {
        notes.push("Some Plug and Play devices report problems. Review the Device Health page before making changes.".into());
    } else { notes.push("Device health was not fully assessed.".into()); }

    let update_text = updates.1.to_lowercase();
    if update_text.contains("pending reboot indicators: true") { notes.push("Windows reports a pending restart indicator.".into()); }
    else if update_text.contains("pending reboot indicators: false") { notes.push("No pending restart indicator was detected by these checks.".into()); }
    else { notes.push("Pending restart status is unknown.".into()); }
    notes.join("\n• ")
}

fn build_report(data: &InventorySnapshot) -> String {
    let mut report = String::from("WinForge Studio - Read-only system report\nGenerated locally; no changes were made.\n\n");
    report.push_str(&format!("OS: {} {}\nHost: {}\nUptime: {}\nCPU: {} ({} logical)\nRAM: {} / {} ({})\nProcesses: {}\n\n",
        data.system.os_name, data.system.os_version, data.system.hostname, data.system.uptime, data.system.cpu_name, data.system.cpu_count,
        data.system.ram_used, data.system.ram_total, data.system.ram_percent, data.system.process_count));
    for (name, value) in [
        ("Startup", &data.startup.1), ("Services", &data.services.1), ("Network", &data.network),
        ("Storage", &data.storage), ("Installed apps", &data.apps.1), ("Drivers", &data.drivers.1),
        ("Scheduled tasks", &data.tasks.1), ("System events", &data.events.1), ("Windows updates", &data.updates.1),
        ("Firewall", &data.firewall.1), ("Restore points", &data.restore.1), ("Environment (redacted)", &data.environment.1),
        ("Battery", &data.battery.1), ("Security", &data.security.1), ("Activation", &data.activation.1),
        ("Network adapters", &data.adapters.1), ("Boot", &data.boot.1), ("Power plans", &data.power.1),
        ("Device health", &data.device_health.1), ("Hardware", &data.hardware.1), ("Privacy", &data.privacy.1),
        ("Health review", &data.health), ("Cleanup preview", &data.cleanup.1),
    ] {
        report.push_str(&format!("\n===== {} =====\n{}\n", name, value));
    }
    crate::output::bounded(report.as_bytes())
}

fn export_report(report: String, weak: slint::Weak<MainWindow>) {
    std::thread::spawn(move || {
        let outcome = (|| -> Result<std::path::PathBuf, String> {
            let profile = std::env::var_os("USERPROFILE").ok_or("User profile path is unavailable")?;
            let folder = std::path::PathBuf::from(profile).join("Documents").join("WinForge Studio").join("Reports");
            std::fs::create_dir_all(&folder).map_err(|e| format!("Could not create report folder: {e}"))?;
            let stamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|e| e.to_string())?.as_secs();
            let path = folder.join(format!("system-report-{stamp}.txt"));
            std::fs::write(&path, report.as_bytes()).map_err(|e| format!("Could not write report: {e}"))?;
            Ok(path)
        })();
        let status = match outcome { Ok(path) => format!("Report saved: {}", path.display()), Err(error) => format!("Export failed: {error}") };
        let _ = slint::invoke_from_event_loop(move || if let Some(ui) = weak.upgrade() { ui.set_scan_status(status.into()); });
    });
}

fn apply_snapshot(ui: &MainWindow, data: InventorySnapshot) {
    let report = build_report(&data);
    let s = data.system;
    ui.set_os_name(s.os_name.into());
    ui.set_os_version(s.os_version.into());
    ui.set_hostname(s.hostname.into());
    ui.set_uptime(s.uptime.into());
    ui.set_cpu_name(s.cpu_name.into());
    ui.set_cpu_count(s.cpu_count.into());
    ui.set_ram_total(s.ram_total.into());
    ui.set_ram_used(s.ram_used.into());
    ui.set_ram_percent(s.ram_percent.into());
    ui.set_process_count(s.process_count.into());
    ui.set_top_processes(s.top_processes.into());

    ui.set_startup_count(data.startup.0.into());
    ui.set_startup_items(data.startup.1.into());
    ui.set_service_count(data.services.0.into());
    ui.set_service_items(data.services.1.into());
    ui.set_network_items(data.network.into());
    ui.set_storage_items(data.storage.into());
    ui.set_apps_count(data.apps.0.into());
    ui.set_apps_items(data.apps.1.into());
    ui.set_drivers_count(data.drivers.0.into());
    ui.set_drivers_items(data.drivers.1.into());
    ui.set_tasks_count(data.tasks.0.into());
    ui.set_tasks_items(data.tasks.1.into());
    ui.set_events_count(data.events.0.into());
    ui.set_events_items(data.events.1.into());
    ui.set_updates_count(data.updates.0.into());
    ui.set_updates_items(data.updates.1.into());
    ui.set_firewall_count(data.firewall.0.into());
    ui.set_firewall_items(data.firewall.1.into());
    ui.set_restore_count(data.restore.0.into());
    ui.set_restore_items(data.restore.1.into());
    ui.set_environment_count(data.environment.0.into());
    ui.set_environment_items(data.environment.1.into());
    ui.set_battery_count(data.battery.0.into());
    ui.set_battery_items(data.battery.1.into());
    ui.set_security_count(data.security.0.into());
    ui.set_security_items(data.security.1.into());
    ui.set_activation_count(data.activation.0.into());
    ui.set_activation_items(data.activation.1.into());
    ui.set_adapter_count(data.adapters.0.into());
    ui.set_adapter_items(data.adapters.1.into());
    ui.set_boot_count(data.boot.0.into());
    ui.set_boot_items(data.boot.1.into());
    ui.set_power_count(data.power.0.into());
    ui.set_power_items(data.power.1.into());
    ui.set_device_health_count(data.device_health.0.into());
    ui.set_device_health_items(data.device_health.1.into());
    ui.set_hardware_count(data.hardware.0.into());
    ui.set_hardware_items(data.hardware.1.into());
    ui.set_privacy_count(data.privacy.0.into());
    ui.set_privacy_items(data.privacy.1.into());
    ui.set_health_summary(data.health.into());
    ui.set_cleanup_count(data.cleanup.0.into());
    ui.set_cleanup_items(data.cleanup.1.into());
    ui.set_report_content(report.into());
}

#[cfg(test)]
mod health_tests {
    use super::*;
    fn snapshot(percent: &str) -> system::Snapshot {
        system::Snapshot {
            os_name: "Windows".into(), os_version: "test".into(), hostname: "host".into(), uptime: "1d".into(),
            cpu_name: "CPU".into(), cpu_count: "8".into(), ram_total: "16 GB".into(), ram_used: "8 GB".into(),
            ram_percent: percent.into(), process_count: "1".into(), top_processes: "test".into(),
        }
    }
    #[test]
    fn reports_high_memory_usage() {
        let sec=("ok".into(),"Real-time protection: True".into());
        let dev=("Devices reporting a problem: 0".into(),"No Plug and Play device problem codes were reported.".into());
        let upd=("ok".into(),"Pending reboot indicators: False".into());
        let result=health_summary(&snapshot("95%"),&sec,&dev,&upd);
        assert!(result.contains("High memory usage"));
        assert!(result.contains("protection reports enabled"));
        assert!(result.contains("No Plug and Play"));
    }
    #[test]
    fn reports_unknown_indicators_without_false_assurance() {
        let sec=("Unavailable".into(),"Unavailable".into());
        let dev=("Unavailable".into(),"Unavailable".into());
        let upd=("Unavailable".into(),"Unavailable".into());
        let result=health_summary(&snapshot("—"),&sec,&dev,&upd);
        assert!(result.contains("could not be assessed"));
        assert!(result.contains("was not confirmed"));
        assert!(result.contains("unknown"));
    }
}
