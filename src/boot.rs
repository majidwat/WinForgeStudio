use std::process::Command;

pub fn inventory() -> (String, String) {
    #[cfg(windows)] {
        return match Command::new("bcdedit").args(["/enum","{current}"]).output() {
            Ok(o) if o.status.success() => ("Current boot entry".into(),String::from_utf8_lossy(&o.stdout).trim().to_string()),
            Ok(o) => ("Unavailable".into(),format!("BCDEdit returned an error: {}",String::from_utf8_lossy(&o.stderr))),
            Err(e) => ("Unavailable".into(),format!("Unable to run BCDEdit: {e}")),
        };
    }
    #[cfg(not(windows))] { ("—".into(),"Boot configuration is available on Windows only.".into()) }
}
