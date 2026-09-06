//! Search-provider port and the bundled providers.
//!
//! [`SearchProvider`] is the pluggable search capability: anything that can rank
//! a bundle against a query implements it, and [`HybridSearch`] fuses the ranked
//! lists of any number of providers with reciprocal rank fusion. The search
//! endpoint in `fawi-server` depends only on this port, and the composition root
//! (`fawi-cli`) decides which providers back it.

use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use fawi_core::{Concept, ConceptSummary};

use crate::FsBundle;

/// A ranked search result: a concept summary plus its score.
///
/// Scores are provider-specific (a keyword relevance weight, a cosine
/// similarity, ...) and are only meaningful within one provider's list;
/// [`HybridSearch`] deliberately does not compare them across providers.
#[derive(Debug, Clone)]
pub struct ScoredSummary {
    pub summary: ConceptSummary,
    pub score: f32,
}

/// A search strategy that ranks the bundle's concepts against a query.
///
/// Implementors must return results best-first and must return an empty list
/// for a blank query.
#[async_trait]
pub trait SearchProvider: Send + Sync {
    /// Rank the bundle's concepts against `query`, best first.
    async fn search(&self, query: &str) -> Vec<ScoredSummary>;
}

/// Keyword search over the bundle: substring matches scored by which field they
/// hit, so a title match outranks a body-only match.
pub struct KeywordProvider {
    bundle: Arc<FsBundle>,
}

impl KeywordProvider {
    pub fn new(bundle: Arc<FsBundle>) -> Self {
        Self { bundle }
    }
}

#[async_trait]
impl SearchProvider for KeywordProvider {
    async fn search(&self, query: &str) -> Vec<ScoredSummary> {
        let concepts = self.bundle.concepts().await;
        rank_keyword(query, &concepts)
    }
}

/// Rank concepts by keyword relevance: substring matches weighted by field.
///
/// Pure (no I/O) so it is unit-testable without a bundle. Blank queries return
/// no results. Ties fall back to title order, then id order.
pub(crate) fn rank_keyword(query: &str, concepts: &[Concept]) -> Vec<ScoredSummary> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return Vec::new();
    }

    let mut scored: Vec<ScoredSummary> = concepts
        .iter()
        .filter_map(|c| {
            score_keyword(&q, c).map(|score| ScoredSummary {
                summary: c.summary(),
                score,
            })
        })
        .collect();

    scored.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(Ordering::Equal)
            .then_with(|| {
                a.summary
                    .title
                    .to_lowercase()
                    .cmp(&b.summary.title.to_lowercase())
            })
            .then_with(|| a.summary.id.cmp(&b.summary.id))
    });
    scored
}

/// Field weights for keyword matching; a match earns the field's weight and
/// multiple matching fields sum. Title and id outrank tags and type, which
/// outrank description and body.
const TITLE_WEIGHT: f32 = 4.0;
const ID_WEIGHT: f32 = 3.0;
const TYPE_WEIGHT: f32 = 2.0;
const TAGS_WEIGHT: f32 = 2.0;
const DESCRIPTION_WEIGHT: f32 = 1.0;
const BODY_WEIGHT: f32 = 1.0;

fn score_keyword(q: &str, concept: &Concept) -> Option<f32> {
    let mut score = 0.0f32;
    if concept.id.to_lowercase().contains(q) {
        score += ID_WEIGHT;
    }
    if concept.title.to_lowercase().contains(q) {
        score += TITLE_WEIGHT;
    }
    if concept.concept_type.to_lowercase().contains(q) {
        score += TYPE_WEIGHT;
    }
    if concept.tags.iter().any(|t| t.to_lowercase().contains(q)) {
        score += TAGS_WEIGHT;
    }
    if concept
        .description
        .as_deref()
        .map(|d| d.to_lowercase().contains(q))
        .unwrap_or(false)
    {
        score += DESCRIPTION_WEIGHT;
    }
    if concept.content.to_lowercase().contains(q) {
        score += BODY_WEIGHT;
    }
    (score > 0.0).then_some(score)
}

/// Search over many providers, fusing their ranked lists with reciprocal rank
/// fusion: `score(doc) = Σ_providers 1 / (k + rank)`, deduplicated by concept
/// id. This needs only each provider's list to be internally well-ordered, not
/// comparable scores across providers.
pub struct HybridSearch {
    providers: Vec<Arc<dyn SearchProvider>>,
}

impl HybridSearch {
    pub fn new(providers: Vec<Arc<dyn SearchProvider>>) -> Self {
        Self { providers }
    }
}

#[async_trait]
impl SearchProvider for HybridSearch {
    async fn search(&self, query: &str) -> Vec<ScoredSummary> {
        if query.trim().is_empty() {
            return Vec::new();
        }
        let mut lists = Vec::with_capacity(self.providers.len());
        for provider in &self.providers {
            lists.push(provider.search(query).await);
        }
        fuse_rrf(&lists, 60.0)
    }
}

/// Reciprocal rank fusion over `lists`, deduplicating by concept id and sorting
/// by fused score descending (ties by id for determinism).
pub(crate) fn fuse_rrf(lists: &[Vec<ScoredSummary>], k: f32) -> Vec<ScoredSummary> {
    let mut fused: HashMap<String, (f32, ConceptSummary)> = HashMap::new();
    for list in lists {
        for (rank, item) in list.iter().enumerate() {
            let entry = fused
                .entry(item.summary.id.clone())
                .or_insert_with(|| (0.0, item.summary.clone()));
            entry.0 += 1.0 / (k + (rank as f32) + 1.0);
        }
    }

    let mut out: Vec<ScoredSummary> = fused
        .into_iter()
        .map(|(_, (score, summary))| ScoredSummary { summary, score })
        .collect();
    out.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(Ordering::Equal)
            .then_with(|| a.summary.id.cmp(&b.summary.id))
    });
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn concept(id: &str, title: &str, body: &str) -> Concept {
        Concept::from_markdown(
            id,
            &format!("---\ntype: Note\ntitle: {title}\n---\n{body}\n"),
        )
    }

    fn ids(results: &[ScoredSummary]) -> Vec<String> {
        results.iter().map(|r| r.summary.id.clone()).collect()
    }

    #[test]
    fn blank_query_returns_no_results() {
        assert!(rank_keyword("   ", &[concept("a", "A", "body")]).is_empty());
    }

    #[test]
    fn body_only_match_is_returned() {
        let results = rank_keyword(
            "income",
            &[concept(
                "metrics/revenue",
                "Financial Summary",
                "annual income grew",
            )],
        );
        assert_eq!(ids(&results), vec!["metrics/revenue".to_string()]);
    }

    #[test]
    fn title_match_ranks_above_body_only_match() {
        let concepts = vec![
            concept("body-only", "Financial Summary", "annual income grew"),
            concept("title-hit", "Income", "unrelated"),
        ];
        let results = rank_keyword("income", &concepts);
        assert_eq!(
            ids(&results),
            vec!["title-hit".to_string(), "body-only".to_string()]
        );
    }

    #[test]
    fn type_and_tags_rank_above_description_and_body() {
        let typed = Concept::from_markdown(
            "typed",
            "---\ntype: Income\ntitle: Server\ntags: [income]\n---\nunrelated\n",
        );
        let concepts = vec![
            concept("meta", "Financial Summary", "annual income grew"),
            typed,
        ];
        let results = rank_keyword("income", &concepts);
        assert_eq!(ids(&results), vec!["typed".to_string(), "meta".to_string()]);
    }

    #[test]
    fn equal_scores_fall_back_to_title_order() {
        let concepts = vec![
            concept("b", "Beta", "annual income"),
            concept("a", "Alpha", "annual income"),
        ];
        let results = rank_keyword("income", &concepts);
        assert_eq!(ids(&results), vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn fuse_rrf_dedupes_and_orders_best_first() {
        let first = vec![
            ScoredSummary {
                summary: concept("a", "A", "").summary(),
                score: 1.0,
            },
            ScoredSummary {
                summary: concept("b", "B", "").summary(),
                score: 1.0,
            },
            ScoredSummary {
                summary: concept("c", "C", "").summary(),
                score: 1.0,
            },
        ];
        let second = vec![
            ScoredSummary {
                summary: concept("c", "C", "").summary(),
                score: 1.0,
            },
            ScoredSummary {
                summary: concept("d", "D", "").summary(),
                score: 1.0,
            },
        ];
        let fused = fuse_rrf(&[first, second], 60.0);
        // c is 1/61 + 1/62, above a (1/61) and b, d (1/62 each, tie -> id order).
        assert_eq!(
            ids(&fused),
            vec![
                "c".to_string(),
                "a".to_string(),
                "b".to_string(),
                "d".to_string()
            ]
        );
    }

    #[tokio::test]
    async fn hybrid_search_blank_query_is_empty() {
        let hybrid = HybridSearch::new(vec![]);
        assert!(hybrid.search("   ").await.is_empty());
    }
}
