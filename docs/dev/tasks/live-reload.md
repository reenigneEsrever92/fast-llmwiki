---
type: Task
title: Auto-refresh the sidebar and notify when the current page changes
status: draft
state: todo
priority: medium
tags: [dev, gui]
---

Implements [live reload and change notifications](/dev/specs/live-reload.md).

- [ ] Replace the page watcher's `use_params_map()` (outside a route) with a
      reactive read of the current location so hydration no longer panics.
- [ ] Drive the sidebar tree refetch from a shared change signal so the tree
      re-fetches automatically on any bundle change.
- [ ] Show a notification when the currently-viewed page is affected, with a
      reload action that re-fetches the page.
- [ ] Keep the page watch following client-side navigation and listening after
      each change.
- [ ] Share a single `GET /api/ws` connection between the sidebar and page
      watcher, preserving the existing `watch`/`change` protocol.
- [ ] Verify no page-change notification is shown for unrelated changes.
