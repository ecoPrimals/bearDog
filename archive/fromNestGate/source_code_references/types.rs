

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompressionAlgorithm {

    Off,

    Lz4,

    Zstd,

    Gzip,

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatasetProperty {

    pub name: String,

    pub value: String,
}

impl DatasetProperty {

    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StorageTier {

    Hot,

    Warm,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsCapabilities {

    pub compression_algorithms: Vec<CompressionAlgorithm>,

    pub deduplication_support: bool,

    pub encryption_support: bool,

    pub snapshot_support: bool,

    pub replication_support: bool,

    pub max_pool_size: u64,

    pub max_datasets_per_pool: u32,

    pub zfs_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierPerformanceTarget {

    pub target_iops: u32,

    pub target_bandwidth_mbps: f64,

    pub target_latency_ms: f64,

    pub target_availability: f64,

    pub target_durability_nines: u32,

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