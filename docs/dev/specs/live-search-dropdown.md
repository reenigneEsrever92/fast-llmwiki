---
type: Specification
title: Live search dropdown in the header
description: Show type-ahead results in the header search field, open a full results page on Enter, and remove the redundant in-page search field.
status: draft
tags: [dev, gui, search]
owner: human:felix
---

# Problem

The header search field is a bare form: typing does nothing until the user
submits, and submitting takes them to a separate search page. There is no
type-ahead feedback, so users cannot tell whether their query will match anything
without leaving the current page.

The search page then renders a second search input. Because the header search
field is already present on every page (including the search page), that second
field is redundant and splits the query the user has typed.

We want the header search field to show live results in a dropdown as the user
types, to navigate to a full results page when the user presses Enter, and to
have exactly one search field — the header one.

# Requirements

- The header search field MUST show a dropdown of live results as the user types,
  without requiring a submit and without leaving the current page.
- The dropdown results MUST reuse the existing search endpoint so they match the
  search page's full result set.
- Pressing Enter in the header search field MUST navigate to `/search?q=<query>`,
  which renders all results for the query.
- An empty query MUST NOT produce a live dropdown.
- The search page MUST NOT render its own search input; the header search field
  is the single search input.
- The header search field SHOULD reflect the active query while on the search
  page so the user can refine it.
- Selecting a dropdown result SHOULD navigate directly to that concept's page.
- The dropdown SHOULD close when the user presses Escape or clicks outside it.

# Acceptance Criteria

- Given the header search field is focused, when the user types a query with at
  least one match, then a dropdown appears listing the matching concepts without
  submitting.
- Given the header search field is focused, when the user types a query with no
  matches, then no dropdown results are shown.
- Given the header search field contains a query, when the user presses Enter,
  then the search page at `/search?q=<query>` is shown with all results.
- Given the search page is rendered, when viewing it, then only the header
  search field is present (there is no second in-page search input).
- Given the search page is rendered, when the header search field is inspected,
  then it reflects the active query.
- Given a dropdown result, when the user selects it, then the corresponding
  concept page is shown.
- Given the dropdown is open, when the user presses Escape or clicks outside it,
  then the dropdown closes.

# Out of scope

- Changes to search result ordering, ranking, or scoring.
- Changing the existing lexical search endpoint.
- Semantic search integration in the dropdown (the dropdown reuses the existing
  results source).
- Keyboard navigation within the dropdown beyond Enter and Escape.