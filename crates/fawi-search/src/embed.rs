//! Embedding abstraction and the local, offline default implementation.

use anyhow::Result;

/// Computes vector embeddings for a batch of texts.
///
/// This is synchronous by design (embedding inference is CPU-bound); callers
/// run it on a blocking thread. It is kept behind a trait so the default local
/// model can be swapped for a mock in tests without touching the index.
pub trait Embedder: Send + Sync {
    /// Embed `texts`, returning one vector per input in the same order.
    fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>>;
}

/// Local, offline embedding via [`fastembed`] (ONNX Runtime backend).
///
/// The default model is `BAAI/bge-small-en-v1.5` (384 dimensions). Weights are
/// downloaded from Hugging Face on first use and cached locally; for air-gapped
/// use, pre-populate the cache directory (`HF_HOME`) or vendor the model.
pub struct FastembedEmbedder {
    model: std::sync::Mutex<fastembed::TextEmbedding>,
}

impl FastembedEmbedder {
    /// Load the default local embedding model.
    pub fn try_default() -> Result<Self> {
        let options = fastembed::TextInitOptions::new(fastembed::EmbeddingModel::BGESmallENV15);
        let model = fastembed::TextEmbedding::try_new(options)
            .map_err(|e| anyhow::anyhow!("failed to load local embedding model: {e}"))?;
        Ok(Self {
            model: std::sync::Mutex::new(model),
        })
    }
}

impl Embedder for FastembedEmbedder {
    fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        let mut model = self
            .model
            .lock()
            .map_err(|_| anyhow::anyhow!("embedding model lock poisoned"))?;
        model
            .embed(texts.to_vec(), None)
            .map_err(|e| anyhow::anyhow!("embedding inference failed: {e}"))
    }
}
