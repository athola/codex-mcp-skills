# Process & Safety

We keep the runtime tidy and avoid surprises for downstream users.

## Child process policy
- On Unix we install a `SIGCHLD` handler with `SA_NOCLDWAIT | SA_RESTART` at
  startup to prevent zombie children.
- If you spawn children, either manage exit status without `waitpid` or
  temporarily override the handler and restore it afterward.
- Non-Unix builds use a no-op stub; add equivalent safeguards before adding
  platform-specific spawns.

## Workflow guardrails
- Concentrate shared logic in `crates/core`; keep `crates/cli` thin.
- Update the changelog for user-visible changes (CLI flags, output shape,
  AGENTS sync behavior, priority rules).
- Run `cargo fmt && cargo test` before publishing or releasing.

## Quick checklist
- [ ] No new zombies or unmanaged children.
- [ ] Changelog entry for user-facing changes.
- [ ] Tests and formatting pass.
- [ ] CLI flags documented in README and the book.
