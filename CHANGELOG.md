# Changelog

## Unreleased
- Added read-only diagnostics for security, battery, Windows activation, network adapters and boot configuration.
- Added power scheme, Plug and Play device-health, and system hardware inventories.
- Moved full inventory collection to a worker thread and added scan status/single-scan protection.
- Capped command output before it reaches the UI and added UTF-8-safe truncation tests.
- Added Windows CI tests, release build and downloadable executable artifact.

## 0.2.0
- Added read-only inventories for services, startup, network, storage, installed apps, drivers, tasks, event logs, updates, firewall, restore points and environment variables.
