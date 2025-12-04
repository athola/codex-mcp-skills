//! Codex adapter for reading/writing ~/.codex configuration.
//! This will be implemented in Task 6.

use super::traits::{AgentAdapter, FieldSupport};
use crate::common::{Command, McpServer, Preferences};
use crate::report::WriteReport;
use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;

/// Adapter for Codex CLI configuration.
pub struct CodexAdapter;

impl AgentAdapter for CodexAdapter {
    fn name(&self) -> &str {
        "codex"
    }

    fn config_root(&self) -> PathBuf {
        PathBuf::from("/tmp/stub")
    }

    fn supported_fields(&self) -> FieldSupport {
        FieldSupport::default()
    }

    fn read_commands(&self) -> Result<Vec<Command>> {
        Ok(Vec::new())
    }

    fn read_mcp_servers(&self) -> Result<HashMap<String, McpServer>> {
        Ok(HashMap::new())
    }

    fn read_preferences(&self) -> Result<Preferences> {
        Ok(Preferences::default())
    }

    fn write_commands(&self, _commands: &[Command]) -> Result<WriteReport> {
        Ok(WriteReport::default())
    }

    fn write_mcp_servers(&self, _servers: &HashMap<String, McpServer>) -> Result<WriteReport> {
        Ok(WriteReport::default())
    }

    fn write_preferences(&self, _prefs: &Preferences) -> Result<WriteReport> {
        Ok(WriteReport::default())
    }
}
