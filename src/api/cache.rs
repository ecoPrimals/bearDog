//! High-Performance Caching System for BearDog API
//!
//! Provides intelligent caching with multiple backends:
//! - Redis for distributed caching
//! - In-memory for single-node deployments
//! - Smart cache key generation and TTL management

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use tracing::{debug, error, warn};

/// Cache provider trait for different implementations
#[async_trait]
pub trait CacheProvider {
    /// Get value from cache
    async fn get(&self, key: &str) -> Option<String>;

    /// Set value in cache with TTL
    async fn set(&self, key: &str, value: &str, ttl: Duration) -> bool;

    /// Delete value from cache
    async fn delete(&self, key: &str) -> bool;

    /// Check if key exists
    async fn exists(&self, key: &str) -> bool;

    /// Clear all cache entries (use with caution)
    async fn clear(&self) -> bool;

    /// Get cache statistics
    async fn stats(&self) -> CacheStats;
}

/// Cache statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    /// Number of cache hits
    pub hits: u64,
    /// Number of cache misses
    pub misses: u64,
    /// Total number of entries in cache
    pub entries: u64,
    /// Hit rate as a percentage (0.0 to 1.0)
    pub hit_rate: f64,
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
}

impl Default for CacheStats {
    fn default() -> Self {
        Self::new()
    }
}

impl CacheStats {
    /// Create a new CacheStats instance with zero values
    pub fn new() -> Self {
        Self {
            hits: 0,
            misses: 0,
            entries: 0,
            hit_rate: 0.0,
            memory_usage_bytes: 0,
        }
    }

    /// Record a cache hit and update hit rate
    pub fn record_hit(&mut self) {
        self.hits += 1;
        self.update_hit_rate();
    }

    /// Record a cache miss and update hit rate
    pub fn record_miss(&mut self) {
        self.misses += 1;
        self.update_hit_rate();
    }

    fn update_hit_rate(&mut self) {
        let total = self.hits + self.misses;
        self.hit_rate = if total > 0 {
            self.hits as f64 / total as f64
        } else {
            0.0
        };
    }
}

/// Redis-based distributed cache
pub struct RedisCache {
    /// Redis client connection
    client: redis::Client,
    /// Cache statistics tracking
    stats: Arc<RwLock<CacheStats>>,
}

impl RedisCache {
    /// Create a new Redis cache instance
    pub async fn new(
    ) -> Result<Box<dyn CacheProvider + Send + Sync>, Box<dyn std::error::Error + Send + Sync>>
    {
        // For now, return error to fall back to in-memory
        Err("Redis not implemented yet".into())
    }
}

#[async_trait]
impl CacheProvider for RedisCache {
    async fn get(&self, key: &str) -> Option<String> {
        match self.client.get_async_connection().await {
            Ok(mut conn) => {
                match redis::cmd("GET")
                    .arg(key)
                    .query_async::<_, Option<String>>(&mut conn)
                    .await
                {
                    Ok(Some(value)) => {
                        self.stats.write().await.record_hit();
                        debug!("Cache HIT: {}", key);
                        Some(value)
                    }
                    Ok(None) => {
                        self.stats.write().await.record_miss();
                        debug!("Cache MISS: {}", key);
                        None
                    }
                    Err(e) => {
                        warn!("Redis GET error for key {}: {}", key, e);
                        self.stats.write().await.record_miss();
                        None
                    }
                }
            }
            Err(e) => {
                warn!("Redis connection error: {}", e);
                self.stats.write().await.record_miss();
                None
            }
        }
    }

    async fn set(&self, key: &str, value: &str, ttl: Duration) -> bool {
        match self.client.get_async_connection().await {
            Ok(mut conn) => {
                let ttl_secs = ttl.as_secs() as usize;
                match redis::cmd("SETEX")
                    .arg(key)
                    .arg(ttl_secs)
                    .arg(value)
                    .query_async::<_, ()>(&mut conn)
                    .await
                {
                    Ok(_) => {
                        debug!("Cache SET: {} (TTL: {}s)", key, ttl_secs);
                        true
                    }
                    Err(e) => {
                        warn!("Redis SET error for key {}: {}", key, e);
                        false
                    }
                }
            }
            Err(e) => {
                warn!("Redis connection error: {}", e);
                false
            }
        }
    }

    async fn delete(&self, key: &str) -> bool {
        match self.client.get_async_connection().await {
            Ok(mut conn) => {
                match redis::cmd("DEL")
                    .arg(key)
                    .query_async::<_, i32>(&mut conn)
                    .await
                {
                    Ok(deleted) => {
                        debug!("Cache DEL: {} (deleted: {})", key, deleted);
                        deleted > 0
                    }
                    Err(e) => {
                        warn!("Redis DEL error for key {}: {}", key, e);
                        false
                    }
                }
            }
            Err(e) => {
                warn!("Redis connection error: {}", e);
                false
            }
        }
    }

    async fn exists(&self, key: &str) -> bool {
        match self.client.get_async_connection().await {
            Ok(mut conn) => {
                match redis::cmd("EXISTS")
                    .arg(key)
                    .query_async::<_, i32>(&mut conn)
                    .await
                {
                    Ok(exists) => exists > 0,
                    Err(e) => {
                        warn!("Redis EXISTS error for key {}: {}", key, e);
                        false
                    }
                }
            }
            Err(e) => {
                warn!("Redis connection error: {}", e);
                false
            }
        }
    }

    async fn clear(&self) -> bool {
        match self.client.get_async_connection().await {
            Ok(mut conn) => match redis::cmd("FLUSHDB").query_async::<_, ()>(&mut conn).await {
                Ok(_) => {
                    warn!("⚠️  Redis cache cleared (FLUSHDB)");
                    true
                }
                Err(e) => {
                    error!("Redis FLUSHDB error: {}", e);
                    false
                }
            },
            Err(e) => {
                warn!("Redis connection error: {}", e);
                false
            }
        }
    }

    async fn stats(&self) -> CacheStats {
        self.stats.read().await.clone()
    }
}

/// In-memory cache entry
#[derive(Debug, Clone)]
struct CacheEntry {
    value: String,
    expires_at: SystemTime,
}

impl CacheEntry {
    fn new(value: String, ttl: Duration) -> Self {
        Self {
            value,
            expires_at: SystemTime::now() + ttl,
        }
    }

    fn is_expired(&self) -> bool {
        SystemTime::now() > self.expires_at
    }
}

/// In-memory cache implementation
pub struct InMemoryCache {
    /// Hash map storing cached entries
    data: Arc<RwLock<HashMap<String, CacheEntry>>>,
    /// Cache statistics tracking
    stats: Arc<RwLock<CacheStats>>,
}

impl Default for InMemoryCache {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryCache {
    /// Create a new in-memory cache instance
    pub fn new() -> Self {
        debug!("🧠 In-memory cache initialized");
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(CacheStats::new())),
        }
    }

    /// Background task to clean up expired entries
    pub async fn cleanup_expired(&self) {
        let mut data = self.data.write().await;
        let initial_count = data.len();

        data.retain(|_key, entry| !entry.is_expired());

        let cleaned_count = initial_count - data.len();
        if cleaned_count > 0 {
            debug!("Cleaned up {} expired cache entries", cleaned_count);
        }

        // Update stats
        let mut stats = self.stats.write().await;
        stats.entries = data.len() as u64;
        stats.memory_usage_bytes = data.len() as u64 * 64; // Rough estimate
    }
}

#[async_trait]
impl CacheProvider for InMemoryCache {
    async fn get(&self, key: &str) -> Option<String> {
        let data = self.data.read().await;

        if let Some(entry) = data.get(key) {
            if !entry.is_expired() {
                self.stats.write().await.record_hit();
                debug!("Cache HIT: {}", key);
                return Some(entry.value.clone());
            }
        }

        self.stats.write().await.record_miss();
        debug!("Cache MISS: {}", key);
        None
    }

    async fn set(&self, key: &str, value: &str, ttl: Duration) -> bool {
        let entry = CacheEntry::new(value.to_string(), ttl);

        let mut data = self.data.write().await;
        data.insert(key.to_string(), entry);

        // Update stats
        let mut stats = self.stats.write().await;
        stats.entries = data.len() as u64;
        stats.memory_usage_bytes = data.len() as u64 * 64; // Rough estimate

        debug!("Cache SET: {} (TTL: {:?})", key, ttl);
        true
    }

    async fn delete(&self, key: &str) -> bool {
        let mut data = self.data.write().await;
        let removed = data.remove(key).is_some();

        if removed {
            // Update stats
            let mut stats = self.stats.write().await;
            stats.entries = data.len() as u64;
            stats.memory_usage_bytes = data.len() as u64 * 64; // Rough estimate

            debug!("Cache DEL: {}", key);
        }

        removed
    }

    async fn exists(&self, key: &str) -> bool {
        let data = self.data.read().await;

        if let Some(entry) = data.get(key) {
            !entry.is_expired()
        } else {
            false
        }
    }

    async fn clear(&self) -> bool {
        let mut data = self.data.write().await;
        let cleared_count = data.len();
        data.clear();

        // Reset stats
        let mut stats = self.stats.write().await;
        stats.entries = 0;
        stats.memory_usage_bytes = 0;

        warn!("⚠️  In-memory cache cleared ({} entries)", cleared_count);
        true
    }

    async fn stats(&self) -> CacheStats {
        // Trigger cleanup before returning stats
        self.cleanup_expired().await;
        self.stats.read().await.clone()
    }
}

/// Smart cache key generator
pub struct CacheKeyBuilder {
    /// Prefix for generated cache keys
    prefix: String,
}

impl CacheKeyBuilder {
    /// Create a new cache key builder for a specific service
    pub fn new(service: &str) -> Self {
        Self {
            prefix: format!("beardog:{service}:"),
        }
    }

    /// Generate cache key for API responses
    pub fn api_response(&self, endpoint: &str, params: &[(&str, &str)]) -> String {
        let params_str = params
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("&");

        format!(
            "{}api:{}:{}",
            self.prefix,
            endpoint,
            if params_str.is_empty() {
                "no_params".to_string()
            } else {
                params_str
            }
        )
    }

    /// Generate cache key for user sessions
    pub fn user_session(&self, user_id: &str) -> String {
        format!("{}session:{}", self.prefix, user_id)
    }

    /// Generate cache key for threat analysis
    pub fn threat_analysis(&self, event_hash: &str) -> String {
        format!("{}threat:{}", self.prefix, event_hash)
    }

    /// Generate cache key for compliance reports
    pub fn compliance_report(&self, report_type: &str, filters_hash: &str) -> String {
        format!("{}compliance:{}:{}", self.prefix, report_type, filters_hash)
    }

    /// Generate cache key for node status
    pub fn node_status(&self, node_id: &str) -> String {
        format!("{}node:{}", self.prefix, node_id)
    }
}

/// Cache TTL constants for different data types
pub mod ttl {
    use std::time::Duration;

    /// API responses - 5 minutes
    pub const API_RESPONSE: Duration = Duration::from_secs(300);

    /// User sessions - 1 hour
    pub const USER_SESSION: Duration = Duration::from_secs(3600);

    /// Threat analysis - 10 minutes (security data changes frequently)
    pub const THREAT_ANALYSIS: Duration = Duration::from_secs(600);

    /// Compliance reports - 30 minutes
    pub const COMPLIANCE_REPORT: Duration = Duration::from_secs(1800);

    /// Node status - 2 minutes
    pub const NODE_STATUS: Duration = Duration::from_secs(120);

    /// Configuration data - 1 hour
    pub const CONFIG_DATA: Duration = Duration::from_secs(3600);

    /// Static content - 24 hours
    pub const STATIC_CONTENT: Duration = Duration::from_secs(86400);
}
