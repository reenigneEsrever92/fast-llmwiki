---
type: Guide
title: Getting Started
description: Run the OKF API server and web UI.
tags: [guide, setup]
status: stable
---

# Getting Started

## Prerequisites

A recent Rust toolchain. The workspace is defined in the repository root.

## Run everything

The unified `okf` CLI starts the REST API, web UI, and semantic search together
in one process, merged onto a single socket:

    cargo run -p okf-cli

Everything is served at <http://127.0.0.1:8080>: the web UI, the REST API under
`/api/`, and semantic search under `/api/search/semantic`. The web UI queries
the API on the same origin automatically. Press `Ctrl-C` to stop.

To run the components on separate sockets, use the `server`, `gui`, and `search`
subcommands below.

## Run the REST API only

    cargo run -p okf-cli -- server

This is equivalent to the standalone `okf-server` binary:

    cargo run -p okf-server -- --data ./docs --bind 127.0.0.1:8080

The API is served at <http://127.0.0.1:8080/api/>. See the
[REST API](api/rest-api.md) reference for the endpoints.

## Run the web UI only

    cargo run -p okf-cli -- gui

This is equivalent to the standalone `okf-gui` binary:

    cargo run -p okf-gui --features ssr -- --api-base-url http://127.0.0.1:8080 --bind 127.0.0.1:8081

Then open <http://127.0.0.1:8081>.

The web UI queries the REST API over HTTP, both during server-side rendering
and in the browser. See [Web UI](gui/leptos-gui.md).

## Run semantic search only

    cargo run -p okf-cli -- search

This is equivalent to the standalone `okf-search` binary:

    cargo run -p okf-search -- --data ./docs --bind 127.0.0.1:8082

The semantic search API is served at <http://127.0.0.1:8082/api/search/semantic>.
On first run the local embedding model is downloaded from Hugging Face and
cached.

# Examples

    # with `cargo run -p okf-cli` (everything on 8080)
    curl http://127.0.0.1:8080/api/concepts/overview
    curl 'http://127.0.0.1:8080/api/search?q=trust'
    curl 'http://127.0.0.1:8080/api/search/semantic?q=revenue'
