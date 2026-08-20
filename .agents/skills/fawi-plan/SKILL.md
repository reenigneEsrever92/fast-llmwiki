---
name: fawi-plan
description: Append an implementation plan to a backlog change request — the technical approach, files to touch, and an ordered step checklist — and move it to state - planned.
---

# Planning a change

A plan is the "how". Append it to an existing `type: ChangeRequest` in
`docs/dev/backlog/` — the request stays a single document; the plan is a section
inside it.

## 1. Find the request

    grep -rl "^state: proposed" docs/dev/backlog

Read the request's `# Problem`, `# Proposal`, and `# Decisions` sections.

## 2. Write the approach

Read the relevant crates and tests. Work out:

- The crate(s), module(s), and file(s) to touch.
- The technology choices and data flow.
- Key decisions and tradeoffs.

## 3. Append the plan

Add an `# Implementation plan` section to the same `<slug>.md`, after the request
sections:

    ## Approach

    <technical details>

    ## Steps

    - [ ] <concrete, ordered step>
    - [ ] ...

Each step should be verifiable and map to an acceptance criterion.

## 4. Update the front matter

Set `state: planned`.

## Next steps

`fawi-implement` implements the steps and marks the request done.
