// SPDX-License-Identifier: AGPL-3.0-only

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use super::base::BaseProvider;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

#[allow(clippy::type_complexity)]
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

    fn exists(key: &str) -> impl std::future::Future<Output = Result<bool, BearDogError>> + Send;

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

    fn expire(
        key: &str,
        ttl: Duration,
    ) -> impl std::future::Future<Output = Result<bool, BearDogError>> + Send;

    /// Gets ttl
    fn get_ttl(
        key: &str,
    ) -> impl std::future::Future<Output = Result<Option<Duration>, BearDogError>> + Send;
}

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

// Temporary type definitions
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

#[allow(clippy::type_complexity)]
pub trait EnhancedCacheProvider: CacheProvider {
    fn batch_get(
        keys: &[&str],
    ) -> impl std::future::Future<Output = Result<HashMap<String, String>, BearDogError>> + Send;

    fn batch_set(
        data: HashMap<&str, Value>,
        ttl: Option<Duration>,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    fn invalidate_pattern(
        pattern: &str,
    ) -> impl std::future::Future<Output = Result<u64, BearDogError>> + Send;

    fn warm_cache(
        keys: &[&str],
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Gets `advanced_stats`
    fn get_advanced_stats(
        &self,
    ) -> impl std::future::Future<Output = Result<AdvancedCacheStats, BearDogError>> + Send;
}
