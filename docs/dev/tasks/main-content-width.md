---
type: Task
title: Give the main content column a consistent width on wide screens
status: stable
state: done
priority: low
tags: [dev, gui]
verified: { by: ai:zed, at: 2026-08-19T09:33:13Z }
---

Implements [main content width](/dev/specs/main-content-width.md).

- [x] Change the `.content` rule in `crates/okf-gui/src/app.rs` so it grows to
      fill the available space as a flex item instead of sizing to its contents,
      while keeping the `56rem` maximum, a `min-width: 0` so it can shrink on
      narrow viewports, and `margin: 0 auto` so it stays centered.
- [x] Confirm the content column fills up to `56rem` on a wide viewport, stays
      centered, and shrinks below `56rem` on a narrow viewport, on every page
      type.
