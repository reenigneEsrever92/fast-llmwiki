---
type: Plan
title: Add a live search dropdown and drop the redundant search-page input
status: stable
state: done
priority: medium
tags: [dev, gui, search]
verified: { by: ai:zed, at: 2026-08-19T10:20:46Z }
---

Implements [live search dropdown in the header](/dev/specs/live-search-dropdown.md).

- [x] Render a live results dropdown under the header search input as the user types, reusing `fetch_search`.
- [x] Submitting the header search navigates to `/search?q=<query>` (client-side, via `use_navigate`).
- [x] Bind the header search input to the current query on the search page.
- [x] Remove the in-page search form from the `Search` component.
- [x] Close the dropdown on Escape and on outside click.
- [x] Verify `cargo build -p fawi-gui --features ssr` (server + WASM client), `cargo build -p fawi-server`, and `cargo test -p fawi-core -p fawi-storage` pass.

Commit: not recorded (changes left uncommitted).