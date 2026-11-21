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
slint::include_modules!();
fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?; apply_snapshot(&ui);
    let weak = ui.as_weak();
    ui.on_refresh_requested(move || { if let Some(ui) = weak.upgrade() { apply_snapshot(&ui); } });
    ui.run()
}
fn apply_snapshot(ui: &MainWindow) {
    let s = system::snapshot();
    let (startup_count, startup_items) = startup::inventory();
    let (service_count, service_items) = services::inventory();
    let network_items = network::inventory();
    let storage_items = storage::inventory();
    let (apps_count, apps_items) = apps::inventory();
    let (drivers_count, drivers_items) = drivers::inventory();
    let (tasks_count, tasks_items) = tasks::inventory();
    let (events_count, events_items) = event_logs::inventory();
    let (updates_count, updates_items) = updates::inventory();
    let (firewall_count, firewall_items) = firewall::inventory();
    let (restore_count, restore_items) = restore_points::inventory();
    let (environment_count, environment_items) = environment::inventory();
    ui.set_os_name(s.os_name.into()); ui.set_os_version(s.os_version.into()); ui.set_hostname(s.hostname.into());
    ui.set_uptime(s.uptime.into()); ui.set_cpu_name(s.cpu_name.into()); ui.set_cpu_count(s.cpu_count.into());
    ui.set_ram_total(s.ram_total.into()); ui.set_ram_used(s.ram_used.into()); ui.set_ram_percent(s.ram_percent.into());
    ui.set_process_count(s.process_count.into()); ui.set_top_processes(s.top_processes.into());
    ui.set_startup_count(startup_count.into()); ui.set_startup_items(startup_items.into());
    ui.set_service_count(service_count.into()); ui.set_service_items(service_items.into());
    ui.set_network_items(network_items.into()); ui.set_storage_items(storage_items.into());
    ui.set_apps_count(apps_count.into()); ui.set_apps_items(apps_items.into());
    ui.set_drivers_count(drivers_count.into()); ui.set_drivers_items(drivers_items.into());
    ui.set_tasks_count(tasks_count.into()); ui.set_tasks_items(tasks_items.into());
    ui.set_events_count(events_count.into()); ui.set_events_items(events_items.into());
    ui.set_updates_count(updates_count.into()); ui.set_updates_items(updates_items.into());
    ui.set_firewall_count(firewall_count.into()); ui.set_firewall_items(firewall_items.into());
    ui.set_restore_count(restore_count.into()); ui.set_restore_items(restore_items.into());
    ui.set_environment_count(environment_count.into()); ui.set_environment_items(environment_items.into());
}
