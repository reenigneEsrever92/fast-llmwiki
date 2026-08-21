# Development

This directory drives development. It is itself an OKF bundle: every concept has
YAML front matter and is machine-readable.

## Contributing

See [Contributing](../contributing.md) for how to propose a change.

## Workflow

Development is change-driven. A change starts as a `type: ChangeRequest` in the
[backlog](backlog/), is planned, implemented, and finally recorded in the
[changelog](changelog.md):

1. **Propose** — inspect the codebase, settle open questions with the
   requestor, and add it to the backlog as a `feature`, `bug`, or `refactor`
   with the `fawi-propose`, `fawi-fix`, or `fawi-refactor` skill.
2. **Plan** — append an implementation plan and set `state: planned` with the
   `fawi-plan` skill.
3. **Implement** — follow the plan, run the build and tests, mark it `state: done`,
   and append an entry to the changelog with the `fawi-implement` skill.
4. **Check** — re-validate the request against the code and update its state if
   it no longer applies with the `fawi-check` skill.

## Conventions

- `status` (OKF §5.4) is used on conventional concepts: `draft`, `stable`,
  `deprecated`.
- `type: ChangeRequest` uses a single `state` field instead of `status`. It is a
  producer extension that captures the whole lifecycle: `proposed`, `planned`,
  `in-progress`, `done`, `rejected`, `superseded`.
- `kind` is a producer extension on change requests that distinguishes the three
  change types: `feature`, `bug`, and `refactor`.
- `priority` and `owner` are producer extensions used on change requests.
- `type: ChangeRequest` documents live in [backlog](backlog/); shipped work is
  recorded in the [changelog](changelog.md).

## Kinds

- [Backlog](backlog/) — proposed and planned change requests.
- [Changelog](changelog.md) — everything that has shipped, newest first.
- [Releases](releases.md) — how release binaries are built and published.
