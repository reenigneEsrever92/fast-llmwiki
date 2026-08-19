---
type: Plan
title: Add GitHub Actions CI and release workflows
status: stable
state: done
priority: medium
tags: [dev, ci, release]
verified: { by: ai:zed, at: 2026-08-19T10:54:44Z }
---

Implements [automated test, build, and release pipeline](/dev/specs/github-release-pipeline.md).

- [x] Add a CI workflow (`.github/workflows/ci.yml`) that runs on pushes to
  `main` and pull requests, installs the `wasm32-unknown-unknown` target and
  `wasm-bindgen-cli`, builds the workspace with the `ssr` feature, and runs the
  unit tests.
- [x] Add a release workflow (`.github/workflows/release.yml`) that triggers on
  `v*` tags and builds on native runners for Linux, Windows, and macOS.
- [x] Produce named, archived release assets (`okf-linux-x86_64`,
  `okf-linux-aarch64`, `okf-windows-x86_64`, `okf-macos-aarch64`) with
  `.tar.gz`/`.zip` as appropriate.
- [x] Attach the built assets to a GitHub Release on the tagged ref.
- [x] Guard the release so no asset is published when the build or tests fail.
- [x] Document the release process and the asset naming scheme in
  [Releases](/dev/releases.md).

Commit: not yet committed.
