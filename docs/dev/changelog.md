---
type: Changelog
title: Changelog
description: Every change implemented in this repository, newest first.
tags: [dev, changelog]
status: stable
---

# Changelog

Every change is recorded here as it ships. Each entry names what changed, why,
and links to the change request in the [backlog](backlog/) where one exists.
See [Development](index.md) for the change-driven workflow.

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
