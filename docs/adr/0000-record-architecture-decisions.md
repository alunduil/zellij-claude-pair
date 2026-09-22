# 0. Record architecture decisions

Date: 2026-09-22

## Status

Accepted

## Context

Decisions accumulate as the project grows. Without a durable record, the _why_
behind each choice lives only in commit messages, PR threads, and issue
comments. Future contributors inherit the _what_ and must reconstruct the _why_
from scratch.

Scattered records also resist discovery. A reader can't tell whether to search
commit messages, PR descriptions, or issues.

## Decision

Record significant decisions as Architecture Decision Records (ADRs) in the
format from Michael Nygard's [Documenting Architecture
Decisions][nygard]. ADRs live under `docs/adr/` as `NNNN-kebab-title.md`,
numbered from 0000, this record.

Each ADR carries a `Status` of `Accepted`, `Deprecated`, or `Superseded by
NNNN`. ADRs merge as `Accepted`: PR review is the deliberation, so the committed
record never sits at `Proposed`.

A decision earns an ADR when it's significant _and_ hard to reverse.
Tactical choices, reversible defaults, and personal style belong in the commit
message.

## Consequences

- Future decisions have one location and one shape.
- Revisiting a decision starts from recorded context.
- Each decision costs one short Markdown file.
- Too many ADRs bury the ones that matter. When unsure whether a decision
  earns one, skip it.

[nygard]:
  https://www.cognitect.com/blog/2011/11/15/documenting-architecture-decisions
