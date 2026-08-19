---
type: Plan
title: Auto-refresh the sidebar and the current page on bundle changes
status: stable
state: done
priority: medium
tags: [dev, gui]
verified: { by: ai:zed, at: 2026-08-19T09:43:33Z }
---

Implements [live reload](/dev/specs/live-reload.md).

- [x] Replace the page watcher's `use_params_map()` (outside a route) with a
      reactive read of the current location so hydration no longer panics.
- [x] Drive the sidebar tree refetch from a shared change signal so the tree
      re-fetches automatically on any bundle change.
- [x] Re-fetch and re-render the currently-viewed page in place when it is
      affected, instead of showing a notification with a manual reload action.
- [x] Keep the page watch following client-side navigation and listening after
      each change.
- [x] Share a single `GET /api/ws` connection between the sidebar and page
      watcher, preserving the existing `watch`/`change` protocol (the `change`
      message now also carries the affected `paths` array so a root watcher can
      decide which page to reload).
- [x] Verify no re-fetch is triggered for unrelated changes.
- [x] Verify `cargo build -p okf-server`, `cargo build -p okf-gui --features ssr`,
      `cargo check -p okf-gui --features hydrate --target wasm32-unknown-unknown`,
      and `cargo test -p okf-core -p okf-storage` pass.