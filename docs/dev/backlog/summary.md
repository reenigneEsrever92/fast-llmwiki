---
type: Reference
title: Condensed backlog
description: Retired change requests condensed to their problem, proposal, decisions, and acceptance criteria.
tags: [dev, backlog]
status: stable
---

# Condensed backlog

Finished change requests, condensed to their irreducible content and retired
from the backlog. Newest first. The [changelog](../changelog.md) links here for
shipped work.

## Hybrid search with pluggable keyword and semantic providers

`kind: feature` · `state: done` · finished 2026-09-06

**Problem** — Search was split across two endpoints the web UI could not
combine: `GET /api/search` ran storage keyword search (substring match on
id/title/type/description/tags, title order, no body recall), while the semantic
index in `fawi-search` was exposed only as `GET /api/search/semantic`, which
nothing in the UI called.

**Proposal** — Make `GET /api/search` the single search endpoint, backed by a
`SearchProvider` port in `fawi-storage` (`async fn search(&self, query: &str) ->
Vec<ScoredSummary>`). `KeywordProvider` wraps substring matching extended with
body matching and field-weighted relevance (title 4 > id 3 > type/tags 2 >
description/body 1, title tie-break); `SemanticProvider` adapts `fawi-search`'s
`SemanticIndex`. A `HybridSearch` engine in `fawi-storage` fuses the providers'
ranked lists with reciprocal rank fusion and is injected by `fawi-cli` (the
composition root); `/api/search/semantic` is removed.

**Decisions**

- `kind: feature`, superseding [Relevance-ranked search results](#relevance-ranked-search-results)
  and [Full-text search in the bundle index](#full-text-search-in-the-bundle-index);
  both behaviours are folded into `KeywordProvider`.
- Reciprocal rank fusion, not weighted scores — cosine similarity and a keyword
  heuristic are not on a comparable scale; RRF needs only each list to be
  internally ordered. `k = 60`, deduplicated by concept id.
- One endpoint: the response stays `Vec<ConceptSummaryResponse>`, so the GUI and
  SSR paths are untouched; `fawi-search` stops serving HTTP. Per-provider scores
  stay internal.
- Port and fusion live in `fawi-storage`, following the `BundleSource`
  precedent, rather than adding async traits to `fawi-core`.
- Graceful degradation: merged `okf` serves keyword-only (with a warning) when
  the embedding model cannot load; standalone `okf search` remains strict.
- Full ranked result set, no cap; learning-to-rank, external engines, and
  exposed provenance are out of scope.

**Acceptance criteria** — One fused, relevance-ranked list from `/api/search`,
with `/api/search/semantic` gone and no routes registered by `fawi-search`; a
concept matched by both providers appears once and ordering is deterministic
(keyword title match outranks a semantic-only body match); keyword behaviour
(body-only match returned, field weighting, title fallback, empty query empty);
semantic behaviour preserved through the fused endpoint including
change-driven reindex; the web UI unchanged; keyword-only degradation when the
model fails (strict failure for `okf search`); the server handler depends only
on the injected engine.

## Relevance-ranked search results

`kind: feature` · `state: superseded` · finished 2026-09-06

**Problem** — Keyword results were sorted by title with body matches unweighted,
so the most relevant result was not necessarily first.

**Proposal** — Score keyword matches, weighting title/type/tags above the body,
and order by score before falling back to title order (semantic ranking left as
is).

**Decisions** — Implemented in the `fawi-core`/storage search path with no new
dependencies; intended to build on [Full-text search](#full-text-search-in-the-bundle-index)
rather than conflict with it; learning-to-rank and external engines out of
scope. Superseded by [Hybrid search](#hybrid-search-with-pluggable-keyword-and-semantic-providers),
which folds scoring into `KeywordProvider`.

**Acceptance criteria** — A title match ranks before a body-only match; equal
scores fall back to title order.

## Full-text search in the bundle index

`kind: feature` · `state: superseded` · finished 2026-09-06

**Problem** — Search matched only a concept's title, slug, type, description,
and tags, so useful matches in the body were missed.

**Proposal** — Extend bundle keyword search to also match the rendered concept
body, keeping the existing title/slug/type/tag matching and title-sorted order.

**Decisions** — The change lived in `fawi-core`'s `FsBundle::search`, requiring
no new dependencies (bodies are already held in memory as `concept.content`);
relevance ranking was out of scope (tracked separately). Superseded by
[Hybrid search](#hybrid-search-with-pluggable-keyword-and-semantic-providers),
which folds body matching into `KeywordProvider`.

**Acceptance criteria** — A body-only query returns the concept; an empty query
returns no results.

## Render Mermaid diagrams

`kind: feature` · `state: done` · finished 2026-08-20

**Problem** — `fawi-core` rendered fenced Mermaid blocks as plain
`<pre><code class="language-mermaid">…</code></pre>` and the web UI injected
the HTML verbatim, so diagrams — including those in `docs/architecture.md` —
showed as raw code.

**Proposal** — Render fenced Mermaid blocks as sanitized inline SVG
server-side at request time with Merman (headless Rust Mermaid) inside
`fawi-core`, so the UI receives a self-contained `<svg>` with no browser
JavaScript, CDN, or vendored asset.

**Decisions** — Substituted after comrak formatting, the only path that emits a
bare `<svg>` (comrak strips raw HTML when `unsafe_ = false` and its
`SyntaxHighlighterAdapter` always wraps in `<pre><code>`); the Merman dependency
is optional behind a `mermaid` feature on `fawi-core`, enabled only by
`fawi-server` so the wasm client never builds it; invalid/unsupported syntax
falls back to the original block; each block gets a unique `diagram_id`;
syntax highlighting for other languages, interactive features, and byte-for-byte
parity with browser Mermaid.js are out of scope.

**Acceptance criteria** — A fenced Mermaid block in a concept body or a
directory `index.md`/`log.md` renders as an SVG diagram, not raw code; it
renders on navigation without a full reload; escaped markup such as `<br/>`
renders with no visible raw entities; the SVG is sanitized with no script
execution.

## Directional sorting with per-field toggle buttons

`kind: feature` · `state: done` · finished 2026-08-20

**Problem** — Listings could be filtered and sorted, but sorting supported only
one ascending direction and was driven by a `<select>` plus form submit; there
was no way to reverse the sort and the filter controls added unneeded
complexity.

**Proposal** — Remove filtering and add a `dir=asc|desc` parameter
(default `asc`) to `/api/dirs`, applying to whichever field `sort` names. The
UI replaces the form with one button per front matter field: a click sorts
ascending, a second descending, a third removes the sort.

**Decisions** — Descending is `Ordering::reverse()` over the existing
`front_matter::compare_values`; `ListOptions` drops `filters` and gains a
`SortDirection` enum; `values_match` loses its caller but stays a tested `pub`
utility (removable later); the buttons are plain `<a href>` elements so the
three-state toggle works without client JS and degrades under SSR; no new
dependencies.

**Acceptance criteria** — `sort=status&dir=desc` orders by `status` descending
with a descending title tiebreak; `sort=status` alone is ascending; no `sort`
means title ascending regardless of `dir`; the UI renders one button per field
and cycles ascending → descending → off; the old `filter` parameter neither
errors nor filters.

## Sort and filter by front matter fields

`kind: feature` · `state: done` · finished 2026-08-20

**Problem** — Listings were always title-sorted and could not be narrowed; the
server modelled a fixed set of front matter fields and silently discarded
everything else, so no field — modelled or producer-defined — could order or
filter a listing.

**Proposal** — Add `sort` and `filter` query parameters to `/api/dirs` that
work against the full front matter rather than a fixed field list, preserving
every key on `Concept`/`ConceptSummary` and their DTOs. `sort=<field>` orders
by that field (falling back to `title`); `filter=<field>=<value>` narrows to
matching concepts (repeated filters are ANDed; a list-valued field matches when
any element matches).

**Decisions** — The logic lands in `fawi-storage`'s `FsBundle::list_dir`, with
`sort`/`filter` threaded through `BundleSource` and the directory handlers, and
controls added in `fawi-gui`; `Concept` carries the raw front matter (as
`serde_yaml::Value`) so unknown keys survive; a generic comparison layer treats
scalars case-insensitively, compares numbers/dates/booleans, and matches lists
by membership, with non-comparable values and missing keys degenerating stably;
no new dependencies; scoped to directory listings (search unaffected).

**Acceptance criteria** — `sort=status` orders by `status`; `filter=type=Metric`
returns only matching concepts; `sort=<field>` works for a producer-defined key;
an arbitrary scalar `filter` matches, and a list-valued key matches when any
element does; non-comparable values and absent keys keep ordering stable and
filtering error-free.

## Surface arbitrary front matter fields in the web UI

`kind: feature` · `state: done` · finished 2026-08-20

**Problem** — The server modelled a fixed set of front matter fields and the UI
rendered only those, so producer extensions such as a change request's `state`,
`priority`, and `owner` were invisible.

**Proposal** — Expose the non-modelled ("extra") front matter fields in the
concept and summary DTOs and render them generically as `key: value` badges on
concept pages and directory listings, without hard-coding field names (scalars
as-is, sequences comma-joined, mappings abbreviated or omitted).

**Decisions** — Extras are `keys(front_matter)` minus a constant `MODELED_KEYS`
set, computed in `fawi-core` with a `display_string` helper; `ConceptSummary`,
`ConceptResponse`, and `ConceptSummaryResponse` carry a
`BTreeMap<String, String>` for deterministic order; no new dependencies; low
risk.

**Acceptance criteria** — A non-modelled field such as `state: proposed` is
visible on a concept page and in a directory listing; a list-valued extra field
shows in comma-joined form; a change request's `state`, `priority`, and `owner`
all show without special-casing; no empty badges appear when every field is
modelled.
