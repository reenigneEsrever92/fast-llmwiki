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

The web UI never touches the bundle. It queries the REST API over HTTP, both
during server-side rendering and after hydration.

    okf-gui (Leptos) --HTTP--> okf-server (REST API) --reads--> bundle directory

    okf-search --reads--> bundle directory (via okf-storage), serves embeddings

Changes on disk are detected by `notify` and broadcast to WebSocket clients, so
the UI can offer a reload. See [Hot reload](api/websocket.md).
