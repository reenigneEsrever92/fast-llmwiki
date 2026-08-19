---
name: okf-init
description: Bootstrap a project's docs/ directory as an Open Knowledge Format (OKF) bundle — inspect the project, scaffold the overview, getting-started, architecture, features, and contributing docs plus the dev/ subtree (roadmap, specs, plans, decisions), then hand off to okf-spec, okf-plan, and okf-dev.
---

# Initializing an OKF Bundle

Turn a project into a spec-driven one by creating its `docs/` directory as an
[Open Knowledge Format](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md)
bundle. A bundle is a directory tree of Markdown files with YAML front matter;
`index.md` and `log.md` are reserved filenames rendered on directory listings.

## 1. Inspect the project

Read the repository before writing anything. From the root, gather:

- The project name and one-line purpose (`README.md`, top-level manifests).
- The language, build system, and package layout (`Cargo.toml`, `package.json`,
  `pyproject.toml`, `go.mod`, etc.).
- How to build, run, and test the project.
- Any existing `docs/` or design notes to preserve.

Use these to fill in the scaffolding below. Do not invent features, commands, or
layout details that you did not actually find.

## 2. Create the canonical structure

    docs/
      index.md            # reserved: top-level landing page
      log.md              # reserved: directory update log
      overview.md         # what the project is and why
      getting-started.md  # build, run, and test
      architecture.md     # modules/components and data flow
      features.md         # what it does
      contributing.md     # the spec-driven workflow
      dev/
        index.md          # development workflow and conventions
        roadmap.md        # milestones and priorities
        specs/
          index.md        # feature specifications
        plans/
          index.md        # implementation plans
        decisions/
          index.md        # architecture decision records

Every non-`index.md` concept starts with YAML front matter containing at least
`type` and `title`. `index.md` and `log.md` are plain Markdown (no front matter).

## 3. Scaffold each file

`docs/overview.md`:

    ---
    type: Overview
    title: <Project Name>
    description: <one-line summary>
    tags: [overview]
    status: draft
    ---

    # Overview

    <what the project is and why, in 1-3 paragraphs>

`docs/getting-started.md` (`type: Guide`) — prerequisites, then build, run, and
test commands, each with a short explanation. Use the commands you found in the
project, not generic placeholders.

`docs/architecture.md` (`type: Architecture`) — the package/crate/module layout
as a table, then the data flow and key decisions.

`docs/features.md` (`type: Feature`) — a bullet list of what the project does.

`docs/contributing.md` (`type: Guide`) — the spec-driven workflow, pointing at
`dev/specs/`, `dev/plans/`, and the `okf-spec` / `okf-plan` / `okf-dev` skills.

`docs/dev/roadmap.md`:

    ---
    type: Roadmap
    title: Roadmap
    description: Planned work and priorities.
    status: draft
    tags: [dev]
    ---

    # Now

    # Next

Fill `# Now` and `# Next` from the README's TODO, planned features, or open
issues. Leave them empty if nothing is known.

`docs/index.md`, `docs/log.md`, and every `index.md` under `dev/` are plain
Markdown. Make the top-level `index.md` link to `overview`, `getting-started`,
`architecture`, `features`, `contributing`, and `dev/`. Give `docs/dev/index.md`
the workflow and conventions (spec → plan → implement → mark done), and make the
`specs/`, `plans/`, and `decisions/` index files link to their children (empty
lists are fine). Start `docs/log.md` with a title and a dated creation entry.

## 4. Verify the bundle

- Every non-`index.md` concept has a `type` field in its front matter.
- `docs/index.md`, `docs/log.md`, and `docs/dev/{index,roadmap}.md` exist.
- `docs/dev/{specs,plans,decisions}/index.md` exist.
- All relative links resolve to files that exist.

## Next steps

Hand off to the rest of the workflow:

1. `okf-spec` — write the first `type: Specification` for the next feature.
2. `okf-plan` — turn it into a `type: Plan`.
3. `okf-dev` — implement it and mark it done.
