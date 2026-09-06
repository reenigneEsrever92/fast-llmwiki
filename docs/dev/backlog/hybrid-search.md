---
type: ChangeRequest
kind: feature
title: Hybrid search with pluggable keyword and semantic providers
description: Merge /api/search and /api/search/semantic into one endpoint that fuses ranked results from any injected search provider.
state: done
priority: medium
tags: [dev, search]
owner: human:felix
verified: { by: human:felix, at: 2026-09-06T18:00:37Z }
---

# Problem

Search today is split across two endpoints that the web UI cannot combine.
`GET /api/search` runs the storage keyword search — a substring match on id,
title, type, description, and tags, ordered by title, with no body matching. The
web UI (header dropdown and `/search` page) calls only this endpoint. The
semantic index in `fawi-search`, which ranks title/type/description/tags/body
by cosine similarity and would find useful body matches, is exposed only as a
separate `GET /api/search/semantic` route that nothing in the UI calls. The two
result sets can never be shown together, and keyword search quality is limited
(title order, no body recall).

Two existing backlog items address keyword quality in isolation —
[Full-text search](full-text-search.md) adds body matching and
[Relevance-ranked search results](relevance-ranked-search.md) scores keyword
matches — but neither unifies keyword with semantic search, and neither
surfaces semantic results in the UI.

Structurally, the search endpoint is coupled to one concrete implementation:
`fawi-server`'s `/api/search` handler calls `FsBundle::search` directly, while
`fawi-search` keeps its index behind its own private route state. There is no
seam at which a crate can contribute a search strategy to the single query
endpoint, and no way to merge rankings from several strategies.

# Proposal

Introduce a search-provider architecture with a single query endpoint.

1. **A provider port.** Define `SearchProvider` in `fawi-storage` (where the
   async `BundleSource` trait already lives, and on which both `fawi-server`
   and `fawi-search` already depend):

       #[async_trait]
       pub trait SearchProvider: Send + Sync {
           /// Rank the bundle against `query`, best first.
           async fn search(&self, query: &str) -> Vec<ScoredSummary>;
       }

   `ScoredSummary` wraps a `fawi_core::ConceptSummary` with a provider score.
   Any crate can contribute a strategy by implementing the port — keyword,
   semantic, or a future engine — without the endpoint changing.

2. **Two providers.** `KeywordProvider` (in `fawi-storage`) wraps the existing
   substring matching, extended with body matching (from the superseded
   full-text item) and a field-weighted relevance score (from the superseded
   relevance-ranked item): id/title matches outrank type/tags, which outrank
   description/body, with title order as the tie-break. `SemanticProvider` (in
   `fawi-search`) adapts the existing `SemanticIndex` — embed query, cosine
   ranking — behind the port, keeping the index and its change-driven rebuild
   where they are.

3. **Fusion at the composition root.** `fawi-cli` assembles a `HybridSearch`
   engine: a list of `Arc<dyn SearchProvider>` whose per-query ranked lists are
   merged with reciprocal rank fusion (`score(doc) = Σ_p 1/(k + rank_p(doc))`,
   `k = 60`, deduplicated by concept id) and injected into `fawi-server`. The
   endpoint owns no strategy; the composition root decides which providers back
   it, so a new engine joins simply by being added to the list.

4. **One endpoint.** `GET /api/search` in `fawi-server` returns the fused
   ranking, still as `Vec<ConceptSummaryResponse>`, so the web UI is unchanged
   and immediately sees both keyword and semantic results. The
   `/api/search/semantic` route is removed; `fawi-search` no longer registers
   HTTP routes and instead exposes its provider and index lifecycle. Merged
   `okf` serves the hybrid engine, degrading to keyword-only (with a warning)
   if the embedding model cannot initialize; standalone `okf server` serves
   keyword-only and `okf search` serves keyword + semantic on its own bundle.

# Decisions

- **Feature, superseding two items.** Recorded as `kind: feature`. It supersedes
  [Relevance-ranked search results](relevance-ranked-search.md) and
  [Full-text search](full-text-search.md): keyword relevance scoring and body
  matching are prerequisites of meaningful fusion and are implemented inside
  `KeywordProvider`.
- **Reciprocal rank fusion, not weighted scores.** Cosine similarity and a
  keyword relevance heuristic are not on a comparable scale, so a weighted sum
  would need fragile calibration. RRF needs only each provider's list to be
  internally well-ordered, which each provider can guarantee. `k = 60` is the
  conventional default; duplicates are removed by concept id.
- **Merge the query endpoints.** There is one search endpoint, `GET /api/search`,
  owned by `fawi-server`; `/api/search/semantic` is removed and `fawi-search`
  stops serving HTTP. The response stays `Vec<ConceptSummaryResponse>`, so the
  GUI and SSR code paths are untouched and the header dropdown and `/search`
  page immediately show fused results. Per-provider scores stay internal for now.
- **Port and fusion live in `fawi-storage`.** This follows the precedent of
  `BundleSource` (an async trait over `fawi_core` types) and avoids adding
  async-trait or behaviour traits to `fawi-core`; both dependent crates already
  use `fawi-storage`. The handler in `fawi-server` is inverted to depend on the
  injected engine rather than `FsBundle::search`.
- **Graceful degradation in merged mode.** Today `serve_all` fails hard if the
  embedding model cannot load. Under this design the merged `okf` builds the
  hybrid engine only when the semantic provider initialises, logs a warning and
  continues keyword-only otherwise. Standalone `okf search` remains strict — a
  semantic search service that cannot load its model is a startup error.
- **Full ranked result set, no cap.** `/api/search` returns the complete fused
  ranking, preserving today's "list all matches" page behaviour; a `limit`
  parameter can be added later.
- **Out of scope.** Exposing per-provider scores or provenance in the payload,
  learning-to-rank, and external search engines.

# Acceptance criteria

- `GET /api/search` returns one fused, relevance-ranked list, and
  `/api/search/semantic` no longer exists; `fawi-search` registers no routes.
- A concept matched by both providers appears exactly once, and — given mocks or
  the real providers — a keyword title match outranks a semantic-only body match
  for the same query; ordering is deterministic.
- Keyword behaviour folded in from the superseded items: a query found only in a
  concept body returns that concept; a title/type/tags match ranks above a
  description/body match; equal scores fall back to title order; an empty query
  returns no results.
- Semantic behaviour is preserved through the fused endpoint: a concept whose
  semantics match the query but whose fields do not still surfaces, and the
  index still rebuilds when the bundle changes (existing `fawi-search` tests
  pass, adapted to the provider API).
- The web UI code is unchanged: the header dropdown and `/search` page keep
  calling `GET /api/search` and render the fused results from the same
  `ConceptSummaryResponse` payload.
- When the embedding model cannot initialise, merged `okf` still serves keyword
  search on `/api/search` and logs a warning; `okf search` fails with a clear
  error.
- `fawi-server`'s search handler depends only on the injected engine/port, not
  on `FsBundle`'s keyword implementation.

# Docs

The change alters documented behaviour. Touch points for the implementation
plan:

- `docs/api/rest-api.md` — merge the two search rows into one `GET /api/search`
  description (hybrid, pluggable providers, RRF) and drop `/api/search/semantic`.
- `docs/architecture.md` — `fawi-search`'s role changes from "serves the
  semantic search API" to "provides the semantic search provider and index";
  update the crate table and data-flow diagram.
- `docs/features.md` — describe hybrid search instead of standalone semantic
  search.
- `docs/getting-started.md` and `docs/server/cli.md` — update what each `okf`
  mode serves (the merged binary, `okf server`, and `okf search`).

# Implementation plan

## Approach

Introduce a search-provider port in `fawi-storage` and make `fawi-server`'s
`GET /api/search` the single search endpoint, backed by an engine injected at
startup. Keyword search moves out of `FsBundle::search` into a scored
`KeywordProvider`; the semantic index in `fawi-search` becomes a
`SemanticProvider` behind the same port; a `HybridSearch` in `fawi-storage`
runs every provider for a query and fuses the ranked lists with reciprocal
rank fusion (`k = 60`, dedupe by concept id). `fawi-cli` is the composition
root: merged `okf` injects keyword + semantic (degrading to keyword-only with a
warning when the embedding model cannot load), `okf server` injects
keyword-only, and `okf search` serves only the search router with keyword +
semantic over its own bundle.

The `SearchProvider` port is infallible (`async fn search(&self, query: &str)
-> Vec<ScoredSummary>`), following `BundleSource`'s `#[async_trait]` precedent;
`SemanticProvider` logs a warning and returns no results on a mid-flight
embedding/index error. Keyword scoring is a pure function over `Concept`s
(weighted substring match, title order as tie-break), so it is unit-testable
without a filesystem bundle.

## Steps

- [ ] `fawi-storage/src/search.rs` (new module, re-exported from `lib.rs`):
      `ScoredSummary { summary: ConceptSummary, score: f32 }`;
      `#[async_trait] pub trait SearchProvider: Send + Sync` with
      `async fn search(&self, query: &str) -> Vec<ScoredSummary>`;
      `KeywordProvider { bundle: Arc<FsBundle> }` implementing the port via a
      pure `rank_keyword(query, &[Concept]) -> Vec<ScoredSummary>` helper; and
      `HybridSearch { providers: Vec<Arc<dyn SearchProvider>> }` implementing
      the port with private `fuse_rrf` (RRF, `k = 60`, dedupe by `summary.id`).
- [ ] Implement keyword scoring in the new module: lowercase substring match
      across id, title, type, description, tags, and body (`concept.content`),
      with weights title 4 > id 3 > type/tags 2 > description/body 1 (summed
      across matching fields), sorted by score descending then title ascending;
      blank query returns no results.
- [ ] Remove `search` from the `BundleSource` trait and the `FsBundle` impl in
      `fawi-storage` (its logic is superseded by `KeywordProvider`); update
      imports in `fs_bundle.rs`.
- [ ] Unit tests in `fawi-storage/src/search.rs`: body-only query returns the
      concept; a title/type/tags match ranks above a description/body match;
      equal scores fall back to title order; blank query is empty; `fuse_rrf`
      dedupes a concept returned by two providers and yields a deterministic
      order (using hand-written mock `SearchProvider`s).
- [ ] `fawi-search`: turn the HTTP `api.rs` module into a provider module that
      exposes `pub async fn init(bundle: Arc<FsBundle>) -> anyhow::Result<Arc<dyn
      fawi_storage::SearchProvider>>` — builds the index, spawns the existing
      change-driven reindexer, and returns a `SemanticProvider` whose `search`
      reads the index and maps `(ConceptSummary, f32)` cosine results to
      `ScoredSummary`; delete the axum router, its handlers, and the
      `SearchState` HTTP state.
- [ ] `fawi-search`: update `lib.rs` module docs and remove `serve()` (no longer
      hosts HTTP); delete `src/main.rs` and the `[[bin]]` block in
      `fawi-search/Cargo.toml` (the crate becomes library-only; standalone
      serving moves to `okf search`). Keep `embed.rs` and `index.rs` and their
      tests unchanged.
- [ ] `fawi-core`: remove the now-unused `SearchResultResponse` DTO and its
      `from_summary` from `dto.rs` (verified unused outside the deleted
      semantic endpoint).
- [ ] `fawi-server/src/api.rs`: add `static SEARCH: OnceLock<Arc<dyn
      SearchProvider>>` with `pub fn init_search(engine)` and a `search_router()`
      that registers only `GET /api/search`; make `router()` merge
      `search_router()`; rewrite the `search` handler to call the injected
      engine (blank query returns early) and map `ScoredSummary` →
      `ConceptSummaryResponse`. The handler no longer calls `FsBundle::search`.
- [ ] `fawi-server/src/lib.rs`: in `serve()`, after `init_bundle`, inject a
      keyword-only engine by default
      (`init_search(Arc::new(KeywordProvider::new(bundle.clone())))`).
- [ ] `fawi-cli/src/lib.rs`: in `serve_all`, build the keyword provider always,
      attempt `fawi_search::init(bundle.clone()).await` — on success build
      `HybridSearch` with both providers and inject it; on error log a warning
      and inject keyword-only — drop the `fawi-search` router merge (the merged
      app is `fawi_server::api::router()` + `fawi_gui::ssr::router()`), and
      remove the `fawi_search::api::init_bundle` call.
- [ ] `fawi-cli/src/lib.rs`: rework the `Command::Search` branch to open its own
      bundle, require `fawi_search::init` to succeed (strict), inject a keyword +
      semantic `HybridSearch`, and serve `fawi_server::api::search_router()` on
      the search bind address.
- [ ] Run `cargo test` across the workspace (new storage search tests, existing
      `fawi-search` index tests, `fawi-server`/`fawi-cli` compile and parse
      tests) and fix any fallout.
- [ ] `docs/api/rest-api.md`: collapse the `/api/search` and
      `/api/search/semantic` rows and paragraph into one `GET /api/search` row
      describing the hybrid keyword + semantic result set fused by reciprocal
      rank fusion and the unchanged `ConceptSummaryResponse` payload.
- [ ] `docs/architecture.md`: update the crate table row and data-flow section
      and diagram — `fawi-search` is now a provider library feeding
      `fawi-server`'s single search endpoint, with no separate semantic route.
- [ ] `docs/features.md`: replace the standalone semantic-search bullet with a
      hybrid-search bullet describing pluggable search providers and RRF
      fusion.
- [ ] `docs/getting-started.md` and `docs/server/cli.md`: update what `okf`,
      `okf server`, and `okf search` serve and the curl examples
      (`/api/search?q=` on 8080 and 8082).

Implemented in commit `bf327e8de3646ff1cec7dafda0e50f936592f4ac`.
