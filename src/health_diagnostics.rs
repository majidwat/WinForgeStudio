use std::process::Command;

/// Runs read-only Windows image/system-file integrity checks. No repair operation is invoked.
pub fn run_read_only_checks() -> String {
    #[cfg(windows)] {
        let mut report = String::from("Read-only integrity diagnostics\n");
        for (label, program, args) in [
            ("DISM image CheckHealth", "dism", vec!["/Online", "/Cleanup-Image", "/CheckHealth"]),
            ("System File Checker verify-only", "sfc", vec!["/verifyonly"]),
        ] {
            report.push_str(&format!("\n== {label} ==\n"));
            match Command::new(program).args(args).output() {
                Ok(out) => {
                    report.push_str(&format!("Exit code: {:?}\n", out.status.code()));
                    report.push_str(&crate::output::bounded(&out.stdout));
                    if !out.stderr.is_empty() { report.push_str(&crate::output::bounded(&out.stderr)); }
                }
                Err(e) => report.push_str(&format!("Could not start {program}: {e}\n")),
            }
        }
        report.push_str("\nNo repair or file replacement was requested.");
        report
    }
    #[cfg(not(windows))]
    { "Windows-only diagnostics.".into() }
}
