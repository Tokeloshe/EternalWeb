//! Configuration management

use serde::{Deserialize, Serialize};

/// EternalWeb configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub network: NetworkConfig,
    pub storage: StorageConfig,
    pub security: SecurityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub bootstrap_peers: Vec<String>,
    pub listen_port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub data_dir: String,
    pub max_storage_gb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub enable_encryption: bool,
    pub key_type: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            network: NetworkConfig {
                bootstrap_peers: vec![],
                listen_port: 0,
            },
            storage: StorageConfig {
                data_dir: "./data".to_string(),
                max_storage_gb: 100,
            },
            security: SecurityConfig {
                enable_encryption: true,
                key_type: "ed25519".to_string(),
            },
        }
    }
}
