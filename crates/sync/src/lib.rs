//! Cross-agent configuration sync for skrills.
//!
//! Syncs commands, MCP servers, preferences, and skills between
//! Claude Code and Codex using a pluggable adapter architecture.

mod adapters;
mod common;
mod orchestrator;
mod report;

// Re-export stub types (will be populated in subsequent tasks)
pub use adapters::{AgentAdapter, ClaudeAdapter, CodexAdapter, FieldSupport};
pub use report::{SkipReason, SyncReport, WriteReport};
