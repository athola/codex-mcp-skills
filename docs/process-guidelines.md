# Process Handling Notes

- The server installs a Unix-only `SIGCHLD` handler with `SA_NOCLDWAIT | SA_RESTART` at startup. This prevents zombie (`<defunct>`) children by auto-reaping any unexpected subprocesses.
- If you intentionally add child processes, make sure your platform-specific handling is compatible with this policy. On Unix that means either:
  - Collect exit status via an alternate mechanism that doesn't rely on `waitpid`, or
  - Temporarily override the signal disposition before spawning and restore it afterward.
- Non-Unix builds use a no-op stub; add equivalent safeguards if you introduce platform-specific process management on other targets.

## Workflow reminders
- Keep shared logic in `crates/core`; `crates/cli` should stay thin.
- Update `docs/CHANGELOG.md` for user-visible changes (CLI flags, structured output shape, AGENTS sync, priority rules).
- Run `cargo fmt && cargo test` before publishing changes.
