---
type: Specification
title: Automated test, build, and release pipeline
description: A GitHub Actions pipeline that tests, builds, and publishes OKF release binaries for Linux (x86_64 and arm64), macOS, and Windows.
status: stable
tags: [dev, ci, release]
owner: human:felix
---

# Problem

OKF is built and verified by hand. There is no continuous integration to catch
breakage on every change, and no automated way to produce release artifacts.
Users who want to run `okf` without a Rust toolchain must build it themselves,
and there are no prebuilt binaries for the three mainstream desktop platforms.

A released `okf` binary is non-trivial to build: the SSR server embeds the
client bundle via a nested `wasm32-unknown-unknown` build that also requires
`wasm-bindgen-cli`, and `okf-search` depends on `fastembed`/ONNX Runtime native
binaries. A release pipeline therefore cannot naively cross-compile and must
build each platform on a matching native runner.

# Requirements

- A GitHub Actions **CI workflow** MUST run on every push to `main` and every
  pull request, and MUST fail the check when verification fails.
- The CI workflow MUST:
  - install the `wasm32-unknown-unknown` target and `wasm-bindgen-cli`;
  - build the workspace, including the `okf-gui` `ssr` feature;
  - run the unit tests.
- A GitHub Actions **release workflow** MUST build and publish release binaries
  on tag pushes matching `v*` (semantic version tags).
- The release workflow MUST build the release binaries natively on each
  platform's own runner rather than cross-compiling.
- The release workflow MUST publish a GitHub Release on the tagged ref and
  attach the built binaries as release assets.
- The following release binaries MUST be produced from the unified `okf` crate
  (`okf-cli`):
  - Linux x86_64 (glibc) on `ubuntu-latest`;
  - Linux arm64 (glibc) on `ubuntu-24.04-arm`;
  - Windows x86_64 on `windows-latest`;
  - macOS on `macos-latest`.
- Binaries MUST be named and archived so that the platform and architecture are
  unambiguous (for example `okf-linux-x86_64`, `okf-linux-aarch64`,
  `okf-windows-x86_64`, `okf-macos-aarch64`), with `.tar.gz` archives on Unix and
  `.zip` on Windows.
- The release workflow MUST NOT publish artifacts when the build or tests fail.

# Acceptance Criteria

- Given a push to `main`, when the CI workflow runs, then the workspace builds
  (including the `ssr` feature), the unit tests pass, and the check is green.
- Given a pull request, when it is opened or updated, then the same CI checks
  run and report status on the pull request.
- Given a tag pushed with the name `v0.1.0`, when the release workflow runs,
  then a GitHub Release is created and assets for Linux, Windows, and macOS are
  attached to it.
- Given a successful release build, when inspecting the assets, then the
  platform and architecture of each archive are identifiable from its name.
- Given a broken build or failing test in the release workflow, when it runs,
  then no GitHub Release or asset is published.

# Out of scope

- Cross-compiling a single build to multiple targets from one runner.
- Code signing and notarization (including Windows Authenticode and macOS
  notarization).
- Publishing to package registries such as crates.io, Homebrew, or Scoop.
- Producing Linux musl, 32-bit ARM, or container/Docker images.
- Semantic-release automation that computes version numbers; release builds are
  triggered by manually pushed `v*` tags.