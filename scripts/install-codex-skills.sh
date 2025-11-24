#!/usr/bin/env bash
# Install codex-mcp-skills into ~/.codex (hook + MCP server registration)
# Flags:
#   --universal        Also sync skills into ~/.agent/skills for cross-agent reuse.
#   --universal-only   Only perform the universal sync (no hook/server install).
set -euo pipefail

UNIVERSAL=0
UNIVERSAL_ONLY=0
for arg in "$@"; do
  case "$arg" in
    --universal) UNIVERSAL=1 ;;
    --universal-only) UNIVERSAL=1; UNIVERSAL_ONLY=1 ;;
    *) echo "Unknown arg: $arg" >&2; exit 1 ;;
  esac
done
if [ "${CODEX_SKILLS_UNIVERSAL:-0}" != "0" ]; then
  UNIVERSAL=1
fi

BIN_PATH="${BIN_PATH:-$HOME/.cargo/bin/codex-mcp-skills}"
HOOK_DIR="$HOME/.codex/hooks/codex"
HOOK_PATH="$HOOK_DIR/prompt.on_user_prompt_submit"
MCP_PATH="$HOME/.codex/mcp_servers.json"
REPO_ROOT="$(cd "${0%/*}/.." && pwd)"

sync_universal() {
  local AGENT_SKILLS="${AGENT_SKILLS_DIR:-$HOME/.agent/skills}"
  local CODEX_SKILLS_DIR="${CODEX_SKILLS_DIR:-$HOME/.codex/skills}"
  local MIRROR_DIR="${CODEX_MIRROR_DIR:-$HOME/.codex/skills-mirror}"
  mkdir -p "$AGENT_SKILLS"
  echo "Universal sync: copying skills into $AGENT_SKILLS"
  copy_tree() {
    local src="$1"
    [ -d "$src" ] || return 0
    if command -v rsync >/dev/null 2>&1; then
      rsync -a --update "$src"/ "$AGENT_SKILLS"/
    else
      (cd "$src" && tar -cf - .) | (cd "$AGENT_SKILLS" && tar -xf -)
    fi
  }
  # Refresh Claude mirror first if binary exists
  if [ -x "$BIN_PATH" ]; then
    "$BIN_PATH" sync || echo "Warning: sync-from-claude failed (continuing)."
  fi
  copy_tree "$CODEX_SKILLS_DIR"
  copy_tree "$MIRROR_DIR"
  echo "Universal sync complete."
}

if [ "$UNIVERSAL_ONLY" -eq 1 ]; then
  sync_universal
  exit 0
fi

mkdir -p "$HOOK_DIR"

cat <<'HOOK' > "$HOOK_PATH"
#!/usr/bin/env bash
# Inject SKILL.md content into Codex on prompt submit via codex-mcp-skills
set -euo pipefail

BIN="${CODEX_SKILLS_BIN:-$HOME/.cargo/bin/codex-mcp-skills}"
REPO="${CODEX_SKILLS_REPO:-$HOME/codex-mcp-skills}"
CMD_ARGS=(emit-autoload)

# Optionally capture prompt text from stdin (Codex passes event payload on prompt submit).
PROMPT_INPUT=""
if [ ! -t 0 ]; then
  if IFS= read -r -t 0.05 first_line; then
    rest=$(cat)
    PROMPT_INPUT="${first_line}${rest}"
  fi
fi

if [ -n "${CODEX_SKILLS_PROMPT:-}" ]; then
  PROMPT_INPUT="$CODEX_SKILLS_PROMPT"
fi

if [ -n "$PROMPT_INPUT" ]; then
  CMD_ARGS+=(--prompt "$PROMPT_INPUT")
fi

run_cmd() {
  if [ -x "$BIN" ]; then
    "$BIN" "${CMD_ARGS[@]}"
  elif [ -d "$REPO" ]; then
    (cd "$REPO" && cargo run --quiet -- "${CMD_ARGS[@]}")
  else
    echo "{}" && exit 0
  fi
}

OUTPUT=$(run_cmd || true)
if [ -n "${OUTPUT:-}" ]; then
  echo "$OUTPUT"
fi
HOOK
chmod +x "$HOOK_PATH"

echo "Hook written to $HOOK_PATH"

# Ensure mcp_servers.json exists
if [ ! -f "$MCP_PATH" ]; then
  mkdir -p "$(dirname "$MCP_PATH")"
  cat <<'JSON' > "$MCP_PATH"
{
  "mcpServers": {}
}
JSON
fi

# Merge/insert codex-skills entry using jq if available
if command -v jq >/dev/null 2>&1; then
  tmp=$(mktemp)
  jq '.mcpServers."codex-skills" = {"command": "'"$BIN_PATH"'", "args": ["serve"]}' "$MCP_PATH" > "$tmp"
  mv "$tmp" "$MCP_PATH"
else
  echo "jq not found; please add codex-skills entry to $MCP_PATH manually." >&2
fi

echo "Registered codex-skills MCP server in $MCP_PATH"

echo "Install complete. To mirror Claude skills: run 'codex-mcp-skills sync' (binary must be built)."

if [ "$UNIVERSAL" -eq 1 ]; then
  sync_universal
fi
