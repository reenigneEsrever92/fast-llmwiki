---
type: Reference
title: CLI
description: Command-line flags.
tags: [cli, reference]
status: stable
---

# CLI

## `okf` (unified launcher)

Starts the REST API, the web UI, the semantic search service, or all three.

    okf                 # start all (API on 8080, UI on 8081, search on 8082)
    okf server          # start the REST API only
    okf gui             # start the web UI only
    okf search          # start the semantic search API only

| Subcommand | Flag | Default | Description |
| --- | --- | --- | --- |
| `server` | `--data` | `./docs` | The bundle directory. |
| `server` | `--bind` | `127.0.0.1:8080` | Address to listen on. |
| `gui` | `--api-base-url` | `http://127.0.0.1:8080` | The REST API base URL. |
| `gui` | `--bind` | `127.0.0.1:8081` | Address to listen on. |
| `search` | `--data` | `./docs` | The bundle directory. |
| `search` | `--bind` | `127.0.0.1:8082` | Address to listen on. |

## Standalone binaries

The `okf-server`, `okf-gui`, and `okf-search` binaries remain available and
accept the same flags as their `okf` subcommands.
