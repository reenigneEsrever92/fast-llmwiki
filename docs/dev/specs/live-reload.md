---
type: Specification
title: Live reload
description: Auto-refresh the sidebar and the currently-viewed page when the bundle changes on disk.
status: stable
tags: [dev, gui]
owner: human:felix
---

# Problem

The web UI is meant to react to filesystem changes without a manual refresh:
both the sidebar and the main content area should re-fetch their data and
re-render on their own. The pages are read-only, so there is nothing for the
user to confirm or lose by silently reloading a page that changed on disk. The
server already supports this: `FsBundle` broadcasts a `ChangeEvent` whenever the
bundle is re-scanned, and `GET /api/ws` accepts a `{"type":"watch","path":...}`
message and replies with `{"type":"change","path":...}` when the watched path is
affected.

This does not work in practice. The GUI opens two independent WebSocket
connections — one in `Sidebar` to refresh the tree, one in `HotReload` to react
to the current page — but the page watcher reads route params from outside a
matched route: it calls `use_params_map()` while rendered as a sibling of
`<Routes>` inside `<Router>`. In `leptos_router`, `use_params_map()` panics when
there is no matched route context, so hydration fails and none of the client
behaviour works. Even if it did not panic, the watcher captures the path once,
never follows client-side navigation, and stops listening after the first
change.

On top of that, the current page watcher only shows a notification asking the
user to reload. Since the page is read-only, that prompt is unnecessary friction:
the UI should just re-fetch the changed page from the API and re-render it.

# Requirements

- The GUI MUST hydrate successfully; the page watcher MUST NOT read route
  params from outside a matched route.
- The sidebar MUST re-fetch `GET /api/tree` and re-render automatically whenever
  the bundle changes on disk, without any user action.
- The currently-viewed page — a concept or a directory — MUST re-fetch its
  content from the API and re-render automatically when it changes on disk,
  without showing a notification and without any user action.
- The automatic page reload MUST update the page in place; it MUST NOT perform a
  full browser reload.
- The current page path MUST be derived reactively from the router (e.g. from
  the current location) so the watch follows client-side navigation without a
  full reload.
- The watch MUST keep listening after a change; a single change MUST NOT tear
  down the connection permanently.
- The GUI SHOULD use a single WebSocket connection to `GET /api/ws`, shared by
  the sidebar and the page watcher, instead of one connection per concern.
- The existing server protocol (`watch`/`change` JSON messages and the
  `is_affected` path matching) MUST be preserved unless it is provably
  incorrect.
- A change to an unrelated path MUST NOT trigger a page re-fetch; only pages
  affected by the change reload.

# Acceptance Criteria

- Given the web UI is loaded, when the bundle changes on disk, then the sidebar
  re-fetches the tree and reflects the change without a manual reload.
- Given the user is viewing a concept page, when that concept's markdown file
  changes on disk, then the UI re-fetches and re-renders that concept without a
  manual reload and without showing a notification.
- Given the user is viewing a directory page, when a file in that directory
  (such as its `index.md`) changes, then the UI re-fetches and re-renders the
  directory page without a manual reload.
- Given the user navigates to a different concept without a full reload, when
  that concept changes on disk, then the newly-viewed page re-renders in place.
- Given a change to a path unrelated to the current page, when it occurs, then
  the current page does not re-fetch.
- Given the app loads in the browser, when hydration runs, then the app does not
  panic and the sidebar and page watcher both function.

# Out of scope

- Partial or diff-based updates; re-fetching the whole page is sufficient.
- Editing, moving, or deleting concepts from the UI.
- Server-side changes beyond the existing `GET /api/ws` change protocol.
- Reconnection with exponential backoff; a simple reconnect on error is enough.