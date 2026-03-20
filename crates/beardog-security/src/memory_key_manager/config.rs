// SPDX-License-Identifier: AGPL-3.0-only

//! Configuration for memory-based key manager
//!
//! This module provides configuration structures for in-memory key management.

use serde::{Deserialize, Serialize};

/// Memory key manager configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryKeyConfig {
    /// Maximum number of keys to store in memory
    /// Number of `max_keys`
    pub max_keys: usize,
    /// Key expiration time in seconds (0 = no expiration)
    /// Number of `key_expiration_seconds`
    pub key_expiration_seconds: u64,
    /// Enable key rotation
    /// Whether `enable_rotation` is enabled
    pub enable_rotation: bool,
}

impl Default for MemoryKeyConfig {
    fn default() -> Self {
        Self {
            max_keys: beardog_errors::process_env::var("BEARDOG_MAX_KEYS")
                .ok()
                .and_then(|k| k.parse().ok())
                .unwrap_or(1000), // 1000 keys default
            key_expiration_seconds: 0,
            enable_rotation: false,
        }
    }
}

/// Type alias for backwards compatibility
pub type KeyManagerConfig = MemoryKeyConfig;

/// Key storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyStorageConfig {
    /// Storage backend type
    /// The backend value
    pub backend: String,
    /// Maximum storage capacity
    /// Number of `max_capacity`
    pub max_capacity: usize,
    /// Enable encryption at rest
    /// Whether `encrypt_at_rest` is enabled
    pub encrypt_at_rest: bool,
}

impl Default for KeyStorageConfig {
    fn default() -> Self {
        Self {
            backend: "memory".to_string(),
            max_capacity: beardog_errors::process_env::var("BEARDOG_KEY_STORAGE_CAPACITY")
                .ok()
                .and_then(|c| c.parse().ok())
                .unwrap_or(10000), // 10000 keys default
            encrypt_at_rest: true,
        }
    }
}
