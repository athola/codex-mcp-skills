//! Persists application state: pinned skills, auto-pinning flags, and autoload history.
//!
//! It provides functions to load, save, and manage persistent data structures
//! for the `skrills` application.

use crate::env::home_dir;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;

/// Represents an entry in the history of autoloaded skills.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// Timestamp of the history entry.
    pub ts: u64,
    /// List of skills included in this entry.
    pub skills: Vec<String>,
}

/// Maximum number of history entries to retain.
const HISTORY_LIMIT: usize = 50;
/// Window size for auto-pinning history.
const AUTO_PIN_WINDOW: usize = 5;
/// Minimum number of hits within the window to auto-pin a skill.
const AUTO_PIN_MIN_HITS: usize = 2;

/// Returns the path to the file where manually pinned skills are persisted.
pub fn pinned_file() -> Result<PathBuf> {
    Ok(home_dir()?.join(".codex/skills-pinned.json"))
}

/// Returns the path to the file where the auto-pinning flag is persisted.
pub fn auto_pin_file() -> Result<PathBuf> {
    Ok(home_dir()?.join(".codex/skills-autopin.json"))
}

/// Returns the path to the file where the history of autoloaded skills is persisted.
pub fn history_file() -> Result<PathBuf> {
    Ok(home_dir()?.join(".codex/skills-history.json"))
}

/// Loads the set of manually pinned skills from the persistence file.
///
/// Returns an empty `HashSet` if the file does not exist.
pub fn load_pinned() -> Result<HashSet<String>> {
    let path = pinned_file()?;
    if !path.exists() {
        return Ok(HashSet::new());
    }
    let data = std::fs::read_to_string(path)?;
    let list: Vec<String> = serde_json::from_str(&data)?;
    Ok(list.into_iter().collect())
}

/// Loads pinned skills, merging persisted pins with env-provided defaults.
///
/// Sets `SKRILLS_PINNED` to a comma-separated list of skill names to pin at startup
/// without touching the persisted file.
pub fn load_pinned_with_defaults() -> Result<HashSet<String>> {
    let mut pins = load_pinned()?;
    if let Ok(env) = std::env::var("SKRILLS_PINNED") {
        for item in env.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
            pins.insert(item.to_string());
        }
    }
    Ok(pins)
}

/// Saves the current set of manually pinned skills to the persistence file.
///
/// Creates parent directories if they do not exist.
pub fn save_pinned(pinned: &HashSet<String>) -> Result<()> {
    let path = pinned_file()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let list: Vec<&String> = pinned.iter().collect();
    std::fs::write(path, serde_json::to_string_pretty(&list)?)?;
    Ok(())
}

/// Loads the auto-pinning flag from its persistence file.
///
/// Returns `false` if the file does not exist.
pub fn load_auto_pin_flag() -> Result<bool> {
    let path = auto_pin_file()?;
    if !path.exists() {
        return Ok(false);
    }
    let data = std::fs::read_to_string(path)?;
    serde_json::from_str(&data).map_err(Into::into)
}

/// Saves the current auto-pinning flag to its persistence file.
///
/// Creates parent directories if they do not exist.
pub fn save_auto_pin_flag(value: bool) -> Result<()> {
    let path = auto_pin_file()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, serde_json::to_string_pretty(&value)?)?;
    Ok(())
}

/// Loads the history of autoloaded skills from the persistence file.
///
/// Returns an empty `Vec` if the file does not exist. Truncates history
/// to `HISTORY_LIMIT` if it exceeds this limit.
pub fn load_history() -> Result<Vec<HistoryEntry>> {
    let path = history_file()?;
    if !path.exists() {
        return Ok(Vec::new());
    }
    let data = std::fs::read_to_string(path)?;
    let mut list: Vec<HistoryEntry> = serde_json::from_str(&data)?;
    if list.len() > HISTORY_LIMIT {
        list.drain(0..list.len() - HISTORY_LIMIT);
    }
    Ok(list)
}

/// Saves the current history of autoloaded skills to the persistence file.
///
/// Truncates the history to `HISTORY_LIMIT` if it exceeds this limit.
/// Creates parent directories if they do not exist.
pub fn save_history(mut history: Vec<HistoryEntry>) -> Result<()> {
    if history.len() > HISTORY_LIMIT {
        history.drain(0..history.len() - HISTORY_LIMIT);
    }
    let path = history_file()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, serde_json::to_string_pretty(&history)?)?;
    Ok(())
}

/// Determines which skills to auto-pin based on recent usage history.
///
/// Considers skills that appear at least `AUTO_PIN_MIN_HITS` times
/// within the last `AUTO_PIN_WINDOW` history entries.
pub fn auto_pin_from_history(history: &[HistoryEntry]) -> HashSet<String> {
    let mut counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    let window_iter = history.iter().rev().take(AUTO_PIN_WINDOW);
    for entry in window_iter {
        for skill in entry.skills.iter() {
            *counts.entry(skill.as_str()).or_default() += 1;
        }
    }
    counts
        .into_iter()
        .filter(|(_, c)| *c >= AUTO_PIN_MIN_HITS)
        .map(|(s, _)| s.to_string())
        .collect()
}

/// Prints a formatted list of recent history entries to stdout.
///
/// Limits the number of entries by the `limit` parameter.
pub fn print_history(limit: usize) -> Result<()> {
    let history = load_history().unwrap_or_default();
    let mut entries: Vec<_> = history.into_iter().rev().take(limit).collect();
    if entries.is_empty() {
        println!("(no history)");
        return Ok(());
    }
    for entry in entries.drain(..) {
        println!("{} | {}", entry.ts, entry.skills.join(", "));
    }
    Ok(())
}
