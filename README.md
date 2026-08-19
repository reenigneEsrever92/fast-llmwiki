# OKF Bundle Server

A read-only server, web UI, and semantic search for
[Open Knowledge Format (OKF)](docs/okf-format.md) bundles — directories of
Markdown files with YAML front matter. Built in Rust with axum and Leptos.

It is **read-only**: it never modifies the bundle. It watches the directory and
reloads on changes, so it can sit alongside a git checkout or an
agent-maintained corpus.

## Layout

The project is a Cargo workspace with six crates under `crates/`:

- [`okf-core`](crates/okf-core) — the OKF model, front matter parsing, Markdown rendering, and shared DTOs.
- [`okf-storage`](crates/okf-storage) — the read-only bundle scanner (`FsBundle`) and filesystem change events.
- [`okf-server`](crates/okf-server) — the REST API and WebSocket hot reload.
- [`okf-gui`](crates/okf-gui) — the Leptos web UI (SSR + hydration).
- [`okf-search`](crates/okf-search) — semantic search over local vector embeddings.
- [`okf-cli`](crates/okf-cli) — the unified `okf` launcher that runs the server, GUI, and search together.

## Running

```sh
cargo run -p okf-cli
```

This serves the REST API, web UI, and semantic search as one merged router on a
single socket at <http://127.0.0.1:8080>.

To run components individually:

```sh
cargo run -p okf-cli -- server    # REST API only (127.0.0.1:8080)
cargo run -p okf-cli -- gui       # web UI only (127.0.0.1:8081)
cargo run -p okf-cli -- search    # semantic search only (127.0.0.1:8082)
```

See [Getting started](docs/getting-started.md) for details and
[CLI](docs/server/cli.md) for all flags.

## REST API

All responses are JSON.

- `GET /api/concepts/{id}` — a concept, with rendered `content_html`.
- `GET /api/dirs/{path}` — a directory listing.
- `GET /api/tree` — the full bundle tree for navigation.
- `GET /api/search?q=` — keyword search over titles, types, descriptions, and tags.
- `GET /api/search/semantic?q=` — semantic search over vector embeddings (via the search service).
- `GET /api/ws` — WebSocket upgrade for hot reload.

`/api/search/semantic` is provided by `okf-search` and is available when running
the merged `okf` or `okf search`. See [REST API](docs/api/rest-api.md).

## Front matter

Every concept is a Markdown file with YAML front matter:

```markdown
---
type: Reference
title: Example
description: A short summary.
tags: [demo]
status: stable
generated: { by: human:maintainer, at: 2026-08-18T00:00:00Z }
verified: { by: human:maintainer, at: 2026-08-18T00:00:00Z }
stale_after: 2026-12-31
sources: []
---

# Body
```

The server reads `type`, `title`, `description`, `resource`, `tags`, `status`,
`generated`, `verified`, `stale_after`, and `sources`. See
[Front matter](docs/frontmatter.md).

## Bundle layout

A bundle is a directory tree of Markdown files:

```text
docs/
  index.md          # reserved: rendered on the directory listing
  log.md            # reserved: directory update log
  overview.md       # a concept
  api/
    rest-api.md     # a concept
```

A **concept ID** is the file path with the `.md` suffix removed (for example
`api/rest-api`). `index.md` and `log.md` are reserved filenames, read on demand
for directory listings. An in-memory index is rebuilt when files change, and the
change is broadcast to WebSocket clients.

See the [documentation](docs/index.md) for the full guide.
