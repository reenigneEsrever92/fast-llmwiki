---
type: Specification
title: Install bundled agent skills from the CLI
description: Add an `okf install` subcommand that materializes the agent skills embedded in the binary into a skills directory.
status: stable
tags: [dev, cli]
owner: human:felix
---

# Problem

The repository ships agent skills under `.agents/skills/` (for example `okf-dev`
and `okf-spec`). They live as loose `SKILL.md` files in the checkout, so a user
who wants those skills activated in a project (for example under that project's
`.agents/skills` directory) must copy them by hand. There is no command that
installs them, and the skills are not bundled into any binary.

Because `okf` is the unified CLI for this project, it is the natural place to
offer a single `install` command that writes the bundled skills to a target
directory. To work from a standalone `okf` binary (without the repository
checkout present), the skills' contents must be embedded into the binary at
compile time rather than read from disk at runtime.

# Requirements

- The `okf` CLI MUST expose an `install` subcommand that installs the agent
  skills available in this repository.
- Every skill shipped in the repository under `.agents/skills/` MUST be compiled
  into the `okf` binary via a Rust macro (`include_str!`) so that the
  install command works without the repository checkout present.
- The macro MUST enumerate all skills in one place, so adding a skill is a single
  entry rather than repeated code.
- The `install` subcommand MUST accept a `--dir` flag naming the destination
  directory for skills.
- When `--dir` is omitted, the destination MUST default to `.agents/skills` in
  the current working directory.
- Each skill MUST be written to `<dir>/<skill-name>/SKILL.md`, where
  `<skill-name>` is the skill's directory name (e.g. `okf-dev`).
- Installation MUST create any missing parent directories and MUST be idempotent
  (re-running overwrites existing files).
- The install command MUST report which skills were written and where.

# Acceptance Criteria

- Given the `install` subcommand, when run with no flags, then every bundled
  skill is written under `./.agents/skills/<name>/SKILL.md`.
- Given the `install` subcommand with `--dir`, when run, then every bundled skill
  is written under the given directory instead.
- Given a bundled skill, when the binary is built, then the skill's `SKILL.md`
  contents are present in the binary (not read from the filesystem at runtime).
- Given a missing destination directory, when installing, then the directory
  tree is created and the command succeeds.
- Given an existing installed skill, when installing again, then the command
  succeeds and the file is overwritten.

# Out of scope

- Installing anything other than the `SKILL.md` for each skill (for example,
  vendoring additional files a skill may grow).
- Uninstalling or updating skills; `install` only writes files and overwrites.
- Remote skill distribution; skills are embedded from the local repository at
  build time.