---
name: okf-dev
description: Implement an OKF project plan — find a state:todo plan under docs/dev/plans, read its linked spec under docs/dev/specs, implement it, run the build/tests, and update the plan.
---

# OKF Development

Work is described by `type: Plan` concepts in `docs/dev/plans/` and
`type: Specification` concepts in `docs/dev/specs/`. The docs are themselves an
OKF bundle served by this project.

## 1. Find work

    grep -rl "^state: todo" docs/dev/plans
    grep -rl "^state: blocked" docs/dev/plans

Pick the highest `priority`. If none exist, check `docs/dev/roadmap.md` for what
to spec next, write it first with the `okf-spec` skill, then plan it with the
`okf-plan` skill.

## 2. Read the spec and plan

Follow the markdown link in the plan to its spec. Read `# Problem`,
`# Requirements`, and `# Acceptance Criteria`, then the plan's `# Approach` and
`# Steps` before writing code.

## 3. Implement

Follow `docs/architecture.md` for the crate layout. Make the smallest change
that satisfies the acceptance criteria, and add or update tests.

## 4. Verify

    cargo build -p fawi-server
    cargo build -p fawi-gui --features ssr
    cargo test -p fawi-core -p fawi-storage

## 5. Update the docs

- Set the plan's `state: done` and `status: stable`.
- Add `verified: { by: <actor>, at: <ISO-8601 timestamp> }`.
- If the feature shipped, set the spec's `status: stable`.
- Link the commit in the plan body.
