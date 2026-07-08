//! Indexing utilities

use super::*;

/// Index content with semantic tagging
pub async fn index_with_semantics(cid: &Cid, content: &[u8], tags: Vec<String>) -> Result<()> {
    tracing::debug!("Indexing CID {} with {} semantic tags", cid, tags.len());

    // TODO: Extract embeddings
    // TODO: Add to vector index

    Ok(())
}
