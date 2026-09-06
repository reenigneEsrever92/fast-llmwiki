//! Semantic search provider: ranks a bundle's concepts by embedding similarity.
//!
//! [`init`] builds the in-memory index over the bundle and returns a
//! [`SearchProvider`] that embeds the query with the same model and ranks by
//! cosine similarity. The index is rebuilt whenever the bundle reports a change.

use std::sync::Arc;

use fawi_storage::{FsBundle, ScoredSummary, SearchProvider};
use tokio::sync::RwLock;

use crate::embed::{Embedder, FastembedEmbedder};
use crate::index::SemanticIndex;

struct SearchState {
    bundle: Arc<FsBundle>,
    embedder: Arc<dyn Embedder>,
    index: RwLock<SemanticIndex>,
}

/// A [`SearchProvider`] backed by the in-memory semantic index.
pub struct SemanticProvider {
    state: Arc<SearchState>,
}

/// Index the bundle at startup and begin rebuilding the index on changes.
///
/// This loads the local embedding model (downloading it on first use) and
/// embeds every concept before returning. A failure here prevents a search
/// service that requires semantic search from starting.
pub async fn init(bundle: Arc<FsBundle>) -> anyhow::Result<Arc<dyn SearchProvider>> {
    let embedder: Arc<dyn Embedder> = Arc::new(FastembedEmbedder::try_default()?);
    let index = build_index(&bundle, &embedder).await?;

    let state = Arc::new(SearchState {
        bundle,
        embedder,
        index: RwLock::new(index),
    });
    spawn_reindexer(state.clone());

    Ok(Arc::new(SemanticProvider { state }))
}

async fn build_index(
    bundle: &Arc<FsBundle>,
    embedder: &Arc<dyn Embedder>,
) -> anyhow::Result<SemanticIndex> {
    let concepts = bundle.concepts().await;
    SemanticIndex::build(&concepts, embedder).await
}

/// Rebuild the index whenever the bundle reports a filesystem change.
fn spawn_reindexer(state: Arc<SearchState>) {
    tokio::spawn(async move {
        let mut rx = state.bundle.subscribe();
        while rx.recv().await.is_ok() {
            match build_index(&state.bundle, &state.embedder).await {
                Ok(index) => {
                    *state.index.write().await = index;
                    tracing::info!("semantic index rebuilt after bundle change");
                }
                Err(e) => tracing::warn!("semantic index rebuild failed: {e}"),
            }
        }
    });
}

#[async_trait::async_trait]
impl SearchProvider for SemanticProvider {
    async fn search(&self, query: &str) -> Vec<ScoredSummary> {
        if query.trim().is_empty() {
            return Vec::new();
        }

        let index = self.state.index.read().await;
        match index.search(query, &self.state.embedder).await {
            Ok(results) => results
                .into_iter()
                .map(|(summary, score)| ScoredSummary { summary, score })
                .collect(),
            Err(e) => {
                tracing::warn!("semantic search failed: {e}");
                Vec::new()
            }
        }
    }
}
