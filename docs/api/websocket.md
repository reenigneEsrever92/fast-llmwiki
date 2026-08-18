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
2. The client sends `{"type":"watch","path":"overview"}`.
3. When a change affects that path, the server sends `{"type":"change","path":"overview"}`.

The web UI shows a toaster with a reload button on receipt. See
[Web UI](../gui/leptos-gui.md).
