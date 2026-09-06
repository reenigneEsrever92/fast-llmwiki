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
| `fawi-core` | OKF model, front matter parsing, markdown rendering, DTOs. |
| `fawi-storage` | Read-only bundle scanner, change events, and search providers. |
| `fawi-server` | REST API and WebSocket (the `fawi-server` binary). |
| `fawi-gui` | Leptos web UI (the `fawi-gui` binary). |
| `fawi-search` | Semantic search provider over bundle embeddings (library; no binary). |
| `fawi-cli` | Unified `okf` launcher that starts the server, GUI, and search. |

```mermaid
flowchart TD
    cli[fawi-cli<br/>okf binary] --> server[fawi-server]
    cli --> gui[fawi-gui]
    cli --> search[fawi-search]
    server --> storage[fawi-storage]
    search --> storage
    server --> core[fawi-core]
    gui --> core
    storage --> core
```

# Data flow

The `okf` binary (from `fawi-cli`) opens the bundle once and serves the REST API
and web UI as a single merged axum router on one socket. The web UI never
touches the bundle directly; it queries the REST API over HTTP — both
during server-side rendering and after hydration — using the same origin, which
is what makes client-side navigation work without a proxy.

Search is a provider port in `fawi-storage` (`SearchProvider`): the `okf`
composition root injects the engines that back `GET /api/search` into
`fawi-server`. `fawi-storage` provides the keyword provider (weighted substring
matching), `fawi-search` provides the semantic provider (a local embedding model
over the same bundle), and `fawi-cli` assembles them into a `HybridSearch` whose
ranked lists are merged with reciprocal rank fusion. The search endpoint depends
only on the port, so a new search strategy is a new provider implementation, not
a new route.

```mermaid
flowchart LR
    browser[Web browser] -->|HTTP /api/*| server[fawi-server REST API]
    browser -->|/ + /pkg/*| gui[fawi-gui SSR + hydration]
    server --> bundle[(bundle directory)]
    storage[fawi-storage] --> bundle
    server --> storage
    server -->|/api/search via search providers| keyword[KeywordProvider in fawi-storage]
    server -->|/api/search via search providers| semantic[SemanticProvider in fawi-search]
    keyword --> bundle
    semantic --> bundle
```

`fawi-server` and `fawi-search` read the bundle directory through `fawi-storage`.
Changes on disk are detected by `notify` and broadcast to WebSocket clients, so
the UI can reload the sidebar and the current page automatically. See
[Hot reload](api/websocket.md).
