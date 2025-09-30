// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod config;
pub mod l1_cache;
pub mod l2_cache;
pub mod l3_cache;
pub mod manager;
pub mod memory_manager;
pub mod metrics;
pub mod strategies;

pub use config::{CacheConfig, EvictionStrategy, WarmingStrategy};
pub use l1_cache::{L1Cache, L1CacheConfig, L1CacheEntry, L1CacheStats};
pub use l2_cache::{L2Cache, L2CacheConfig, L2CacheEntry, L2CacheStats};
pub use l3_cache::{L3Cache, L3CacheConfig, L3CacheEntry, L3CacheStats};
pub use manager::AdvancedCacheManager;
pub use memory_manager::{MemoryManager, MemoryStats};
pub use metrics::CacheMetrics;
pub use strategies::{EvictionHandler, WarmingHandler};
