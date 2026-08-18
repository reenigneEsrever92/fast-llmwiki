---
name: okf-dev
description: Implement an OKF project task — find a state:todo task under docs/dev/tasks, read its linked spec under docs/dev/specs, implement it, run the build/tests, and update the task.
---

# OKF Development

Work is described by `type: Task` concepts in `docs/dev/tasks/` and
`type: Specification` concepts in `docs/dev/specs/`. The docs are themselves an
OKF bundle served by this project.

## 1. Find work

    grep -rl "^state: todo" docs/dev/tasks
    grep -rl "^state: blocked" docs/dev/tasks

Pick the highest `priority`. If none exist, check `docs/dev/roadmap.md` for what
to spec next and write it first with the `okf-spec` skill.

## 2. Read the spec

Follow the markdown link in the task to its spec. Read `# Problem`,
`# Requirements`, and `# Acceptance Criteria` before writing code.

## 3. Implement

Follow `docs/architecture.md` for the crate layout. Make the smallest change
that satisfies the acceptance criteria, and add or update tests.

## 4. Verify

    cargo build -p okf-server
    cargo build -p okf-gui --features ssr
    cargo test -p okf-core -p okf-storage

## 5. Update the docs

- Set the task's `state: done` and `status: stable`.
- Add `verified: { by: <actor>, at: <ISO-8601 timestamp> }`.
- If the feature shipped, set the spec's `status: stable`.
- Link the commit in the task body.
