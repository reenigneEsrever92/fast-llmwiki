---
type: Specification
title: Open Knowledge Format
description: The Open Knowledge Format (OKF) specification.
resource: https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md
tags: [okf, spec]
status: stable
generated: { by: human:maintainer, at: 2026-08-18T00:00:00Z }
verified: { by: human:maintainer, at: 2026-08-18T00:00:00Z }
sources:
  - id: okf-spec
    resource: https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md
    title: Open Knowledge Format (OKF) specification
---

# Overview

OKF is a minimal, human- and agent-friendly format: a directory of markdown
files with YAML front matter. See the [front matter](frontmatter.md) reference
for the fields this server reads.

# Concepts

- A **bundle** is a directory tree of markdown files.
- A **concept** is one markdown document with a required `type` field.
- A **concept ID** is the file path with the `.md` suffix removed.
- `index.md` and `log.md` are reserved filenames.

# Trust, lifecycle, provenance

See the [Trust model](trust-model.md) for how `generated`, `verified`, `status`,
`stale_after`, and `sources` are interpreted.
