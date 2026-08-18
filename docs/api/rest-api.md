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
| GET | `/api/dirs/{path}` | A directory listing. |
| GET | `/api/search?q=` | Search results. |
| GET | `/api/ws` | WebSocket upgrade for hot reload. |

# Examples

    curl http://127.0.0.1:8080/api/concepts/overview
    curl 'http://127.0.0.1:8080/api/search?q=trust'

See [Hot reload](websocket.md) for the WebSocket protocol.
