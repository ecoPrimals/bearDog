//! Storage Configuration
//!
//! This module defines storage configurations for production deployments.

use serde::{Deserialize, Serialize};

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Enable storage
    pub enabled: bool,
    /// Storage volumes
    pub volumes: Vec<StorageVolume>,
}

/// Storage volume
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageVolume {
    /// Volume name
    pub name: String,
    /// Volume size in GB
    pub size_gb: u64,
    /// Volume type
    pub volume_type: String,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            volumes: Vec::new(),
        }
    }
}
