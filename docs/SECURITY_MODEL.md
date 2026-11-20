# Security model
The migration preview is read-only. It does not change Windows configuration. Future mutating operations must have a reviewed allow-list, clear before/after preview, explicit confirmation, permission checks, journal, validation, and a documented recovery path. Never target security-critical services for disabling. Do not run downloaded scripts or elevate implicitly.
