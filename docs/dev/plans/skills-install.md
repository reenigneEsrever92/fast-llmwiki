---
type: Plan
title: Add an `okf install` subcommand for bundled agent skills
status: stable
state: done
priority: medium
tags: [dev, cli]
verified: { by: ai:zed, at: 2026-08-19T09:59:34Z }
---

Implements [install bundled agent skills](/dev/specs/skills-install.md).

- [x] Add a compile-time `include_skills!` macro to `okf-cli` that embeds every
  skill under `.agents/skills/` into the binary with `include_str!`.
- [x] Add an `install` subcommand to the `okf` CLI with a `--dir` flag that
  defaults to `.agents/skills`.
- [x] Write each embedded skill to `<dir>/<name>/SKILL.md`, creating parent
  directories and overwriting on re-run.
- [x] Add tests for the skill manifest and install behavior.
- [x] Update the CLI reference and getting-started docs.

Commit: not yet committed.