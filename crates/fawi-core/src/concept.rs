use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::front_matter::split_front_matter;

/// Lifecycle status of a concept.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Draft,
    #[default]
    Stable,
    Deprecated,
}

impl Status {
    pub fn as_str(self) -> &'static str {
        match self {
            Status::Draft => "draft",
            Status::Stable => "stable",
            Status::Deprecated => "deprecated",
        }
    }
}

/// Trust tier derived from a concept's `verified` field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TrustTier {
    Unverified,
    MachineConfirmed,
    HumanReviewed,
}

impl TrustTier {
    pub fn as_str(self) -> &'static str {
        match self {
            TrustTier::Unverified => "unverified",
            TrustTier::MachineConfirmed => "machine-confirmed",
            TrustTier::HumanReviewed => "human-reviewed",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Generated {
    pub by: String,
    pub at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verification {
    pub by: String,
    pub at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub id: Option<String>,
    pub resource: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub usage_count: Option<u64>,
    pub last_modified: Option<NaiveDate>,
}

/// A single knowledge document in a bundle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concept {
    /// Concept ID: the file path within the bundle with the `.md` suffix removed.
    pub id: String,
    /// The required `type` front matter field.
    pub concept_type: String,
    pub title: String,
    pub description: Option<String>,
    pub resource: Option<String>,
    pub tags: Vec<String>,
    pub status: Status,
    pub generated: Option<Generated>,
    pub verified: Vec<Verification>,
    pub stale_after: Option<NaiveDate>,
    pub sources: Vec<Source>,
    /// The markdown body (without front matter).
    pub content: String,
}

impl Concept {
    pub fn from_markdown(id: &str, markdown: &str) -> Self {
        let (yaml, body) = split_front_matter(markdown);
        let meta = yaml.as_deref().and_then(parse_meta).unwrap_or_default();
        let title = meta
            .title
            .filter(|t| !t.trim().is_empty())
            .unwrap_or_else(|| title_from_id(id));

        Concept {
            id: id.to_string(),
            concept_type: meta.concept_type,
            title,
            description: meta.description,
            resource: meta.resource,
            tags: meta.tags,
            status: meta.status,
            generated: meta.generated,
            verified: meta.verified,
            stale_after: meta.stale_after,
            sources: meta.sources,
            content: body,
        }
    }

    /// The derived trust tier (§5.3 of the spec).
    pub fn trust_tier(&self) -> TrustTier {
        if self.verified.is_empty() {
            TrustTier::Unverified
        } else if self.verified.iter().any(|v| v.by.starts_with("human:")) {
            TrustTier::HumanReviewed
        } else {
            TrustTier::MachineConfirmed
        }
    }

    /// Whether the concept is stale on `today` (§5.5 of the spec).
    pub fn is_stale(&self, today: NaiveDate) -> bool {
        self.stale_after.map(|d| today >= d).unwrap_or(false)
    }

    /// The parent directory of this concept (empty string for the bundle root).
    pub fn dir(&self) -> &str {
        match self.id.rfind('/') {
            Some(i) => &self.id[..i],
            None => "",
        }
    }

    pub fn summary(&self) -> ConceptSummary {
        ConceptSummary {
            id: self.id.clone(),
            title: self.title.clone(),
            concept_type: self.concept_type.clone(),
            description: self.description.clone(),
            tags: self.tags.clone(),
            status: self.status,
            trust_tier: self.trust_tier(),
            stale_after: self.stale_after,
        }
    }
}

/// Lightweight view of a concept for listings and search results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptSummary {
    pub id: String,
    pub title: String,
    pub concept_type: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub status: Status,
    pub trust_tier: TrustTier,
    pub stale_after: Option<NaiveDate>,
}

#[derive(Debug, Default)]
struct Meta {
    concept_type: String,
    title: Option<String>,
    description: Option<String>,
    resource: Option<String>,
    tags: Vec<String>,
    status: Status,
    generated: Option<Generated>,
    verified: Vec<Verification>,
    stale_after: Option<NaiveDate>,
    sources: Vec<Source>,
}

fn parse_meta(yaml: &str) -> Option<Meta> {
    let value: Value = serde_yaml::from_str(yaml).ok()?;
    let map = value.as_mapping()?;

    let mut meta = Meta::default();
    meta.concept_type = get_str(map, "type").unwrap_or_default();
    meta.title = get_str(map, "title");
    meta.description = get_str(map, "description");
    meta.resource = get_str(map, "resource");
    meta.tags = get_str_list(map, "tags");
    meta.status = get_str(map, "status")
        .map(|s| parse_status(&s))
        .unwrap_or(Status::Stable);
    meta.generated = get_map(map, "generated").map(parse_generated);
    meta.verified = parse_verified(mapping_get(map, "verified"));
    meta.stale_after = get_str(map, "stale_after").and_then(|s| parse_date(&s));
    meta.sources = parse_sources(mapping_get(map, "sources"));
    Some(meta)
}

fn parse_status(s: &str) -> Status {
    match s.trim().to_ascii_lowercase().as_str() {
        "draft" => Status::Draft,
        "deprecated" => Status::Deprecated,
        _ => Status::Stable,
    }
}

fn mapping_get<'a>(map: &'a serde_yaml::Mapping, key: &str) -> Option<&'a Value> {
    map.iter()
        .find(|(k, _)| k.as_str() == Some(key))
        .map(|(_, v)| v)
}

fn get_str(map: &serde_yaml::Mapping, key: &str) -> Option<String> {
    mapping_get(map, key)
        .and_then(|v| v.as_str())
        .map(String::from)
}

fn get_map<'a>(map: &'a serde_yaml::Mapping, key: &str) -> Option<&'a serde_yaml::Mapping> {
    mapping_get(map, key).and_then(|v| v.as_mapping())
}

fn get_u64(map: &serde_yaml::Mapping, key: &str) -> Option<u64> {
    match mapping_get(map, key) {
        Some(v) => {
            if let Some(n) = v.as_u64() {
                return Some(n);
            }
            v.as_str().and_then(|s| s.trim().parse().ok())
        }
        None => None,
    }
}

fn get_str_list(map: &serde_yaml::Mapping, key: &str) -> Vec<String> {
    match mapping_get(map, key) {
        Some(v) => value_to_string_list(v),
        None => Vec::new(),
    }
}

fn value_to_string_list(v: &Value) -> Vec<String> {
    if let Some(seq) = v.as_sequence() {
        return seq
            .iter()
            .filter_map(|item| item.as_str())
            .map(String::from)
            .collect();
    }
    if let Some(s) = v.as_str() {
        return s
            .split(',')
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
            .collect();
    }
    Vec::new()
}

fn parse_generated(m: &serde_yaml::Mapping) -> Generated {
    Generated {
        by: get_str(m, "by").unwrap_or_default(),
        at: get_str(m, "at").and_then(|s| parse_datetime(&s)),
    }
}

fn parse_verified(v: Option<&Value>) -> Vec<Verification> {
    let Some(v) = v else { return Vec::new(); };

    if let Some(m) = v.as_mapping() {
        return vec![parse_verification(m)];
    }
    if let Some(seq) = v.as_sequence() {
        return seq
            .iter()
            .filter_map(|item| item.as_mapping())
            .map(parse_verification)
            .collect();
    }
    Vec::new()
}

fn parse_verification(m: &serde_yaml::Mapping) -> Verification {
    Verification {
        by: get_str(m, "by").unwrap_or_default(),
        at: get_str(m, "at").and_then(|s| parse_datetime(&s)),
    }
}

fn parse_sources(v: Option<&Value>) -> Vec<Source> {
    let Some(v) = v else { return Vec::new(); };
    let Some(seq) = v.as_sequence() else { return Vec::new(); };
    seq.iter()
        .filter_map(|item| item.as_mapping())
        .map(parse_source)
        .collect()
}

fn parse_source(m: &serde_yaml::Mapping) -> Source {
    Source {
        id: get_str(m, "id"),
        resource: get_str(m, "resource"),
        title: get_str(m, "title"),
        author: get_str(m, "author"),
        usage_count: get_u64(m, "usage_count"),
        last_modified: get_str(m, "last_modified").and_then(|s| parse_date(&s)),
    }
}

fn parse_date(s: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(s.trim(), "%Y-%m-%d").ok()
}

fn parse_datetime(s: &str) -> Option<DateTime<Utc>> {
    let s = s.trim();
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Some(dt.with_timezone(&Utc));
    }
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .ok()
        .and_then(|d| d.and_hms_opt(0, 0, 0))
        .map(|nd| nd.and_utc())
}

fn title_from_id(id: &str) -> String {
    let last = id.rsplit('/').next().unwrap_or(id);
    let humanized = last.replace('-', " ").replace('_', " ");
    let mut chars = humanized.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_front_matter_families() {
        let md = "---\ntype: Metric\ntitle: Revenue\nstatus: stable\n\
                  generated: { by: agent/x, at: 2026-06-20T22:53:05Z }\n\
                  verified: [{ by: human:alice, at: 2026-06-25T09:00:00Z }]\n\
                  stale_after: 2026-12-31\ntags: [finance, revenue]\n\
                  sources:\n  - id: rev\n    resource: https://example.com\n    usage_count: 5000\n    last_modified: 2026-04-02\n\
                  ---\n# Body\n";
        let c = Concept::from_markdown("metrics/revenue", md);

        assert_eq!(c.concept_type, "Metric");
        assert_eq!(c.title, "Revenue");
        assert_eq!(c.status, Status::Stable);
        assert_eq!(c.tags, vec!["finance", "revenue"]);
        assert_eq!(c.trust_tier(), TrustTier::HumanReviewed);
        assert_eq!(c.stale_after, Some(NaiveDate::from_ymd_opt(2026, 12, 31).unwrap()));
        assert_eq!(c.sources.len(), 1);
        assert_eq!(c.sources[0].usage_count, Some(5000));
        assert_eq!(c.sources[0].last_modified, Some(NaiveDate::from_ymd_opt(2026, 4, 2).unwrap()));
        assert_eq!(c.content, "# Body\n");
    }

    #[test]
    fn trust_tiers() {
        let base = Concept::from_markdown("x", "---\ntype: T\n---\nbody\n");
        assert_eq!(base.trust_tier(), TrustTier::Unverified);

        let machine = Concept::from_markdown(
            "x",
            "---\ntype: T\nverified: [{ by: process:n, at: 2026-01-01T00:00:00Z }]\n---\nbody\n",
        );
        assert_eq!(machine.trust_tier(), TrustTier::MachineConfirmed);

        let human = Concept::from_markdown(
            "x",
            "---\ntype: T\nverified: { by: human:a, at: 2026-01-01T00:00:00Z }\n---\nbody\n",
        );
        assert_eq!(human.trust_tier(), TrustTier::HumanReviewed);
    }
}
