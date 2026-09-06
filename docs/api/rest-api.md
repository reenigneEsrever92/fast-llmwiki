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
| GET | `/api/search?q=` | Hybrid search results (keyword + semantic, fused). |
| GET | `/api/ws` | WebSocket upgrade for hot reload. |

`/api/search` runs every configured search provider over the bundle and merges
their ranked lists with reciprocal rank fusion. Keyword search matches the id,
title, type, tags, description, and body, weighting title and id matches above
tags/type and description/body; the semantic provider ranks by cosine
similarity over local vector embeddings and is included when the embedding
model is available (the merged `okf` binary falls back to keyword-only if it
cannot load). Results are returned best-first; an empty `q` returns no results.

The payload is a list of concept summaries (same shape as directory listings),
so clients can treat search results and listings identically.

`/api/dirs` also accepts `sort=<field>` and `dir=asc|desc`. `sort` orders the
listing by any front matter key (falling back to title); `dir` sets the sort
direction and defaults to `asc` when omitted.

# Examples

    curl http://127.0.0.1:8080/api/concepts/overview
    curl 'http://127.0.0.1:8080/api/search?q=trust'
    curl 'http://127.0.0.1:8080/api/dirs?sort=status&dir=desc'

See [Hot reload](websocket.md) for the WebSocket protocol.
