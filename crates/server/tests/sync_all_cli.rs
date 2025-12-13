//! CLI integration test for `skrills sync-all --from codex`.
//!
//! Verifies end-to-end argument plumbing copies Codex skills into Claude.

use std::path::PathBuf;
use std::{fs, process::Stdio};

use anyhow::Result;
use tokio::process::Command;

#[tokio::test]
async fn sync_all_cli_copies_skills_from_codex_to_claude() -> Result<()> {
    // Isolate filesystem side effects
    let tmp = tempfile::tempdir()?;
    std::env::set_var("HOME", tmp.path());

    // Seed a Codex skill
    let codex_skills = tmp.path().join(".codex/skills");
    fs::create_dir_all(&codex_skills)?;
    fs::write(codex_skills.join("cli-test.md"), "# CLI Test")?;

    // Reuse workspace target dir to avoid repeated rebuilds across tests
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crate dir")
        .parent()
        .expect("workspace root")
        .to_path_buf();
    let target_dir = std::env::var("CARGO_TARGET_DIR")
        .unwrap_or_else(|_| workspace_root.join("target").to_string_lossy().into_owned());
    let cargo_home = std::env::var("CARGO_HOME").unwrap_or_else(|_| {
        workspace_root
            .join(".cargo-home")
            .to_string_lossy()
            .into_owned()
    });

    // Run CLI sync-all from Codex
    let status = Command::new("cargo")
        .current_dir(&workspace_root)
        .env("HOME", tmp.path())
        .env("CARGO_HOME", &cargo_home)
        .env("CARGO_TARGET_DIR", &target_dir)
        .args(["run", "-p", "skrills", "--", "sync-all", "--from", "codex"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await?;

    assert!(
        status.success(),
        "sync-all command should succeed (status={status})"
    );

    let claude_skill = tmp.path().join(".claude/skills/cli-test.md");
    assert!(
        claude_skill.exists(),
        "Claude skills directory should receive synced skill"
    );

    Ok(())
}
