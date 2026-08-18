use comrak::{markdown_to_html, Options};
use regex::Regex;

/// Render a markdown body to safe HTML, rewriting bundle links so that `.md`
/// suffixes are dropped (concepts are addressed by their concept ID) and
/// relative links resolve against `base_dir`.
pub fn render_markdown(markdown: &str, base_dir: &str) -> String {
    let expanded = rewrite_links(markdown, base_dir);

    let mut options = Options::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.footnotes = true;
    options.extension.superscript = true;
    options.render.hardbreaks = true;
    options.render.unsafe_ = false;

    markdown_to_html(&expanded, &options)
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
}
