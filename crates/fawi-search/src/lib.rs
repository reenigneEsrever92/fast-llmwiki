//! Semantic search provider for OKF bundles.
//!
//! `fawi-search` indexes every concept in a bundle — title, description, tags,
//! and body — into vector embeddings with a local model, then implements the
//! [`fawi_storage::SearchProvider`] port so the embeddings can be searched
//! through the unified search endpoint served by `fawi-server`. It reuses
//! [`fawi_storage::FsBundle`] for reads and change events, and rebuilds its
//! index when the bundle changes.
//!
//! Logging is left to the caller so a process hosting multiple components can
//! initialize it exactly once.

pub mod embed;
pub mod index;
pub mod provider;
