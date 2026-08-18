---
type: Specification
title: Client-side navigation
description: Navigate between pages without a full browser reload.
status: draft
tags: [dev, gui]
owner: human:felix
---

# Problem

The web UI is a Leptos app with an SSR server and a client hydration bundle.
Moving between internal pages — brand link, sidebar tree, breadcrumbs, directory
entries, concept links, and search results — currently triggers a full browser
reload instead of an in-place data fetch from the REST API.

The reload is not caused by using plain HTML `<a>` anchors: `leptos_router`
(0.8) already intercepts clicks on ordinary `<a>` elements with a global
`click` listener and performs client-side navigation. That listener is only
installed once the client hydration bundle runs in the browser. When hydration
does not run, every internal link falls through to native browser navigation and
the page is reloaded.

The frontend should therefore ensure its client bundle is built, served, and
hydrated so that internal navigation is handled client-side and the new page's
data is fetched from the API without a full reload.

# Requirements

- The GUI MUST build and serve its client (WASM hydration) bundle so the app
  hydrates in the browser.
- After hydration, selecting an internal link (a path served by this app,
  beginning with `/`) MUST navigate without a full browser reload.
- The browser URL MUST update on navigation so pages remain shareable and
  deep-linkable.
- Back/forward browser navigation MUST work without a full reload.
- Each navigated page MUST fetch its data from the REST API (the existing
  `fetch_page` / `fetch_search` path), rather than embedding data in the page.
- Direct loads (opening a URL directly or refreshing) MUST continue to render
  server-side via SSR.
- Links to external resources (for example a concept's `resource` URL or a
  source's resource URL) MUST remain ordinary anchors and open as before.

# Acceptance Criteria

- Given the client bundle is built and served, when the app loads in a browser,
  then hydration runs and internal links do not trigger a full page reload.
- Given a hydrated app, when an internal link is selected, then the new page's
  data is fetched from the API and rendered without a full page reload.
- Given a hydrated app, when the browser back button is pressed, then the
  previous page is shown without a full page reload.
- Given a concept page, when the URL is opened directly in a new tab, then the
  page renders server-side and hydrates.
- Given a concept page with an external `resource` URL, when that link is
  selected, then the browser follows it as a normal external link.
- Given a hydrated app, when an internal link is selected, then the browser URL
  changes to the destination path and the page's data is fetched once from the
  API.

# Out of scope

- Replacing internal `<a>` anchors with the `<A>` component: `leptos_router`
  already routes plain anchors client-side, and `<A>` only adds `aria-current`
  and relative-route resolution, neither of which affects the reload behavior.
- Converting the search form submission to client-side navigation (the search
  box is a `GET` form and may be handled separately).
- Prefetching, caching, or optimistic updates of page data.
- Animating or transitioning between pages.
- Changes to the REST API surface; the API stays as-is.
