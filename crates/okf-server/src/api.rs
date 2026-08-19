//! REST API and WebSocket endpoints.

use std::sync::Arc;
use std::sync::OnceLock;

use axum::{
    extract::{Path, Query, WebSocketUpgrade},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use chrono::NaiveDate;
use futures::{SinkExt, StreamExt};
use okf_core::dto::{
    ConceptResponse, ConceptSummaryResponse, DirListingResponse, TreeNodeResponse,
};
use okf_core::render_markdown;
use okf_storage::{BundleSource, ChangeEvent, DirListing, FsBundle, TreeNode};
use serde::Deserialize;

static BUNDLE: OnceLock<Arc<FsBundle>> = OnceLock::new();

pub fn init_bundle(bundle: Arc<FsBundle>) {
    let _ = BUNDLE.set(bundle);
}

fn bundle() -> Arc<FsBundle> {
    BUNDLE.get().cloned().expect("bundle not initialized")
}

fn today() -> NaiveDate {
    chrono::Utc::now().date_naive()
}

pub fn router() -> Router {
    Router::new()
        .route("/api/concepts/{*id}", get(get_concept))
        .route("/api/concepts", get(not_found))
        .route("/api/dirs/{*path}", get(get_dir))
        .route("/api/dirs", get(get_dir_root))
        .route("/api/search", get(search))
        .route("/api/tree", get(get_tree))
        .route("/api/ws", get(ws_handler))
}

#[derive(Debug, Deserialize)]
struct SearchQuery {
    q: Option<String>,
}

async fn get_concept(Path(id): Path<String>) -> Response {
    match bundle().concept(&id).await {
        Some(concept) => Json(ConceptResponse::from_concept(&concept, today())).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn get_dir(Path(path): Path<String>) -> Response {
    let path = path.trim_end_matches('/').to_string();
    match bundle().list_dir(&path).await {
        Some(listing) => Json(dir_response(listing)).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn get_dir_root() -> Response {
    match bundle().list_dir("").await {
        Some(listing) => Json(dir_response(listing)).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn get_tree() -> Response {
    Json(tree_response(bundle().tree().await)).into_response()
}

async fn not_found() -> Response {
    StatusCode::NOT_FOUND.into_response()
}

fn dir_response(listing: DirListing) -> DirListingResponse {
    let path = listing.path.clone();
    let index_html = listing.index_markdown.map(|md| render_markdown(&md, &path));
    let log_html = listing.log_markdown.map(|md| render_markdown(&md, &path));
    let concepts = listing
        .concepts
        .iter()
        .map(|s| ConceptSummaryResponse::from_summary(s, today()))
        .collect();
    DirListingResponse {
        path,
        index_html,
        log_html,
        concepts,
        subdirs: listing.subdirs,
    }
}

fn tree_response(node: TreeNode) -> TreeNodeResponse {
    TreeNodeResponse {
        name: node.name,
        path: node.path,
        concepts: node
            .concepts
            .iter()
            .map(|s| ConceptSummaryResponse::from_summary(s, today()))
            .collect(),
        children: node.children.into_iter().map(tree_response).collect(),
    }
}

async fn search(Query(query): Query<SearchQuery>) -> Response {
    let q = query.q.unwrap_or_default();
    let results = bundle().search(&q).await;
    Json(
        results
            .iter()
            .map(|s| ConceptSummaryResponse::from_summary(s, today()))
            .collect::<Vec<_>>(),
    )
    .into_response()
}

async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(socket: axum::extract::ws::WebSocket) {
    use axum::extract::ws::Message;

    let (mut sender, mut receiver) = socket.split();
    let mut watched: Option<String> = None;
    let mut change_rx = bundle().subscribe();

    loop {
        tokio::select! {
            msg = receiver.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) {
                            if value.get("type").and_then(|t| t.as_str()) == Some("watch") {
                                watched = value.get("path").and_then(|p| p.as_str()).map(String::from);
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {}
                }
            }
            change = change_rx.recv() => {
                match change {
                    Ok(ChangeEvent { paths }) => {
                        if let Some(w) = &watched {
                            if paths.iter().any(|p| is_affected(w, p)) {
                                // Include the full affected-path set so a client
                                // watching the bundle root ("") can decide which
                                // of its pages actually need to re-fetch.
                                let payload = serde_json::json!({ "type": "change", "path": w, "paths": paths });
                                if sender.send(Message::Text(payload.to_string().into())).await.is_err() {
                                    break;
                                }
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
        }
    }
}

fn is_affected(watched: &str, changed: &str) -> bool {
    let w = watched.trim_matches('/');
    let c = changed.trim_matches('/');
    if w.is_empty() {
        return true;
    }
    if w == c {
        return true;
    }
    if c.is_empty() {
        return true;
    }
    w.starts_with(&format!("{c}/")) || c.starts_with(&format!("{w}/"))
}
