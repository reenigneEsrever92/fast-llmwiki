---
type: Reference
title: Web UI
description: The Leptos frontend.
tags: [gui, leptos, reference]
status: stable
---

# Web UI

The frontend is a [Leptos](https://leptos.dev/) app with server-side rendering
and hydration.

It queries the [REST API](../api/rest-api.md) over HTTP, both during SSR and in
the browser. It does not touch the bundle directly.

# Hot reload

On hydration it opens the [WebSocket](../api/websocket.md), watches the current
path, and shows a reload toaster when a change arrives.
