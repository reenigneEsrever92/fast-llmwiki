---
type: Task
title: Serve a hydrated SPA from a single binary
status: stable
state: done
priority: high
tags: [dev, gui]
verified: { by: ai:zed, at: 2026-08-18T18:15:00Z }
---

Implements [client-side navigation](/dev/specs/client-side-navigation.md).

- [x] Resolve Leptos options from `[package.metadata.leptos]` instead of empty
      env vars (`ssr.rs` uses `get_configuration(Some(.../Cargo.toml))`).
- [x] Add `build.rs` to build the client (wasm32, hydrate) bundle and run
      `wasm-bindgen` into `OUT_DIR`.
- [x] Embed the client bundle with `include_bytes!` (`assets.rs`).
- [x] Serve `/pkg/okf.js` and `/pkg/okf_bg.wasm` from memory in `ssr.rs`.
- [x] Combine the API, semantic-search, and GUI routers onto a single socket in
      `okf-cli` so the client's relative `/api` requests resolve on one origin.
- [x] Fix the client fetch to use an absolute URL (`window.location().origin()`)
      instead of a relative path, since `reqwest` on WASM cannot resolve relative
      URLs and returned `NotFound` without making a request.
- [x] Verify `cargo build --bin okf`, `cargo build -p okf-gui --features ssr`,
      and `cargo test -p okf-cli -p okf-core -p okf-storage` pass, and that the
      single socket serves the page, API, and embedded bundle.
