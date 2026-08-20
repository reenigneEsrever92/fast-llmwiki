---
type: Specification
title: Unified CLI launcher
description: A single fawi-cli crate that starts fawi-server, fawi-gui, or both together.
status: stable
tags: [dev, cli]
owner: human:felix
---

# Problem

Running OKF today requires starting two binaries separately:

    cargo run -p fawi-server -- --data ./docs --bind 127.0.0.1:8080
    cargo run -p fawi-gui --features ssr -- --api-base-url http://127.0.0.1:8080 --bind 127.0.0.1:8081

The GUI must be told the API's address, and the two processes have to be
started, coordinated, and stopped independently. There is no single command to
launch the whole app.

# Requirements

- A new `fawi-cli` crate provides the primary `okf` binary as the unified entry point.
- The CLI MUST expose a `server` subcommand that starts the REST API.
- The CLI MUST expose a `gui` subcommand that starts the web UI.
- With no subcommand, the CLI MUST start the server and GUI concurrently.
- Defaults MUST match the current binaries: API on `127.0.0.1:8080`, GUI on
  `127.0.0.1:8081`, data at `./docs`.
- When starting both, the GUI's `api-base-url` MUST default to the server's bind address.
- Startup logic MUST be reused from the existing crates rather than duplicated.
- The server router (`fawi_server::api::router`) and the GUI router
  (`fawi_gui::ssr::router`) MUST remain in their current crates; `fawi-cli` only
  composes them into a single process.
- Logging MUST be initialized once regardless of how many components are started.
- A bind failure (e.g. port already in use) MUST fail the command with a clear error.
- Sending SIGINT or SIGTERM MUST shut down all running components cleanly.
- The `fawi-server` crate's binary MUST be renamed from `okf` to `fawi-server` so its
  name no longer collides with the unified `okf` binary.

# Acceptance Criteria

- Given the `server` subcommand, when run with defaults, then the REST API serves
  `GET /api/concepts/overview`.
- Given the `gui` subcommand, when run with defaults, then the web UI is served on
  `127.0.0.1:8081`.
- Given no subcommand, when run with defaults, then both the API (8080) and the UI
  (8081) respond and the UI queries the API at `http://127.0.0.1:8080`.
- Given a port already in use, when any subcommand starts, then the command exits
  non-zero with a message naming the port.
- Given a running default (both) process, when it receives SIGINT, then both servers
  stop and the process exits zero.

# Out of scope

- Process supervision, automatic restart, and daemonization.
- Orchestration across multiple hosts.
