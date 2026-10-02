# Backlog

The backlog holds open work — proposed and planned changes. Each item is a
`type: ChangeRequest` document whose `kind` distinguishes the four change
types — `feature`, `bug`, `refactor`, and `improvement` — and whose `state` moves through
`proposed` → `planned` → `in-progress` → `done` (or `rejected` / `superseded`
when it no longer applies).

Finished requests are retired from the backlog: once a request is `done`,
`rejected`, or `superseded` and has been settled for a while, it is condensed
into [Condensed backlog](summary.md) and its original document is deleted.

See [Development](../index.md) for the workflow and the `fawi-propose`,
`fawi-fix`, `fawi-refactor`, `fawi-improve`, `fawi-plan`, `fawi-implement`,
`fawi-check`, `fawi-review`, `fawi-review-all`, and `fawi-condense` skills.
