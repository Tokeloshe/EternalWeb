//! Badge system

use super::*;
use dashmap::DashMap;

/// Badge manager
pub struct BadgeManager {
    badges: DashMap<String, Vec<Badge>>,
}

impl BadgeManager {
    pub fn new() -> Self {
        Self {
            badges: DashMap::new(),
        }
    }

    pub fn award(&self, peer_id: &str, badge: Badge) {
        let mut entry = self.badges.entry(peer_id.to_string()).or_insert_with(Vec::new);
        entry.push(badge);
    }

    pub fn get_badges(&self, peer_id: &str) -> Vec<Badge> {
        self.badges.get(peer_id)
            .map(|entry| entry.clone())
            .unwrap_or_default()
    }

    pub fn count_badges(&self, peer_id: &str) -> usize {
        self.get_badges(peer_id).len()
    }
}

impl Default for BadgeManager {
    fn default() -> Self {
        Self::new()
    }
}
