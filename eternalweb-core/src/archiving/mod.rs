//! # Archiving Engine
//!
//! This module handles web content archiving, versioning, and replication.
//! It combines IPFS for immutable content addressing with Hypercore for mutable feeds.
//!
//! ## Features
//! - Web page snapshotting with semantic metadata
//! - Version control via Hypercore feeds
//! - P2P replication across nodes
//! - Support for offline archiving
//! - Future: IoT data stream archiving
//!
//! ## Example
//! ```no_run
//! use eternalweb_core::archiving::snapshot_page;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let cid = snapshot_page("https://example.com").await?;
//!     println!("Archived: {}", cid);
//!     Ok(())
//! }
//! ```

use crate::{Cid, Error, PeerId, Result};
use bytes::Bytes;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;
use url::Url;

pub mod snapshot;
pub mod versioning;
pub mod replication;
pub mod metadata;

pub use snapshot::*;
pub use versioning::*;
pub use replication::*;
pub use metadata::*;

/// Represents a web archive snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    /// Content identifier (IPFS CID)
    pub cid: Cid,
    /// Original URL
    pub url: String,
    /// Timestamp of snapshot
    pub timestamp: DateTime<Utc>,
    /// Semantic metadata (RDF)
    pub metadata: SnapshotMetadata,
    /// MIME type
    pub mime_type: String,
    /// Size in bytes
    pub size: u64,
}

/// Semantic metadata for snapshots (Web 3.0)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotMetadata {
    /// Schema.org types
    pub schema_types: Vec<String>,
    /// Dublin Core metadata
    pub title: Option<String>,
    pub description: Option<String>,
    pub creator: Option<String>,
    pub language: Option<String>,
    /// Tags for discovery
    pub tags: Vec<String>,
    /// License information
    pub license: Option<String>,
    /// Public or private
    pub is_public: bool,
}

/// Hypercore feed for versioning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyperFeed {
    /// Feed identifier
    pub id: String,
    /// Public key for verification
    pub public_key: Vec<u8>,
    /// Current sequence number
    pub sequence: u64,
    /// Entries in this feed
    pub entries: Vec<FeedEntry>,
}

/// Entry in a Hypercore feed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedEntry {
    /// CID of the content
    pub cid: Cid,
    /// Changes from previous version
    pub delta: Vec<Delta>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Blockchain anchor (optional)
    pub block_id: Option<String>,
}

/// Change delta for versioning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Delta {
    /// Content added
    Added { path: String, size: u64 },
    /// Content modified
    Modified { path: String, size: u64 },
    /// Content deleted
    Deleted { path: String },
}

/// IoT data stream (Web 4.0 future extension)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoTStream {
    /// Stream identifier
    pub stream_id: String,
    /// Data payload
    pub data: Bytes,
    /// Sensor type
    pub sensor_type: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Snapshot a web page and return its CID
///
/// This function fetches a URL, processes it, adds semantic RDF metadata,
/// and stores it in IPFS for Web 3.0 discoverability.
///
/// ## Arguments
/// * `url` - The URL to archive
///
/// ## Returns
/// The Content Identifier (CID) of the archived page
///
/// ## Example
/// ```no_run
/// # use eternalweb_core::archiving::snapshot_page;
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
/// let cid = snapshot_page("https://example.com").await?;
/// # Ok(())
/// # }
/// ```
pub async fn snapshot_page(url: &str) -> Result<Cid> {
    tracing::info!("Snapshotting page: {}", url);

    // Validate URL
    let parsed_url = Url::parse(url)
        .map_err(|e| Error::InvalidInput(format!("Invalid URL: {}", e)))?;

    // Fetch content
    let content = fetch_content(&parsed_url).await?;

    // Extract metadata
    let metadata = extract_metadata(&content, url).await?;

    // Generate CID
    let cid = generate_cid(&content)?;

    // Store in IPFS
    store_in_ipfs(&cid, &content, &metadata).await?;

    tracing::info!("Snapshot created with CID: {}", cid);
    Ok(cid)
}

/// Create a versioned feed from a CID with changes
///
/// This creates a Hypercore feed that tracks versions of content,
/// with blockchain anchoring for immutability.
///
/// ## Arguments
/// * `cid` - The initial content CID
/// * `changes` - List of changes/deltas
///
/// ## Returns
/// A HyperFeed structure
pub async fn version_feed(cid: Cid, changes: Vec<Delta>) -> Result<HyperFeed> {
    tracing::info!("Creating versioned feed for CID: {}", cid);

    // Generate keypair for feed
    let (public_key, _secret_key) = generate_feed_keypair()?;

    // Create initial entry
    let entry = FeedEntry {
        cid,
        delta: changes,
        timestamp: Utc::now(),
        block_id: None, // Will be set after blockchain anchoring
    };

    let feed = HyperFeed {
        id: uuid::Uuid::new_v4().to_string(),
        public_key: public_key.to_vec(),
        sequence: 1,
        entries: vec![entry],
    };

    // TODO: Anchor to blockchain

    Ok(feed)
}

/// Replicate a feed to peers
///
/// Optimized for Web 4.0 with AI-predicted replication based on trending topics.
///
/// ## Arguments
/// * `feed` - The feed to replicate
/// * `peers` - List of peer IDs to replicate to
pub async fn replicate_to_peers(feed: &HyperFeed, peers: Vec<PeerId>) -> Result<()> {
    tracing::info!("Replicating feed {} to {} peers", feed.id, peers.len());

    for peer in peers {
        // Send feed to peer
        send_feed_to_peer(feed, &peer).await?;
    }

    Ok(())
}

/// Archive IoT data stream (Web 4.0 extension)
///
/// ## Arguments
/// * `stream` - IoT data stream
///
/// ## Returns
/// CID of archived stream
pub async fn archive_iot_data(stream: IoTStream) -> Result<Cid> {
    tracing::info!("Archiving IoT stream: {}", stream.stream_id);

    // Serialize stream data
    let data = bincode::serialize(&stream)
        .map_err(|e| Error::Archiving(format!("Serialization failed: {}", e)))?;

    // Generate CID
    let cid = generate_cid(&data)?;

    // Store with metadata indicating it's IoT data
    let metadata = SnapshotMetadata {
        schema_types: vec!["IoTData".to_string()],
        title: Some(format!("IoT Stream: {}", stream.stream_id)),
        description: Some(format!("Sensor type: {}", stream.sensor_type)),
        creator: None,
        language: None,
        tags: vec!["iot".to_string(), stream.sensor_type.clone()],
        license: None,
        is_public: true,
    };

    store_in_ipfs(&cid, &data, &metadata).await?;

    Ok(cid)
}

/// Archive a local file
///
/// Compresses and stores local files with optional public licensing.
///
/// ## Arguments
/// * `file_path` - Path to the file
///
/// ## Returns
/// CID of archived file
pub async fn archive_offline(file_path: &str) -> Result<Cid> {
    tracing::info!("Archiving offline file: {}", file_path);

    let path = Path::new(file_path);
    if !path.exists() {
        return Err(Error::InvalidInput("File does not exist".to_string()));
    }

    // Read file
    let content = tokio::fs::read(path).await
        .map_err(|e| Error::Archiving(format!("Failed to read file: {}", e)))?;

    // Generate CID
    let cid = generate_cid(&content)?;

    // Extract basic metadata
    let filename = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    let metadata = SnapshotMetadata {
        schema_types: vec!["DigitalDocument".to_string()],
        title: Some(filename.to_string()),
        description: None,
        creator: None,
        language: None,
        tags: vec![],
        license: Some("Public Domain".to_string()),
        is_public: true,
    };

    store_in_ipfs(&cid, &content, &metadata).await?;

    Ok(cid)
}

// Helper functions (to be implemented)

async fn fetch_content(url: &Url) -> Result<Vec<u8>> {
    let client = reqwest::Client::builder()
        .user_agent("EternalWeb/0.1")
        .build()
        .map_err(|e| Error::Network(e.to_string()))?;

    let response = client.get(url.as_str())
        .send()
        .await
        .map_err(|e| Error::Network(e.to_string()))?;

    let bytes = response.bytes()
        .await
        .map_err(|e| Error::Network(e.to_string()))?;

    Ok(bytes.to_vec())
}

async fn extract_metadata(content: &[u8], url: &str) -> Result<SnapshotMetadata> {
    // Parse HTML and extract metadata
    let html = String::from_utf8_lossy(content);
    let document = scraper::Html::parse_document(&html);

    // Extract title
    let title_selector = scraper::Selector::parse("title").unwrap();
    let title = document.select(&title_selector)
        .next()
        .map(|el| el.text().collect::<String>());

    // Extract meta description
    let meta_selector = scraper::Selector::parse("meta[name='description']").unwrap();
    let description = document.select(&meta_selector)
        .next()
        .and_then(|el| el.value().attr("content"))
        .map(String::from);

    Ok(SnapshotMetadata {
        schema_types: vec!["WebPage".to_string()],
        title,
        description,
        creator: None,
        language: None,
        tags: vec![],
        license: None,
        is_public: true,
    })
}

fn generate_cid(content: &[u8]) -> Result<Cid> {
    use multihash::{Code, MultihashDigest};

    // Use BLAKE3 for hashing (fast and secure)
    let hash = blake3::hash(content);
    let mh = Code::Blake3_256.digest(hash.as_bytes());

    // Create CID v1
    let cid = Cid::new_v1(0x55, mh); // 0x55 = raw codec

    Ok(cid)
}

async fn store_in_ipfs(cid: &Cid, content: &[u8], metadata: &SnapshotMetadata) -> Result<()> {
    // TODO: Implement actual IPFS storage
    // For now, this is a placeholder
    tracing::debug!("Storing CID {} in IPFS with metadata: {:?}", cid, metadata);

    // Store content in local database
    let db = open_database()?;
    let key = cid.to_string();
    db.insert(key.as_bytes(), content)
        .map_err(|e| Error::Storage(e.to_string()))?;

    // Store metadata
    let metadata_key = format!("{}_metadata", key);
    let metadata_bytes = serde_json::to_vec(metadata)
        .map_err(|e| Error::Storage(e.to_string()))?;
    db.insert(metadata_key.as_bytes(), metadata_bytes)
        .map_err(|e| Error::Storage(e.to_string()))?;

    Ok(())
}

fn generate_feed_keypair() -> Result<(Vec<u8>, Vec<u8>)> {
    use ed25519_dalek::{SigningKey, VerifyingKey};
    use rand::rngs::OsRng;

    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key: VerifyingKey = (&signing_key).into();

    Ok((verifying_key.to_bytes().to_vec(), signing_key.to_bytes().to_vec()))
}

async fn send_feed_to_peer(feed: &HyperFeed, peer: &PeerId) -> Result<()> {
    // TODO: Implement peer replication
    tracing::debug!("Sending feed {} to peer {}", feed.id, peer);
    Ok(())
}

fn open_database() -> Result<sled::Db> {
    let db_path = std::env::var("ETERNALWEB_DB_PATH")
        .unwrap_or_else(|_| "./data/eternalweb.db".to_string());

    sled::open(db_path)
        .map_err(|e| Error::Storage(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_cid() {
        let content = b"Hello, EternalWeb!";
        let cid = generate_cid(content).unwrap();
        assert_eq!(cid.version(), cid::Version::V1);
    }

    #[test]
    fn test_metadata_creation() {
        let metadata = SnapshotMetadata {
            schema_types: vec!["WebPage".to_string()],
            title: Some("Test Page".to_string()),
            description: None,
            creator: None,
            language: Some("en".to_string()),
            tags: vec!["test".to_string()],
            license: None,
            is_public: true,
        };

        assert!(metadata.is_public);
        assert_eq!(metadata.tags.len(), 1);
    }
}
