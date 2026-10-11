# Contributing to Zellij Claude Pair

This guide covers getting a change reviewed and merged.

## Your first contribution

Start with an issue labelled [good first issue][good-first-issue] or
[help wanted][help-wanted]. Typo and formatting fixes need no issue; open the
pull request directly.

For anything larger,
[open an issue](https://github.com/alunduil/zellij-claude-pair/issues/new/choose)
first so we can agree on the approach before you write code.

## Set up

You need `rustup`, Zellij, and [pre-commit](https://pre-commit.com). The first
build downloads the pinned compiler.

Install both hook stages. The `commit-msg` stage is the only local check on
commit subjects:

```sh
pre-commit install --hook-type pre-commit --hook-type commit-msg
```

## Build and test

To try a change by hand, follow
[Build the plugin](docs/how-to/build-the-plugin.md).

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

CI runs the same gate. Some hooks fix files in place, so stage what they touch
and run again.

Every new file carries an SPDX header. A file that can't hold a comment gets an
annotation in `REUSE.toml` instead. When Vale flags a correct term, add it to
`.vale/styles/config/vocabularies/zellij-claude-pair/accept.txt`.

## Commit messages and pull requests

Commit subjects and pull request titles follow
[Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).
Pull requests merge by squash, so the title becomes the commit subject on
`main`. [release-plz](https://release-plz.dev) reads those subjects to choose
the next version and write `CHANGELOG.md`. Don't bump versions or edit that file
by hand.

Open pull requests as drafts, fill in the template, and mark them ready once CI
passes. The project has one maintainer, so reviews are best-effort.

[good-first-issue]:
  https://github.com/alunduil/zellij-claude-pair/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22
[help-wanted]:
  https://github.com/alunduil/zellij-claude-pair/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22
