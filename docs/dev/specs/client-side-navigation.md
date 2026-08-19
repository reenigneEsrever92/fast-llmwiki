---
type: Specification
title: Client-side navigation
description: Serve a hydrated SPA from a single distributable binary.
status: draft
tags: [dev, gui]
owner: human:felix
---

# Problem

The web UI is a Leptos app with an SSR server and a client hydration bundle.
Moving between internal pages — brand link, sidebar tree, breadcrumbs, directory
entries, concept links, and search results — triggered a full browser reload
instead of an in-place data fetch from the REST API.

The reload was not caused by using plain HTML `<a>` anchors: `leptos_router`
(0.8) already intercepts clicks on ordinary `<a>` elements with a global
`click` listener and performs client-side navigation. That listener is only
installed once the client hydration bundle runs in the browser. Two things
prevented it from running:

1. The server resolved Leptos options from empty environment variables instead
   of the manifest, so `output_name`/`site_root`/`site_pkg_dir` were wrong and
   the hydration script pointed at the wrong asset paths.
2. The client (WASM) bundle was never built under plain `cargo`; it is a
   `cargo-leptos` artifact, so `target/site/pkg` stayed empty.

The frontend must therefore embed the client bundle into the server binary and
serve it from memory, so a single executable serves a fully hydrated SPA and
navigates client-side without a full reload.

# Requirements

- The server MUST build and embed the client (WASM hydration) bundle into the
  server binary at compile time, so the web UI can be distributed as a single
  binary.
- The server MUST serve the embedded `okf.js` and `okf_bg.wasm` at
  `/pkg/okf.js` and `/pkg/okf_bg.wasm` with the correct content types.
- The web UI and the REST API MUST be served from a single socket (single
  origin), so the client's `/api/...` requests resolve correctly during
  hydration without proxying or CORS.
- On the client, page data requests MUST use an absolute URL (derived from the
  current page origin) rather than a relative path, because `reqwest` on WASM
  cannot resolve relative URLs.
- The server MUST resolve Leptos options from the crate manifest so the
  hydration script references the embedded assets correctly, independent of
  environment variables or launcher.
- After hydration, selecting an internal link (a path beginning with `/`) MUST
  navigate without a full browser reload.
- The browser URL MUST update on navigation so pages remain shareable and
  deep-linkable.
- Back/forward browser navigation MUST work without a full reload.
- Each navigated page MUST fetch its data from the REST API (the existing
  `fetch_page` / `fetch_search` path).
- Direct loads (opening a URL directly or refreshing) MUST continue to render
  server-side via SSR.
- Links to external resources (for example a concept's `resource` URL or a
  source's resource URL) MUST remain ordinary anchors and open as before.

# Acceptance Criteria

- Given a single built server binary, when the app loads in a browser, then the
  hydration bundle is served from the binary and the app hydrates without any
  external static files.
- Given the rendered page, when its HTML is inspected, then it references
  `/pkg/okf.js` and `/pkg/okf_bg.wasm`, and those requests return the embedded
  assets with `text/javascript` and `application/wasm` respectively.
- Given a hydrated app, when an internal link is selected, then the new page's
  data is fetched from the API and rendered without a full page reload.
- Given a hydrated app, when the browser back button is pressed, then the
  previous page is shown without a full page reload.
- Given a concept page, when the URL is opened directly in a new tab, then the
  page renders server-side and hydrates.
- Given a concept page with an external `resource` URL, when that link is
  selected, then the browser follows it as a normal external link.

# Out of scope

- Replacing internal `<a>` anchors with the `<A>` component: `leptos_router`
  already routes plain anchors client-side, and `<A>` only adds `aria-current`
  and relative-route resolution, neither of which affects the reload behavior.
- Converting the search form submission to client-side navigation (the search
  box is a `GET` form and may be handled separately).
- Prefetching, caching, or optimistic updates of page data.
- Animating or transitioning between pages.
- Changes to the REST API surface; the API stays as-is.
