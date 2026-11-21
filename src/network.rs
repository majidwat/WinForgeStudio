use std::process::Command;

pub fn inventory() -> String {
    #[cfg(windows)]
    {
        match Command::new("ipconfig").arg("/all").output() {
            Ok(out) if out.status.success() => crate::output::bounded(&out.stdout),
            Ok(out) => format!("ipconfig failed: {}", String::from_utf8_lossy(&out.stderr)),
            Err(e) => format!("Unable to run ipconfig: {}", e),
        }
    }
    #[cfg(not(windows))]
    { "Network inventory is only available on Windows.".into() }
}
