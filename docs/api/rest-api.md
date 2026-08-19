---
type: Reference
title: REST API
description: The JSON API endpoints.
tags: [api, reference]
status: stable
---

# REST API

All responses are JSON.

# Endpoints

| Method | Path | Description |
| --- | --- | --- |
| GET | `/api/concepts/{id}` | A concept, with rendered `content_html`. |
| GET | `/api/dirs` / `/api/dirs/{path}` | A directory listing (root or nested). |
| GET | `/api/tree` | The full bundle tree for navigation. |
| GET | `/api/search?q=` | Keyword search results. |
| GET | `/api/search/semantic?q=` | Semantic search results (via `okf-search`). |
| GET | `/api/ws` | WebSocket upgrade for hot reload. |

`/api/search` matches titles, types, descriptions, and tags.
`/api/search/semantic` is provided by the `okf-search` crate and ranks results
by cosine similarity using a local embedding model; it is served on the same
socket when running the merged `okf` binary.

# Examples

    curl http://127.0.0.1:8080/api/concepts/overview
    curl 'http://127.0.0.1:8080/api/search?q=trust'

See [Hot reload](websocket.md) for the WebSocket protocol.
