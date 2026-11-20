# Build instructions
Install Rust stable with rustup on Windows. From the repository root, run `cargo run` for a debug build or `cargo build --release` for an optimized build. The first build downloads dependencies and can take longer. Windows CI is configured to compile the release target. Build status is not considered verified until a workflow run completes successfully.
