---
type: ChangeRequest
title: Surface dev fields in the web UI
description: Render state, priority, and owner as badges on change requests.
state: proposed
priority: low
tags: [dev, gui]
owner: human:felix
---

# Problem

The dev front matter fields used to drive development — `state`, `priority`, and
`owner` — are not visible in the web UI, so a reader cannot tell at a glance
whether a change request is proposed, planned, or done.

# Proposal

Render `state`, `priority`, and `owner` as badges (or a compact metadata row) on
concept pages when they are present, mirroring how `status` and trust are already
displayed.

# Feasibility

- The web UI already renders front matter metadata (status, trust tier,
  provenance), so this extends existing display logic in `fawi-gui`.
- These fields are producer extensions; the server does not currently parse
  them, so surfacing them may require exposing them through `fawi-core` DTOs or
  reading them from the raw front matter.
- Low risk and no new dependencies; scope is limited to presentation.

# Acceptance criteria

- Given a concept with `state`, `priority`, or `owner` in its front matter, when
  rendered, then those values are visible as badges.
- Given a concept without those fields, when rendered, then no empty badges are
  shown.
