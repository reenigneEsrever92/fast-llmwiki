---
type: Task
title: Enable client-side navigation without a full reload
status: draft
state: todo
priority: high
tags: [dev, gui]
---

Implements [client-side navigation](/dev/specs/client-side-navigation.md).

- [ ] Build the client (WASM hydration) bundle for `okf-gui` so the app hydrates
      in the browser (e.g. `cargo leptos build` / `cargo leptos serve`).
- [ ] Confirm the hydration bundle is served from `target/site` and loads in the
      browser with no console errors.
- [ ] Verify clicking an internal link fetches the new page's data from the API
      without a full page reload.
- [ ] Verify back/forward navigation and direct deep links still work.
- [ ] Update the run/build docs so the client bundle is built alongside the SSR
      binary, not just `cargo build -p okf-gui --features ssr`.
