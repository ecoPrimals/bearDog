//! ZFS type definitions
//!
//! Common types used across the ZFS system

use serde::{Serialize, Deserialize};

/// Compression algorithms supported by ZFS
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompressionAlgorithm {
    /// No compression
    Off,
    /// LZ4 compression (fast)
    Lz4,
    /// ZSTD compression (balanced)
    Zstd,
    /// GZIP compression (level 6)
    Gzip,
    /// GZIP compression (level 9, maximum)
    Gzip9,
}

impl Default for CompressionAlgorithm {
    fn default() -> Self {
        Self::Lz4
    }
}

impl std::fmt::Display for CompressionAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Off => write!(f, "off"),
            Self::Lz4 => write!(f, "lz4"),
            Self::Zstd => write!(f, "zstd"),
            Self::Gzip => write!(f, "gzip"),
            Self::Gzip9 => write!(f, "gzip-9"),
        }
    }
}

/// ZFS dataset property
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatasetProperty {
    /// Property name
    pub name: String,
    /// Property value
    pub value: String,
}

impl DatasetProperty {
    /// Create a new dataset property
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

/// Storage tier for data classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StorageTier {
    /// Hot storage for frequently accessed data
    Hot,
    /// Warm storage for moderately accessed data
    Warm,
    /// Cold storage for infrequently accessed data
    Cold,
}

impl Default for StorageTier {
    fn default() -> Self {
        Self::Warm
    }
}

impl std::fmt::Display for StorageTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hot => write!(f, "hot"),
            Self::Warm => write!(f, "warm"),
            Self::Cold => write!(f, "cold"),
        }
    }
}

/// ZFS capabilities and feature support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsCapabilities {
    /// Supported compression algorithms
    pub compression_algorithms: Vec<CompressionAlgorithm>,
    /// Deduplication support
    pub deduplication_support: bool,
    /// Encryption support
    pub encryption_support: bool,
    /// Snapshot support
    pub snapshot_support: bool,
    /// Replication support
    pub replication_support: bool,
    /// Maximum pool size in bytes
    pub max_pool_size: u64,
    /// Maximum dataset count per pool
    pub max_datasets_per_pool: u32,
    /// ZFS version
    pub zfs_version: String,
}

/// Performance target for a storage tier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierPerformanceTarget {
    /// Target IOPS
    pub target_iops: u32,
    /// Target bandwidth in MB/s
    pub target_bandwidth_mbps: f64,
    /// Target latency in milliseconds
    pub target_latency_ms: f64,
    /// Target availability percentage
    pub target_availability: f64,
    /// Target durability (number of 9s)
    pub target_durability_nines: u32,
    /// Storage tier this target applies to
    pub tier: StorageTier,
}

impl Default for ZfsCapabilities {
    fn default() -> Self {
        Self {
            compression_algorithms: vec![
                CompressionAlgorithm::Lz4,
                CompressionAlgorithm::Zstd,
                CompressionAlgorithm::Gzip,
            ],
            deduplication_support: true,
            encryption_support: true,
            snapshot_support: true,
            replication_support: true,
            max_pool_size: 256 * 1024 * 1024 * 1024 * 1024, // 256TB
            max_datasets_per_pool: 1000,
            zfs_version: "2.1.0".to_string(),
        }
    }
}

impl Default for TierPerformanceTarget {
    fn default() -> Self {
        Self {
            target_iops: 1000,
            target_bandwidth_mbps: 100.0,
            target_latency_ms: 10.0,
            target_availability: 99.9,
            target_durability_nines: 11,
            tier: StorageTier::Hot,
        }
    }
} 