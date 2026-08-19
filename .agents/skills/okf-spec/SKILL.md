---
name: okf-spec
description: Write or refine an OKF feature specification under docs/dev/specs with the required front matter and Problem / Requirements / Acceptance Criteria sections.
---

# Writing an OKF Specification

Create `docs/dev/specs/<slug>.md`.

## Front matter

    ---
    type: Specification
    title: <Title>
    description: <one-line summary>
    status: draft
    tags: [dev, <topic>]
    owner: human:<id>
    ---

`status` is the OKF lifecycle field (`draft` → `stable` when shipped).

## Sections

- `# Problem` — the problem or motivation.
- `# Requirements` — a list of MUST/SHOULD requirements.
- `# Acceptance Criteria` — testable Given/When/Then statements.
- `# Out of scope` — what is explicitly excluded.

## Next steps

A spec is the “what”. Write the “how” separately with the `okf-plan` skill,
which creates a matching `type: Plan` in `docs/dev/plans/` and links back to
this spec.
