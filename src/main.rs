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
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;
    let weak = ui.as_weak();
    refresh_async(weak.clone());
    ui.on_refresh_requested(move || refresh_async(weak.clone()));
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
    InventorySnapshot {
        system: system::snapshot(),
        startup: startup::inventory(),
        services: services::inventory(),
        network: network::inventory(),
        storage: storage::inventory(),
        apps: apps::inventory(),
        drivers: drivers::inventory(),
        tasks: tasks::inventory(),
        events: event_logs::inventory(),
        updates: updates::inventory(),
        firewall: firewall::inventory(),
        restore: restore_points::inventory(),
        environment: environment::inventory(),
        battery: battery::inventory(),
        security: security::inventory(),
        activation: activation::inventory(),
        adapters: adapters::inventory(),
        boot: boot::inventory(),
    }
}

fn apply_snapshot(ui: &MainWindow, data: InventorySnapshot) {
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
}
