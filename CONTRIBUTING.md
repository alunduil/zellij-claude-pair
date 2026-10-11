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

You need `rustup`, Zellij, [pre-commit](https://pre-commit.com), and
[lychee](https://lychee.cli.rs). The link-check hook needs `lychee` on your
`PATH`. On x86-64 Linux, install the pinned version:

```sh
./scripts/install-lychee.sh
```

Then install the hooks:

```sh
pre-commit install --hook-type pre-commit --hook-type commit-msg
```

## Build and test

To try a change by hand, follow
[Build the plugin](docs/how-to/build-the-plugin.md).

CI doesn't run the tests, so run them before you open a pull request. Target
your host, since WASM test binaries can't execute locally:

```sh
cargo test --target host-tuple
```

## Checks

Run every hook before you open a pull request:

```sh
pre-commit run --all-files
```

Some hooks fix files in place, so stage what they touch and run again. When Vale
flags a correct term, add it to
`.vale/styles/config/vocabularies/zellij-claude-pair/accept.txt`.

CI also builds the plugin and checks dependencies, so a clean local run can still
fail CI.

## Commit messages and pull requests

Commit subjects and pull request titles follow
[Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).
Pull requests merge by squash, so the title becomes the commit subject on
`main`. [release-plz](https://release-plz.dev) reads those subjects to choose
the next version and write `CHANGELOG.md`. Don't bump versions or edit that file
by hand.

Fill in the pull request template. CI on your first pull request waits until
the maintainer approves the run. The project has one maintainer, so approvals
and reviews are best-effort.

[good-first-issue]:
  https://github.com/alunduil/zellij-claude-pair/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22
[help-wanted]:
  https://github.com/alunduil/zellij-claude-pair/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22
