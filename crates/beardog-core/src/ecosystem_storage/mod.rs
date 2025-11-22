//! # Ecosystem Storage System
//!
//! Provides comprehensive ecosystem storage capabilities enabling distributed, scalable,
//! and resilient data storage across the BearDog ecosystem through a unified,
//! extensible, and type-safe interface.
//!
//! ## Overview
//!
//! The ecosystem storage system offers:
//! - **Multi-backend support**: Memory, filesystem, distributed storage
//! - **Caching layer**: Performance optimization with configurable policies
//! - **Replication**: Data consistency across distributed nodes
//! - **Metrics**: Real-time monitoring and performance tracking
//! - **Type safety**: Strongly-typed storage operations
//!
//! ## Module Organization
//!
//! - `types` - Core storage types and data structures
//! - Storage configuration
//! - Storage management (main API)
//! - Storage backend implementations
//! - Caching layer
//! - Data replication
//! - Metrics and monitoring
//! - Storage operations
//!
//! ## Example
//!
//! ```rust,ignore
//! use beardog_core::ecosystem_storage::{EcosystemStorageManager, EcosystemStorageConfig};
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! let config = EcosystemStorageConfig::default();
//! let storage = EcosystemStorageManager::new(config)?;
//!
//! // Store data
//! storage.store("key", b"value").await?;
//!
//! // Retrieve data
//! let data = storage.retrieve("key").await?;
//! # Ok(())
//! # }
//! ```

/// Storage backend implementations and traits
pub mod backends;

/// Caching layer and cache management
pub mod cache;

/// Configuration management for ecosystem storage
pub mod config;
/// Storage management and orchestration
pub mod manager;
/// Metrics and monitoring
pub mod metrics;
/// Storage operations
pub mod operations;
/// Data replication
pub mod replication;
/// Storage type definitions
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

#[cfg(test)]
mod tests;
