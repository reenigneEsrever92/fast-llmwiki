//! In-memory semantic index over the bundle's concepts.

use std::sync::Arc;

use anyhow::Result;
use fawi_core::{Concept, ConceptSummary};

use crate::embed::Embedder;

/// A single ranked document reference and its similarity score.
#[derive(Debug, Clone, Copy)]
pub struct ScoredDoc {
    /// Index into the document list backing the index.
    pub id: usize,
    /// Cosine similarity in `[-1, 1]`.
    pub score: f32,
}

/// A pluggable vector index.
///
/// The brute-force implementation below is sufficient for a local bundle. When
/// the bundle grows large, this can be swapped for an approximate nearest
/// neighbor index (e.g. HNSW) without changing the search caller.
pub trait VectorIndex: Send + Sync {
    /// Rank documents against `query` and return the top `limit` by descending
    /// similarity.
    fn search(&self, query: &[f32], limit: usize) -> Vec<ScoredDoc>;
}

/// Exact brute-force cosine similarity over in-memory vectors.
pub struct BruteForceIndex {
    embeddings: Vec<Vec<f32>>,
}

impl BruteForceIndex {
    pub fn new(embeddings: Vec<Vec<f32>>) -> Self {
        Self { embeddings }
    }
}

impl VectorIndex for BruteForceIndex {
    fn search(&self, query: &[f32], limit: usize) -> Vec<ScoredDoc> {
        let mut scored: Vec<ScoredDoc> = self
            .embeddings
            .iter()
            .enumerate()
            .map(|(id, emb)| ScoredDoc {
                id,
                score: cosine(query, emb),
            })
            .collect();
        scored.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        scored.truncate(limit);
        scored
    }
}

fn cosine(a: &[f32], b: &[f32]) -> f32 {
    let (dot, na, nb) = a.iter().zip(b.iter()).fold((0.0f32, 0.0f32, 0.0f32), |(dot, na, nb), (x, y)| {
        (dot + x * y, na + x * x, nb + y * y)
    });
    let denom = na.sqrt() * nb.sqrt();
    if denom == 0.0 {
        0.0
    } else {
        dot / denom
    }
}

/// A searchable index over concept summaries.
///
/// The concept list and its embeddings are kept in lockstep: the `id` returned
/// by the [`VectorIndex`] indexes into `docs`.
pub struct SemanticIndex {
    docs: Vec<ConceptSummary>,
    index: Arc<dyn VectorIndex>,
}

impl SemanticIndex {
    /// Build an index over `concepts`, embedding each with `embedder`.
    pub async fn build(concepts: &[Concept], embedder: &Arc<dyn Embedder>) -> Result<Self> {
        let texts: Vec<String> = concepts.iter().map(concept_text).collect();
        let embeddings = embed_batch(embedder, &texts).await?;
        let docs = concepts.iter().map(Concept::summary).collect();
        Ok(Self {
            docs,
            index: Arc::new(BruteForceIndex::new(embeddings)),
        })
    }

    /// Rank all concepts by similarity to `query`, returning summaries with
    /// their scores in descending relevance order. An empty query returns no
    /// results.
    pub async fn search(&self, query: &str, embedder: &Arc<dyn Embedder>) -> Result<Vec<(ConceptSummary, f32)>> {
        if query.trim().is_empty() {
            return Ok(Vec::new());
        }

        let query_embedding = embed_batch(embedder, &[query.to_string()]).await?;
        let Some(query_vec) = query_embedding.into_iter().next() else {
            return Ok(Vec::new());
        };

        Ok(self
            .index
            .search(&query_vec, self.docs.len())
            .into_iter()
            .map(|d| (self.docs[d.id].clone(), d.score))
            .collect())
    }
}

async fn embed_batch(embedder: &Arc<dyn Embedder>, texts: &[String]) -> Result<Vec<Vec<f32>>> {
    let embedder = embedder.clone();
    let texts = texts.to_vec();
    tokio::task::spawn_blocking(move || embedder.embed(&texts))
        .await
        .map_err(|e| anyhow::anyhow!("embedding task failed: {e}"))?
}

/// The text indexed for a concept: everything searchable, including its body.
fn concept_text(concept: &Concept) -> String {
    let mut parts = vec![concept.title.clone(), concept.concept_type.clone()];
    if let Some(description) = &concept.description {
        parts.push(description.clone());
    }
    parts.extend(concept.tags.iter().cloned());
    parts.push(concept.content.clone());
    parts.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::embed::Embedder;

    /// A deterministic mock embedder that maps text onto two axes:
    /// axis 0 = money/revenue/income, axis 1 = server/api.
    ///
    /// This gives us predictable "semantic" similarity without downloading a
    /// model, and lets a query match a concept only through its body.
    struct MockEmbedder;

    impl Embedder for MockEmbedder {
        fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
            Ok(texts
                .iter()
                .map(|t| {
                    let mut v = vec![0.0f32, 0.0f32];
                    let lower = t.to_lowercase();
                    if lower.contains("revenue") || lower.contains("income") || lower.contains("money") {
                        v[0] = 1.0;
                    }
                    if lower.contains("server") || lower.contains("api") {
                        v[1] = 1.0;
                    }
                    v
                })
                .collect())
        }
    }

    fn concept(id: &str, title: &str, body: &str) -> Concept {
        Concept::from_markdown(
            id,
            &format!("---\ntype: Note\ntitle: {title}\n---\n{body}\n"),
        )
    }

    fn embedder() -> Arc<dyn Embedder> {
        Arc::new(MockEmbedder)
    }

    #[tokio::test]
    async fn empty_query_returns_no_results() {
        let concepts = vec![concept("a", "Revenue", "money")];
        let index = SemanticIndex::build(&concepts, &embedder()).await.unwrap();
        let results = index.search("   ", &embedder()).await.unwrap();
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn body_only_match_is_returned_and_ranked_first() {
        // The money semantics live only in the body ("income"), not in the
        // title, type, description, or tags.
        let concepts = vec![
            concept("metrics/revenue", "Financial Summary", "annual income grew"),
            concept("api/server", "Server", "serves the api"),
        ];
        let index = SemanticIndex::build(&concepts, &embedder()).await.unwrap();
        let results = index.search("money", &embedder()).await.unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0.id, "metrics/revenue");
        assert!(results[0].1 > results[1].1);
    }

    #[tokio::test]
    async fn results_are_ranked_descending_by_score() {
        let concepts = vec![
            concept("api", "Server", "serves the api"),
            concept("income", "Income", "annual income"),
            concept("both", "Fintech", "money api"),
        ];
        let index = SemanticIndex::build(&concepts, &embedder()).await.unwrap();
        let results = index.search("money", &embedder()).await.unwrap();

        assert!(results.len() >= 3);
        let scores: Vec<f32> = results.iter().map(|(_, s)| *s).collect();
        for pair in scores.windows(2) {
            assert!(pair[0] >= pair[1], "scores must be descending: {scores:?}");
        }
    }

    #[tokio::test]
    async fn rebuilding_reflects_new_concepts() {
        let embedder = embedder();
        let before = vec![concept("old-income", "Old", "annual revenue")];
        let index = SemanticIndex::build(&before, &embedder).await.unwrap();
        let ids: Vec<String> = index
            .search("money", &embedder)
            .await
            .unwrap()
            .into_iter()
            .map(|(s, _)| s.id)
            .collect();
        assert_eq!(ids, vec!["old-income".to_string()]);

        // Simulate a bundle change: the concept set is replaced, and a rebuild
        // must reflect the new concept.
        let after = vec![concept("new-income", "New", "annual revenue")];
        let rebuilt = SemanticIndex::build(&after, &embedder).await.unwrap();
        let ids: Vec<String> = rebuilt
            .search("money", &embedder)
            .await
            .unwrap()
            .into_iter()
            .map(|(s, _)| s.id)
            .collect();
        assert_eq!(ids, vec!["new-income".to_string()]);
    }

    #[test]
    fn cosine_handles_zero_vectors() {
        assert_eq!(cosine(&[0.0, 0.0], &[0.0, 0.0]), 0.0);
        assert!((cosine(&[1.0, 0.0], &[1.0, 0.0]) - 1.0).abs() < f32::EPSILON);
        assert!((cosine(&[1.0, 0.0], &[0.0, 1.0]) - 0.0).abs() < f32::EPSILON);
    }
}
