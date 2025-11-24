# CLI Usage

```bash
codex-mcp-skills serve [--skill-dir DIR] [--cache-ttl-ms N] [--watch]
codex-mcp-skills emit-autoload [--include-claude] [--max-bytes N] \
  [--prompt TEXT] [--auto-pin] [--skill-dir DIR]... [--diagnose]
codex-mcp-skills list
codex-mcp-skills list-pinned
codex-mcp-skills pin <skill>...
codex-mcp-skills unpin <skill>... [--all]
codex-mcp-skills auto-pin --enable
codex-mcp-skills history [--limit N]
codex-mcp-skills sync-agents [--path AGENTS.md]
codex-mcp-skills sync
codex-mcp-skills tui
```

- **Serve** starts the MCP server over stdio. `--watch` enables live
  filesystem invalidation (feature flag `watch`). `--cache-ttl-ms` sets the
  discovery cache TTL; also configurable via env/manifest.
- **emit-autoload** returns concatenated skill content plus diagnostics and
  structured metadata; obeys prompt filtering, pins, auto-pins, and byte limit.
- **pin / unpin / auto-pin** manage manual and heuristic pins (history-based).
- **sync-agents** writes `<available_skills>` XML into AGENTS.md with priority
  ranks and locations.
- **sync** mirrors `~/.claude/skills` into `~/.codex/skills-mirror`.
- **tui** provides interactive pinning and optional mirror sync.
