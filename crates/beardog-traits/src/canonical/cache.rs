// SPDX-License-Identifier: AGPL-3.0-or-later

//! String-keyed cache providers with TTL support and extended batch helpers.

use super::base::BaseProvider;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

/// Minimal key/value cache with string payloads and optional TTL.
pub trait CacheProvider: BaseProvider {
    /// Sets value
    fn set(
        key: &str,
        value: &str,
        ttl: Option<Duration>,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Gets value
    fn get(
        key: &str,
    ) -> impl std::future::Future<Output = Result<Option<String>, BearDogError>> + Send;

    /// Removes item
    fn remove(key: &str) -> impl std::future::Future<Output = Result<bool, BearDogError>> + Send;

    /// Returns whether `key` is present.
    fn exists(key: &str) -> impl std::future::Future<Output = Result<bool, BearDogError>> + Send;

    /// Drops all entries in the namespace.
    fn clear(&self) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Gets many
    fn get_many(
        &self,
        keys: &[&str],
    ) -> impl std::future::Future<Output = Result<HashMap<String, String>, BearDogError>> + Send;

    /// Sets many
    fn set_many(
        &self,
        data: HashMap<&str, &str>,
        ttl: Option<Duration>,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Removes many
    fn remove_many(
        &self,
        keys: &[&str],
    ) -> impl std::future::Future<Output = Result<u64, BearDogError>> + Send;

    /// Gets stats
    fn get_stats(
        &self,
    ) -> impl std::future::Future<Output = Result<CacheStats, BearDogError>> + Send;

    /// Refreshes or sets expiry on an existing key.
    fn expire(
        key: &str,
        ttl: Duration,
    ) -> impl std::future::Future<Output = Result<bool, BearDogError>> + Send;

    /// Gets ttl
    fn get_ttl(
        key: &str,
    ) -> impl std::future::Future<Output = Result<Option<Duration>, BearDogError>> + Send;
}

/// Aggregate cache performance and memory view.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CacheStats {
    /// Memory usage in bytes
    /// Number of `memory_usage_bytes`
    pub memory_usage_bytes: u64,
    /// Number of evictions
    /// Number of eviction
    pub eviction_count: u64,
    /// Hit rate percentage
    /// The hit rate percent value
    pub hit_rate_percent: f64,
    /// Average TTL in seconds
    /// The average ttl seconds value
    pub average_ttl_seconds: f64,
    /// Hot keys list
    /// Collection of hot keys
    pub hot_keys: Vec<String>,
}

/// Extended hit/miss and memory stats for cache dashboards.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedCacheStats {
    /// The hit rate value
    pub hit_rate: f64,
    /// The miss rate value
    pub miss_rate: f64,
    /// Number of eviction
    pub eviction_count: u64,
    /// Number of `memory_usage`
    pub memory_usage: u64,
}

/// Batch and pattern operations for high-throughput caches.
pub trait EnhancedCacheProvider: CacheProvider {
    /// Multi-get returning only found keys.
    fn batch_get(
        keys: &[&str],
    ) -> impl std::future::Future<Output = Result<HashMap<String, String>, BearDogError>> + Send;

    /// Multi-set for JSON values.
    fn batch_set(
        data: HashMap<&str, Value>,
        ttl: Option<Duration>,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Deletes all keys matching a glob-style `pattern`.
    fn invalidate_pattern(
        pattern: &str,
    ) -> impl std::future::Future<Output = Result<u64, BearDogError>> + Send;

    /// Preloads `keys` from origin into the cache.
    fn warm_cache(
        keys: &[&str],
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Gets `advanced_stats`
    fn get_advanced_stats(
        &self,
    ) -> impl std::future::Future<Output = Result<AdvancedCacheStats, BearDogError>> + Send;
}
