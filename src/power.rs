use std::process::Command;

pub fn inventory() -> (String, String) {
    #[cfg(windows)] {
        return match Command::new("powercfg").args(["/list"]).output() {
            Ok(o) if o.status.success() => {
                let s=crate::output::bounded(&o.stdout).trim().to_string();
                let count=s.lines().filter(|line| line.contains("GUID:")).count().to_string();
                ("Power schemes: ".to_owned()+&count,s)
            },
            Ok(o) => ("Unavailable".into(),String::from_utf8_lossy(&o.stderr).to_string()),
            Err(e) => ("Unavailable".into(),format!("Unable to run powercfg: {e}")),
        };
    }
    #[cfg(not(windows))] { ("—".into(),"Power plan inventory is available on Windows only.".into()) }
}
