# Security Policy

WinForge Studio can change Windows configuration. Treat safety defects as high priority.

## Reporting

Do not publish exploit details or personal system inventories in public issues. Open a private security advisory through GitHub's Security tab when available, or contact the repository owner privately.

## Security expectations

- No secrets or telemetry.
- No silent elevation.
- No automatic destructive changes.
- No disabling Windows security controls as a performance preset.
- Every mutation must be confirmed, journaled before execution, and verified after execution.
- Unknown services and packages must be treated as requiring review.
- Never execute remote scripts or binaries as part of a scan or optimization.
