# GitHub Copilot instructions

Read the root `AGENTS.md` before changing code. It is the canonical project contract.

This application inspects and carefully manages a live Windows 10/11 installation. Scans must be read-only. Never add automatic debloating, bulk service disabling, security-control removal, hidden elevation, background telemetry, or downloaded script execution.

Current baseline: C# 12, .NET 8, WPF, Windows x64. Keep dependencies minimal and UI responsive. Put Windows API access in services/adapters, not XAML code-behind. Add tests for policy and safety decisions. Do not change framework/language without an Architecture Decision Record and owner approval. Update docs/ROADMAP.md only when work is actually implemented and verified.
