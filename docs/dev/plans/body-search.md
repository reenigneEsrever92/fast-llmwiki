---
type: Plan
title: Add body search to FsBundle::search
status: draft
state: todo
priority: medium
tags: [dev, search]
---

Implements [full-text search](/dev/specs/full-text-search.md).

- [ ] Extend `FsBundle::search` to also match `concept.content`.
- [ ] Add a test that a body-only match is returned.
