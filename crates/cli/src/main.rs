//! Command-line interface for the `skrills` CLI.
//!
//! This is the main entry point for the `skrills` executable,
//! delegating to the `skrills-server` crate.

fn main() -> anyhow::Result<()> {
    skrills_server::run()
}
