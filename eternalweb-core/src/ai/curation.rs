//! Autonomous curation

use super::*;

/// Curation rules
pub struct CurationRules {
    pub min_quality_score: f32,
    pub max_age_days: i64,
}

impl Default for CurationRules {
    fn default() -> Self {
        Self {
            min_quality_score: 0.7,
            max_age_days: 365,
        }
    }
}

/// Apply curation rules to content
pub async fn apply_curation_rules(
    content_score: f32,
    age_days: i64,
    rules: &CurationRules,
) -> bool {
    content_score >= rules.min_quality_score && age_days <= rules.max_age_days
}
