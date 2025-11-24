# Development

## Toolchain
- Rust 1.78+ (via rustup), `cargo fmt`, `clippy`.
- Optional: `mdbook` for this book (`cargo install mdbook --locked`).

## Make targets
```bash
make fmt lint check test build build-min
make serve-help emit-autoload
make demo-all     # sandboxed CLI dogfood
make docs         # cargo doc with warnings denied
make book         # mdBook build + open index
make book-serve   # live-reload book (localhost:3000)
make clean clean-demo
```

## Demo sandbox
`make demo-all` builds the release binary, prepares a temporary HOME with a demo
skill, and runs list/pin/unpin/auto-pin/history/sync-agents/sync/emit-autoload
commands to validate end-to-end behavior without touching your real `$HOME`.

## Testing
`cargo test --workspace --all-features` (also wired to `make test` and `make ci`).
