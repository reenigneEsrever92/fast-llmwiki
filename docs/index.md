# OKF Bundle Server

A read-only server and web UI for the [Open Knowledge Format](okf-format.md),
built with Rust, axum, and Leptos.

## Documentation

- [Overview](overview.md) — what this project is and why.
- [Getting started](getting-started.md) — run the API server and the web UI.
- [Architecture](architecture.md) — the crate layout and data flow.
- [Features](features.md) — what the server does.
- [Contributing](contributing.md) — how to propose and land a feature.

## Reference

- [OKF format](okf-format.md) — the Open Knowledge Format specification.
- [Front matter](frontmatter.md) — the YAML fields the server reads.
- [Trust model](trust-model.md) — trust, lifecycle, and provenance.
- [REST API](api/) — the JSON API.
- [Hot reload](api/websocket.md) — WebSocket change events.
- [CLI](server/cli.md) — command-line flags.
- [Web UI](gui/leptos-gui.md) — the Leptos frontend.

## Development

- [Development](dev/) — the change-driven backlog and changelog that guide work.
