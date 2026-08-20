//! The GUI's own axum server: the Leptos SSR integration.

use axum::routing::get;
use axum::Router;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};

use crate::assets::{client_js, client_wasm};

/// Build the axum router that serves the Leptos app (SSR + the embedded client bundle).
///
/// The returned router has already been given its `LeptosOptions` state, so it
/// is a `Router<()>` that can be merged with other routers and served directly.
pub fn router() -> Router {
    let leptos_options =
        get_configuration(Some(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml")))
            .expect("failed to load Leptos configuration from Cargo.toml")
            .leptos_options;
    let routes = generate_route_list(crate::app::App);

    Router::new()
        .route("/pkg/okf.js", get(client_js))
        .route("/pkg/okf_bg.wasm", get(client_wasm))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || crate::app::shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(crate::app::shell))
        .with_state(leptos_options)
}

/// Serve the web UI on `bind`, querying the REST API at `api_base_url`.
///
/// This sets the SSR API base URL, binds the listener, and serves until the
/// future is dropped or an error occurs. On a bind failure the returned error
/// names the address that could not be bound.
pub async fn serve(api_base_url: String, bind: String) -> anyhow::Result<()> {
    crate::api_client::set_api_base_url(api_base_url.clone());

    let app = router();

    let listener = tokio::net::TcpListener::bind(&bind)
        .await
        .map_err(|e| anyhow::anyhow!("failed to bind {bind}: {e}"))?;
    tracing::info!("OKF web UI on http://{bind} (API: {api_base_url})");
    axum::serve(listener, app)
        .await
        .map_err(|e| anyhow::anyhow!("server error: {e}"))?;

    Ok(())
}
