//! Sync orchestrator that coordinates adapters and manages sync flow.

use crate::adapters::AgentAdapter;
use crate::report::SyncReport;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

/// Direction of sync operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SyncDirection {
    /// Sync from Claude to Codex
    ClaudeToCodex,
    /// Sync from Codex to Claude
    CodexToClaude,
}

/// Parameters for a sync operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncParams {
    /// Source agent name: "claude", "codex", or "auto"
    pub from: Option<String>,
    /// Perform dry run (preview only)
    pub dry_run: bool,
    /// Skip confirmation prompts
    pub force: bool,
    /// Sync skills
    #[serde(default = "default_true")]
    pub sync_skills: bool,
    /// Sync commands
    #[serde(default = "default_true")]
    pub sync_commands: bool,
    /// Sync MCP servers
    #[serde(default = "default_true")]
    pub sync_mcp_servers: bool,
    /// Sync preferences
    #[serde(default = "default_true")]
    pub sync_preferences: bool,
}

impl Default for SyncParams {
    fn default() -> Self {
        Self {
            from: None,
            dry_run: false,
            force: false,
            sync_skills: true,
            sync_commands: true,
            sync_mcp_servers: true,
            sync_preferences: true,
        }
    }
}

fn default_true() -> bool {
    true
}

/// Orchestrates sync operations between agents.
pub struct SyncOrchestrator<S: AgentAdapter, T: AgentAdapter> {
    source: S,
    target: T,
}

impl<S: AgentAdapter, T: AgentAdapter> SyncOrchestrator<S, T> {
    /// Creates a new orchestrator with source and target adapters.
    pub fn new(source: S, target: T) -> Self {
        Self { source, target }
    }

    /// Returns the source adapter name.
    pub fn source_name(&self) -> &str {
        self.source.name()
    }

    /// Returns the target adapter name.
    pub fn target_name(&self) -> &str {
        self.target.name()
    }

    /// Performs the sync operation.
    pub fn sync(&self, params: &SyncParams) -> Result<SyncReport> {
        let mut report = SyncReport::new();

        // Sync commands
        if params.sync_commands {
            let commands = self.source.read_commands()?;
            if !params.dry_run {
                report.commands = self.target.write_commands(&commands)?;
            } else {
                report.commands.written = commands.len();
            }
        }

        // Sync MCP servers
        if params.sync_mcp_servers {
            let servers = self.source.read_mcp_servers()?;
            if !params.dry_run {
                report.mcp_servers = self.target.write_mcp_servers(&servers)?;
            } else {
                report.mcp_servers.written = servers.len();
            }
        }

        // Sync preferences
        if params.sync_preferences {
            let prefs = self.source.read_preferences()?;
            if !params.dry_run {
                report.preferences = self.target.write_preferences(&prefs)?;
            } else {
                // Count non-empty preferences
                if prefs.model.is_some() {
                    report.preferences.written += 1;
                }
            }
        }

        report.success = true;
        report.summary = report.format_summary(self.source.name(), self.target.name());

        Ok(report)
    }
}

/// Determines sync direction from string input.
pub fn parse_direction(from: &str) -> Result<SyncDirection> {
    match from.to_lowercase().as_str() {
        "claude" => Ok(SyncDirection::ClaudeToCodex),
        "codex" => Ok(SyncDirection::CodexToClaude),
        _ => bail!("Unknown source '{}'. Use 'claude' or 'codex'", from),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::{ClaudeAdapter, CodexAdapter};
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn sync_commands_between_adapters() {
        let src_dir = tempdir().unwrap();
        let tgt_dir = tempdir().unwrap();

        // Create source command
        let src_cmd_dir = src_dir.path().join("commands");
        fs::create_dir_all(&src_cmd_dir).unwrap();
        fs::write(src_cmd_dir.join("hello.md"), "# Hello").unwrap();

        let source = ClaudeAdapter::with_root(src_dir.path().to_path_buf());
        let target = CodexAdapter::with_root(tgt_dir.path().to_path_buf());

        let orchestrator = SyncOrchestrator::new(source, target);
        let params = SyncParams {
            sync_commands: true,
            sync_mcp_servers: false,
            sync_preferences: false,
            sync_skills: false,
            ..Default::default()
        };

        let report = orchestrator.sync(&params).unwrap();
        assert_eq!(report.commands.written, 1);

        // Verify file was created
        let tgt_file = tgt_dir.path().join("commands/hello.md");
        assert!(tgt_file.exists());
        assert_eq!(fs::read_to_string(&tgt_file).unwrap(), "# Hello");
    }

    #[test]
    fn dry_run_does_not_write() {
        let src_dir = tempdir().unwrap();
        let tgt_dir = tempdir().unwrap();

        let src_cmd_dir = src_dir.path().join("commands");
        fs::create_dir_all(&src_cmd_dir).unwrap();
        fs::write(src_cmd_dir.join("hello.md"), "# Hello").unwrap();

        let source = ClaudeAdapter::with_root(src_dir.path().to_path_buf());
        let target = CodexAdapter::with_root(tgt_dir.path().to_path_buf());

        let orchestrator = SyncOrchestrator::new(source, target);
        let params = SyncParams {
            dry_run: true,
            ..Default::default()
        };

        let report = orchestrator.sync(&params).unwrap();
        assert_eq!(report.commands.written, 1);

        // Verify nothing was actually written
        let tgt_file = tgt_dir.path().join("commands/hello.md");
        assert!(!tgt_file.exists());
    }

    #[test]
    fn sync_mcp_servers() {
        let src_dir = tempdir().unwrap();
        let tgt_dir = tempdir().unwrap();

        // Create source MCP config
        let settings_path = src_dir.path().join("settings.json");
        fs::write(
            &settings_path,
            r#"{
            "mcpServers": {
                "test-server": {
                    "command": "/usr/bin/test"
                }
            }
        }"#,
        )
        .unwrap();

        let source = ClaudeAdapter::with_root(src_dir.path().to_path_buf());
        let target = CodexAdapter::with_root(tgt_dir.path().to_path_buf());

        let orchestrator = SyncOrchestrator::new(source, target);
        let params = SyncParams {
            sync_commands: false,
            sync_mcp_servers: true,
            sync_preferences: false,
            sync_skills: false,
            ..Default::default()
        };

        let report = orchestrator.sync(&params).unwrap();
        assert_eq!(report.mcp_servers.written, 1);

        // Verify config was created
        let tgt_config = tgt_dir.path().join("config.json");
        assert!(tgt_config.exists());
    }

    #[test]
    fn parse_direction_claude() {
        let dir = parse_direction("claude").unwrap();
        assert_eq!(dir, SyncDirection::ClaudeToCodex);
    }

    #[test]
    fn parse_direction_codex() {
        let dir = parse_direction("codex").unwrap();
        assert_eq!(dir, SyncDirection::CodexToClaude);
    }

    #[test]
    fn parse_direction_invalid() {
        let result = parse_direction("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn orchestrator_names() {
        let src_dir = tempdir().unwrap();
        let tgt_dir = tempdir().unwrap();

        let source = ClaudeAdapter::with_root(src_dir.path().to_path_buf());
        let target = CodexAdapter::with_root(tgt_dir.path().to_path_buf());

        let orchestrator = SyncOrchestrator::new(source, target);
        assert_eq!(orchestrator.source_name(), "claude");
        assert_eq!(orchestrator.target_name(), "codex");
    }
}
