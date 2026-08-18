---
type: Reference
title: Trust Model
description: How trust, lifecycle, and provenance are derived from front matter.
tags: [okf, trust, provenance]
status: stable
generated: { by: human:maintainer, at: 2026-08-18T00:00:00Z }
verified: { by: human:maintainer, at: 2026-08-18T00:00:00Z }
sources:
  - id: okf-spec
    resource: https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md
    title: Open Knowledge Format (OKF) specification
---

# Trust Model

OKF records objective signals and lets the consumer derive verdicts.

# Trust tiers

- no `verified` → **unverified**
- `verified` by non-human actors → **machine-confirmed**
- `verified` by a `human:` actor → **human-reviewed**

# Lifecycle

- `status`: `draft`, `stable` (default), or `deprecated`.
- `stale_after`: an absolute date; a concept is stale when `today >= stale_after`.

# Provenance

`sources` records where a concept came from. Each entry carries a `resource`,
optional `id`, `title`, and credibility signals `author`, `usage_count`, and
`last_modified`.

# Actors

The actor convention is `<producer>/<version>` for agents, `human:<id>` for
people, and `process:<id>` for automated processes.
