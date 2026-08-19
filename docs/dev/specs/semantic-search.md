---
type: Specification
title: Semantic search across the bundle
description: Embed the whole bundle and serve relevance-ranked search from a dedicated okf-search crate launched by the CLI.
status: stable
tags: [dev, search]
owner: human:felix
---

# Problem

Search today (`FsBundle::search`, `GET /api/search`) is lexical: it matches
literal substrings against a concept's id, title, type, description, and tags
(with body matching tracked separately). It does not understand meaning, so a
query for "revenue" misses a concept titled "income", and paraphrases or
synonyms are never matched. There is no way to search the whole bundle by
semantic similarity.

We want the entire bundle — every concept, including its body — to be
searchable semantically: matched by meaning rather than exact substring. This
must be a dedicated `okf-search` crate launched by `okf-cli`, the same way
`okf-gui` and `okf-server` are launched.

# Requirements

- A new workspace crate `okf-search` MUST implement semantic search.
- The crate MUST index the whole bundle: every concept, using its title,
  description, tags, and body.
- The crate MUST reuse bundle storage and hot-reload from `okf-storage` (no
  independent directory scanning) and MUST reindex when the bundle changes.
- The crate MUST compute a vector embedding for each concept and for each query,
  and rank results by embedding similarity in descending order.
- The crate MUST work offline with a local embedding model by default; it MUST
  NOT require an external network service or secret to operate.
- The crate MUST expose an HTTP endpoint that accepts a query and returns ranked
  concept summaries, each with a relevance score.
- The existing lexical `GET /api/search` endpoint and its behavior MUST be
  preserved unchanged.
- `okf-cli` MUST gain a `search` subcommand that launches the `okf-search`
  service alone, reusing the crate's startup function. It MUST accept `--data`
  and `--bind` flags defaulting to `./docs` and `127.0.0.1:8082`.
- With no subcommand, `okf-cli` MUST start the server, the GUI, and the semantic
  search service together.
- Startup, logging, and signal handling MUST follow the existing pattern: logic
  reused from the crate, logging initialized once, a bind failure fails with a
  clear error naming the address, and SIGINT/SIGTERM shuts everything down.

# Acceptance Criteria

- Given a query whose meaning matches a concept only in its body (not in its
  title, type, description, or tags), when searching semantically, then that
  concept is returned.
- Given a semantic query, when searching, then results are returned in
  descending relevance order, each with a numeric score.
- Given an empty query, when searching semantically, then no results are
  returned.
- Given the `search` subcommand, when run with defaults, then the semantic
  search API responds on `127.0.0.1:8082`.
- Given no subcommand, when run with defaults, then the REST API, the GUI, and
  the semantic search API all respond.
- Given a bundle change, when it occurs, then the semantic index is rebuilt and
  subsequent queries reflect the change.
- Given the existing lexical search, when queried, then its behavior is
  unchanged.

# Out of scope

- Training or fine-tuning an embedding model.
- Approximate nearest neighbor indexing; brute-force is sufficient and the index
  is pluggable.
- Persisting the embedding index to disk; it is rebuilt from the bundle at
  startup.
- Relevance re-ranking beyond vector similarity (e.g. cross-encoder re-ranking).
- Changes to the lexical search results or ranking.
- Multi-host orchestration, load balancing, or clustering.
