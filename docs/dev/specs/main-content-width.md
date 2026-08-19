---
type: Specification
title: Main content width
description: The main content column fills available width up to a fixed maximum on wide screens instead of shrinking to its contents.
status: stable
tags: [dev, gui]
owner: human:felix
---

# Problem

The main content area (`<main class="content">`) is a flex item inside the
`.layout` flex row, next to the sidebar. Its only width rule is
`max-width: 56rem` with no grow factor or width, so as a flex item it sizes to
its own content (`flex-basis: auto`) rather than filling the available space.

This means a page with little content — a short concept, an empty directory, or
a short search result — renders as a very narrow column on a wide screen,
leaving a large empty gap next to the sidebar. The intended behaviour is a
consistent reading column that fills the available width up to a fixed maximum
and only narrows when the viewport cannot accommodate that width.

# Requirements

- The main content column MUST fill the available horizontal space up to a fixed
  maximum width on wide screens, rather than sizing to its contents.
- The content column MUST remain bounded by a maximum width (currently `56rem`)
  so long lines stay readable.
- The content column MUST only narrow when the remaining viewport space (after
  the sidebar) is smaller than the maximum width.
- The content column MUST remain horizontally centered in the remaining space
  next to the sidebar (including when it is capped at the maximum width).
- The existing horizontal padding and vertical spacing MUST be preserved.
- The behaviour MUST apply consistently to every page: root, directory, concept,
  search, and not-found.

# Acceptance Criteria

- Given a wide viewport and a page with little content, when the page renders,
  then the main content column fills the available width up to the maximum
  width, not the width of its contents.
- Given a wide viewport and a page with a large amount of content, when the page
  renders, then the content column is capped at the maximum width.
- Given a viewport wider than the content column's maximum width, when the page
  renders, then the content column is centered in the space next to the sidebar.
- Given a viewport whose available space is narrower than the maximum width, when
  the page renders, then the content column shrinks to fit the available space.
- Given any page type (root, directory, concept, search, not-found), when it
  renders, then the content column behaves identically.

# Out of scope

- Changing the `56rem` maximum or making it user-configurable.
- Restyling the sidebar, header, or footer.
- Theming or dark-mode support.
- Server-side or API changes.
