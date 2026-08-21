---
type: ChangeRequest
kind: feature
title: Relevance-ranked search results
description: Rank keyword and semantic search results by relevance rather than title.
state: proposed
priority: low
tags: [dev, search]
owner: human:felix
---

# Problem

Keyword search results are sorted by title, and body matches are not weighted,
so the most relevant result is not necessarily first.

# Proposal

Introduce a relevance score for keyword search — weighting matches in the title,
type, and tags above matches in the body — and order results by that score before
falling back to title order. Leave semantic search's cosine-similarity ranking as
is.

# Decisions

- Keyword search is implemented in the `fawi-core` / storage search path; scoring
  can be added there without new dependencies.
- The change interacts with [Full-text search](full-text-search.md), which adds
  body matching, and should build on it rather than conflict with it.
- Out of scope: learning-to-rank or external search engines.

# Acceptance criteria

- Given a query that matches both a title and a body, when searching, then the
  title match is ranked before the body-only match.
- Given results with equal scores, when searching, then order falls back to title.
