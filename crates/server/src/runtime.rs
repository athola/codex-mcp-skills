//! Runtime override helpers for `skrills-server`.
//!
//! These are the public API for CLI and MCP runtime tools
//! (`runtime-status`, `set-runtime-options`).
//!
//! This module always compiles. The optional `watch` feature in the parent
//! crate enables filesystem watching.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

use skrills_state::{
    env_auto_pin, env_diag, env_include_claude, env_manifest_first, env_render_mode_log,
    load_auto_pin_flag,
};
use std::sync::LazyLock;
use std::sync::Mutex;

/// Runtime overrides for skill rendering and behavior.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RuntimeOverrides {
    /// Override the manifest-first rendering behavior.
    pub manifest_first: Option<bool>,
    /// Override logging of render mode decisions.
    pub render_mode_log: Option<bool>,
    /// Override minimal manifest rendering.
    pub manifest_minimal: Option<bool>,
}

impl RuntimeOverrides {
    /// Load runtime overrides from the configuration path.
    pub fn load() -> Result<Self> {
        if let Some(path) = runtime_overrides_path() {
            if let Ok(text) = fs::read_to_string(&path) {
                if let Ok(val) = serde_json::from_str::<RuntimeOverrides>(&text) {
                    return Ok(val);
                }
            }
        }
        Ok(RuntimeOverrides::default())
    }

    /// Save the current runtime overrides to the configuration path.
    pub fn save(&self) -> Result<()> {
        if let Some(path) = runtime_overrides_path() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            let text = serde_json::to_string_pretty(self)?;
            fs::write(path, text)?;
        }
        Ok(())
    }

    /// Return the effective `manifest_first` setting (overrides and environment variables).
    pub fn manifest_first(&self) -> bool {
        self.manifest_first.unwrap_or_else(env_manifest_first)
    }

    /// Return the effective `render_mode_log` setting (overrides and environment variables).
    pub fn render_mode_log(&self) -> bool {
        self.render_mode_log.unwrap_or_else(env_render_mode_log)
    }

    /// Return the effective `manifest_minimal` setting (overrides and environment variables).
    pub fn manifest_minimal(&self) -> bool {
        self.manifest_minimal
            .unwrap_or_else(skrills_state::env_manifest_minimal)
    }
}

/// Gets the default value for the auto-pin flag.
///
/// Reads the persisted toggle or uses the environment default.
pub fn env_auto_pin_default() -> bool {
    env_auto_pin(load_auto_pin_flag().unwrap_or(false))
}

/// Checks if autoload responses should emit diagnostics by default.
pub fn env_diag_default() -> bool {
    env_diag()
}

/// Checks if `~/.claude` skills are included by default.
pub fn env_include_claude_default() -> bool {
    env_include_claude()
}

static RUNTIME_CACHE: LazyLock<Mutex<Option<RuntimeOverrides>>> =
    LazyLock::new(|| Mutex::new(None));

/// Loads overrides once per process; subsequent calls use the cached value.
pub fn runtime_overrides_cached() -> RuntimeOverrides {
    if let Ok(mut guard) = RUNTIME_CACHE.lock() {
        if let Some(val) = guard.as_ref() {
            return val.clone();
        }
        if let Ok(val) = RuntimeOverrides::load() {
            *guard = Some(val.clone());
            return val;
        }
    }
    RuntimeOverrides::default()
}

/// Reset the runtime cache for testing purposes.
pub fn reset_runtime_cache_for_tests() {
    if let Ok(mut guard) = RUNTIME_CACHE.lock() {
        *guard = None;
    }
}

pub use skrills_state::runtime_overrides_path;
