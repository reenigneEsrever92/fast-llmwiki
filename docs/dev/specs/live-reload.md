---
type: Specification
title: Live reload and change notifications
description: Auto-refresh the sidebar and prompt the user when the current page changes on disk.
status: draft
tags: [dev, gui]
owner: human:felix
---

# Problem

The web UI is meant to react to filesystem changes without a manual refresh:
the sidebar should re-fetch the bundle tree on its own, and the user should be
notified when the contents of the page they are looking at have changed so they
can reload it. The server already supports this: `FsBundle` broadcasts a
`ChangeEvent` whenever the bundle is re-scanned, and `GET /api/ws` accepts a
`{"type":"watch","path":...}` message and replies with
`{"type":"change","path":...}` when the watched path is affected.

This does not work in practice. The GUI opens two independent WebSocket
connections — one in `Sidebar` to refresh the tree, one in `HotReload` to show a
reload prompt — but the page watcher reads route params from outside a matched
route: it calls `use_params_map()` while rendered as a sibling of `<Routes>`
inside `<Router>`. In `leptos_router`, `use_params_map()` panics when there is
no matched route context, so hydration fails and none of the client behaviour
works. Even if it did not panic, the watcher captures the path once, never
follows client-side navigation, and stops listening after the first change.

# Requirements

- The GUI MUST hydrate successfully; the page watcher MUST NOT read route
  params from outside a matched route.
- The sidebar MUST re-fetch `GET /api/tree` and re-render automatically whenever
  the bundle changes on disk, without any user action.
- The GUI MUST show a notification when the currently-viewed page — a concept or
  a directory — has changed on disk.
- The notification MUST offer an action that reloads the current page.
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
- A change to an unrelated path MUST NOT trigger a page-change notification;
  only pages affected by the change notify.

# Acceptance Criteria

- Given the web UI is loaded, when the bundle changes on disk, then the sidebar
  re-fetches the tree and reflects the change without a manual reload.
- Given the user is viewing a concept page, when that concept's markdown file
  changes on disk, then the UI shows a notification that the page changed.
- Given the user is viewing a directory page, when a file in that directory
  (such as its `index.md`) changes, then the UI shows a notification that the
  page changed.
- Given a change notification is visible, when the user activates the reload
  action, then the current page reloads and shows the updated content.
- Given the user navigates to a different concept without a full reload, when
  that concept changes on disk, then the notification is shown for the
  newly-viewed page.
- Given a change to a path unrelated to the current page, when it occurs, then
  no page-change notification is shown for the current page.
- Given the app loads in the browser, when hydration runs, then the app does not
  panic and the sidebar and page watcher both function.

# Out of scope

- Auto-reloading the main content area; only the sidebar reloads automatically.
- Editing, moving, or deleting concepts from the UI.
- Server-side changes beyond the existing `GET /api/ws` change protocol.
- Reconnection with exponential backoff; a simple reconnect on error is enough.
