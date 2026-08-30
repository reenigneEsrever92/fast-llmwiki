---
type: Guide
title: Contributing
description: How to propose, plan, implement, and record a change through the change-driven workflow.
tags: [dev, contributing]
status: stable
---

# Contributing

Development is change-driven. A change to this repository starts as a
`type: ChangeRequest` in the [backlog](dev/backlog/) rather than as code.

## Workflow

1. **Propose** — inspect the codebase, settle any open questions with the
   requestor, and add a change request to the [backlog](dev/backlog/) describing
   the problem, proposal, key decisions, and acceptance criteria. See the
   bundled `fawi-propose` (feature), `fawi-fix` (bug), and `fawi-refactor`
   (redesign/quality) skills.
2. **Plan** — append an implementation plan (technical approach and an ordered
   step checklist) to the request and set its `state` to `planned`. See
   `fawi-plan`.
3. **Implement** — follow the plan, run the build and tests, update any docs the
   change affects, set the request to `state: done`, and append a short entry to
   the [changelog](dev/changelog.md). See `fawi-implement`.
4. **Check** — before or during implementation, re-validate the request against
   the current codebase and update its state to `rejected` or `superseded` if it
   no longer applies. See `fawi-check`.

## Writing a change request

Change requests live in `docs/dev/backlog/<slug>.md`. They have the front matter
shown in the [Development](dev/index.md) conventions — including a `kind` of
`feature`, `bug`, or `refactor` — and these sections:

- `# Problem` — the motivation or gap.
- `# Proposal` — the change in one or two paragraphs.
- `# Decisions` — the key decisions and where the change lands, agreed with the
  requestor while proposing.
- `# Acceptance criteria` — testable, concrete outcomes.
- `# Implementation plan` — added later by `fawi-plan` (approach and steps).

## Implementing a change

Before opening a pull request, verify with:

    cargo build -p fawi-server
    cargo build -p fawi-gui --features ssr
    cargo test -p fawi-core -p fawi-storage

When the change ships, update any docs under `docs/` that it affects (features,
architecture, API, CLI), set the request's `state` to `done`, then append an
entry to the [changelog](dev/changelog.md). See the bundled `fawi-implement`
skill for the full workflow.

Continuous integration runs the build and unit tests on every push to `main`
and every pull request, and a [release workflow](dev/releases.md) publishes
platform binaries from `v*` tags.
