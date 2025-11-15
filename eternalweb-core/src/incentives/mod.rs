//! # Incentives Module
//!
//! Reputation system and badges for ethical contributions.
//!
//! ## Features
//! - On-chain badges for contributions
//! - Non-speculative reputation tokens
//! - Community recognition
//! - Ethical behavior rewards

use crate::{PeerId, Result};
use serde::{Deserialize, Serialize};

pub mod badges;
pub mod reputation;

pub use badges::*;
pub use reputation::*;

/// Contribution type
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ContribType {
    /// Archiving content
    Archive,
    /// Hosting/replication
    Hosting,
    /// Curation/moderation
    Curation,
    /// Development
    Development,
    /// Community support
    Support,
}

/// Badge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Badge {
    pub id: String,
    pub name: String,
    pub description: String,
    pub contribution_type: ContribType,
    pub level: u32,
}

/// Award a badge
///
/// On-chain badges for ethical contributions.
///
/// ## Arguments
/// * `peer` - Peer to award
/// * `contrib` - Type of contribution
pub async fn award_badge(peer: &PeerId, contrib: ContribType) -> Result<Badge> {
    tracing::info!("Awarding {:?} badge to peer {}", contrib, peer);

    let badge = Badge {
        id: uuid::Uuid::new_v4().to_string(),
        name: format!("{:?} Contributor", contrib),
        description: "Awarded for valuable contributions".to_string(),
        contribution_type: contrib,
        level: 1,
    };

    // TODO: Record badge on blockchain

    Ok(badge)
}

/// Get reputation score for peer
pub async fn get_reputation(peer: &PeerId) -> Result<u64> {
    tracing::debug!("Fetching reputation for peer {}", peer);

    // TODO: Query blockchain

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contrib_types() {
        let types = vec![
            ContribType::Archive,
            ContribType::Hosting,
            ContribType::Curation,
        ];
        assert_eq!(types.len(), 3);
    }
}
