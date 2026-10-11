# Contributing to Zellij Claude Pair

Thanks for your interest in the plugin. These notes cover what you need to get a
change reviewed and merged.

## Your first contribution

Start with an issue labelled [good first issue][good-first-issue] or
[help wanted][help-wanted]. Typo and formatting fixes need no issue; open the
pull request directly.

For anything larger, open an issue first so we can agree on the approach before
you write code.

## Set up

You need `rustup`, Zellij, and [pre-commit](https://pre-commit.com).
`rust-toolchain.toml` pins the compiler and the `wasm32-wasip1` target, so the
first build downloads both.

Install the hooks for both stages:

```sh
pre-commit install --hook-type pre-commit --hook-type commit-msg
```

Without the `commit-msg` stage, nothing checks your commit subjects locally.

## Build and test

`cargo build` targets `wasm32-wasip1` by default.
[Build the plugin](docs/how-to/build-the-plugin.md) covers loading the result
into a running Zellij session to try a change by hand.

WASM test binaries can't run on your machine, so run the tests for your host
instead:

```sh
cargo test --target host-tuple
```

## Checks

Run every hook before you open a pull request:

```sh
pre-commit run --all-files
```

This is the same gate CI runs. It covers formatting, Clippy, prose, links, and
licence headers. Some hooks fix files in place, so stage what they touch and run
again.

Every new file carries an SPDX header. A file that can't hold a comment gets an
annotation in `REUSE.toml` instead. When Vale flags a correct term, add it to
`.vale/styles/config/vocabularies/zellij-claude-pair/accept.txt`.

## Commit messages and pull requests

Commit subjects and pull request titles follow
[Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).
Pull requests merge by squash, so the title becomes the commit subject on
`main`. [release-plz](https://release-plz.dev) reads those subjects to choose
the next version and write `CHANGELOG.md`, so don't bump versions or edit that
file by hand.

Open pull requests as drafts and mark them ready once CI passes. The pull
request template asks for a summary, anything that might surprise a reviewer,
and the checks you ran beyond CI.

## Report a bug or request a feature

Pick a template when you
[open an issue](https://github.com/alunduil/zellij-claude-pair/issues/new/choose).
A bug report needs the steps to reproduce, what you expected, what happened, and
your Zellij version.

## Code review

The project has one maintainer, so reviews are best-effort. Feedback arrives as
review comments on the pull request.

[good-first-issue]:
  https://github.com/alunduil/zellij-claude-pair/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22
[help-wanted]:
  https://github.com/alunduil/zellij-claude-pair/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22
