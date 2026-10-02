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
   requestor, and add it to the backlog as a `feature`, `bug`, `refactor`, or
   `improvement` with the `fawi-propose`, `fawi-fix`, `fawi-refactor`, or
   `fawi-improve` skill.
2. **Plan** — append an implementation plan and set `state: planned` with the
   `fawi-plan` skill.
3. **Implement** — follow the plan, run the build and tests, mark it `state: done`,
   and append an entry to the changelog with the `fawi-implement` skill.
4. **Check** — re-validate the request against the code and update its state if
   it no longer applies with the `fawi-check` skill.
5. **Review** — review the current work (everything since the last commit) with
   the `fawi-review` skill, or audit the whole project with the
   `fawi-review-all` skill; a review settles its findings with the requestor and
   records the agreed follow-up as a new change request.
6. **Document** — bring the docs under `docs/` back in line with the code and
   the backlog with the `fawi-docs` skill.
7. **Condense** — when the backlog grows unwieldy, retire finished change
   requests into a single condensed summary with the `fawi-condense` skill.

## Conventions

- `status` (OKF §5.4) is used on conventional concepts: `draft`, `stable`,
  `deprecated`.
- `type: ChangeRequest` uses a single `state` field instead of `status`. It is a
  producer extension that captures the whole lifecycle: `proposed`, `planned`,
  `in-progress`, `done`, `rejected`, `superseded`.
- `kind` is a producer extension on change requests that distinguishes the four
  change types: `feature`, `bug`, `refactor`, and `improvement`.
- `priority` and `owner` are producer extensions used on change requests.
- `type: ChangeRequest` documents live in [backlog](backlog/); finished ones
  are condensed into [Condensed backlog](backlog/summary.md), and shipped work
  is recorded in the [changelog](changelog.md).

## Kinds

- [Backlog](backlog/) — open change requests (proposed and planned).
- [Condensed backlog](backlog/summary.md) — retired change requests in summary form.
- [Changelog](changelog.md) — everything that has shipped, newest first.
- [Releases](releases.md) — how release binaries are built and published.
