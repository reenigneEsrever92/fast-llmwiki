---
name: fawi-propose
description: Turn a feature or change request into a backlog item — inspect the codebase to validate feasibility, then write a type - ChangeRequest under docs/dev/backlog.
---

# Proposing a change

A change request is the entry point of the change-driven workflow. Before any
code is written, capture the request as a `type: ChangeRequest` in the backlog
and validate that it is actually feasible against the current codebase.

## 1. Understand the request

Clarify what is being asked, why, and who wants it. Read the codebase to ground
the request in reality — the crates under `crates/`, the relevant docs, and the
existing tests. Do not invent files, crates, or commands.

## 2. Validate feasibility

Check the request against the code:

- Where would the change land? Name the crate(s), module(s), and file(s).
- Is there an existing mechanism it can extend, or does it need something new?
- What are the risks, tradeoffs, and out-of-scope concerns?
- Is it blocked by a missing dependency, an external service, or a hard constraint?

Record these findings in the `# Feasibility` section.

## 3. Write the change request

Create `docs/dev/backlog/<slug>.md` with this front matter:

    ---
    type: ChangeRequest
    title: <Title>
    description: <one-line summary>
    state: proposed
    priority: <low|medium|high>
    tags: [dev, <topic>]
    owner: <actor>
    ---

A change request uses a single `state` field (no `status`) to capture its whole
lifecycle: `proposed` → `planned` → `in-progress` → `done`. `fawi-check` may
move it to `rejected` or `superseded`.

## 4. Fill in the sections

- `# Problem` — the motivation or gap.
- `# Proposal` — the change in one or two paragraphs.
- `# Feasibility` — the findings from step 2.
- `# Acceptance criteria` — testable, concrete outcomes.

## Next steps

`fawi-plan` appends the implementation plan to this request.
