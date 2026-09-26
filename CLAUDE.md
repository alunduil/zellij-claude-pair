# AI agent notes

Audience: AI coding assistants. Humans start at [docs/](docs/README.md).

## Project

A Zellij plugin for pairing with Claude Code. One Rust `cdylib` crate built to
`wasm32-wasip1`, the target Zellij loads plugins from. Entry point:
`src/lib.rs`.

The plugin API is the `zellij-tile` crate at the version pinned in
`Cargo.toml`: `ZellijPlugin` is the lifecycle trait and `prelude` re-exports the
host calls. Read its source under `~/.cargo/registry/src/*/zellij-tile-*/`
after any build, or on docs.rs.

## Commands

`rust-toolchain.toml` pins the compiler; the first cargo call installs it.

- `cargo build --release`: builds
  `target/wasm32-wasip1/release/zellij_claude_pair.wasm`.
  [docs/how-to/build-the-plugin.md](docs/how-to/build-the-plugin.md) loads it
  into Zellij.
- `cargo clippy --all-targets`: `Cargo.toml` denies `clippy::all` and warns on
  `clippy::pedantic`.
- `cargo fmt --check`
- `cargo test --target "$(rustc -vV | sed -n 's/^host: //p')"`: tests run on
  the host triple. `.cargo/config.toml` defaults every build to
  `wasm32-wasip1`, which nothing here can execute, so a bare `cargo test`
  fails with `Exec format error`.
- `pre-commit run --all-files`: the CI gate. Install with
  `pre-commit install --hook-type pre-commit --hook-type commit-msg`.

## Conventions

- Commit subjects and PR titles follow Conventional Commits, enforced by the
  `commit-msg` hook and `.github/workflows/pr-title.yml`. `release-plz` derives
  the version bump and CHANGELOG from subjects on `main`.
- Open pull requests as drafts.
- New files carry an SPDX header (copy one from an existing file of the same
  type). Files that can't hold a comment get a `REUSE.toml` annotation instead.
- Prose passes Vale, `markdownlint`, and `lychee` via pre-commit; add terms
  to `.vale/styles/config/vocabularies/zellij-claude-pair/accept.txt`.
