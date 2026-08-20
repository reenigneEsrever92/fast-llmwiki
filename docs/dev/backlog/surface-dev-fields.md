---
type: ChangeRequest
title: Surface arbitrary front matter fields in the web UI
description: Render non-modeled front matter fields generically on concept pages and directory listings.
state: done
priority: low
tags: [dev, gui]
owner: human:felix
verified: { by: human:felix, at: 2026-08-20T21:08:23Z }
---

# Problem

The server models only a fixed set of front matter fields (`type`, `title`,
`description`, `resource`, `tags`, `status`, `generated`, `verified`,
`stale_after`, `sources`) and the web UI renders only those. Any other field — the
producer extensions `state`, `priority`, and `owner` on change requests, or any
bundle-specific key — is invisible, so a reader cannot tell at a glance what
lifecycle a change request is in.

# Proposal

Expose the *non-modeled* ("extra") front matter fields in the concept and summary
DTOs and render them generically in the web UI, on both concept pages and
directory listings, without hard-coding field names. Every top-level field that
the server does not already model is surfaced with a readable string form
(scalars as-is, sequences comma-joined, mappings abbreviated or omitted), so new
producer or bundle fields show up with no further code changes.

# Decisions

- `fawi-core` already preserves the raw front matter in `Concept.front_matter`
  and provides `keys()` / `get_field()` (`crates/fawi-core/src/front_matter.rs`),
  so the extra fields are just `keys(front_matter)` minus the modeled-key set.
  No new parsing is required.
- The DTOs `ConceptResponse` and `ConceptSummaryResponse`
  (`crates/fawi-core/src/dto.rs`) and `ConceptSummary` must carry the extra
  fields; today `Concept::summary()` (`crates/fawi-core/src/concept.rs`) drops
  the map, so the extras have to be threaded through
  `crates/fawi-server/src/api.rs`'s `dir_response`/`get_concept` handlers.
- `fawi-gui` (`crates/fawi-gui/src/app.rs`) adds one generic metadata row in
  `ConceptView` and `ConceptListItem`; `DirListingResponse.fields` already
  enumerates available keys for the sort/filter pickers, so the same notion
  extends naturally to display.
- **Decisions/tradeoffs**: keep a constant set of "modeled" keys to subtract;
  define how sequences and mappings are stringified for display; and whether the
  defaulted `status: stable` on change requests should be hidden is a separate
  concern. Low risk and no new dependencies.

# Acceptance criteria

- Given a concept with a non-modeled front matter field such as
  `state: proposed`, when viewed on its page and in a directory listing, then the
  field and its value are visible.
- Given a concept with a list-valued extra field, when rendered, then the value
  is shown in a readable (comma-joined) form.
- Given a change request with `state`, `priority`, and `owner`, when listed, then
  all three are visible without special-casing.
- Given a concept whose fields are all modeled, when rendered, then no empty
  extra-field badges are shown.

# Implementation plan

## Approach

Derive the *extra* front matter fields once in `fawi-core` and thread them
through the summary/DTO layer to the GUI, where they render generically. No new
dependencies and no per-field code in the UI.

- **Core helper**: add `extra_fields(&Value) -> BTreeMap<String, String>` and a
  private `display_string(&Value) -> Option<String>` to
  `crates/fawi-core/src/front_matter.rs`. `extra_fields` starts from the existing
  `keys()` output, subtracts the modeled keys, and stringifies each value:
  scalars (string/number/bool) become their text form, sequences comma-join their
  elements, and `null`/mappings/tagged values are omitted. A constant
  `MODELED_KEYS` lists the fields the server already models: `type`, `title`,
  `description`, `resource`, `tags`, `status`, `generated`, `verified`,
  `stale_after`, `sources`.
- **Summary**: add `extra_fields: BTreeMap<String, String>` to `ConceptSummary`
  in `crates/fawi-core/src/concept.rs` and populate it in `Concept::summary()`
  from `front_matter::extra_fields(&self.front_matter)`. `BTreeMap` gives
  deterministic (key-sorted) order and a clean JSON object shape.
- **DTOs**: add `extra_fields` to `ConceptResponse` and
  `ConceptSummaryResponse` in `crates/fawi-core/src/dto.rs`, populated in
  `from_concept` (computed from `concept.front_matter`) and `from_summary`
  (cloned from the summary). Search results get it for free via the flattened
  `SearchResultResponse`.
- **GUI**: render a generic metadata row of `key: value` badges in `ConceptView`
  and `ConceptListItem` in `crates/fawi-gui/src/app.rs`, using a new
  `.badge.field` style, only when `extra_fields` is non-empty.

## Steps

- [ ] Add `MODELED_KEYS`, `extra_fields`, and `display_string` to
      `crates/fawi-core/src/front_matter.rs`; add unit tests covering
      non-modeled scalars/sequences, omitted mappings/null, and modeled-key
      exclusion.
- [ ] Add `extra_fields: BTreeMap<String, String>` to `ConceptSummary` and
      populate it in `Concept::summary()`; extend the core tests to assert a
      change-request-like concept's summary carries `state`/`priority`/`owner`.
- [ ] Add `extra_fields` to `ConceptResponse` and `ConceptSummaryResponse` and
      populate them in `from_concept`/`from_summary`.
- [ ] Render `extra_fields` as `key: value` badges in `ConceptView` and
      `ConceptListItem`, and add the `.badge.field` style.
- [ ] Run `cargo test` and `cargo build` (workspace; the `fawi-gui` build needs
      the `wasm32-unknown-unknown` target and `wasm-bindgen`).

Implemented and verified (builds and tests pass); recorded in the
[changelog](../changelog.md) for 2026-08-20.
