---
type: Reference
title: Front Matter
description: The YAML fields the server reads from a concept.
tags: [okf, reference, frontmatter]
status: stable
---

# Front Matter

Every concept begins with a YAML block delimited by `---`.

# Schema

| Field | Required | Description |
| --- | --- | --- |
| `type` | yes | The concept kind. |
| `title` | no | Display name. |
| `description` | no | One-line summary. |
| `resource` | no | Canonical URI for the underlying asset. |
| `tags` | no | List of short strings. |
| `status` | no | `draft`, `stable`, or `deprecated`. |
| `generated` | no | `{ by, at }` — who produced the content and when. |
| `verified` | no | One or more `{ by, at }` confirmation events. |
| `stale_after` | no | Absolute `YYYY-MM-DD` staleness date. |
| `sources` | no | Provenance entries with credibility signals. |

# Examples

    ---
    type: Reference
    title: Example
    tags: [demo]
    status: draft
    ---

See [Trust model](trust-model.md) for the derived trust tiers and staleness.
