//! REST API server for OKF bundles.
//!
//! The server composes an [`api::router`] with a filesystem bundle and serves it
//! over HTTP. Logging is left to the caller so that a process hosting multiple
//! components can initialize it exactly once.

pub mod api;

use std::path::PathBuf;

use fawi_storage::FsBundle;

/// Open the bundle at `data` and serve the REST API on `bind`.
///
/// This binds the listener and serves until the future is dropped or an error
/// occurs. On a bind failure the returned error names the address that could
/// not be bound.
pub async fn serve(data: PathBuf, bind: String) -> anyhow::Result<()> {
    let bundle = FsBundle::open(&data).await?;
    api::init_bundle(bundle);

    let app = api::router();

    let listener = tokio::net::TcpListener::bind(&bind)
        .await
        .map_err(|e| anyhow::anyhow!("failed to bind {bind}: {e}"))?;
    tracing::info!("OKF REST API on http://{bind}");
    axum::serve(listener, app.into_make_service())
        .await
        .map_err(|e| anyhow::anyhow!("server error: {e}"))?;

    Ok(())
}
