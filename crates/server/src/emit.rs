//! Autoload emission for shell hook integration.
//!
//! Emits JSON payloads for Claude Code hooks, enabling dynamic skill injection
//! based on prompts and configuration.

use anyhow::Result;
use serde::Deserialize;
use skrills_discovery::{discover_skills, extract_refs_from_agents, Diagnostics};
use skrills_state::{
    auto_pin_from_history, env_max_bytes, load_history, load_pinned, save_history, HistoryEntry,
};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::autoload::{env_embed_threshold, render_autoload, AutoloadOptions};
use crate::discovery::{agents_manifest, collect_skills, skill_roots};
use crate::runtime::runtime_overrides_cached;

/// Configuration for autoload emission, typically deserialized from JSON.
#[derive(Deserialize, Default)]
pub(crate) struct AutoloadArgs {
    /// Include skills from the `~/.claude` directory.
    pub(crate) include_claude: Option<bool>,
    /// Maximum number of bytes for the autoloaded content.
    pub(crate) max_bytes: Option<usize>,
    /// Prompt string to filter relevant skills.
    pub(crate) prompt: Option<String>,
    /// Embedding similarity threshold (0-1) for fuzzy prompt matching.
    pub(crate) embed_threshold: Option<f32>,
    /// Enable heuristic auto-pinning based on recent prompt matches.
    pub(crate) auto_pin: Option<bool>,
    /// Emit diagnostic information (included/skipped skills).
    pub(crate) diagnose: Option<bool>,
}

/// Determines the appropriate render mode based on runtime configuration.
///
/// Uses runtime overrides and client capabilities to select between:
/// - `Dual`: Manifest + content (default)
/// - `ManifestOnly`: Just the manifest
/// - `ContentOnly`: Legacy mode
fn manifest_render_mode(
    runtime: &crate::runtime::RuntimeOverrides,
    _peer_info: Option<&rmcp::model::ClientInfo>,
) -> crate::autoload::RenderMode {
    if runtime.manifest_first() {
        crate::autoload::RenderMode::ManifestOnly
    } else {
        crate::autoload::RenderMode::Dual
    }
}

/// Emits a JSON payload to stdout for shell hook installations.
///
/// This function:
/// 1. Discovers relevant skills based on prompt and configuration.
/// 2. Applies pinning logic (manual + auto-pin).
/// 3. Renders the autoload content.
/// 4. Saves match history for future auto-pinning.
/// 5. Outputs a JSON payload with the autoload content.
pub(crate) fn emit_autoload(
    include_claude: bool,
    max_bytes: Option<usize>,
    prompt: Option<String>,
    embed_threshold: Option<f32>,
    auto_pin: bool,
    extra_dirs: &[PathBuf],
    diagnose: bool,
) -> Result<()> {
    let mut diag_opt = if diagnose {
        Some(Diagnostics::default())
    } else {
        None
    };

    let skills = if let Some(d) = &mut diag_opt {
        discover_skills(&skill_roots(extra_dirs)?, Some(&mut d.duplicates))?
    } else {
        collect_skills(extra_dirs)?
    };

    let manual_pins = load_pinned().unwrap_or_default();
    let history = load_history().unwrap_or_default();
    let auto_pins = if auto_pin {
        auto_pin_from_history(&history)
    } else {
        HashSet::new()
    };
    let mut effective_pins = manual_pins.clone();
    effective_pins.extend(auto_pins.iter().cloned());

    let mut matched = HashSet::new();
    let mut diag = diag_opt;

    let preload_terms = if let Some(path) = agents_manifest()? {
        if let Ok(text) = fs::read_to_string(&path) {
            Some(extract_refs_from_agents(&text))
        } else {
            None
        }
    } else {
        None
    };

    let preload_terms_ref = preload_terms.as_ref();
    let prompt = prompt.or_else(|| std::env::var("SKRILLS_PROMPT").ok());
    let runtime = runtime_overrides_cached();
    let render_mode = manifest_render_mode(&runtime, None);

    let content = render_autoload(
        &skills,
        AutoloadOptions {
            include_claude,
            max_bytes: max_bytes.or(env_max_bytes()),
            prompt: prompt.as_deref(),
            embed_threshold: Some(embed_threshold.unwrap_or_else(env_embed_threshold)),
            preload_terms: preload_terms_ref,
            pinned: Some(&effective_pins),
            matched: Some(&mut matched),
            diagnostics: diag.as_mut(),
            render_mode,
            log_render_mode: runtime.render_mode_log(),
            gzip_ok: false,
            minimal_manifest: runtime.manifest_minimal(),
        },
    )?;

    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let mut history = history;
    let mut matched_vec: Vec<String> = matched.into_iter().collect();
    matched_vec.sort();
    history.push(HistoryEntry {
        ts,
        skills: matched_vec,
    });
    let _ = save_history(history);

    let payload = serde_json::json!({
        "hookSpecificOutput": {
            "hookEventName": "UserPromptSubmit",
            "additionalContext": content
        }
    });

    println!("{}", serde_json::to_string(&payload)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn autoload_args_parses_embed_threshold() -> Result<()> {
        let json = r#"{
            "include_claude": false,
            "max_bytes": 1024,
            "prompt": "typo prompt",
            "embed_threshold": 0.42,
            "auto_pin": false,
            "diagnose": true
        }"#;
        let args: AutoloadArgs = serde_json::from_str(json)?;
        assert_eq!(args.embed_threshold, Some(0.42));
        assert_eq!(args.prompt, Some("typo prompt".into()));
        assert_eq!(args.max_bytes, Some(1024));
        Ok(())
    }
}
