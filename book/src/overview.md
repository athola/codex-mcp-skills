# Overview

`codex-mcp-skills` is an MCP server that turns local `SKILL.md` files into
first-class resources and tools. It mirrors external skill trees, filters and
pins skills for each prompt, and emits rich autoload context for Codex or other
agents. The project also syncs `AGENTS.md` with a machine-readable list of
available skills.

## Highlights
- MCP server over stdio with resource & tool endpoints.
- Priority-aware discovery across Codex, Claude mirror, Claude, and Agent skill
  roots, with duplicate suppression.
- Autoload tool: prompt-based filtering, pinning, auto-pin from history,
  diagnostics, byte budget truncation.
- Sync helpers: Claude→Codex mirror, AGENTS.md XML export, TUI for pinning.
- Standalone installers (curl/PowerShell) plus cargo builds and a demo-oriented
  Makefile.
