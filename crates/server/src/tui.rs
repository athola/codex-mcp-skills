//! Interactive terminal UI for skill management.
//!
//! It provides a user-friendly interface to:
//! - Synchronizing skills from `~/.claude`.
//! - Selecting and pinning skills.
//! - Managing skill visibility.

use anyhow::{anyhow, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect};
use skrills_state::{home_dir, load_pinned, save_pinned};
use std::collections::HashSet;
use std::io::IsTerminal;
use std::path::PathBuf;

use crate::discovery::collect_skills;
use crate::sync::sync_from_claude;

/// Runs an interactive TUI for sync and pin management.
///
/// Users can:
/// 1. Optionally sync skills from `~/.claude` to `~/.codex/skills-mirror`.
/// 2. Select which skills to pin for autoload.
pub(crate) fn tui_flow(extra_dirs: &[PathBuf]) -> Result<()> {
    if !std::io::stdout().is_terminal() {
        return Err(anyhow!("TUI requires a TTY"));
    }
    let theme = ColorfulTheme::default();

    if Confirm::with_theme(&theme)
        .with_prompt("Run claude → codex mirror sync first?")
        .default(false)
        .interact()?
    {
        let home = home_dir()?;
        let report = sync_from_claude(&home.join(".claude"), &home.join(".codex/skills-mirror"))?;
        println!(
            "Mirror sync complete (copied: {}, skipped: {})",
            report.copied, report.skipped
        );
    }

    let skills = collect_skills(extra_dirs)?;
    if skills.is_empty() {
        println!("No skills found.");
        return Ok(());
    }

    let pinned = load_pinned().unwrap_or_default();
    let mut items = Vec::new();
    let mut defaults = Vec::new();
    for s in skills.iter() {
        let display = format!(
            "[{} | {}] {}",
            s.source.label(),
            s.source.location(),
            s.name
        );
        items.push(display);
        defaults.push(pinned.contains(&s.name));
    }

    let selected = MultiSelect::with_theme(&theme)
        .with_prompt("Select skills to pin (space to toggle, enter to save)")
        .items(&items)
        .defaults(&defaults)
        .interact()?;

    let mut new_pins = HashSet::new();
    for idx in selected {
        new_pins.insert(skills[idx].name.clone());
    }
    save_pinned(&new_pins)?;
    println!("Pinned {} skills.", new_pins.len());
    Ok(())
}
