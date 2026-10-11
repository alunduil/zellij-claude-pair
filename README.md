# Zellij Claude Pair

[![License: MIT](https://img.shields.io/github/license/alunduil/zellij-claude-pair)](LICENSE)
[![pre-commit](https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit)](https://github.com/pre-commit/pre-commit)
[![Renovate](https://img.shields.io/badge/renovate-enabled-brightgreen?logo=renovatebot)](https://docs.renovatebot.com)

A Zellij plugin for pairing with Claude Code, one repository per tab.

**By [Alex Brandt](https://github.com/alunduil)**

## What it does

The plugin is the Zellij side of a Claude pairing workflow: it owns everything
you see and drive from inside a session. It's pre-release and renders a
placeholder pane; the first releases add:

- Status widgets for the current branch and its pull request, styled from your
  Zellij theme.
- A picker over your worktrees, local clones, and remote repositories. Picking
  one opens a pair tab for it.

After 1.0, the plugin grows into a session orchestrator that starts Claude Code
in worktrees and tracks each task's pull request through to merge. Track
progress on the
[milestones page](https://github.com/alunduil/zellij-claude-pair/milestones).

## Install

No release exists yet, so the plugin runs from a clone. You need `rustup` and
Zellij. [Build the plugin](docs/how-to/build-the-plugin.md) covers cloning,
building, and loading it into a running session.

## Get help

Open an [issue](https://github.com/alunduil/zellij-claude-pair/issues/new/choose)
to report a bug, request a feature, or ask a question.

## Contribute

[CONTRIBUTING.md](CONTRIBUTING.md) covers the development loop and pull request
conventions. User-visible changes appear in [CHANGELOG.md](CHANGELOG.md), and
deeper documentation lives in [docs/](docs/README.md).

## License

[MIT](LICENSE)
