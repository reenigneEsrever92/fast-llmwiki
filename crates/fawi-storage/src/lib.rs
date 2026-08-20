//! Read-only storage and hot reloading for OKF bundles.

mod fs_bundle;

use async_trait::async_trait;
use fawi_core::{Concept, ConceptSummary};

pub use fs_bundle::{ChangeEvent, FsBundle};

/// A directory listing for the bundle browser.
#[derive(Debug, Clone, Default)]
pub struct DirListing {
    /// Directory path relative to the bundle root (`""` for the root).
    pub path: String,
    /// Raw `index.md` body (front matter stripped), if present.
    pub index_markdown: Option<String>,
    /// Raw `log.md` body (front matter stripped), if present.
    pub log_markdown: Option<String>,
    pub concepts: Vec<ConceptSummary>,
    pub subdirs: Vec<String>,
}

/// A single node in the bundle directory tree.
#[derive(Debug, Clone, Default)]
pub struct TreeNode {
    /// Directory segment name (empty for the bundle root).
    pub name: String,
    /// Directory path relative to the bundle root (`""` for the root).
    pub path: String,
    /// Concepts directly inside this directory.
    pub concepts: Vec<ConceptSummary>,
    /// Nested subdirectories.
    pub children: Vec<TreeNode>,
}

/// Read-only access to an OKF bundle.
#[async_trait]
pub trait BundleSource: Send + Sync {
    async fn concept(&self, id: &str) -> Option<Concept>;
    async fn list_dir(&self, dir: &str) -> Option<DirListing>;
    async fn search(&self, query: &str) -> Vec<ConceptSummary>;
    /// The full directory tree from the root down, for the navigation sidebar.
    async fn tree(&self) -> TreeNode;
}
