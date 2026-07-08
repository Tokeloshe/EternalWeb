//! Version control for archived content using Hypercore

use super::*;

/// Version manager for tracking content changes
pub struct VersionManager {
    feeds: dashmap::DashMap<String, HyperFeed>,
}

impl VersionManager {
    /// Create a new version manager
    pub fn new() -> Self {
        Self {
            feeds: dashmap::DashMap::new(),
        }
    }

    /// Add a new version to a feed
    pub async fn add_version(&self, feed_id: &str, cid: Cid, delta: Vec<Delta>) -> Result<()> {
        let mut feed = self.feeds.get_mut(feed_id)
            .ok_or_else(|| Error::Storage("Feed not found".to_string()))?;

        let entry = FeedEntry {
            cid,
            delta,
            timestamp: Utc::now(),
            block_id: None,
        };

        feed.sequence += 1;
        feed.entries.push(entry);

        Ok(())
    }

    /// Get all versions for a feed
    pub fn get_versions(&self, feed_id: &str) -> Result<Vec<FeedEntry>> {
        let feed = self.feeds.get(feed_id)
            .ok_or_else(|| Error::Storage("Feed not found".to_string()))?;

        Ok(feed.entries.clone())
    }

    /// Compare two versions
    pub fn diff_versions(&self, feed_id: &str, seq1: u64, seq2: u64) -> Result<Vec<Delta>> {
        let feed = self.feeds.get(feed_id)
            .ok_or_else(|| Error::Storage("Feed not found".to_string()))?;

        let entry1 = feed.entries.get(seq1 as usize)
            .ok_or_else(|| Error::InvalidInput("Invalid sequence 1".to_string()))?;

        let entry2 = feed.entries.get(seq2 as usize)
            .ok_or_else(|| Error::InvalidInput("Invalid sequence 2".to_string()))?;

        // Compute diff
        let mut deltas = entry1.delta.clone();
        deltas.extend(entry2.delta.clone());

        Ok(deltas)
    }
}

impl Default for VersionManager {
    fn default() -> Self {
        Self::new()
    }
}
