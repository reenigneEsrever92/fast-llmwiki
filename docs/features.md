---
type: Feature
title: Features
description: What the server does.
tags: [features, overview]
status: stable
---

# Features

- **Read-only** bundle browsing, addressed by concept ID (file path without `.md`).
- **REST API** for concepts, directories, the navigation tree, and search. See [REST API](api/rest-api.md).
- **Semantic search** over vector embeddings with a local model, reindexed on change. See [REST API](api/rest-api.md).
- **Trust, lifecycle, and provenance** rendering from front matter. See [Trust model](trust-model.md).
- **Hot reload** over WebSocket with a reload toaster. See [Hot reload](api/websocket.md).
- **Markdown rendering** with bundle link resolution.
- **Server-side rendering** plus hydration via Leptos. See [Web UI](gui/leptos-gui.md).
- **Self-documenting**: this `docs/` directory is an OKF bundle.
