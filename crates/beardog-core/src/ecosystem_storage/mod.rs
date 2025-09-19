// **ECOSYSTEM STORAGE SYSTEM** - Modularized Architecture
//
// This module provides a comprehensive ecosystem storage system that enables
// distributed, scalable, and resilient data storage across the BearDog ecosystem
// through a unified, extensible, and type-safe interface.
//
// ## Module Organization
//
// - `types` - Core storage types, enums, and data structures
// - `config` - Storage configuration and settings
// - `manager` - Core storage management implementation
// - `backends` - Storage backend implementations and traits
// - `cache` - Caching layer and cache management
// - `replication` - Data replication and consistency
// - `metrics` - Storage metrics and monitoring
// - `operations` - Storage operation handling

// Public API modules
pub mod backends;
pub mod cache;
/// Configuration management
/// Configuration management
pub mod config;
pub mod manager;
pub mod metrics;
pub mod operations;
pub mod replication;
pub mod types;

// Re-export main types for backwards compatibility
pub use backends::StorageBackend;
pub use cache::{CacheConfig, CacheManager};
pub use config::EcosystemStorageConfig;
pub use manager::EcosystemStorageManager;
pub use metrics::StorageMetrics;
pub use operations::{StorageRequest, StorageResponse};
pub use replication::ReplicationStatus;
pub use types::*;
