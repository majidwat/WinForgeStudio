use std::process::Command;

pub fn inventory() -> (String, String) {
    #[cfg(windows)] {
        let script = "$items=Get-CimInstance SoftwareLicensingProduct -Filter \"PartialProductKey IS NOT NULL\" -ErrorAction SilentlyContinue | Where-Object { $_.Name -like '*Windows*' -and $_.ApplicationID -eq '55c92734-d682-4d71-983e-d6ec3f16059f' }; if ($items) { $items | Select-Object Name,Description,LicenseStatus,GracePeriodRemaining,PartialProductKey | ForEach-Object { $state=switch ($_.LicenseStatus) { 0 {'Unlicensed'} 1 {'Licensed'} 2 {'OOB grace'} 3 {'OOT grace'} 4 {'Non-genuine grace'} 5 {'Notification'} 6 {'Extended grace'} default {'Unknown'} }; [pscustomobject]@{Name=$_.Name; Description=$_.Description; Status=$state; GraceMinutes=$_.GracePeriodRemaining; ProductKeySuffix=$_.PartialProductKey} } | Format-List | Out-String -Width 180 } else { 'Windows licensing information is unavailable.' }; 'Only the final five product-key characters are queried; no full key is collected.'";
        return match Command::new("powershell").args(["-NoProfile","-NonInteractive","-Command",script]).output() {
            Ok(o) if o.status.success() => { let s=String::from_utf8_lossy(&o.stdout).trim().to_string(); ("Windows license".into(),s) },
            Ok(o) => ("Unavailable".into(),String::from_utf8_lossy(&o.stderr).to_string()),
            Err(e) => ("Unavailable".into(),format!("PowerShell could not start: {e}")),
        };
    }
    #[cfg(not(windows))] { ("—".into(),"Activation details are available on Windows only.".into()) }
}
