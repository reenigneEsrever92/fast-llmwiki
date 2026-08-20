use comrak::{format_html, parse_document, Arena, Options};
use regex::Regex;

#[cfg(feature = "mermaid")]
use comrak::nodes::NodeValue;

/// Render a markdown body to safe HTML, rewriting bundle links so that `.md`
/// suffixes are dropped (concepts are addressed by their concept ID) and
/// relative links resolve against `base_dir`.
///
/// When the `mermaid` feature is enabled, fenced `mermaid` code blocks are
/// rendered to inline SVG with Merman and substituted into the HTML.
pub fn render_markdown(markdown: &str, base_dir: &str) -> String {
    let expanded = rewrite_links(markdown, base_dir);
    let options = build_options();

    let arena = Arena::new();
    let root = parse_document(&arena, &expanded, &options);

    #[cfg(feature = "mermaid")]
    let mermaid_sources = collect_mermaid_sources(root);

    let mut out = Vec::new();
    format_html(root, &options, &mut out).expect("writing HTML to a Vec cannot fail");
    let html = String::from_utf8(out).expect("comrak output is UTF-8");

    #[cfg(feature = "mermaid")]
    {
        inline_mermaid(&html, &mermaid_sources)
    }
    #[cfg(not(feature = "mermaid"))]
    {
        html
    }
}

fn build_options() -> Options {
    let mut options = Options::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.footnotes = true;
    options.extension.superscript = true;
    options.render.hardbreaks = true;
    options.render.unsafe_ = false;
    options
}

/// Collect the raw source of every fenced `mermaid` code block, in document order.
#[cfg(feature = "mermaid")]
fn collect_mermaid_sources<'a>(root: &'a comrak::nodes::AstNode<'a>) -> Vec<String> {
    let mut sources = Vec::new();
    for node in root.descendants() {
        if let NodeValue::CodeBlock(cb) = &node.data.borrow().value {
            if cb.fenced && is_mermaid_info(&cb.info) {
                sources.push(cb.literal.clone());
            }
        }
    }
    sources
}

#[cfg(feature = "mermaid")]
fn is_mermaid_info(info: &str) -> bool {
    info.split_whitespace()
        .next()
        .map(|s| s.eq_ignore_ascii_case("mermaid"))
        .unwrap_or(false)
}

/// Replace each `<pre><code class="language-mermaid">…</code></pre>` block in the
/// comrak HTML with the rendered SVG for the corresponding source, preserving the
/// original block when rendering fails.
#[cfg(feature = "mermaid")]
fn inline_mermaid(html: &str, sources: &[String]) -> String {
    let re = Regex::new(r#"<pre><code class="language-mermaid">[\s\S]*?</code></pre>"#)
        .expect("mermaid block regex is valid");

    let mut result = String::with_capacity(html.len());
    let mut last = 0;
    for (i, m) in re.find_iter(html).enumerate() {
        result.push_str(&html[last..m.start()]);
        match sources.get(i).and_then(|src| render_mermaid(src, i)) {
            Some(svg) => {
                result.push_str("<div class=\"mermaid\">");
                result.push_str(&svg);
                result.push_str("</div>");
            }
            None => result.push_str(m.as_str()),
        }
        last = m.end();
    }
    result.push_str(&html[last..]);
    result
}

/// Render one Mermaid diagram to sanitized SVG, using a unique id per diagram so
/// several diagrams on a page do not collide on SVG `id`s.
#[cfg(feature = "mermaid")]
fn render_mermaid(source: &str, index: usize) -> Option<String> {
    let id = format!("mermaid-{index}");
    merman::render::HeadlessRenderer::new()
        .with_diagram_id(&id)
        .render_svg_sync(source)
        .ok()
        .flatten()
}

fn rewrite_links(markdown: &str, base_dir: &str) -> String {
    let link = Regex::new(r"\[([^\]]*)\]\(([^)\s]+)\)").unwrap();
    link.replace_all(markdown, |caps: &regex::Captures<'_>| {
        let label = &caps[1];
        let url = &caps[2];
        format!("[{label}]({})", resolve_link(url, base_dir))
    })
    .into_owned()
}

fn resolve_link(url: &str, base_dir: &str) -> String {
    let (path, fragment) = match url.find('#') {
        Some(i) => (&url[..i], Some(&url[i..])),
        None => (url, None),
    };

    // Anchors and external URLs are left untouched.
    if path.is_empty() || is_external(path) {
        return url.to_string();
    }

    let resolved = if path.starts_with('/') {
        normalize_path(path)
    } else {
        let base = base_dir.trim_matches('/');
        if base.is_empty() {
            normalize_path(path)
        } else {
            normalize_path(&format!("{base}/{path}"))
        }
    };

    // Concepts are addressed by their concept ID (no `.md` suffix).
    let mut resolved = resolved.trim_end_matches(".md").to_string();
    if !resolved.starts_with('/') {
        resolved = format!("/{resolved}");
    }

    match fragment {
        Some(f) => format!("{resolved}{f}"),
        None => resolved,
    }
}

fn is_external(url: &str) -> bool {
    match url.split_once(':') {
        Some((scheme, _)) => {
            let mut chars = scheme.chars();
            chars
                .next()
                .map(|c| c.is_ascii_alphabetic())
                .unwrap_or(false)
                && scheme
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '-' || c == '.')
        }
        None => false,
    }
}

fn normalize_path(path: &str) -> String {
    let mut segments: Vec<&str> = Vec::new();
    for seg in path.split('/') {
        match seg {
            "" | "." => {}
            ".." => {
                segments.pop();
            }
            s => segments.push(s),
        }
    }
    segments.join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_basic_markdown() {
        assert_eq!(render_markdown("# Title", ""), "<h1>Title</h1>\n");
    }

    #[test]
    fn resolves_absolute_bundle_link() {
        assert_eq!(
            resolve_link("/tables/customers.md", ""),
            "/tables/customers"
        );
    }

    #[test]
    fn resolves_relative_links() {
        assert_eq!(resolve_link("./other.md", "tables"), "/tables/other");
        assert_eq!(
            resolve_link("../computations/x.md", "tables"),
            "/computations/x"
        );
    }

    #[test]
    fn leaves_external_links_alone() {
        assert_eq!(
            resolve_link("https://example.com/x.md", ""),
            "https://example.com/x.md"
        );
    }

    #[test]
    fn preserves_anchors() {
        assert_eq!(
            resolve_link("/tables/customers.md#schema", ""),
            "/tables/customers#schema"
        );
        assert_eq!(resolve_link("#schema", ""), "#schema");
    }

    #[cfg(feature = "mermaid")]
    #[test]
    fn renders_mermaid_block_to_svg() {
        let html = render_markdown(
            "```mermaid\nflowchart TD\n    A[Start] --> B[End]\n```\n",
            "",
        );
        assert!(html.contains("<svg"), "expected SVG, got: {html}");
        assert!(
            !html.contains("language-mermaid"),
            "mermaid code block should be replaced, got: {html}"
        );
    }

    #[cfg(feature = "mermaid")]
    #[test]
    fn leaves_non_mermaid_code_blocks_alone() {
        let html = render_markdown("```rust\nfn main() {}\n```\n", "");
        assert!(html.contains("language-rust"));
        assert!(!html.contains("<svg"));
    }

    #[cfg(feature = "mermaid")]
    #[test]
    fn renders_multiple_mermaid_diagrams_with_distinct_ids() {
        let md = "```mermaid\nflowchart TD\n    A --> B\n```\n\n```mermaid\nflowchart LR\n    C --> D\n```\n";
        let html = render_markdown(md, "");
        assert_eq!(html.matches("<svg").count(), 2, "got: {html}");
        assert!(
            html.contains("mermaid-0"),
            "missing first diagram id: {html}"
        );
        assert!(
            html.contains("mermaid-1"),
            "missing second diagram id: {html}"
        );
    }

    #[cfg(feature = "mermaid")]
    #[test]
    fn falls_back_to_code_block_when_mermaid_fails() {
        let html = "<pre><code class=\"language-mermaid\">bad</code></pre>\n";
        let sources = vec!["this is not a diagram".to_string()];
        let out = inline_mermaid(html, &sources);
        assert!(
            out.contains("language-mermaid"),
            "expected fallback to code block, got: {out}"
        );
    }
}
