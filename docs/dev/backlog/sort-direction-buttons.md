---
type: ChangeRequest
kind: feature
title: Directional sorting with per-field toggle buttons
description: Replace the sort/filter controls with per-field sort buttons that cycle ascending, descending, and off.
state: done
priority: medium
tags: [dev, gui]
owner: human:felix
verified: { by: human:felix, at: 2026-08-20T21:42:50Z }
---

# Problem

Directory listings can currently be narrowed with a `filter` query parameter and
ordered with a `sort` parameter, but sorting only supports a single (ascending)
direction and is driven by a `<select>` plus a form submit. There is no way to
reverse the sort order, and the filtering controls add complexity that is not
needed right now.

# Proposal

Remove filtering for now and extend sorting to support a direction. The
`/api/dirs` endpoint gains a `dir=asc|desc` parameter (defaulting to `asc`) that
applies to whichever field `sort` names; the `filter` parameter and all of its
plumbing are dropped. In the web UI the `<select>`/filter form is replaced by a
row of buttons — one per front matter field — where clicking a field sorts by it
ascending, a second click sorts descending, and a third click removes the sort.

# Feasibility

- Directional sort is cheap: `fawi_core::front_matter::compare_values` already
  returns a `std::cmp::Ordering`, so descending is just `Ordering::reverse()` in
  `FsBundle::list_dir`. No changes to the comparison layer are required.
- `ListOptions` (`fawi-storage/src/lib.rs`) drops its `filters` field and gains a
  `SortDirection` enum (`Ascending`/`Descending`). The `BundleSource::list_dir`
  signature is unchanged (still takes `&ListOptions`).
- Filtering is removed from `fawi-storage/src/fs_bundle.rs` (`apply_filters`), the
  server (`fawi-server/src/api.rs` `DirQuery.filter` + `parse_filters`), the
  client (`fawi-gui/src/api_client.rs` `fetch_dir`), and `DirView`.
  `fawi_core::front_matter::values_match` loses its only caller but stays as a
  tested `pub` utility (no dead-code warning, and it keeps the door open to
  re-adding filtering without rewriting the matcher). It can be deleted later if
  strict dead-code elimination is preferred.
- The GUI (`crates/fawi-gui/src/app.rs`) renders the per-field buttons as plain
  `<a href>` elements computed from `DirListingResponse.fields` and the current
  `sort`/`dir` query, so the three-state toggle works without client JS and
  degrades gracefully under SSR. `fields` already enumerates every top-level
  front matter key (including `title` when declared), sorted and deduped.
- No new dependencies. Docs: only `docs/api/rest-api.md` describes `sort`/`filter`
  today, so that is the only doc to update.

# Acceptance criteria

- Given `sort=status&dir=desc`, then concepts are ordered by `status` descending
  (with `title` as the tiebreaker, also descending).
- Given `sort=status` with no `dir`, then concepts are ordered by `status`
  ascending.
- Given no `sort`, then concepts are ordered by `title` ascending regardless of
  any `dir` value.
- Given a directory listing, then the UI renders one button per field in
  `DirListingResponse.fields`; clicking an inactive field sorts ascending,
  clicking it again sorts descending, and a third click removes the sort.
- Given the old `filter` query parameter, then the endpoint neither errors nor
  applies any filtering, and the UI shows no filter controls.

# Implementation plan

## Approach

Extend the existing sort path with a direction and delete the filter path
end-to-end, keeping the sort logic server-side and the browser thin.

- **Storage**: add `SortDirection` (default `Ascending`) and reshape `ListOptions`
  to `{ sort: Option<String>, direction: SortDirection }`. In
  `FsBundle::list_dir`, drop the `apply_filters` pass and reverse the sort
  comparator (field comparison plus the `title` tiebreaker) when the direction is
  `Descending`. Keep `apply_filters` deletion paired with removing the
  `values_match` import.
- **Server**: replace `DirQuery { sort, filter }` with
  `DirQuery { sort, dir }`; add a `parse_direction` helper and remove
  `parse_filters`. `get_dir`/`get_dir_root` pass `sort` and `direction` through.
- **Client**: `fetch_dir` takes `(path, sort, dir)` and emits `sort=` and `dir=`
  params; drop the `filter` param.
- **GUI**: `Page` keys its resource on `sort`/`dir` and calls `fetch_dir` when a
  sort is present, `fetch_page` otherwise. `DirView` replaces the form with a
  button per field in `dir.fields`; each button computes its arrow/state from the
  current query and links to `sort=<field>&dir=asc`,
  `sort=<field>&dir=desc`, or the bare path (off).
- **Docs**: update the `/api/dirs` paragraph and example in
  `docs/api/rest-api.md`.

## Steps

- [ ] Add `SortDirection` to `fawi-storage/src/lib.rs` and change `ListOptions`
  to `{ sort: Option<String>, direction: SortDirection }`, removing `filters`.
- [ ] In `fawi-storage/src/fs_bundle.rs`, delete `apply_filters` (and the
  `values_match` import), and make `list_dir` apply the direction when sorting;
  update the `list_dir_sorts_and_filters_by_front_matter` test to assert both
  ascending and descending `priority` order and drop the filter assertions.
- [ ] In `fawi-server/src/api.rs`, drop `filter` from `DirQuery`, add `dir`, add
  a `parse_direction` helper, remove `parse_filters`, and thread `direction` into
  the `ListOptions` built by `get_dir`/`get_dir_root`.
- [ ] In `fawi-gui/src/api_client.rs`, change `fetch_dir` to accept
  `(path, sort, dir)` and build `sort=`/`dir=` query params (no `filter`).
- [ ] In `fawi-gui/src/app.rs`, update `Page` to read `sort`/`dir` and call the
  new `fetch_dir` signature; replace `DirView`'s form with per-field `<a>` toggle
  buttons (arrow indicator, asc → desc → off) and add any needed `STYLE` rules.
- [ ] Update `docs/api/rest-api.md` to document `sort=<field>&dir=asc|desc` and
  remove the `filter` parameter and example.
- [ ] Run `cargo build` and `cargo test` across the workspace to confirm the
  change compiles and all tests pass.

Implemented and verified (builds and tests pass); recorded in the
[changelog](../changelog.md) for 2026-08-20.
