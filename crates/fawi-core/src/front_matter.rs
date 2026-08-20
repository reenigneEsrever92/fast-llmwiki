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

use std::cmp::Ordering;
use std::collections::BTreeMap;

use serde_yaml::Value;

/// Look up a top-level front matter key in a YAML mapping. Returns `None` when
/// `front_matter` is not a mapping or the key is absent.
pub fn get_field<'a>(front_matter: &'a Value, key: &str) -> Option<&'a Value> {
    front_matter.as_mapping().and_then(|map| {
        map.iter()
            .find(|(k, _)| k.as_str() == Some(key))
            .map(|(_, v)| v)
    })
}

/// Compare two front matter values into a total order for sorting.
///
/// Kind order is `null < bool < number < string < sequence < mapping`; within a
/// kind, values compare naturally: booleans `false < true`, numbers numerically,
/// strings case-insensitively, and sequences element-wise. Mappings are treated
/// as equal and thus keep the caller's stable fallback.
pub fn compare_values(a: &Value, b: &Value) -> Ordering {
    use Value::{Bool, Mapping, Null, Number, Sequence, String};

    match (a, b) {
        (Null, Null) => Ordering::Equal,
        (Bool(x), Bool(y)) => x.cmp(y),
        (Number(x), Number(y)) => compare_numbers(x, y),
        (String(x), String(y)) => x.to_lowercase().cmp(&y.to_lowercase()),
        (Sequence(x), Sequence(y)) => compare_sequences(x, y),
        (Mapping(_), Mapping(_)) => Ordering::Equal,
        (x, y) => value_rank(x).cmp(&value_rank(y)),
    }
}

/// Whether `value` matches a filter string (case-insensitive).
///
/// Scalars match when their string form equals the query; a sequence matches
/// when any element matches; mappings and null never match.
pub fn values_match(value: &Value, query: &str) -> bool {
    let q = query.trim().to_lowercase();
    match value {
        Value::Null => false,
        Value::Bool(b) => b.to_string().to_lowercase() == q,
        Value::Number(n) => n.to_string().to_lowercase() == q,
        Value::String(s) => s.to_lowercase() == q,
        Value::Sequence(seq) => seq.iter().any(|v| values_match(v, &q)),
        Value::Mapping(_) => false,
        _ => false,
    }
}

/// The distinct top-level front matter keys, sorted.
pub fn keys(front_matter: &Value) -> Vec<String> {
    let mut out: Vec<String> = front_matter
        .as_mapping()
        .into_iter()
        .flat_map(|m| m.keys())
        .filter_map(|k| k.as_str().map(String::from))
        .collect();
    out.sort();
    out.dedup();
    out
}

/// Top-level front matter keys the server models and renders in a dedicated,
/// structured way. These are excluded from [`extra_fields`].
pub const MODELED_KEYS: &[&str] = &[
    "type",
    "title",
    "description",
    "resource",
    "tags",
    "status",
    "generated",
    "verified",
    "stale_after",
    "sources",
];

/// The top-level front matter fields the server does not model, as a
/// deterministic (key-sorted) map of key → display string.
///
/// Sequences are comma-joined; nulls, mappings, and tagged values are omitted.
pub fn extra_fields(front_matter: &Value) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for key in keys(front_matter) {
        if MODELED_KEYS.contains(&key.as_str()) {
            continue;
        }
        if let Some(value) = get_field(front_matter, &key).and_then(display_string) {
            out.insert(key, value);
        }
    }
    out
}

/// Stringify a front matter value for display. Scalars become their text form,
/// sequences comma-join their elements, and nulls/mappings/tagged values return
/// `None` (nothing to show).
fn display_string(value: &Value) -> Option<String> {
    match value {
        Value::Null => None,
        Value::Bool(b) => Some(b.to_string()),
        Value::Number(n) => Some(n.to_string()),
        Value::String(s) => Some(s.to_string()),
        Value::Sequence(seq) => {
            let parts: Vec<String> = seq.iter().filter_map(display_string).collect();
            if parts.is_empty() {
                None
            } else {
                Some(parts.join(", "))
            }
        }
        _ => None,
    }
}

fn compare_numbers(a: &serde_yaml::Number, b: &serde_yaml::Number) -> Ordering {
    match (a.as_f64(), b.as_f64()) {
        (Some(x), Some(y)) => x.partial_cmp(&y).unwrap_or(Ordering::Equal),
        _ => a.to_string().cmp(&b.to_string()),
    }
}

fn compare_sequences(a: &[Value], b: &[Value]) -> Ordering {
    a.len().cmp(&b.len()).then_with(|| {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| compare_values(x, y))
            .find(|o| *o != Ordering::Equal)
            .unwrap_or(Ordering::Equal)
    })
}

fn value_rank(v: &Value) -> u8 {
    match v {
        Value::Null => 0,
        Value::Bool(_) => 1,
        Value::Number(_) => 2,
        Value::String(_) => 3,
        Value::Sequence(_) => 4,
        Value::Mapping(_) => 5,
        _ => 6,
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

#[cfg(test)]
mod value_tests {
    use super::*;
    use std::cmp::Ordering;

    fn mapping(yaml: &str) -> Value {
        serde_yaml::from_str(yaml).unwrap()
    }

    #[test]
    fn get_field_reads_any_key() {
        let fm = mapping("priority: high\ncustom: x\n");
        assert_eq!(
            get_field(&fm, "priority").and_then(|v| v.as_str()),
            Some("high")
        );
        assert_eq!(get_field(&fm, "custom").and_then(|v| v.as_str()), Some("x"));
        assert!(get_field(&fm, "absent").is_none());
    }

    #[test]
    fn compares_scalars_and_numbers() {
        let alpha = get_field(&mapping("v: Alpha\n"), "v").unwrap().clone();
        let beta = get_field(&mapping("v: beta\n"), "v").unwrap().clone();
        assert_eq!(compare_values(&alpha, &beta), Ordering::Less);
        assert_eq!(compare_values(&beta, &alpha), Ordering::Greater);
        assert_eq!(compare_values(&alpha, &alpha), Ordering::Equal);

        let ten = Value::from(10u64);
        let two = Value::from(2u64);
        assert_eq!(compare_values(&ten, &two), Ordering::Greater);
        assert_eq!(
            compare_values(&Value::Bool(false), &Value::Bool(true)),
            Ordering::Less
        );
        assert_eq!(
            compare_values(&Value::Null, &Value::Bool(true)),
            Ordering::Less
        );
    }

    #[test]
    fn numeric_compare_treats_int_and_float_equal() {
        let two = Value::from(2u64);
        let two_f = get_field(&mapping("v: 2.0\n"), "v").unwrap().clone();
        assert_eq!(compare_values(&two, &two_f), Ordering::Equal);
    }

    #[test]
    fn values_match_scalars_and_lists() {
        let tags = get_field(&mapping("tags: [one, two]\n"), "tags")
            .unwrap()
            .clone();
        assert!(values_match(&tags, "one"));
        assert!(values_match(&tags, "TWO"));
        assert!(!values_match(&tags, "three"));

        let s = get_field(&mapping("type: Metric\n"), "type")
            .unwrap()
            .clone();
        assert!(values_match(&s, "metric"));
        assert!(!values_match(&s, "reference"));

        let n = get_field(&mapping("n: 5\n"), "n").unwrap().clone();
        assert!(values_match(&n, "5"));
        assert!(!values_match(&n, "6"));

        let m = mapping("nested:\n  a: b\n");
        assert!(!values_match(&m, "anything"));
    }

    #[test]
    fn keys_lists_sorted_distinct_top_level_fields() {
        let fm = mapping("type: Metric\ntags: [a]\npriority: high\n");
        assert_eq!(
            keys(&fm),
            vec![
                "priority".to_string(),
                "tags".to_string(),
                "type".to_string()
            ]
        );
    }

    #[test]
    fn extra_fields_excludes_modeled_and_omits_unrenderable() {
        let fm = mapping(
            "type: ChangeRequest\n\
             state: proposed\n\
             priority: high\n\
             owner: human:felix\n\
             aliases: [a, b]\n\
             nested:\n  x: y\n\
             empty: ~\n",
        );
        let extra = extra_fields(&fm);
        assert_eq!(
            extra.keys().map(|s| s.as_str()).collect::<Vec<_>>(),
            vec!["aliases", "owner", "priority", "state"]
        );
        assert_eq!(extra.get("state").map(|s| s.as_str()), Some("proposed"));
        assert_eq!(extra.get("priority").map(|s| s.as_str()), Some("high"));
        assert_eq!(extra.get("owner").map(|s| s.as_str()), Some("human:felix"));
        assert_eq!(extra.get("aliases").map(|s| s.as_str()), Some("a, b"));
        assert!(!extra.contains_key("type"));
        assert!(!extra.contains_key("nested"));
        assert!(!extra.contains_key("empty"));
    }
}
