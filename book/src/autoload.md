# Autoload & Skills

## Discovery priority
1. `~/.codex/skills`
2. `~/.codex/skills-mirror` (Claude mirror)
3. `~/.claude/skills`
4. `~/.agent/skills`

Manifest override: `~/.codex/skills-manifest.json` can set `priority`,
`expose_agents`, and `cache_ttl_ms`.

## Autoload filtering
- Tokenized prompt terms (>=3 chars) matched against skill names and first 4KB
  of content.
- Manual pins always included; auto-pins sourced from recent history.
- Sources Claude/Mirror can be gated by `include_claude` flag.
- Byte budget via `--max-bytes`; truncation is annotated.
- Diagnostics footer lists included, skipped, duplicates, truncation.

## Caching
- Discovery cache with TTL (env `CODEX_SKILLS_CACHE_TTL_MS` or manifest
  `cache_ttl_ms`); invalidated by file watcher (`--watch`) or manual
  `refresh-cache` tool.
- Content cache keyed by path+hash; refreshes on file changes or hash mismatch.
