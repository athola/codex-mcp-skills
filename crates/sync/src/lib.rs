//! Cross-agent configuration sync for skrills.
//!
//! Syncs commands, MCP servers, preferences, and skills between
//! Claude Code and Codex using a pluggable adapter architecture.

mod adapters;
mod common;
mod orchestrator;
mod report;

pub use adapters::{AgentAdapter, ClaudeAdapter, CodexAdapter, FieldSupport};
pub use common::{Command, CommonConfig, McpServer, Preferences, SyncMeta};
pub use orchestrator::{parse_direction, SyncDirection, SyncOrchestrator, SyncParams};
pub use report::{SkipReason, SyncReport, WriteReport};
