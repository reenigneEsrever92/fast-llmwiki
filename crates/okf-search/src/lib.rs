//! Semantic search service for OKF bundles.
//!
//! `okf-search` indexes every concept in a bundle — title, description, tags,
//! and body — into vector embeddings with a local model, then serves
//! relevance-ranked search over HTTP. It reuses [`okf_storage::FsBundle`] for
//! reads and change events, and rebuilds its index when the bundle changes.
//!
//! Logging is left to the caller so a process hosting multiple components can
//! initialize it exactly once.

pub mod api;
pub mod embed;
pub mod index;

use std::path::PathBuf;

use okf_storage::FsBundle;

/// Open the bundle at `data` and serve semantic search on `bind`.
///
/// This loads the embedding model (downloading weights on first use), indexes
/// the whole bundle, binds the listener, and serves until the future is dropped
/// or an error occurs. On a bind failure the returned error names the address
/// that could not be bound.
pub async fn serve(data: PathBuf, bind: String) -> anyhow::Result<()> {
    let bundle = FsBundle::open(&data).await?;
    api::init_bundle(bundle).await?;

    let app = api::router();

    let listener = tokio::net::TcpListener::bind(&bind)
        .await
        .map_err(|e| anyhow::anyhow!("failed to bind {bind}: {e}"))?;
    tracing::info!("OKF semantic search API on http://{bind}");
    axum::serve(listener, app.into_make_service())
        .await
        .map_err(|e| anyhow::anyhow!("server error: {e}"))?;

    Ok(())
}
