mod system;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;
    let initial = system::snapshot();
    apply_snapshot(&ui, initial);

    let weak = ui.as_weak();
    ui.on_refresh_requested(move || {
        if let Some(ui) = weak.upgrade() {
            apply_snapshot(&ui, system::snapshot());
        }
    });

    ui.run()
}

fn apply_snapshot(ui: &MainWindow, data: system::Snapshot) {
    ui.set_os_name(data.os_name.into());
    ui.set_cpu_name(data.cpu_name.into());
    ui.set_cpu_count(data.cpu_count.into());
    ui.set_ram_total(data.ram_total.into());
    ui.set_ram_used(data.ram_used.into());
    ui.set_ram_percent(data.ram_percent.into());
    ui.set_host_name(data.host_name.into());
}
