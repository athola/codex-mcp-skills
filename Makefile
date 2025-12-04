# Common developer and demo tasks for skrills
# Set CARGO_HOME to a writable path to avoid sandbox/root perms issues.
CARGO ?= cargo
CARGO_HOME ?= .cargo
HOME_DIR ?= $(CURDIR)/.home-tmp
BIN ?= skrills
BIN_PATH ?= target/release/$(BIN)
MDBOOK ?= mdbook

.PHONY: help fmt lint check test test-unit test-integration test-setup build build-min serve-help emit-autoload \
	githooks \
	demo-fixtures demo-list demo-list-pinned demo-pin demo-unpin demo-autopin \
	demo-history demo-sync-agents demo-sync demo-emit-autoload demo-doctor demo-all \
	demo-setup-claude demo-setup-codex demo-setup-both demo-setup-uninstall \
	demo-setup-reinstall demo-setup-universal demo-setup-first-run demo-setup-all \
	docs book book-serve clean clean-demo ci lint-md precommit

help:
	@echo "Targets:"
	@echo "  fmt                     format workspace"
	@echo "  lint                    clippy with -D warnings"
	@echo "  check                   cargo check all targets"
	@echo "  test                    cargo test --all-features"
	@echo "  test-unit               run unit tests only"
	@echo "  test-integration        run integration tests only"
	@echo "  test-setup              run setup module tests"
	@echo "  build                   release build with features"
	@echo "  build-min               release build without default features"
	@echo "  serve-help              binary --help smoke check"
	@echo "  emit-autoload           sample emit-autoload run"
	@echo "  githooks                point git core.hooksPath at repo githooks/"
	@echo "  demo-all                run all CLI demos"
	@echo "  demo-doctor             demo doctor diagnostics"
	@echo "  demo-setup-all          run all setup flow demos"
	@echo "  demo-setup-claude       demo setup for Claude Code"
	@echo "  demo-setup-codex        demo setup for Codex"
	@echo "  demo-setup-both         demo setup for both clients"
	@echo "  demo-setup-uninstall    demo uninstall flow"
	@echo "  demo-setup-reinstall    demo reinstall flow"
	@echo "  demo-setup-universal    demo universal sync"
	@echo "  demo-setup-first-run    demo first-run detection"
	@echo "  book                    build mdBook then open in default browser"
	@echo "  book-serve              live-reload mdBook on localhost:3000"
	@echo "  clean                   cargo clean"
	@echo "  clean-demo              remove demo HOME sandbox"
	@echo "  ci                      fmt + lint + test"

fmt:
	CARGO_HOME=$(CARGO_HOME) $(CARGO) fmt --all

lint:
	CARGO_HOME=$(CARGO_HOME) $(CARGO) clippy --workspace --all-targets -- -D warnings

lint-md:
	./scripts/lint-markdown.sh

check:
	CARGO_HOME=$(CARGO_HOME) $(CARGO) check --workspace --all-targets

test:
	CARGO_HOME=$(CARGO_HOME) $(CARGO) test --workspace --all-features

test-unit:
	CARGO_HOME=$(CARGO_HOME) $(CARGO) test --workspace --lib --all-features

test-integration:
	CARGO_HOME=$(CARGO_HOME) $(CARGO) test --workspace --test '*' --all-features

test-setup:
	CARGO_HOME=$(CARGO_HOME) $(CARGO) test --package skrills-server --lib setup --all-features

build:
	CARGO_HOME=$(CARGO_HOME) $(CARGO) build --workspace --all-features --release

build-min:
	CARGO_HOME=$(CARGO_HOME) $(CARGO) build --workspace --no-default-features --release

serve-help:
	CARGO_HOME=$(CARGO_HOME) $(CARGO) run --quiet --bin $(BIN) -- --help >/dev/null

emit-autoload:
	CARGO_HOME=$(CARGO_HOME) $(CARGO) run --quiet --bin $(BIN) -- emit-autoload --prompt "sample" --diagnose --max-bytes 512 >/dev/null

githooks:
	./scripts/install-git-hooks.sh

docs:
	CARGO_HOME=$(CARGO_HOME) RUSTDOCFLAGS="-D warnings" $(CARGO) doc --workspace --all-features --no-deps
	@doc_index="$(CURDIR)/target/doc/skrills/index.html"; \
	if [ -f "$$doc_index" ]; then \
	  if command -v xdg-open >/dev/null 2>&1; then xdg-open "$$doc_index" >/dev/null 2>&1 || true; \
	  elif command -v open >/dev/null 2>&1; then open "$$doc_index" >/dev/null 2>&1 || true; \
	  elif command -v start >/dev/null 2>&1; then start "$$doc_index" >/dev/null 2>&1 || true; \
	  else echo "Docs at $$doc_index"; fi; \
	else echo "Docs built, index not found at $$doc_index"; fi

book:
	@if ! command -v $(MDBOOK) >/dev/null 2>&1; then \
	  echo "mdbook not found; installing to $(CARGO_HOME)/bin"; \
	  CARGO_HOME=$(CARGO_HOME) $(CARGO) install mdbook --locked >/dev/null; \
	fi
	CARGO_HOME=$(CARGO_HOME) $(MDBOOK) build book
	@book_index="$(CURDIR)/book/book/index.html"; \
	if [ -f "$$book_index" ]; then \
	  if command -v xdg-open >/dev/null 2>&1; then xdg-open "$$book_index" >/dev/null 2>&1 || true; \
	  elif command -v open >/dev/null 2>&1; then open "$$book_index" >/dev/null 2>&1 || true; \
	  elif command -v start >/dev/null 2>&1; then start "$$book_index" >/dev/null 2>&1 || true; \
	  else echo "Book at $$book_index"; fi; \
	else echo "Book built, index not found at $$book_index"; fi

book-serve:
	@if ! command -v $(MDBOOK) >/dev/null 2>&1; then \
	  echo "mdbook not found; installing to $(CARGO_HOME)/bin"; \
	  CARGO_HOME=$(CARGO_HOME) $(CARGO) install mdbook --locked >/dev/null; \
	fi
	CARGO_HOME=$(CARGO_HOME) $(MDBOOK) serve book --open --hostname 127.0.0.1 --port 3000

# --- Demo helpers ---------------------------------------------------------

demo-fixtures:
	@mkdir -p $(HOME_DIR)/.codex/skills/demo
	@mkdir -p $(HOME_DIR)/.codex
	@echo "demo skill content" > $(HOME_DIR)/.codex/skills/demo/SKILL.md
	@echo "# Agents" > $(HOME_DIR)/.codex/AGENTS.md
	@echo "Prepared demo HOME at $(HOME_DIR)"

demo-list: demo-fixtures build
	HOME=$(HOME_DIR) CARGO_HOME=$(CARGO_HOME) $(BIN_PATH) list >/dev/null

demo-list-pinned: demo-fixtures build
	HOME=$(HOME_DIR) CARGO_HOME=$(CARGO_HOME) $(BIN_PATH) list-pinned >/dev/null

demo-pin: demo-fixtures build
	HOME=$(HOME_DIR) CARGO_HOME=$(CARGO_HOME) $(BIN_PATH) pin demo/SKILL.md >/dev/null

demo-unpin: demo-fixtures build
	HOME=$(HOME_DIR) CARGO_HOME=$(CARGO_HOME) $(BIN_PATH) unpin demo/SKILL.md >/dev/null

demo-autopin: demo-fixtures build
	HOME=$(HOME_DIR) CARGO_HOME=$(CARGO_HOME) $(BIN_PATH) auto-pin --enable >/dev/null

demo-history: demo-fixtures build
	HOME=$(HOME_DIR) CARGO_HOME=$(CARGO_HOME) $(BIN_PATH) history --limit 5 >/dev/null

demo-sync-agents: demo-fixtures build
	HOME=$(HOME_DIR) CARGO_HOME=$(CARGO_HOME) $(BIN_PATH) sync-agents --path $(HOME_DIR)/.codex/AGENTS.md >/dev/null

demo-sync: demo-fixtures build
	HOME=$(HOME_DIR) CARGO_HOME=$(CARGO_HOME) $(BIN_PATH) sync >/dev/null || true

demo-emit-autoload: demo-fixtures build
	HOME=$(HOME_DIR) CARGO_HOME=$(CARGO_HOME) $(BIN_PATH) emit-autoload --prompt demo --diagnose --max-bytes 2048 >/dev/null

demo-doctor: demo-fixtures build
	@echo "==> Demo: Doctor diagnostics"
	HOME=$(HOME_DIR) CARGO_HOME=$(CARGO_HOME) $(BIN_PATH) doctor
	@echo "==> Doctor demo complete"

demo-all: demo-fixtures build demo-list demo-pin demo-list-pinned demo-unpin demo-list-pinned demo-autopin demo-history demo-sync-agents demo-sync demo-emit-autoload demo-doctor

# --- Setup flow demos -----------------------------------------------------

demo-setup-claude: demo-fixtures build
	@echo "==> Demo: Setup for Claude Code (non-interactive)"
	@rm -rf $(HOME_DIR)/.claude
	HOME=$(HOME_DIR) CARGO_HOME=$(CARGO_HOME) $(BIN_PATH) setup --client claude --bin-dir $(HOME_DIR)/.claude/bin --yes
	@echo "==> Verifying Claude setup..."
	@test -f $(HOME_DIR)/.claude/hooks/prompt.on_user_prompt_submit || (echo "ERROR: Hook not created" && exit 1)
	@test -f $(HOME_DIR)/.claude/.mcp.json || (echo "ERROR: MCP config not created" && exit 1)
	@test -x $(HOME_DIR)/.claude/bin/skrills || (echo "ERROR: Binary not installed" && exit 1)
	@echo "==> Claude setup verified successfully"

demo-setup-codex: demo-fixtures build
	@echo "==> Demo: Setup for Codex (non-interactive)"
	@rm -rf $(HOME_DIR)/.codex
	HOME=$(HOME_DIR) CARGO_HOME=$(CARGO_HOME) $(BIN_PATH) setup --client codex --bin-dir $(HOME_DIR)/.codex/bin --yes
	@echo "==> Verifying Codex setup..."
	@test -x $(HOME_DIR)/.codex/bin/skrills || (echo "ERROR: Binary not installed" && exit 1)
	@echo "==> Codex setup verified successfully (TLS certs optional)"

demo-setup-both: demo-fixtures build
	@echo "==> Demo: Setup for both Claude Code and Codex"
	@rm -rf $(HOME_DIR)/.claude $(HOME_DIR)/.codex
	HOME=$(HOME_DIR) CARGO_HOME=$(CARGO_HOME) $(BIN_PATH) setup --client both --bin-dir $(HOME_DIR)/.claude/bin --yes
	@echo "==> Verifying both clients setup..."
	@test -f $(HOME_DIR)/.claude/hooks/prompt.on_user_prompt_submit || (echo "ERROR: Claude hook not created" && exit 1)
	@test -f $(HOME_DIR)/.claude/.mcp.json || (echo "ERROR: Claude MCP config not created" && exit 1)
	@test -x $(HOME_DIR)/.claude/bin/skrills || (echo "ERROR: Binary not installed" && exit 1)
	@echo "==> Both clients setup verified successfully"

demo-setup-uninstall: demo-setup-claude
	@echo "==> Demo: Uninstall Claude setup"
	HOME=$(HOME_DIR) CARGO_HOME=$(CARGO_HOME) $(BIN_PATH) setup --uninstall --client claude --yes
	@echo "==> Verifying uninstall..."
	@test ! -f $(HOME_DIR)/.claude/hooks/prompt.on_user_prompt_submit || (echo "ERROR: Hook still exists" && exit 1)
	@echo "==> Uninstall verified successfully"

demo-setup-reinstall: demo-setup-claude
	@echo "==> Demo: Reinstall Claude setup"
	HOME=$(HOME_DIR) CARGO_HOME=$(CARGO_HOME) $(BIN_PATH) setup --client claude --bin-dir $(HOME_DIR)/.claude/bin --reinstall --yes
	@echo "==> Verifying reinstall..."
	@test -f $(HOME_DIR)/.claude/hooks/prompt.on_user_prompt_submit || (echo "ERROR: Hook not recreated" && exit 1)
	@echo "==> Reinstall verified successfully"

demo-setup-universal: demo-fixtures build
	@echo "==> Demo: Setup with universal sync"
	@rm -rf $(HOME_DIR)/.claude $(HOME_DIR)/.agent
	@mkdir -p $(HOME_DIR)/.claude/skills
	@echo "test skill" > $(HOME_DIR)/.claude/skills/test.md
	HOME=$(HOME_DIR) CARGO_HOME=$(CARGO_HOME) $(BIN_PATH) setup --client claude --bin-dir $(HOME_DIR)/.claude/bin --universal --yes
	@echo "==> Verifying universal sync..."
	@test -d $(HOME_DIR)/.agent/skills || (echo "ERROR: Universal skills dir not created" && exit 1)
	@echo "==> Universal sync verified successfully"

demo-setup-first-run: demo-fixtures build
	@echo "==> Demo: First-run detection (simulated with doctor command)"
	@rm -rf $(HOME_DIR)/.claude $(HOME_DIR)/.codex
	@echo "==> Running doctor command on fresh install (should NOT prompt for setup as it's not served by first-run logic)"
	HOME=$(HOME_DIR) CARGO_HOME=$(CARGO_HOME) $(BIN_PATH) doctor 2>&1 || true
	@echo "==> First-run detection demo complete"

demo-setup-all: demo-setup-claude demo-setup-codex demo-setup-both demo-setup-uninstall demo-setup-reinstall demo-setup-universal demo-setup-first-run
	@echo "==> All setup demos completed successfully"

clean:
	CARGO_HOME=$(CARGO_HOME) $(CARGO) clean

clean-demo:
	@rm -rf $(HOME_DIR)
	@echo "Removed demo HOME $(HOME_DIR)"

ci: fmt lint test

precommit: fmt lint lint-md test
