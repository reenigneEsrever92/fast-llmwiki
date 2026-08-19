# Development

This directory drives development. It is itself an OKF bundle: every concept has
YAML front matter and is machine-readable.

## Contributing

See [Contributing](../contributing.md) for how to propose a feature.

## Workflow

1. Find a task in [tasks](tasks/) with `state: todo` (or `blocked`).
2. Read the spec it links to under [specs](specs/).
3. Implement, then set the task's `state` to `done` and `status` to `stable`.
4. When a feature ships, set its spec's `status` to `stable`.

## Conventions

- `status` (OKF §5.4) is used on **every** concept: `draft`, `stable`, `deprecated`.
- `state` (producer extension) is used only on `type: Task`: `todo`, `in-progress`, `blocked`, `done`.
- `priority` and `owner` are producer extensions used on tasks.

## Kinds

- [Roadmap](roadmap.md) — milestones and priorities.
- [Specifications](specs/) — feature specs.
- [Tasks](tasks/) — units of work.
- [Decisions](decisions/) — architecture decision records.
