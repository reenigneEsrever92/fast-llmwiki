//! Serialization DTOs shared between the REST API and the frontend.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::concept::{Concept, ConceptSummary, Generated, Source, Status, TrustTier, Verification};
use crate::render::render_markdown;

/// A concept, with derived trust/staleness and its body rendered to HTML.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptResponse {
    pub id: String,
    pub concept_type: String,
    pub title: String,
    pub description: Option<String>,
    pub resource: Option<String>,
    pub tags: Vec<String>,
    pub status: Status,
    pub trust_tier: TrustTier,
    pub stale: bool,
    pub stale_after: Option<NaiveDate>,
    pub generated: Option<Generated>,
    pub verified: Vec<Verification>,
    pub sources: Vec<Source>,
    pub content_html: String,
}

impl ConceptResponse {
    pub fn from_concept(concept: &Concept, today: NaiveDate) -> Self {
        ConceptResponse {
            id: concept.id.clone(),
            concept_type: concept.concept_type.clone(),
            title: concept.title.clone(),
            description: concept.description.clone(),
            resource: concept.resource.clone(),
            tags: concept.tags.clone(),
            status: concept.status,
            trust_tier: concept.trust_tier(),
            stale: concept.is_stale(today),
            stale_after: concept.stale_after,
            generated: concept.generated.clone(),
            verified: concept.verified.clone(),
            sources: concept.sources.clone(),
            content_html: render_markdown(&concept.content, concept.dir()),
        }
    }
}

/// A concept summary for listings and search results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptSummaryResponse {
    pub id: String,
    pub title: String,
    pub concept_type: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub status: Status,
    pub trust_tier: TrustTier,
    pub stale: bool,
    pub stale_after: Option<NaiveDate>,
}

impl ConceptSummaryResponse {
    pub fn from_summary(summary: &ConceptSummary, today: NaiveDate) -> Self {
        ConceptSummaryResponse {
            id: summary.id.clone(),
            title: summary.title.clone(),
            concept_type: summary.concept_type.clone(),
            description: summary.description.clone(),
            tags: summary.tags.clone(),
            status: summary.status,
            trust_tier: summary.trust_tier,
            stale: summary.stale_after.map(|d| today >= d).unwrap_or(false),
            stale_after: summary.stale_after,
        }
    }
}

/// A relevance-ranked semantic search result: a concept summary plus a score.
///
/// The summary fields are flattened so the JSON shape matches a plain concept
/// summary with one extra `score` field (cosine similarity in `[-1, 1]`, higher
/// is more relevant).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultResponse {
    #[serde(flatten)]
    pub summary: ConceptSummaryResponse,
    /// Cosine similarity between the query and the concept embedding.
    pub score: f32,
}

impl SearchResultResponse {
    pub fn from_summary(summary: &ConceptSummary, today: NaiveDate, score: f32) -> Self {
        SearchResultResponse {
            summary: ConceptSummaryResponse::from_summary(summary, today),
            score,
        }
    }
}

/// A directory listing, with `index.md`/`log.md` rendered to HTML.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirListingResponse {
    pub path: String,
    pub index_html: Option<String>,
    pub log_html: Option<String>,
    pub concepts: Vec<ConceptSummaryResponse>,
    pub subdirs: Vec<String>,
}

/// A recursive directory tree node returned by `GET /api/tree`.
///
/// The root node has an empty `path`; every other node describes one directory
/// in the bundle together with the concepts it directly contains and its
/// nested subdirectories.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TreeNodeResponse {
    /// Directory segment name (empty for the bundle root).
    pub name: String,
    /// Directory path relative to the bundle root (`""` for the root).
    pub path: String,
    /// Concepts directly inside this directory.
    pub concepts: Vec<ConceptSummaryResponse>,
    /// Nested subdirectories.
    pub children: Vec<TreeNodeResponse>,
}
