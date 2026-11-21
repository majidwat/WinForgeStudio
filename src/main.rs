mod system;
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;
    apply_snapshot(&ui);
    let weak = ui.as_weak();
    ui.on_refresh_requested(move || {
        if let Some(ui) = weak.upgrade() {
            apply_snapshot(&ui);
        }
    });
    ui.run()
}

fn apply_snapshot(ui: &MainWindow) {
    let snapshot = system::snapshot();
    ui.set_os_name(snapshot.os_name.into());
    ui.set_os_version(snapshot.os_version.into());
    ui.set_hostname(snapshot.hostname.into());
    ui.set_uptime(snapshot.uptime.into());
    ui.set_cpu_name(snapshot.cpu_name.into());
    ui.set_cpu_count(snapshot.cpu_count.into());
    ui.set_ram_total(snapshot.ram_total.into());
    ui.set_ram_used(snapshot.ram_used.into());
    ui.set_ram_percent(snapshot.ram_percent.into());
}
