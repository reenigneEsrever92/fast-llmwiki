---
type: Plan
title: Add fawi-cli crate to launch server, GUI, or both
status: stable
state: done
priority: medium
tags: [dev, cli]
verified: { by: ai:zed, at: 2026-08-18T15:05:17Z }
---

Implements [unified CLI launcher](/dev/specs/fawi-cli.md).

- [x] Add `crates/fawi-cli` to the workspace with a `clap`-based `okf` binary.
- [x] Expose reusable start functions from `fawi-server` and `fawi-gui` (no tracing init in the library code), keeping the routers in their current crates.
- [x] Rename the `fawi-server` binary from `okf` to `fawi-server`.
- [x] Depend on `fawi-gui` with the `ssr` feature so the UI runs in-process.
- [x] Implement `server` and `gui` subcommands, and make the default (no subcommand) start both.
- [x] Wire the GUI `api-base-url` to the server bind address in the default both mode.
- [x] Handle SIGINT/SIGTERM so the default both mode shuts down both components.
- [x] Update `docs/getting-started.md` with the new launch commands.
