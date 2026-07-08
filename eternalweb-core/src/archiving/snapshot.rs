//! Snapshot utilities for web archiving

use super::*;

/// Advanced snapshot options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotOptions {
    /// Include linked resources (CSS, JS, images)
    pub include_resources: bool,
    /// Maximum depth for recursive archiving
    pub max_depth: usize,
    /// Follow external links
    pub follow_external: bool,
    /// Custom user agent
    pub user_agent: Option<String>,
    /// Timeout in seconds
    pub timeout_secs: u64,
}

impl Default for SnapshotOptions {
    fn default() -> Self {
        Self {
            include_resources: true,
            max_depth: 1,
            follow_external: false,
            user_agent: Some("EternalWeb/0.1".to_string()),
            timeout_secs: 30,
        }
    }
}

/// Create a comprehensive snapshot with options
pub async fn snapshot_with_options(url: &str, options: SnapshotOptions) -> Result<Snapshot> {
    // TODO: Implement full snapshot with resources
    let cid = snapshot_page(url).await?;

    Ok(Snapshot {
        cid,
        url: url.to_string(),
        timestamp: Utc::now(),
        metadata: SnapshotMetadata {
            schema_types: vec!["WebPage".to_string()],
            title: None,
            description: None,
            creator: None,
            language: None,
            tags: vec![],
            license: None,
            is_public: true,
        },
        mime_type: "text/html".to_string(),
        size: 0,
    })
}
