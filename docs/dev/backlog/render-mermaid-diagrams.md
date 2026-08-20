---
type: ChangeRequest
title: Render Mermaid diagrams
description: Render fenced Mermaid code blocks as diagrams in the web UI instead of showing raw code.
state: done
priority: medium
tags: [dev, gui]
owner: human:felix
verified: { by: human:felix, at: 2026-08-20T21:47:49Z }
---

# Problem

`fawi-core` renders Markdown to HTML with `comrak`
(`crates/fawi-core/src/render.rs`). A fenced Mermaid code block is emitted as a
plain `<pre><code class="language-mermaid">…</code></pre>`, and the web UI injects
that HTML verbatim with `inner_html` (`crates/fawi-gui/src/app.rs`). The result is
that Mermaid diagrams — including the two in `docs/architecture.md` — are shown as
raw code blocks rather than rendered diagrams.

# Proposal

Render Mermaid fenced code blocks as SVG diagrams instead of showing them as raw
code. Rendered server-side at request time with Merman, the headless Rust
implementation of Mermaid, inside `fawi-core`/`fawi-server`, so the web UI
receives a self-contained `<svg>` with no browser JavaScript, no CDN, and no
vendored asset. The renderer is invoked on the Markdown that already flows through
`render_markdown`, turning each fenced Mermaid code block into inline SVG.

# Feasibility

- **Where the change lands**: server-side, in `fawi-core/src/render.rs` (extend
  `render_markdown` to detect fenced Mermaid code blocks and replace them with
  inline SVG via a pure-Rust renderer) and `fawi-core/Cargo.toml` (the new
  dependency). This covers all three injection sites — concept `content_html`,
  directory `index_html`, and directory `log_html` — since they all go through
  `render_markdown`.
- **No existing mechanism**: `fawi-core` has no Mermaid renderer in its dependency
  graph, and `render_markdown` has no code-fence hook today. The change adds a
  pure-Rust renderer and a code-fence detection pass.
- **Renderer**: [Merman](https://github.com/Latias94/merman), Mermaid's headless
  Rust implementation, used as the server-side renderer inside `fawi-core`. It
  tracks `mermaid@11.16.1`, is Apache-2.0/MIT licensed, and is already used by
  Zed. It is pre-1.0, and HTML-label/`foreignObject` markup plus font metrics can
  differ from the browser Mermaid.js output.
- **Server-side rendering considerations**: Mermaid syntax must be covered for the
  bundle's diagram families (today just `flowchart TD` and `flowchart LR`). HTML
  labels written with raw HTML or `foreignObject` (e.g. `<br/>` inside a node
  label) are a known area where pure-Rust output can differ from browser
  Mermaid.js, and the renderer must sanitize the SVG so nothing executable reaches
  the page. Merman applies sanitization and uses `mermaid@11.x` as its reference;
  these become the acceptance baseline for behavior.
- **Out of scope**: syntax highlighting for other code languages, interactive
  Mermaid features such as clickable links, and byte-for-byte parity with the
  browser Mermaid.js output when a pure-Rust renderer is used.

# Acceptance criteria

- Given a concept whose body contains a fenced Mermaid code block, when viewed
  in the web UI, then the block is rendered as an SVG diagram, not as raw code.
- Given a directory whose `index.md` or `log.md` contains a fenced Mermaid block, when
  viewed, then the block is rendered as a diagram.
- Given a page with a Mermaid diagram, when the user navigates to another page and
  back (or the bundle reloads in place), then the diagram renders on each view
  without a full page reload.
- Given a Mermaid block that uses escaped markup such as `<br/>`, when rendered,
  then the diagram is correct and no raw entities are visible.
- Given the rendered diagram SVG, then it is sanitized and introduces no arbitrary
  script execution.

# Implementation plan

## Approach

Render Mermaid fences server-side in `fawi-core`, where every Markdown body
already flows through `render_markdown` (`crates/fawi-core/src/render.rs`) before
being served as `content_html`, `index_html`, or `log_html`. No client-side code
changes: the browser receives a self-contained `<svg>`.

- **Renderer**: Merman (`merman = "0.7"`, feature `render`), Mermaid's headless
  Rust implementation. Each fenced `mermaid` code block is rendered with
  `merman::render::HeadlessRenderer::render_svg_sync`, which returns
  `Result<Option<String>, _>` — the SVG string, or `None` when no diagram is
  detected. Give each diagram a unique `diagram_id` (e.g. `mermaid-0`,
  `mermaid-1`) so several diagrams on one page do not collide on SVG `id`s.
- **Data flow**: split `render_markdown` into three phases — (1) `comrak`
  `parse_document`/`format_html` (same options as today), (2) a walk over the
  comrak AST to collect the raw `literal` of every fenced code block whose first
  info-string token is `mermaid` (raw source, so no HTML-entity decoding is
  needed), and (3) a substitution pass that replaces each corresponding
  `<pre><code class="language-mermaid">…</code></pre>` in the HTML with its
  rendered SVG.
- **Why substitute after comrak, not inject AST nodes or use the codefence
  plugin**: comrak's HTML formatter strips raw HTML when `render.unsafe_ = false`
  (it emits `<!-- raw HTML omitted -->`), and its `SyntaxHighlighterAdapter`
  hook always wraps output in `<pre><code>…</code></pre>` — so neither path can
  yield a bare `<svg>`. Substituting after formatting is the only approach that
  keeps comrak's existing escaping for everything else while emitting raw SVG.
- **Fallback**: if Merman returns `Err` or `None` (invalid or unsupported syntax),
  leave the original code block in place rather than dropping the diagram source.
- **Feature gating**: `fawi-core` is also compiled into the GUI's wasm32 client.
  Make the Merman dependency optional behind a new `mermaid` feature on
  `fawi-core` and enable it only in `fawi-server`, so the wasm client never
  builds Merman (large dependency; MSRV 1.95). The transformation in
  `render_markdown` is `#[cfg(feature = "mermaid")]`; without the feature the
  output is byte-for-byte identical to today.
- **Sanitization**: Merman emits sanitized SVG (satisfying the "no script"
  criterion), and its `<foreignObject>` HTML-label support covers the `<br/>`
  markup in the bundle's diagrams.

## Steps

- [ ] Add `merman` as an optional dependency and a `mermaid` feature to
  `fawi-core/Cargo.toml`, and enable `fawi-core/mermaid` in
  `fawi-server/Cargo.toml`.
- [ ] Refactor `render_markdown` (`fawi-core/src/render.rs`) to parse via
  `comrak::parse_document` on an `Arena` and format via `comrak::format_html`,
  preserving today's options and output exactly.
- [ ] Add a helper that walks the comrak AST and collects the raw source of
  fenced `mermaid` code blocks in document order.
- [ ] Add a `#[cfg(feature = "mermaid")]` pass that renders each collected source
  with Merman (unique `diagram_id` per block) and substitutes the SVGs into the
  formatted HTML, falling back to the original block on error/`None`.
- [ ] Add unit tests in `fawi-core` (gated on the feature): a rendered `mermaid`
  block (assert `<svg` is present and `language-mermaid` is absent), a
  non-mermaid code block (unchanged), two diagrams on one page (distinct ids),
  and invalid Mermaid (original block preserved).
- [ ] Run `cargo test --workspace` and
  `cargo build --workspace --features fawi-gui/ssr` (which also builds the wasm
  client without the `mermaid` feature) to confirm the gating keeps the client
  build unaffected.
- [ ] (Polish) Add `.page-body .mermaid` CSS to `fawi-gui/src/app.rs` (e.g.
  `max-width: 100%`, horizontal overflow) so wide diagrams stay within the
  content column.

Implemented and verified (builds and tests pass); recorded in the
[changelog](../changelog.md) for 2026-08-20.
