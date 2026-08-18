---
type: Specification
title: Sidebar navigation
description: A persistent sidebar for navigating the bundle tree.
status: stable
tags: [dev, gui]
owner: human:felix
---

# Problem

The web UI shows one page at a time. To move between concepts and directories a
user must return to the root listing or use search; there is no persistent
navigation that stays visible while reading a concept or browsing a directory.

# Requirements

- The GUI MUST render a persistent sidebar alongside the main content on every
  page (root, directory, concept, search, and not-found).
- The sidebar MUST list the bundle's directories and concepts as a tree from the
  root down.
- Directory entries MUST link to their directory page and SHOULD expand in place
  to reveal their children.
- Concept entries MUST link to their concept page.
- The entry for the current page MUST be visually highlighted.
- The sidebar MUST reflect the bundle contents and update when the bundle changes.
- The server MUST expose a `GET /api/tree` endpoint that returns the full
  directory tree (directories and the concepts within each) for the sidebar.
- On narrow viewports the sidebar MUST collapse behind a toggle.

# Acceptance Criteria

- Given the app is rendered, when viewing any page, then a sidebar is visible
  listing the bundle's root directories and concepts.
- Given a bundle with nested directories, when a directory entry is expanded,
  then its child directories and concepts are shown.
- Given the current page is a concept, when viewing it, then that concept is
  highlighted in the sidebar.
- Given a concept in a nested directory, when its sidebar link is selected, then
  the concept page is shown.
- Given a directory entry, when its sidebar link is selected, then the directory
  listing is shown.
- Given a narrow viewport, when viewing any page, then the sidebar is hidden
  behind a toggle that reveals it.
- Given `GET /api/tree`, when called, then the response contains every directory
  and concept in the bundle, recursively.

# Out of scope

- Editing, moving, or deleting the bundle through the sidebar.
- Reordering the tree (entries stay in the bundle's on-disk layout).
- Changes to the existing in-page breadcrumbs.
