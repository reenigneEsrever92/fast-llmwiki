---
type: ChangeRequest
title: Full-text search in the bundle index
description: Search concept bodies, not just titles and slugs.
state: planned
priority: medium
tags: [dev, search]
owner: human:felix
---

# Problem

Search today only matches a concept's title, slug, type, description, and tags.
Useful matches in the body are missed.

# Proposal

Extend the bundle keyword search so a query also matches the rendered concept
body, keeping the existing title/slug/type/tag matching and the title-sorted
result order.

# Feasibility

- The change lives entirely in `fawi-core`'s `FsBundle::search` (or the storage
  crate's search path), where title/slug/type/tag matching already happens.
- No new dependencies are required; bodies are already held in memory as
  `concept.content`.
- Out of scope: relevance ranking and scoring (tracked separately in
  [Relevance-ranked search results](relevance-ranked-search.md)).

# Acceptance criteria

- Given a query that appears only in a body, when searching, then the concept is
  returned.
- Given an empty query, when searching, then no results are returned.

# Implementation plan

## Approach

Extend `FsBundle::search` to also match `concept.content`, keeping the existing
title/slug/type/tag matching and adding no new dependencies.

## Steps

- [ ] Extend `FsBundle::search` to also match `concept.content`.
- [ ] Add a test that a body-only match is returned.
