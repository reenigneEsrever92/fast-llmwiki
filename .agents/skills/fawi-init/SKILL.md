---
name: fawi-init
description: Initialize a project's docs/ directory as an OKF bundle — create the standard scaffold of index, log, overview, front matter reference, and dev files so the change-driven workflow has somewhere to record work.
---

# Initializing the docs folder

A bundle is a directory of markdown files with YAML front matter. This skill
bootstraps a fresh `docs/` directory into a valid OKF bundle: the reserved
`index.md` and `log.md` files, a starter `overview.md` concept, the front matter
reference, and the `dev/` section the change-driven workflow writes into.

## 1. Check for an existing bundle

    ls docs/index.md docs/log.md 2>/dev/null

If either file exists, the directory already has a bundle. Do not overwrite it —
report what is there and stop, unless the user explicitly asks to start over.

## 2. Create the directories

    mkdir -p docs/dev/backlog

## 3. Write the reserved files

`index.md` and `log.md` are reserved filenames and carry no front matter.

- `docs/index.md` — the bundle entry point and navigation.
- `docs/log.md` — the directory update log.

## 4. Write the concept documents

Every other document is a concept. It must start with YAML front matter whose
only required field is `type`; the common optional fields are `title`,
`description`, `tags`, and `status`. Create:

- `docs/overview.md` — `type: Overview`; what the bundle is about.
- `docs/frontmatter.md` — `type: Reference`; the front matter schema.
- `docs/dev/index.md` — the development section index (no front matter needed;
  index filenames are reserved).
- `docs/dev/changelog.md` — `type: Changelog`; shipped changes, newest first.
- `docs/dev/backlog/index.md` — the backlog index; change requests land here.

Use this front matter on concept documents, adjusting `type` and fields:

    ---
    type: Overview
    title: <Title>
    description: <one-line summary>
    tags: [<topic>]
    status: draft
    ---

## 5. Record the creation

Append a dated entry to `docs/log.md`:

    ## YYYY-MM-DD
    * **Creation**: Initialized the docs directory as an OKF bundle.

## Next steps

`fawi-propose` records the first change request in `docs/dev/backlog/`.
