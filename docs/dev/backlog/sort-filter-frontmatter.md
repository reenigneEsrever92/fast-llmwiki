---
type: ChangeRequest
kind: feature
title: Sort and filter by front matter fields
description: Order and narrow directory listings by any front matter field.
state: done
priority: medium
tags: [dev, gui]
owner: human:felix
verified: { by: human:felix, at: 2026-08-20T19:21:46Z }
---

# Problem

Directory listings are always sorted by title and cannot be narrowed. The server
parses only a fixed set of front matter fields (`type`, `title`, `description`,
`resource`, `tags`, `status`, `generated`, `verified`, `stale_after`, `sources`)
and silently discards everything else, so there is no way to order or filter a
listing by any of those fields — let alone by a producer- or bundle-specific
field the server does not natively know about.

# Proposal

Add `sort` and `filter` query parameters to the directory listing endpoint
(`/api/dirs`) that work against the full front matter of each concept rather than
a fixed field list. To make that possible, preserve every front matter key on
`Concept`/`ConceptSummary` (and their DTOs) instead of dropping unknown ones, then
implement a generic sort/filter that treats values uniformly: case-insensitive
string comparison for scalars, numeric and date comparison where values are
comparable, and list membership for list-valued fields. The web UI gains
lightweight sort and filter controls on the directory pages.

`sort=<field>` orders by that field (falling back to `title`), and
`filter=<field>=<value>` narrows to matching concepts (repeated filters are
ANDed; a list-valued field matches when any element matches). Values that cannot
be compared (e.g. nested maps) or keys that are absent are handled stably rather
than erroring.

# Decisions

- The sort/filter logic lands in `fawi-storage`, where `FsBundle::list_dir`
  already produces a sorted `Vec<ConceptSummary>`. The `BundleSource` trait and
  the directory handlers in `fawi-server/src/api.rs` must thread `sort`/`filter`
  through; `fawi-gui` adds controls.
- Fully generic support requires preserving raw front matter. Today
  `fawi-core`'s `Concept::from_markdown`/`parse_meta` extracts only the known
  fields and drops the rest, so `Concept` must also carry the full front matter
  (e.g. as a `serde_yaml::Value` map). Sorting and filtering are applied
  server-side in `FsBundle::list_dir`, so the browser only sends the
  `sort`/`filter` query params; exposing the available field names in the DTO is
  deferred until a field picker is built.
- A generic comparison layer holds the main complexity: define how scalars,
  numbers, dates, and booleans compare, how lists are matched, and how
  non-comparable values (nested maps) and missing keys degenerate. These cases
  each need tests.
- No new dependencies; `serde_yaml` is already used to parse front matter.
- Scoped to directory listings only; keyword and semantic search are unaffected
  and remain title-sorted / relevance-ranked respectively.

# Acceptance criteria

- Given a directory listing with `sort=status`, when fetched, then concepts are
  ordered by `status` rather than title.
- Given a directory listing with `filter=type=Metric`, when fetched, then only
  concepts whose `type` is `Metric` are returned.
- Given a directory listing with `sort=<field>` for a producer-defined key the
  server does not natively model, when fetched, then concepts are ordered by that
  key's value.
- Given `filter=<field>=<value>` for an arbitrary scalar front matter key, when
  fetched, then only concepts whose value matches are returned; for list-valued
  keys, a concept matches when any list element matches.
- Given a non-comparable value (e.g. a nested map) or an absent key, when
  requested, then ordering remains stable and filtering does not error.

# Implementation plan

## Approach

Preserve the raw front matter on the domain model and make sorting/filtering a
server-side operation in the storage crate, driven by two query parameters on the
directory endpoint. The browser stays thin: it sends `sort`/`filter` and renders
whatever the server returns.

- **Raw front matter**: add a `front_matter: serde_yaml::Value` field to
  `fawi_core::Concept`. `Concept::from_markdown` already deserializes the YAML
  once; retain that value instead of dropping unknown keys, and derive the typed
  `Meta` fields from it. `Concept::summary()` does not need to carry the map.
- **Generic comparison**: extend `fawi-core`'s `front_matter.rs` with helpers to
  read a key from a YAML mapping and to compare/match two YAML values uniformly:
  scalars compare case-insensitively by string, numbers numerically, booleans
  with `false < true`, and lists sort by their elements and match when any
  element matches. Non-comparable values (nested maps) and missing keys sort last
  and never match, so nothing errors.
- **Storage**: extend `BundleSource::list_dir` to take a small `ListOptions`
  struct (`{ sort: Option<String>, filters: Vec<(String, String)> }`). In
  `FsBundle::list_dir`, filter by parent first, then apply each `filter` against
  `Concept.front_matter`, then sort by the requested field (fall back to `title`),
  then map to summaries.
- **Server**: `api::get_dir`/`get_dir_root` parse `sort` and repeated `filter`
  query params (splitting each `filter` on its first `=`), then pass them through.
- **GUI**: `Page`/`DirView` read `sort`/`filter` from the URL query and forward
  them to `/api/dirs`; `DirView` renders controls (a sort `<select>` and filter
  inputs) that navigate to the same path with updated query params.

## Steps

- [ ] Add `front_matter: serde_yaml::Value` to `fawi_core::Concept` and retain
  the parsed mapping in `Concept::from_markdown`; test that unknown keys survive.
- [ ] Add generic `front_matter` helpers (`get_field`, `compare_values`,
  `values_match`) in `fawi-core` with unit tests for scalars, numbers, booleans,
  lists, missing keys, and nested maps.
- [ ] Introduce `ListOptions { sort, filters }` and extend
  `BundleSource::list_dir` to accept it.
- [ ] Apply filtering then sorting in `FsBundle::list_dir` against raw front
  matter, defaulting sort to `title`; add a temp-bundle test for `sort=status`
  and a `filter=type=…` case.
- [ ] Parse `sort` and repeated `filter` params in `fawi-server/src/api.rs`
  `get_dir`/`get_dir_root` and forward them to `list_dir`.
- [ ] In `fawi-gui`, forward `sort`/`filter` query params from the URL to
  `/api/dirs` and add controls to `DirView` that update the URL.
- [ ] Document the new parameters in `docs/api/rest-api.md` and update
  `docs/frontmatter.md` if needed.

Implemented and verified (builds and tests pass); recorded in the
[changelog](../changelog.md) for 2026-08-20.