//! Isomorphic HTTP client for the OKF REST API.
//!
//! During SSR it fetches from `http://127.0.0.1:{port}` (the API base URL is
//! injected by the server). During hydration it fetches from the same origin
//! using relative `/api/...` paths.

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use fawi_core::dto::{
    ConceptResponse, ConceptSummaryResponse, DirListingResponse, TreeNodeResponse,
};

#[cfg(feature = "ssr")]
static API_BASE: std::sync::OnceLock<String> = std::sync::OnceLock::new();

/// Set the base URL used by SSR to reach the REST API. Called once by the server.
#[cfg(feature = "ssr")]
pub fn set_api_base_url(url: String) {
    let _ = API_BASE.set(url);
}

async fn get_json<T: DeserializeOwned>(path: &str) -> Result<T, String> {
    #[cfg(feature = "ssr")]
    let url = {
        let base = API_BASE
            .get()
            .cloned()
            .unwrap_or_else(|| "http://127.0.0.1:8080".to_string());
        format!("{base}/api/{path}")
    };
    #[cfg(feature = "hydrate")]
    let url = {
        // `reqwest` on WASM cannot resolve a relative URL (there is no base
        // URL), so build an absolute URL from the current page's origin. This
        // also keeps the API on the same origin as the served UI.
        let origin = web_sys::window()
            .and_then(|w| w.location().origin().ok())
            .unwrap_or_default();
        format!("{origin}/api/{path}")
    };

    let resp = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("status {}", resp.status()));
    }
    resp.json::<T>().await.map_err(|e| e.to_string())
}

/// The result of resolving a bundle path: a concept, a directory, or nothing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PageData {
    Concept(ConceptResponse),
    Dir(DirListingResponse),
    NotFound,
}

pub async fn fetch_page(path: &str) -> PageData {
    let path = path.trim_matches('/');
    let concept_path = if path.is_empty() {
        "concepts".to_string()
    } else {
        format!("concepts/{path}")
    };
    let dir_path = if path.is_empty() {
        "dirs".to_string()
    } else {
        format!("dirs/{path}")
    };

    if let Ok(concept) = get_json::<ConceptResponse>(&concept_path).await {
        return PageData::Concept(concept);
    }
    if let Ok(dir) = get_json::<DirListingResponse>(&dir_path).await {
        return PageData::Dir(dir);
    }
    PageData::NotFound
}

/// Fetch a directory listing with optional `sort` and `dir` (`asc`/`desc`)
/// query parameters. Returns `NotFound` when there is no directory at `path`.
pub async fn fetch_dir(path: &str, sort: Option<&str>, dir: Option<&str>) -> PageData {
    let path = path.trim_matches('/');
    let dir_path = if path.is_empty() {
        "dirs".to_string()
    } else {
        format!("dirs/{path}")
    };

    let mut params: Vec<String> = Vec::new();
    if let Some(s) = sort.map(str::trim).filter(|s| !s.is_empty()) {
        params.push(format!("sort={}", urlencode(s)));
        let desc = dir
            .map(str::trim)
            .map(str::to_ascii_lowercase)
            .map(|d| d == "desc" || d == "descending")
            .unwrap_or(false);
        params.push(if desc {
            "dir=desc".to_string()
        } else {
            "dir=asc".to_string()
        });
    }

    let url = if params.is_empty() {
        dir_path
    } else {
        format!("{dir_path}?{}", params.join("&"))
    };

    match get_json::<DirListingResponse>(&url).await {
        Ok(dir) => PageData::Dir(dir),
        Err(_) => PageData::NotFound,
    }
}

pub async fn fetch_search(query: &str) -> Vec<ConceptSummaryResponse> {
    let path = format!("search?q={}", urlencode(query));
    get_json::<Vec<ConceptSummaryResponse>>(&path)
        .await
        .unwrap_or_default()
}

/// Fetch the full bundle directory tree for the navigation sidebar.
pub async fn fetch_tree() -> TreeNodeResponse {
    get_json::<TreeNodeResponse>("tree")
        .await
        .unwrap_or_default()
}

pub(crate) fn urlencode(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || "-_.~".contains(c) {
                c.to_string()
            } else {
                format!("%{:02X}", c as u32)
            }
        })
        .collect()
}

/// Make a future `Send` on the client (WASM is single-threaded, so wrapping a
/// non-`Send` browser future is sound). On the server the future is already
/// `Send`, so it is passed through unchanged.
#[cfg(feature = "hydrate")]
pub fn to_send_future<F>(fut: F) -> send_wrapper::SendWrapper<F>
where
    F: std::future::Future + 'static,
{
    send_wrapper::SendWrapper::new(fut)
}

#[cfg(feature = "ssr")]
pub fn to_send_future<F>(fut: F) -> F
where
    F: std::future::Future + Send + 'static,
{
    fut
}
