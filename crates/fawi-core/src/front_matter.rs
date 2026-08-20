//! Front matter extraction: the YAML block delimited by `---` at the top of a
//! markdown file. Parsing of the YAML itself lives in `concept`.

/// If `content` starts with a `---`-delimited front matter block, return the
/// raw YAML text (without the delimiters) plus the body that follows it.
/// Otherwise return `None` and the original content unchanged.
pub fn split_front_matter(content: &str) -> (Option<String>, String) {
    let first_line_end = content.find('\n').unwrap_or(content.len());
    let first_line = content[..first_line_end].trim_end_matches('\r');
    if first_line.trim() != "---" {
        return (None, content.to_string());
    }

    let mut cursor = if first_line_end < content.len() {
        first_line_end + 1
    } else {
        content.len()
    };

    loop {
        if cursor >= content.len() {
            // Unterminated front matter: treat the whole document as body.
            return (None, content.to_string());
        }

        let line_end = content[cursor..]
            .find('\n')
            .map(|i| cursor + i)
            .unwrap_or(content.len());
        let line = content[cursor..line_end].trim_end_matches('\r');

        if line.trim() == "---" || line.trim() == "..." {
            let yaml = content[first_line_end + 1..cursor].to_string();
            let body_start = if line_end < content.len() {
                line_end + 1
            } else {
                content.len()
            };
            return (Some(yaml), content[body_start..].to_string());
        }

        if line_end == content.len() {
            return (None, content.to_string());
        }
        cursor = line_end + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_front_matter_and_body() {
        let input = "---\ntype: Metric\n---\n# Body\n";
        let (yaml, body) = split_front_matter(input);
        assert_eq!(yaml.as_deref(), Some("type: Metric\n"));
        assert_eq!(body, "# Body\n");
    }

    #[test]
    fn no_front_matter() {
        let input = "# Just a page\n";
        let (yaml, body) = split_front_matter(input);
        assert!(yaml.is_none());
        assert_eq!(body, "# Just a page\n");
    }

    #[test]
    fn unterminated_front_matter_is_body() {
        let input = "---\ntype: Metric\n";
        let (yaml, body) = split_front_matter(input);
        assert!(yaml.is_none());
        assert_eq!(body, input);
    }
}
