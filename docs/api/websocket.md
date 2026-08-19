---
type: Reference
title: Hot Reload
description: WebSocket change notifications.
tags: [api, websocket, hot-reload]
status: stable
---

# Hot Reload

The server watches the bundle and notifies clients when a watched page changes.

# Protocol

1. The client connects to `/api/ws`.
2. The client sends `{"type":"watch","path":"overview"}`. Watching the root
   (`""`) receives a change for every bundle change.
3. When a change affects that path, the server sends
   `{"type":"change","path":"overview","paths":["overview",...]}` where `paths`
   is the full set of affected concept IDs and directory paths. The `paths` field
   lets a client watching the root decide which of its pages to reload.

The web UI watches the root on a single connection, re-fetches the sidebar tree
on every change, and re-fetches the current page in place when its path is
affected. See [Web UI](../gui/leptos-gui.md).
