---
type: Plan
title: Preserve scroll position on hot reload
status: draft
state: done
priority: medium
tags: [dev, gui]
---

Implements [Preserve scroll position on hot reload](/dev/specs/preserve-scroll-on-reload.md).

# Approach

The fix lives entirely in `crates/fawi-gui/src/app.rs`, in the `Page` component,
and is gated on `#[cfg(feature = "hydrate")]` (no server-side behaviour changes,
no new dependencies).

`Page` already has everything needed to detect a reload and to restore scroll:

- `page_reload` (`RwSignal<u64>`) is bumped by `HotReload` only when the current
  page's path is affected by a bundle change.
- `id()` is the current route's `rest` param. Navigation changes `id` but not
  `page_reload`; a hot reload changes `page_reload` but not `id`.
- `data` is the `Resource` whose source is `(id(), page_reload.get())`.

So a same-page reload is unambiguously "`page_reload` increased while `id` stayed
the same". We detect that in a hydration-only `Effect` keyed on
`(id(), page_reload.get())`, capture the current `window.scroll_y()` before the
re-fetch re-renders the content, and restore it after the re-fetch settles.

Key facts that make this straightforward:

- `Resource` derefs to `reactive_graph::computed::AsyncDerived<T>`, which keeps
  the previous value while a refetch is in flight (so `data.get()` does not go
  through `None` and the `Suspense` fallback does not unmount the content).
- `data.ready().await` is the future that resolves when the resource "next
  finishes loading", i.e. exactly when a reload's fetch completes. We `spawn_local`
  that future so the restore waits for the new content rather than racing it.
- `window.scroll_y()`, `window.scroll_to_with_x_and_y(x, y)` and the
  `request_animation_frame` helper are all available under the existing
  `web-sys` `Window` feature (and `leptos_dom::helpers`, re-exported through
  `leptos::prelude`), so no `Cargo.toml` change is required.
- The browser clamps `scroll_to` to the new document height, which also covers the
  "new content is shorter than the old offset" acceptance criterion for free.

Restoration order in the effect:

1. Read `cur = (id(), page_reload.get())` and compare to the previous value it
   returned on the prior run (Leptos `Effect::new` passes the previous return
   value back in; the first run sees `None`).
2. If `id` is unchanged and `page_reload` increased, capture
   `window.scroll_y()`, then `spawn_local` a task that does
   `data.ready().await`, and finally calls `window.scroll_to_with_x_and_y(0.0, offset)`
   inside `request_animation_frame` so the offset is applied after the new DOM has
   been laid out.
3. Leave `id`-change runs (navigation) and the initial run alone — no capture, no
   restore, so navigation keeps its normal scroll behaviour.

## Tradeoffs

- Detecting the reload in `Page` (via `page_reload` vs `id`) keeps the logic local
  and avoids adding a new cross-component context signal or coordinating ordering
  between `HotReload` and `Page`.
- We rely on `data.ready().await` to sequence the restore; this is the same API
  the SSR path uses to know when a resource is done, so it is robust to the
  resource's in-flight value retention.
- `request_animation_frame` adds one frame of latency before the restore, which is
  imperceptible and avoids restoring onto a not-yet-laid-out page.

# Steps

- [x] In `Page`, add a hydration-only `Effect::new(move |prev: Option<(String, u64)>| { … })`
      that computes `cur = (id(), page_reload.get())` and returns `cur`, using the
      previous value to detect the same-id/reload-incremented case.
- [x] On that case, capture `web_sys::window().scroll_y()` and
      `wasm_bindgen_futures::spawn_local` a task that runs `data.ready().await`
      then restores via `window().scroll_to_with_x_and_y(0.0, offset)` wrapped in
      `request_animation_frame`.
- [x] Confirm the initial run (`prev == None`) and navigation runs (id changed)
      do not capture or restore, so navigation is unaffected.
- [x] Verify `cargo build -p fawi-gui --features ssr`,
      `cargo check -p fawi-gui --features hydrate --target wasm32-unknown-unknown`,
      and `cargo build -p fawi-server` still pass.
- [ ] Manually verify in the browser: scroll down a long page, edit its markdown
      on disk, and confirm the offset is preserved; then navigate to another page
      and confirm it starts at the top.