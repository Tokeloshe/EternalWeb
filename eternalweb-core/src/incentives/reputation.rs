//! Reputation token system (non-speculative)

use super::*;

/// Calculate reputation from contributions
pub fn calculate_reputation(contrib_counts: Vec<(ContribType, u64)>) -> u64 {
    let mut total = 0u64;

    for (contrib_type, count) in contrib_counts {
        let weight = match contrib_type {
            ContribType::Archive => 10,
            ContribType::Hosting => 5,
            ContribType::Curation => 15,
            ContribType::Development => 20,
            ContribType::Support => 8,
        };

        total += weight * count;
    }

    total
}

/// Reputation levels
pub fn get_reputation_level(reputation: u64) -> &'static str {
    match reputation {
        0..=99 => "Newcomer",
        100..=499 => "Contributor",
        500..=999 => "Guardian",
        1000..=4999 => "Sentinel",
        _ => "Eternal",
    }
}
