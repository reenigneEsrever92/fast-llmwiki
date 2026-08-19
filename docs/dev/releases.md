---
type: Guide
title: Releases
description: How release binaries are built and published, and the asset naming scheme.
tags: [dev, ci, release]
status: stable
---

# Releases

Release binaries are built and published automatically by GitHub Actions. There
is no cross-compilation: each platform is built on its own native runner, which
is required because `okf-search` links native ONNX Runtime binaries and the
`okf` server binary embeds a `wasm32-unknown-unknown` client bundle.

## Cutting a release

1. Tag the commit you want to release with a semantic-version tag:

       git tag v0.1.0
       git push origin v0.1.0

2. The [Release workflow](.github/workflows/release.yml) runs the unit tests,
   then builds the `okf` binary on each platform and publishes a GitHub Release
   named after the tag with the binaries attached.

The release is only created when every test and build step succeeds, so a
failing build or test never publishes assets.

## Asset naming

Each archive encodes the platform and architecture of the binary inside it:

| Asset | Platform | Runner | Archive |
| --- | --- | --- | --- |
| `okf-linux-x86_64` | Linux x86_64 (glibc) | `ubuntu-latest` | `.tar.gz` |
| `okf-linux-aarch64` | Linux arm64 (glibc) | `ubuntu-24.04-arm` | `.tar.gz` |
| `okf-windows-x86_64` | Windows x86_64 | `windows-latest` | `.zip` |
| `okf-macos-aarch64` | macOS (Apple Silicon) | `macos-latest` | `.tar.gz` |

Archives use `.tar.gz` on Unix and `.zip` on Windows, matching the convention
that platform and architecture must be identifiable from the filename alone.

## Building locally

A release-equivalent build on any platform is:

    cargo build --release -p okf-cli

This produces the `okf` binary (`okf.exe` on Windows) in `target/release/`.
Because `okf-cli` depends on `okf-gui` with the `ssr` feature, the build also
compiles the `wasm32-unknown-unknown` client bundle, so it requires:

    rustup target add wasm32-unknown-unknown
    cargo install wasm-bindgen-cli --version 0.2.127
