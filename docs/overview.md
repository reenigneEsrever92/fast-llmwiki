---
type: Overview
title: OKF Bundle Server
description: A read-only server and web UI for the Open Knowledge Format.
tags: [overview, okf]
status: stable
---

# Overview

This project serves a directory of [OKF](okf-format.md) concept documents —
markdown files with YAML front matter — over a REST API and a server-rendered
web UI.

It is **read-only**: it never modifies the bundle. It watches the directory and
reloads on changes, so it can sit alongside a git checkout or an
agent-maintained corpus.

See [Architecture](architecture.md) for how the pieces fit together and
[Features](features.md) for what it does.

# Examples

This `docs/` directory is itself an OKF bundle, so the documentation you are
reading is served by the server itself.
