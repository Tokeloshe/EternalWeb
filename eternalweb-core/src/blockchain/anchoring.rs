//! Content anchoring on blockchain

use super::*;

/// Anchor a batch of CIDs
pub async fn anchor_batch(cids: Vec<Cid>) -> Result<BlockId> {
    tracing::info!("Anchoring batch of {} CIDs", cids.len());

    // TODO: Create merkle tree of CIDs
    // TODO: Anchor root hash

    Ok("block-batch-123".to_string())
}

/// Verify anchor
pub async fn verify_anchor(cid: &Cid, block_id: &str) -> Result<bool> {
    tracing::debug!("Verifying anchor for CID {} in block {}", cid, block_id);

    // TODO: Query blockchain and verify

    Ok(true)
}
