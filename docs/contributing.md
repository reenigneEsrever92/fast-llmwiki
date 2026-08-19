---
type: Guide
title: Contributing
description: How to propose and land a feature through specs, plans, and pull requests.
tags: [dev, contributing]
status: stable
---

# Contributing

Features are spec-driven. A change to this repository starts as a
`type: Specification` in [specs](dev/specs/) rather than as code.

## Workflow

1. Write a spec under [specs](dev/specs/) describing the problem, requirements, and
   acceptance criteria. Open a pull request with the spec.
2. The pull request may stop at the spec, or it may optionally also add the
   matching [plan](dev/plans/) and the implementation right away.
3. Merge the spec first when it needs review on its own. The plan and
   implementation can follow in a later pull request.

## Writing a spec

Specs live in `docs/dev/specs/<slug>.md`. They have the front matter shown in
the [Development](dev/index.md) conventions and these sections:

- `# Problem` — the motivation.
- `# Requirements` — MUST/SHOULD requirements.
- `# Acceptance Criteria` — testable Given/When/Then statements.
- `# Out of scope` — what is explicitly excluded.

See the bundled `okf-spec` skill for the full template.

## Adding a plan and an implementation

A plan is a `type: Plan` concept in [plans](dev/plans/) that links back to its
spec and lays out the technical approach and concrete implementation steps. The
same pull request may carry the implementation that satisfies the spec's
acceptance criteria.

Before opening the pull request, verify with:

    cargo build -p okf-server
    cargo build -p okf-gui --features ssr
    cargo test -p okf-core -p okf-storage

When the feature ships, set the plan's `state` to `done` and the spec and plan
`status` to `stable`. See the bundled `okf-dev` skill for the full workflow.

Continuous integration runs the build and unit tests on every push to `main`
and every pull request, and a [release workflow](dev/releases.md) publishes
platform binaries from `v*` tags.
