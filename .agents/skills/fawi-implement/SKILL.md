---
name: fawi-implement
description: Implement a planned change request from the backlog — follow its implementation plan, run the build and tests, mark it done, and append a short entry to docs/dev/changelog.md.
---

# Implementing a change

## 1. Find work

    grep -rl "^state: planned" docs/dev/backlog

Pick the highest `priority`. Read the request and its `# Implementation plan`
before writing any code.

## 2. Implement

Follow `docs/architecture.md` for the crate layout and consult the relevant docs
under `docs/` (`features.md`, and the API/CLI references) so the change stays
consistent with what is documented. Make the smallest change that satisfies the
acceptance criteria, and add or update tests.

## 3. Verify

    cargo build -p fawi-server
    cargo build -p fawi-gui --features ssr
    cargo test -p fawi-core -p fawi-storage

## 4. Close the request

- Set `state: done`.
- Add `verified: { by: <actor>, at: <ISO-8601 timestamp> }`.
- Link the commit in the body.

## 5. Update the docs

If the change alters documented behaviour — features, architecture, the REST API,
the CLI, or run instructions — update the affected docs under `docs/`
(`features.md`, `architecture.md`, `api/`, `server/cli.md`,
`getting-started.md`) so they stay accurate. Prefer editing the existing doc
over adding a new one.

## 6. Update the changelog

Append a short entry to `docs/dev/changelog.md` under today's date: what changed,
why, and the request slug.
