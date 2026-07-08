//! # EternalWeb Core
//!
//! Core library for EternalWeb - A decentralized, peer-to-peer web archiving and sharing platform.
//!
//! ## Features
//! - Decentralized archiving with IPFS and Hypercore
//! - P2P networking with libp2p
//! - Post-quantum cryptography
//! - AI-powered semantic search
//! - Blockchain-based governance
//! - Web 3.0 and Web 4.0 ready
//!
//! ## Architecture
//! The library is organized into modules:
//! - `archiving`: Content preservation and versioning
//! - `networking`: P2P communication and discovery
//! - `security`: Cryptography and trust management
//! - `search`: Semantic search and discovery
//! - `blockchain`: DAO governance and immutability
//! - `ai`: Machine learning and autonomous curation
//! - `collaboration`: Collective tools and sharing
//! - `incentives`: Reputation and badge system
//!
//! ## Example
//! ```no_run
//! use eternalweb_core::archiving::snapshot_page;
//! use eternalweb_core::networking::bootstrap_network;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Start the network
//!     let swarm = bootstrap_network().await?;
//!
//!     // Archive a webpage
//!     let cid = snapshot_page("https://example.com").await?;
//!     println!("Archived with CID: {}", cid);
//!
//!     Ok(())
//! }
//! ```

#![allow(dead_code)]
#![allow(unused_variables)]

pub mod archiving;
pub mod networking;
pub mod security;
pub mod search;
pub mod blockchain;
pub mod ai;
pub mod collaboration;
pub mod incentives;
pub mod utils;

// Re-export commonly used types
pub use cid::Cid;
pub use libp2p::PeerId;

/// Result type for EternalWeb operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for EternalWeb
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Network-related errors
    #[error("Network error: {0}")]
    Network(String),

    /// Storage errors
    #[error("Storage error: {0}")]
    Storage(String),

    /// Cryptography errors
    #[error("Cryptography error: {0}")]
    Crypto(String),

    /// Archiving errors
    #[error("Archiving error: {0}")]
    Archiving(String),

    /// Search errors
    #[error("Search error: {0}")]
    Search(String),

    /// Blockchain errors
    #[error("Blockchain error: {0}")]
    Blockchain(String),

    /// AI/ML errors
    #[error("AI error: {0}")]
    AI(String),

    /// Invalid input
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Generic error
    #[error("{0}")]
    Other(String),
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error::Other(err.to_string())
    }
}

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Protocol version for compatibility checking
pub const PROTOCOL_VERSION: &str = "0.1.0";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
