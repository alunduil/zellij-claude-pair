# AI agent notes

Audience: AI coding assistants. Humans start at [docs/](docs/README.md).

## Project

A Zellij plugin for pairing with Claude Code. The plugin API is the
`zellij-tile` crate; read its source under
`~/.cargo/registry/src/*/zellij-tile-*/` after a build.

## Commands

- `cargo build --release`; load the result with
  [docs/how-to/build-the-plugin.md](docs/how-to/build-the-plugin.md).
- `cargo clippy --all-targets`
- `cargo fmt --check`
- `cargo test --target host-tuple`: a bare `cargo test` builds for
  `wasm32-wasip1`, which can't execute here.
- `pre-commit run --all-files`: the CI gate. Install with
  `pre-commit install --hook-type pre-commit --hook-type commit-msg`.

## Conventions

- Commit subjects and PR titles follow Conventional Commits; `release-plz`
  derives the version bump and CHANGELOG from them.
- Open pull requests as drafts.
- New files carry an SPDX header. Files that can't hold a comment get a
  `REUSE.toml` annotation instead.
- Add Vale vocabulary to
  `.vale/styles/config/vocabularies/zellij-claude-pair/accept.txt`.
