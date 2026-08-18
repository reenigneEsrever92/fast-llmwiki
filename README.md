# OKF Wiki

A small, self-contained wiki server written in Rust. Pages are stored as plain
Markdown files on disk and rendered to HTML, with `[[WikiLink]]` support.

## Layout

The project is a Cargo workspace with crates under `crates/`:

- [`okf-core`](crates/okf-core) — page model, title/slug helpers, and Markdown rendering.
- [`okf-storage`](crates/okf-storage) — the `Storage` trait and a filesystem-backed implementation.
- [`okf-server`](crates/okf-server) — the `axum` HTTP server and CLI (`okf` binary).

## Running

```sh
cargo run -p okf-server -- --data ./wiki --bind 127.0.0.1:8080
```

Then open <http://127.0.0.1:8080>.

## Usage

- `GET /` — list all pages.
- `GET /new` — create a page.
- `GET /page/{slug}` — view a rendered page.
- `GET /page/{slug}/edit` — edit a page.
- `GET /page/{slug}/raw` — view the raw Markdown.
- `GET /search?q=...` — search page titles and slugs.

Markdown supports common extensions (tables, strikethrough, task lists,
footnotes, etc.) and wiki links such as `[[Other Page]]` or `[[label|Other Page]]`.

## Front matter

Each page is a self-contained Markdown file with YAML front matter:

```markdown
---
title: Hello World
created_at: 2026-08-18T10:59:26Z
updated_at: 2026-08-18T10:59:26Z
tags: [intro, guide]
---
Welcome to [[My Page]]
```

The wiki manages `title`, `created_at`, `updated_at`, and `tags`. Other keys
you add by hand are preserved across edits.

## Storage

Pages live under the data directory:

```text
wiki/
  pages/<slug>.md
```

The slug is derived from the page title (lowercased, spaces become dashes). An
in-memory index is rebuilt by scanning these files at startup.
