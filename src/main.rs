mod system;
mod startup;
mod services;
mod network;
mod storage;
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
    ui.set_os_name(s.os_name.into()); ui.set_os_version(s.os_version.into()); ui.set_hostname(s.hostname.into());
    ui.set_uptime(s.uptime.into()); ui.set_cpu_name(s.cpu_name.into()); ui.set_cpu_count(s.cpu_count.into());
    ui.set_ram_total(s.ram_total.into()); ui.set_ram_used(s.ram_used.into()); ui.set_ram_percent(s.ram_percent.into());
    ui.set_process_count(s.process_count.into()); ui.set_top_processes(s.top_processes.into());
    ui.set_startup_count(startup_count.into()); ui.set_startup_items(startup_items.into());
    ui.set_service_count(service_count.into()); ui.set_service_items(service_items.into());
    ui.set_network_items(network_items.into()); ui.set_storage_items(storage_items.into());
}
