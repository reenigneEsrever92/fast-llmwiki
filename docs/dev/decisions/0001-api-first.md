---
type: Decision
title: ADR-0001: API-first architecture
description: The web UI queries the REST API rather than touching the bundle.
status: stable
tags: [dev, adr]
---

# Context

The web UI and the REST API were initially coupled. We wanted the UI to be a
pure client so the API stays the single source of truth for the bundle.

# Decision

The `okf-gui` crate queries the REST API over HTTP, both during SSR and in the
browser. It never accesses the bundle directly.

# Consequences

- `okf-server` and `okf-gui` are separate binaries and can run on different hosts.
- SSR makes an HTTP request to the API at `127.0.0.1:<port>`.
- The API is independently consumable by other clients.
