//! HTTP endpoint for semantic search.

use std::sync::{Arc, OnceLock};

use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use fawi_core::dto::SearchResultResponse;
use fawi_storage::FsBundle;
use serde::Deserialize;

use crate::embed::{Embedder, FastembedEmbedder};
use crate::index::SemanticIndex;

#[derive(Debug, Deserialize)]
struct SearchQuery {
    q: Option<String>,
}

struct SearchState {
    bundle: Arc<FsBundle>,
    embedder: Arc<dyn Embedder>,
    index: tokio::sync::RwLock<SemanticIndex>,
}

static STATE: OnceLock<Arc<SearchState>> = OnceLock::new();

/// Index the bundle at startup and begin rebuilding the index on changes.
///
/// This loads the local embedding model (downloading it on first use) and
/// embeds every concept before returning, so a failure here prevents the
/// service from starting with an empty index.
pub async fn init_bundle(bundle: Arc<FsBundle>) -> anyhow::Result<()> {
    let embedder: Arc<dyn Embedder> = Arc::new(FastembedEmbedder::try_default()?);
    let index = build_index(&bundle, &embedder).await?;

    let state = Arc::new(SearchState {
        bundle,
        embedder,
        index: tokio::sync::RwLock::new(index),
    });
    let _ = STATE.set(state.clone());

    spawn_reindexer(state);
    Ok(())
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

pub fn router() -> Router {
    Router::new().route("/api/search/semantic", get(search))
}

fn state() -> Arc<SearchState> {
    STATE.get().cloned().expect("search state not initialized")
}

async fn search(Query(query): Query<SearchQuery>) -> Response {
    let q = query.q.unwrap_or_default();
    if q.trim().is_empty() {
        return Json(Vec::<SearchResultResponse>::new()).into_response();
    }

    let state = state();
    let index = state.index.read().await;
    match index.search(&q, &state.embedder).await {
        Ok(results) => {
            let today = chrono::Utc::now().date_naive();
            let out: Vec<SearchResultResponse> = results
                .into_iter()
                .map(|(summary, score)| SearchResultResponse::from_summary(&summary, today, score))
                .collect();
            Json(out).into_response()
        }
        Err(e) => {
            tracing::warn!("semantic search failed: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
