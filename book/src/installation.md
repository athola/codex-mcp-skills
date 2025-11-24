# Installation

## One-liners (recommended)
```bash
# macOS / Linux
curl -LsSf https://raw.githubusercontent.com/athola/codex-mcp-skills/main/scripts/install.sh | sh

# Windows
powershell -ExecutionPolicy ByPass -c "irm https://raw.githubusercontent.com/athola/codex-mcp-skills/main/scripts/install.ps1 | iex"
```
Env overrides: `CODEX_SKILLS_GH_REPO`, `CODEX_SKILLS_VERSION`, `CODEX_SKILLS_BIN_DIR`, `CODEX_SKILLS_TARGET`.

## From source
```bash
cargo install --path crates/cli --force
```

## Hook & MCP registration
```bash
./scripts/install-codex-skills.sh [--universal] [--universal-only]
```
- Hook: `~/.codex/hooks/codex/prompt.on_user_prompt_submit`
- MCP server: `~/.codex/mcp_servers.json`
- `--universal` also mirrors into `~/.agent/skills`.

## Make targets
```bash
make build     # release build
make serve-help
make emit-autoload
make demo-all  # full CLI dogfood in a sandbox HOME
```
