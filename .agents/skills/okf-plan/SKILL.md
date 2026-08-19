---
name: okf-plan
description: Write or refine an OKF implementation plan under docs/dev/plans that turns a spec into the technical approach and step-by-step implementation checklist.
---

# Writing an OKF Plan

A plan is the "how": it turns a `type: Specification` in `docs/dev/specs/` into
the technical details and concrete steps needed to implement it.

Create `docs/dev/plans/<slug>.md` using the same slug as its spec.

## Front matter

    ---
    type: Plan
    title: <Title>
    status: draft
    state: todo
    priority: <low|medium|high>
    tags: [dev, <topic>]
    ---

`status` is the OKF lifecycle field (`draft` → `stable` when shipped).
`state` and `priority` are producer extensions used on plans; `state` moves
`todo` → `in-progress` → `done` (or `blocked`) as work proceeds.

## Sections

- Link back to the spec at the top: `Implements [<spec title>](/dev/specs/<slug>.md).`
- `# Approach` — the technical details: crates and files touched, technology
  choices, data flow, key decisions, and tradeoffs.
- `# Steps` — a concrete, ordered `- [ ]` checklist that implements the approach
  and satisfies the spec's acceptance criteria.

## Example

    Implements [Full-text search](/dev/specs/full-text-search.md).

    ## Approach

    Extend `FsBundle::search` to also match `concept.content`, keeping the
    existing title/slug/type/tag matching and adding no new dependencies.

    ## Steps

    - [ ] Extend `FsBundle::search` to also match `concept.content`.
    - [ ] Add a test that a body-only match is returned.
