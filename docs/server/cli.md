---
type: Reference
title: CLI
description: Command-line flags.
tags: [cli, reference]
status: stable
---

# CLI

## `okf` (unified launcher)

The `okf` binary runs the REST API, web UI, and search together on a
single socket, or any one of them on its own.

    okf                 # everything on a single socket (127.0.0.1:8080)
    okf server          # REST API only (127.0.0.1:8080)
    okf gui             # web UI only (127.0.0.1:8081)
    okf search          # search API only (127.0.0.1:8082)
    okf install         # install bundled agent skills (./.agents/skills)

When no subcommand is given, `okf` merges the server and web UI routers on
one socket, so the web UI and its API share an origin. The merged binary loads
the embedding model at startup and injects a keyword + semantic search engine;
if the model cannot load (for example on an air-gapped machine), it logs a
warning and serves keyword-only. `okf server` is keyword-only and `okf search`
is keyword + semantic over its own bundle.

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
| `install` | `--dir` | `.agents/skills` | Directory to install bundled skills into. |

`okf install` writes every agent skill embedded in the binary to
`<dir>/<name>/SKILL.md`, one directory per skill.

## Standalone binaries

The `fawi-server` and `fawi-gui` binaries remain available and accept the same
flags as their `okf` subcommands. `fawi-search` is a library that provides the
semantic search provider; run it through `okf search`. The web UI binary must
be built with the `ssr` feature:

    cargo run -p fawi-gui --features ssr -- --api-base-url http://127.0.0.1:8080 --bind 127.0.0.1:8081
