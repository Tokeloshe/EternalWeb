//! Query processing

use super::*;

/// Parse and optimize search query
pub fn parse_query(query: &str) -> Result<Vec<String>> {
    let terms: Vec<String> = query
        .split_whitespace()
        .map(|s| s.to_lowercase())
        .collect();

    Ok(terms)
}

/// Semantic query expansion using AI
pub async fn expand_query_semantically(query: &str) -> Result<Vec<String>> {
    // TODO: Use AI to expand query with related terms
    Ok(vec![query.to_string()])
}
