//! Trend prediction

use super::*;

/// Analyze topic trends over time
pub async fn analyze_trends(topics: Vec<String>) -> Result<Vec<Topic>> {
    // TODO: Time-series analysis
    // TODO: ML-based prediction

    Ok(topics.into_iter().map(|name| Topic {
        name,
        relevance: 0.5,
    }).collect())
}
