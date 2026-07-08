//! Trust and reputation management

use super::*;
use dashmap::DashMap;
use std::sync::Arc;

/// Trust scores for peers
pub struct TrustManager {
    scores: Arc<DashMap<PeerId, TrustScore>>,
}

/// Trust score for a peer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustScore {
    /// Overall trust score (0.0 to 1.0)
    pub score: f64,
    /// Number of successful interactions
    pub successful_interactions: u64,
    /// Number of failed interactions
    pub failed_interactions: u64,
    /// Last updated timestamp
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl TrustManager {
    pub fn new() -> Self {
        Self {
            scores: Arc::new(DashMap::new()),
        }
    }

    /// Get trust score for a peer
    pub fn get_score(&self, peer: &PeerId) -> Option<TrustScore> {
        self.scores.get(peer).map(|entry| entry.clone())
    }

    /// Update trust score based on interaction
    pub fn record_interaction(&self, peer: &PeerId, success: bool) {
        let mut entry = self.scores.entry(*peer).or_insert_with(|| TrustScore {
            score: 0.5,
            successful_interactions: 0,
            failed_interactions: 0,
            last_updated: chrono::Utc::now(),
        });

        if success {
            entry.successful_interactions += 1;
        } else {
            entry.failed_interactions += 1;
        }

        // Recalculate score
        let total = entry.successful_interactions + entry.failed_interactions;
        entry.score = entry.successful_interactions as f64 / total as f64;
        entry.last_updated = chrono::Utc::now();
    }

    /// Check if peer is trustworthy (score > 0.7)
    pub fn is_trustworthy(&self, peer: &PeerId) -> bool {
        self.get_score(peer)
            .map(|score| score.score > 0.7)
            .unwrap_or(false)
    }
}

impl Default for TrustManager {
    fn default() -> Self {
        Self::new()
    }
}
