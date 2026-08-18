---
type: Task
title: Add a persistent sidebar for bundle navigation
status: stable
state: done
priority: medium
tags: [dev, gui]
verified: { by: ai:zed, at: 2026-08-18T15:37:09Z }
---

Implements [sidebar navigation](/dev/specs/sidebar-navigation.md).

- [x] Add a `tree` method to `FsBundle` returning the full directory tree (directories and the concept summaries within each).
- [x] Add a `GET /api/tree` route and a recursive tree DTO in `okf-core`.
- [x] Fetch the tree and render a persistent sidebar next to the main content in the Leptos shell.
- [x] Link directory and concept entries, and highlight the active page.
- [x] Collapse the sidebar behind a toggle on narrow viewports.
