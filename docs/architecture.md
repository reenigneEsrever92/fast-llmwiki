---
type: Architecture
title: Architecture
description: The crate layout and data flow.
tags: [architecture, rust]
status: stable
---

# Architecture

The project is a Cargo workspace with six crates.

# Schema

| Crate | Role |
| --- | --- |
| `okf-core` | OKF model, front matter parsing, markdown rendering, DTOs. |
| `okf-storage` | Read-only bundle scanner and change events. |
| `okf-server` | REST API and WebSocket (the `okf-server` binary). |
| `okf-gui` | Leptos web UI (the `okf-gui` binary). |
| `okf-search` | Semantic search over bundle embeddings (the `okf-search` binary). |
| `okf-cli` | Unified `okf` launcher that starts the server, GUI, and search. |

# Data flow

The `okf` binary (from `okf-cli`) opens the bundle once and serves the REST API,
web UI, and semantic search as a single merged axum router on one socket. The web
UI never touches the bundle directly; it queries the REST API over HTTP — both
during server-side rendering and after hydration — using the same origin, which
is what makes client-side navigation work without a proxy.

    okf (single socket)
      ├── /api/*       -> okf-server REST API
      ├── /pkg/*       -> embedded okf-gui client bundle
      ├── /api/search  -> okf-search semantic search
      └── /            -> okf-gui Leptos SSR + hydration

    okf-server / okf-search --read--> bundle directory (via okf-storage)

Changes on disk are detected by `notify` and broadcast to WebSocket clients, so
the UI can reload the sidebar and the current page automatically. See
[Hot reload](api/websocket.md).
