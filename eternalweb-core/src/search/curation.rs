//! Community curation system

use super::*;
use dashmap::DashMap;

/// Curation scores
pub struct CurationManager {
    scores: DashMap<String, CurationScore>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurationScore {
    pub upvotes: u64,
    pub downvotes: u64,
    pub flags: u64,
    pub quality_score: f64,
}

impl CurationManager {
    pub fn new() -> Self {
        Self {
            scores: DashMap::new(),
        }
    }

    pub fn record_vote(&self, cid: &str, vote: VoteType) {
        let mut entry = self.scores.entry(cid.to_string()).or_insert_with(|| CurationScore {
            upvotes: 0,
            downvotes: 0,
            flags: 0,
            quality_score: 0.5,
        });

        match vote {
            VoteType::Upvote => entry.upvotes += 1,
            VoteType::Downvote => entry.downvotes += 1,
            VoteType::Flag => entry.flags += 1,
        }

        // Recalculate quality score
        let total = entry.upvotes + entry.downvotes;
        if total > 0 {
            entry.quality_score = entry.upvotes as f64 / total as f64;
        }
    }

    pub fn get_quality_score(&self, cid: &str) -> f64 {
        self.scores.get(cid)
            .map(|entry| entry.quality_score)
            .unwrap_or(0.5)
    }
}

impl Default for CurationManager {
    fn default() -> Self {
        Self::new()
    }
}
