//! The client (WASM hydration) bundle, embedded into the server binary so the
//! web UI can be distributed as a single executable.
//!
//! These bytes are produced by `build.rs` into `OUT_DIR` and then baked in at
//! compile time.

use axum::http::header;
use axum::response::IntoResponse;

pub static CLIENT_JS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/okf.js"));
pub static CLIENT_WASM: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/okf_bg.wasm"));

pub async fn client_js() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/javascript; charset=utf-8")],
        crate::assets::CLIENT_JS,
    )
}

pub async fn client_wasm() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "application/wasm")],
        crate::assets::CLIENT_WASM,
    )
}
