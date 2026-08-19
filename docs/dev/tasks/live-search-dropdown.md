---
type: Task
title: Add a live search dropdown and drop the redundant search-page input
status: draft
state: todo
priority: medium
tags: [dev, gui, search]
---

Implements [live search dropdown in the header](/dev/specs/live-search-dropdown.md).

- [ ] Render a live results dropdown under the header search input as the user types, reusing `fetch_search`.
- [ ] Submitting the header search navigates to `/search?q=<query>` (already the form action; keep it).
- [ ] Bind the header search input to the current query on the search page.
- [ ] Remove the in-page search form from the `Search` component.
- [ ] Close the dropdown on Escape and on outside click.
- [ ] Add a test that the search page renders no second search input.