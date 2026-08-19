---
type: Reference
title: CLI
description: Command-line flags.
tags: [cli, reference]
status: stable
---

# CLI

## `okf` (unified launcher)

The `okf` binary runs the REST API, web UI, and semantic search together on a
single socket, or any one of them on its own.

    okf                 # everything on a single socket (127.0.0.1:8080)
    okf server          # REST API only (127.0.0.1:8080)
    okf gui             # web UI only (127.0.0.1:8081)
    okf search          # semantic search only (127.0.0.1:8082)

When no subcommand is given, `okf` merges all three routers and serves them on
one socket, so the web UI and its API share an origin.

| Scope | Flag | Default | Description |
| --- | --- | --- | --- |
| `okf` | `--data` | `./docs` | The bundle directory. |
| `okf` | `--bind` | `127.0.0.1:8080` | Address to listen on. |
| `server` | `--data` | `./docs` | The bundle directory. |
| `server` | `--bind` | `127.0.0.1:8080` | Address to listen on. |
| `gui` | `--api-base-url` | `http://127.0.0.1:8080` | The REST API base URL. |
| `gui` | `--bind` | `127.0.0.1:8081` | Address to listen on. |
| `search` | `--data` | `./docs` | The bundle directory. |
| `search` | `--bind` | `127.0.0.1:8082` | Address to listen on. |

## Standalone binaries

The `okf-server`, `okf-gui`, and `okf-search` binaries remain available and
accept the same flags as their `okf` subcommands. The web UI binary must be
built with the `ssr` feature:

    cargo run -p okf-gui --features ssr -- --api-base-url http://127.0.0.1:8080 --bind 127.0.0.1:8081
