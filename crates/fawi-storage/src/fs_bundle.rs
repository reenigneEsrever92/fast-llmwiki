use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use notify::{RecursiveMode, Watcher};
use fawi_core::{Concept, ConceptSummary};
use tokio::sync::{broadcast, mpsc, RwLock};

use crate::{BundleSource, DirListing, TreeNode};

/// A filesystem change, expressed as the affected concept IDs and directory
/// paths (relative to the bundle root; `""` is the root).
#[derive(Debug, Clone)]
pub struct ChangeEvent {
    pub paths: Vec<String>,
}

#[derive(Default)]
struct ScannedBundle {
    concepts: HashMap<String, Concept>,
    dirs: HashSet<String>,
}

/// A filesystem-backed, read-only OKF bundle.
///
/// The bundle directory tree is scanned at startup and re-scanned whenever the
/// filesystem reports a change. Subscribers can listen for change events via
/// [`FsBundle::subscribe`].
pub struct FsBundle {
    root: PathBuf,
    concepts: RwLock<HashMap<String, Concept>>,
    dirs: RwLock<HashSet<String>>,
    change_tx: broadcast::Sender<ChangeEvent>,
}

impl FsBundle {
    /// Open a bundle rooted at `root` and start watching it for changes.
    pub async fn open(root: impl AsRef<Path>) -> Result<Arc<Self>> {
        let root = root.as_ref().to_path_buf();
        if !root.exists() {
            return Err(anyhow::anyhow!(
                "bundle directory does not exist: {}",
                root.display()
            ));
        }
        if !root.is_dir() {
            return Err(anyhow::anyhow!(
                "bundle path is not a directory: {}",
                root.display()
            ));
        }

        // Canonicalize to an absolute, symlink-free path. The filesystem watcher
        // reports changed paths as absolute (it joins the process cwd onto a
        // relative watch root), and `affected_paths` strips this root as a
        // prefix, so both sides must agree on the exact path. A relative `--data`
        // path (e.g. "./docs") previously produced a root that did not prefix the
        // watcher's absolute paths, so no change was ever broadcast.
        let root = std::fs::canonicalize(&root)
            .map_err(|e| anyhow::anyhow!("failed to canonicalize bundle directory: {e}"))?;

        let (change_tx, _) = broadcast::channel(64);
        let bundle = Arc::new(Self {
            root: root.clone(),
            concepts: RwLock::new(HashMap::new()),
            dirs: RwLock::new(HashSet::new()),
            change_tx,
        });
        bundle.rescan().await?;
        spawn_watcher(bundle.clone());

        Ok(bundle)
    }

    /// Subscribe to filesystem change events.
    pub fn subscribe(&self) -> broadcast::Receiver<ChangeEvent> {
        self.change_tx.subscribe()
    }

    /// All concepts currently in the bundle, in no particular order.
    pub async fn concepts(&self) -> Vec<Concept> {
        self.concepts.read().await.values().cloned().collect()
    }

    async fn rescan(&self) -> Result<()> {
        let root = self.root.clone();
        let scanned = tokio::task::spawn_blocking(move || scan_bundle(&root))
            .await
            .map_err(|e| anyhow::anyhow!("bundle scan task failed: {e}"))?;

        *self.concepts.write().await = scanned.concepts;
        *self.dirs.write().await = scanned.dirs;
        Ok(())
    }
}

/// Start a background filesystem watcher that reloads the bundle and broadcasts
/// a change event whenever a change is detected.
fn spawn_watcher(bundle: Arc<FsBundle>) {
    let root = bundle.root.clone();
    let watch_root = root.clone();
    let change_tx = bundle.change_tx.clone();
    let (tx, mut rx) = mpsc::unbounded_channel::<PathBuf>();

    // `notify` is synchronous, so it runs on its own thread and forwards paths
    // into the async runtime through an mpsc channel.
    std::thread::spawn(move || {
        let (event_tx, event_rx) = std::sync::mpsc::channel::<PathBuf>();
        let mut watcher =
            match notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
                if let Ok(event) = res {
                    if is_relevant(&event) {
                        for path in event.paths {
                            let _ = event_tx.send(path);
                        }
                    }
                }
            }) {
                Ok(w) => w,
                Err(e) => {
                    tracing::warn!("failed to start file watcher: {e}");
                    return;
                }
            };
        if let Err(e) = watcher.watch(&watch_root, RecursiveMode::Recursive) {
            tracing::warn!("failed to watch bundle directory: {e}");
            return;
        }
        // Keep the watcher alive for the lifetime of the process.
        for path in event_rx {
            let _ = tx.send(path);
        }
    });

    tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Some(first) => {
                    let mut paths = vec![first];
                    // Coalesce bursts of events (e.g. a save firing several).
                    loop {
                        match tokio::time::timeout(Duration::from_millis(300), rx.recv()).await {
                            Ok(Some(path)) => paths.push(path),
                            _ => break,
                        }
                    }

                    if let Err(e) = bundle.rescan().await {
                        tracing::warn!("bundle reload failed: {e}");
                    } else {
                        let affected = affected_paths(&root, &paths);
                        if !affected.is_empty() {
                            let _ = change_tx.send(ChangeEvent { paths: affected });
                        }
                        tracing::info!("bundle reloaded after filesystem change");
                    }
                }
                None => break,
            }
        }
    });
}

fn is_relevant(event: &notify::Event) -> bool {
    !matches!(event.kind, notify::EventKind::Access(_))
}

/// Map changed file paths to the concept IDs / directory paths they affect.
fn affected_paths(root: &Path, changed: &[PathBuf]) -> Vec<String> {
    let mut out = HashSet::new();
    for path in changed {
        let Ok(rel) = path.strip_prefix(root) else {
            continue;
        };
        let rel = rel_to_string(rel);

        if let Some(id) = rel.strip_suffix(".md") {
            let name = id.rsplit('/').next().unwrap_or(id);
            if name == "index" || name == "log" {
                // A reserved file change affects its directory.
                let dir = match id.rfind('/') {
                    Some(i) => &id[..i],
                    None => "",
                };
                out.insert(dir.to_string());
            } else {
                out.insert(id.to_string());
            }
        } else {
            // A directory (or non-markdown file) affects its own path.
            out.insert(rel);
        }
    }
    out.into_iter().collect()
}

/// Synchronously walk the bundle tree and build the in-memory index.
fn scan_bundle(root: &Path) -> ScannedBundle {
    let mut out = ScannedBundle::default();
    out.dirs.insert(String::new());

    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let entries = match std::fs::read_dir(&dir) {
            Ok(entries) => entries,
            Err(_) => continue,
        };

        for entry in entries.flatten() {
            let path = entry.path();
            let rel = match path.strip_prefix(root) {
                Ok(rel) => rel_to_string(rel),
                Err(_) => continue,
            };
            let Ok(file_type) = entry.file_type() else {
                continue;
            };

            if file_type.is_dir() {
                out.dirs.insert(rel);
                stack.push(path);
            } else if file_type.is_file() && path.extension().and_then(|e| e.to_str()) == Some("md")
            {
                let Some(file_name) = path.file_name().and_then(|s| s.to_str()) else {
                    continue;
                };
                if file_name == "index.md" || file_name == "log.md" {
                    continue; // reserved files are read on demand
                }
                let Some(id) = rel.strip_suffix(".md") else {
                    continue;
                };
                let Ok(content) = std::fs::read_to_string(&path) else {
                    continue;
                };
                let concept = Concept::from_markdown(id, &content);
                out.concepts.insert(id.to_string(), concept);
            }
        }
    }

    out
}

fn rel_to_string(path: &Path) -> String {
    path.components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("/")
}

fn parent_of(id: &str) -> &str {
    match id.rfind('/') {
        Some(i) => &id[..i],
        None => "",
    }
}

/// Recursively build a tree node for `dir` from the scanned bundle index.
fn build_tree(dir: &str, dirs: &HashSet<String>, concepts: &HashMap<String, Concept>) -> TreeNode {
    let mut node_concepts: Vec<ConceptSummary> = concepts
        .iter()
        .filter(|(id, _)| parent_of(id) == dir)
        .map(|(_, c)| c.summary())
        .collect();
    node_concepts.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));

    let mut child_dirs: Vec<String> = dirs
        .iter()
        .filter(|d| !d.is_empty() && parent_of(d) == dir)
        .cloned()
        .collect();
    child_dirs.sort();

    let children = child_dirs
        .iter()
        .map(|d| build_tree(d, dirs, concepts))
        .collect();

    TreeNode {
        name: dir.rsplit('/').next().unwrap_or("").to_string(),
        path: dir.to_string(),
        concepts: node_concepts,
        children,
    }
}

async fn read_reserved(root: &Path, dir: &str, name: &str) -> Option<String> {
    let mut path = root.to_path_buf();
    if !dir.is_empty() {
        path.push(dir);
    }
    path.push(name);

    let content = tokio::fs::read_to_string(&path).await.ok()?;
    // Strip optional front matter (a root `index.md` may declare `okf_version`).
    let (_, body) = fawi_core::split_front_matter(&content);
    Some(body)
}

#[async_trait::async_trait]
impl BundleSource for FsBundle {
    async fn concept(&self, id: &str) -> Option<Concept> {
        self.concepts.read().await.get(id).cloned()
    }

    async fn list_dir(&self, dir: &str) -> Option<DirListing> {
        let dir = dir.trim_matches('/');
        if !self.dirs.read().await.contains(dir) {
            return None;
        }

        let concepts = self.concepts.read().await;
        let dirs = self.dirs.read().await;

        let mut concept_summaries: Vec<ConceptSummary> = concepts
            .iter()
            .filter(|(id, _)| parent_of(id) == dir)
            .map(|(_, c)| c.summary())
            .collect();
        concept_summaries.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));

        let mut subdirs: Vec<String> = dirs
            .iter()
            .filter(|d| !d.is_empty() && parent_of(d) == dir)
            .map(|d| d.rsplit('/').next().unwrap_or(d).to_string())
            .collect();
        subdirs.sort();

        Some(DirListing {
            path: dir.to_string(),
            index_markdown: read_reserved(&self.root, dir, "index.md").await,
            log_markdown: read_reserved(&self.root, dir, "log.md").await,
            concepts: concept_summaries,
            subdirs,
        })
    }

    async fn search(&self, query: &str) -> Vec<ConceptSummary> {
        let q = query.to_lowercase();
        let concepts = self.concepts.read().await;

        let mut out: Vec<ConceptSummary> = concepts
            .iter()
            .filter(|(id, c)| {
                id.to_lowercase().contains(&q)
                    || c.title.to_lowercase().contains(&q)
                    || c.concept_type.to_lowercase().contains(&q)
                    || c.description
                        .as_deref()
                        .map(|d| d.to_lowercase().contains(&q))
                        .unwrap_or(false)
                    || c.tags.iter().any(|t| t.to_lowercase().contains(&q))
            })
            .map(|(_, c)| c.summary())
            .collect();
        out.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
        out
    }

    async fn tree(&self) -> TreeNode {
        let concepts = self.concepts.read().await;
        let dirs = self.dirs.read().await;
        build_tree("", &dirs, &concepts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn concept(id: &str, title: &str) -> Concept {
        Concept::from_markdown(id, &format!("---\ntype: T\ntitle: {title}\n---\nbody\n"))
    }

    #[test]
    fn builds_nested_tree_from_index() {
        let mut dirs = HashSet::new();
        dirs.insert(String::new());
        dirs.insert("dev".to_string());
        dirs.insert("dev/plans".to_string());

        let mut concepts = HashMap::new();
        concepts.insert("root-concept".to_string(), concept("root-concept", "Root"));
        concepts.insert("dev/child".to_string(), concept("dev/child", "Child"));
        concepts.insert(
            "dev/plans/leaf".to_string(),
            concept("dev/plans/leaf", "Leaf"),
        );

        let tree = build_tree("", &dirs, &concepts);

        assert_eq!(tree.path, "");
        assert_eq!(tree.name, "");
        assert_eq!(tree.concepts.len(), 1);
        assert_eq!(tree.concepts[0].id, "root-concept");
        assert_eq!(tree.children.len(), 1);

        let dev = &tree.children[0];
        assert_eq!(dev.path, "dev");
        assert_eq!(dev.name, "dev");
        assert_eq!(dev.concepts.len(), 1);
        assert_eq!(dev.concepts[0].id, "dev/child");
        assert_eq!(dev.children.len(), 1);

        let plans = &dev.children[0];
        assert_eq!(plans.path, "dev/plans");
        assert_eq!(plans.name, "plans");
        assert_eq!(plans.concepts.len(), 1);
        assert_eq!(plans.concepts[0].id, "dev/plans/leaf");
        assert!(plans.children.is_empty());
    }

    #[test]
    fn sorts_siblings_alphabetically() {
        let mut dirs = HashSet::new();
        dirs.insert(String::new());
        dirs.insert("b".to_string());
        dirs.insert("a".to_string());

        let mut concepts = HashMap::new();
        concepts.insert("zeta".to_string(), concept("zeta", "Zeta"));
        concepts.insert("alpha".to_string(), concept("alpha", "Alpha"));

        let tree = build_tree("", &dirs, &concepts);

        let concept_ids: Vec<&str> = tree.concepts.iter().map(|c| c.id.as_str()).collect();
        assert_eq!(concept_ids, vec!["alpha", "zeta"]);

        let child_paths: Vec<&str> = tree.children.iter().map(|c| c.path.as_str()).collect();
        assert_eq!(child_paths, vec!["a", "b"]);
    }

    #[tokio::test]
    async fn open_canonicalizes_the_root() {
        let base =
            std::env::temp_dir().join(format!("okf-open-canonicalize-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(base.join("docs")).unwrap();
        std::fs::write(
            base.join("docs").join("a.md"),
            "---\ntype: T\ntitle: A\n---\nbody\n",
        )
        .unwrap();

        // A path with an embedded `.` component (as `--data ./docs` is passed)
        // must be canonicalized so the watcher and `affected_paths` agree on a
        // single absolute prefix.
        let relative = base.join(".").join("docs");
        let bundle = FsBundle::open(&relative).await.unwrap();

        let expected = std::fs::canonicalize(base.join("docs")).unwrap();
        assert_eq!(bundle.root, expected);
        assert!(bundle.root.is_absolute());

        let _ = std::fs::remove_dir_all(&base);
    }
}
