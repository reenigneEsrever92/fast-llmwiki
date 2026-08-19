---
type: Plan
title: Add okf-search crate and wire semantic search into the CLI
status: stable
state: done
priority: medium
tags: [dev, search]
verified: { by: ai:zed, at: 2026-08-18T15:40:01Z }
---

Implements [semantic search](/dev/specs/semantic-search.md).

## Approach

- Embeddings are computed in-process with [`fastembed`](https://crates.io/crates/fastembed)
  (ONNX Runtime backend), using the local `BAAI/bge-small-en-v1.5` model
  (384 dimensions).
- Similarity is cosine similarity over the embedding vectors.
- Results are ranked by brute-force cosine similarity in memory, behind a small
  trait so the index can be swapped for an approximate nearest neighbor index
  (e.g. `usearch`/HNSW) if the bundle grows large.
- Model weights are cached locally; for air-gapped use the model is vendorable
  or a cache directory pre-populated.

## Steps

- [x] Add `crates/okf-search` to the workspace with a reusable `serve(data, bind)` startup function and an `okf-search` binary.
- [x] Index the whole bundle (title, description, tags, and body) with a local embedding model, reusing `okf-storage` for bundle reads and change events.
- [x] Expose an HTTP endpoint that returns relevance-ranked concept summaries with a score (new DTO in `okf-core`).
- [x] Add a `search` subcommand to `okf-cli` and start the service together with the server and GUI in the default (no-subcommand) mode.
- [x] Keep the existing lexical `GET /api/search` behavior unchanged.
- [x] Add tests for ranking, body-only matches, empty queries, and index rebuilds on change.

Commit: not recorded (no git repository present at implementation time).
