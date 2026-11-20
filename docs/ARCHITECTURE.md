# Architecture
Slint UI -> thin callbacks -> Rust app layer -> system inspection. Safety engine gates future mutations. Current preview reads CPU, RAM and OS snapshot only. Planned modules: processes, services, startup, network, storage, settings, localization, safety journal and recovery. Long-running scans must not block UI.
