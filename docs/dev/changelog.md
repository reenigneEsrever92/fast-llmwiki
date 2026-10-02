---
type: Changelog
title: Changelog
description: Every change implemented in this repository, newest first.
tags: [dev, changelog]
status: stable
---

# Changelog

Every change is recorded here as it ships. Each entry names what changed, why,
and links to the change request in the [backlog](backlog/) — or its section in
the [condensed backlog](backlog/summary.md) once it has been retired — where one
exists. See [Development](index.md) for the change-driven workflow.

## 2026-10-02

- **`fawi-docs` skill** — added a bundled skill that reconciles the docs under
  `docs/` with the code and the backlog and records the update in
  `docs/log.md`; registered it with `okf install`.
- **`fawi-condense` skill** — added a bundled skill that retires finished
  change requests older than 10 days into entries in a single
  `docs/dev/backlog/summary.md`, deleting the originals and repointing the
  changelog's links to it; registered it with `okf install`.
- **Condensed backlog** — retired the seven finished change requests (hybrid
  search and its two superseded predecessors, Mermaid rendering, directional
  sorting, sort/filter by front matter, and surfacing extra front matter fields)
  into [backlog/summary.md](backlog/summary.md), leaving the backlog holding
  only open work.

## 2026-09-06

- **Hybrid search with pluggable keyword and semantic providers** — `/api/search`
  now returns results fused from keyword and semantic providers by reciprocal
  rank fusion; `/api/search/semantic` is removed, `fawi-search` became a library
  exposing a semantic search provider, and keyword search matches bodies with
  field-weighted relevance. See
  [Hybrid search with pluggable providers](backlog/summary.md#hybrid-search-with-pluggable-keyword-and-semantic-providers).

## 2026-08-20

- **Render Mermaid diagrams server-side** — fenced `mermaid` code blocks are now
  rendered to sanitized SVG at request time by Merman (a headless Rust Mermaid
  implementation) in `fawi-core`, so the web UI shows diagrams instead of raw
  code with no client-side JavaScript. See
  [Render Mermaid diagrams](backlog/summary.md#render-mermaid-diagrams).
- **Directional sorting with per-field toggle buttons** — `/api/dirs` now sorts
  with a direction via `dir=asc|desc`, and filtering was removed for now. The
  web UI replaces the sort/filter form with one button per front matter field
  that cycles ascending → descending → off. See
  [Directional sorting with per-field toggle buttons](backlog/summary.md#directional-sorting-with-per-field-toggle-buttons).
- **Sort and filter directory listings by front matter** — `/api/dirs` now accepts
  `sort=<field>` and `filter=<field>=<value>` to order and narrow a directory
  listing by any front matter key, including producer-defined ones. The web UI
  gains sort/filter controls on directory pages. See
  [Sort and filter by front matter fields](backlog/summary.md#sort-and-filter-by-front-matter-fields).
- **Surface arbitrary front matter fields in the web UI** — non-modeled front
  matter fields (such as a change request's `state`, `priority`, and `owner`) are
  now rendered generically as `key: value` badges on concept pages and directory
  listings, so producer and bundle extensions show up without per-field code. See
  [Surface arbitrary front matter fields in the web UI](backlog/summary.md#surface-arbitrary-front-matter-fields-in-the-web-ui).

## 2026-08-19

- **Documentation rework** — replaced the spec-driven development cycle with a
  change-driven one. Removed `docs/dev/specs/`, `docs/dev/plans/`, and
  `docs/dev/roadmap.md`; added `docs/dev/backlog/` for change requests and this
  changelog; and replaced the `okf-*` skills with `fawi-propose`, `fawi-plan`,
  `fawi-implement`, and `fawi-check`.
- **GitHub Actions CI and release** — added workflows that build the workspace
  (including the `ssr` feature) and publish native release binaries on `v*`
  tags. See [Releases](releases.md).
- **`okf install` subcommand** — added a CLI command that writes the agent skills
  embedded in the binary into `.agents/skills/`.
- **Live search dropdown** — added a header search dropdown and removed the
  redundant search-page input.
- **Live reload** — the sidebar and current page auto-refresh on bundle changes.
- **Main content width** — the content column now fills a consistent `56rem`
  maximum on wide screens.

## 2026-08-18

- **Unified `okf` launcher** — added the `fawi-cli` crate so one binary serves
  the REST API, web UI, and semantic search on a single socket.
- **Semantic search** — added the `fawi-search` crate with local vector
  embeddings, reindexed on change.
- **Persistent sidebar** — added bundle navigation in the web UI.
- **Client-side navigation** — the web UI serves a hydrated SPA so pages navigate
  without a full reload.
- **Project documentation** — established `docs/` as an OKF bundle covering the
  format, REST API, trust model, and web UI.
