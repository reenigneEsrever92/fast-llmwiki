---
type: Specification
title: Preserve scroll position on hot reload
description: Keep the browser's scroll position when the current page re-renders in place after a bundle change.
status: draft
tags: [dev, gui]
owner: human:felix
---

# Problem

Live reload re-fetches and re-renders the currently-viewed page when its
underlying markdown changes on disk, so the reader always sees fresh content
without a manual refresh. That in-place reload holds the URL and the page frame
steady, but nothing preserves the browser's `window.scrollY` across the
re-render.

The result is that a small edit anywhere in a long concept or directory page
snaps the reader back to the top of the page. For a short page this is harmless,
but for a long page a tiny downstream change loses the reader's place and forces
them to scroll back down to find their spot every time. Live reload is meant to
be an unobtrusive refresh; resetting scroll on every change makes it
disorienting and discards the reader's reading position.

# Requirements

- The browser window scroll position (`window.scrollY`) MUST be preserved across
  an in-place hot reload of the currently-viewed page, so the reader stays at
  the same scroll offset after the new content is rendered.
- Scroll preservation MUST restore the offset only after the new page content has
  been rendered; restoring it while the loading fallback is still shown MUST NOT
  be relied on to produce the correct offset.
- Scroll preservation MUST apply only to a hot-reload re-render of the same page
  path. Navigating to a different path MUST NOT reuse a stale offset; it MUST
  keep the router/browser's normal scroll behaviour (that is, start at the top or
  use normal scroll restoration).
- The restoration MUST allow the browser to clamp the offset to the new content
  height; if the re-fetched page is shorter than the previous offset, the view
  SHOULD settle at the bottom of the new content rather than showing an
  impossible offset.
- A change to an unrelated path MUST NOT change the scroll offset (the current
  page already does not re-fetch for unrelated changes; scroll preservation MUST
  NOT introduce spurious scroll changes or forced re-renders).
- Preserving scroll MUST NOT require a full browser reload or a manual refresh.

# Acceptance Criteria

- Given the user is viewing a long concept page scrolled down to a section, when
  that concept's markdown file changes on disk and the page re-renders in place,
  then the browser's scroll offset is unchanged and the reader remains at the
  same section.
- Given the user is scrolled down on page A, when they navigate to a different
  page B without a full reload, then page B does not inherit page A's scroll
  offset and is shown at its normal start position.
- Given the user is scrolled down on a page, when that page hot-reloads and the
  new content is shorter than the previous scroll offset, then the view settles
  at the bottom of the new content without an exception or stuck position.
- Given a long page, when a path unrelated to the current page changes on disk,
  then the current page does not re-fetch and its scroll offset does not change.
- Given the web UI is loaded, when the current page hot-reloads while the user is
  not scrolled to the top, then the offset is restored without a full browser
  reload.

# Out of scope

- Preserving scroll across a full browser reload (e.g. pressing F5) or across
  separate browser sessions.
- Browser history scroll restoration for back/forward navigation, which is the
  router/browser's existing behaviour.
- Scroll anchoring or pinning to a specific heading or line across edits; the
  goal is only to keep the current scroll offset, not to track content that has
  moved.
- Preserving the sidebar's internal scroll position, if any, across a sidebar
  tree re-fetch.