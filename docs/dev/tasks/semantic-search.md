---
type: Task
title: Add okf-search crate and wire semantic search into the CLI
status: stable
state: done
priority: medium
tags: [dev, search]
verified: { by: ai:zed, at: 2026-08-18T15:40:01Z }
---

Implements [semantic search](/dev/specs/semantic-search.md).

- [x] Add `crates/okf-search` to the workspace with a reusable `serve(data, bind)` startup function and an `okf-search` binary.
- [x] Index the whole bundle (title, description, tags, and body) with a local embedding model, reusing `okf-storage` for bundle reads and change events.
- [x] Expose an HTTP endpoint that returns relevance-ranked concept summaries with a score (new DTO in `okf-core`).
- [x] Add a `search` subcommand to `okf-cli` and start the service together with the server and GUI in the default (no-subcommand) mode.
- [x] Keep the existing lexical `GET /api/search` behavior unchanged.
- [x] Add tests for ranking, body-only matches, empty queries, and index rebuilds on change.

Commit: not recorded (no git repository present at implementation time).
