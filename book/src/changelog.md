# Changelog (highlights)

## 2025-11-24
- Split into a Rust workspace: `crates/core` (library/MCP server) and
  `crates/cli` (binary wrapper).
- Added structured `_meta` outputs across tools with priority ranks and
  duplicate info.
- Synced AGENTS.md generation to include per-skill `priority_rank` and overall
  priority list.
- Refreshed README with clearer install/usage, universal sync, TUI, and
  structured output examples.

## 2025-11-22
- Exposed AGENTS.md as `doc://agents` with manifest/env opt-out.
- Added universal sync helper and hook installer flags.
- Improved duplicate handling and diagnostics during autoload.

For the full history, see `docs/CHANGELOG.md` in the repository.
