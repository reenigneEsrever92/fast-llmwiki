---
type: Specification
title: Full-text search in the bundle index
description: Search concept bodies, not just titles and slugs.
status: draft
tags: [dev, search]
owner: human:felix
---

# Problem

Search today only matches a concept's title, slug, type, description, and tags.
Useful matches in the body are missed.

# Requirements

- A query matches text in a concept body.
- Results remain sorted by title.
- Existing title/slug/type/tag matching is preserved.

# Acceptance Criteria

- Given a query that appears only in a body, when searching, then the concept is returned.
- Given an empty query, when searching, then no results are returned.

# Out of scope

- Relevance ranking and scoring.
