

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use tracing::{debug, error, warn, info};
use std::sync::atomic::AtomicBool;

use beardog_traits::canonical::CacheProvider as CanonicalCacheProvider;
use beardog_traits::canonical::BaseProvider;
use beardog_errors::BearDogResult;

#[allow(async_fn_in_trait)]
#[deprecated(since = "3.1.0", note = "Use EnhancedCacheProvider instead")]
#[deprecated(since = "3.1.0", note = "Use EnhancedCacheProvider instead")]
pub trait SimpleCacheProvider: Send + Sync {

    async fn get(&self, key: &str) -> Option<String>;

    async fn set(&self, key: &str, value: &str, ttl: Duration) -> bool;

    async fn delete(&self, key: &str) -> bool;

    async fn exists(&self, key: &str) -> bool;

    async fn clear(&self) -> bool;

    async fn stats(&self) -> CacheStats;
}

pub use SimpleCacheProvider as CacheProvider;

#[derive(Clone)]
pub enum CacheProviderType {
    InMemory(InMemoryCache),
    Redis(RedisCache),
}

impl CacheProviderType {

    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {

        match RedisCache::new().await {
            Ok(redis_cache) => Ok(CacheProviderType::Redis(redis_cache)),
            Err(_) => Ok(CacheProviderType::InMemory(InMemoryCache::new())),
        }
    }
}

impl EnhancedCacheProvider for CacheProviderType {
    async fn get(&self, key: &str) -> Option<String> {
        match self {
            CacheProviderType::InMemory(cache) => cache.get(key).await,
            CacheProviderType::Redis(cache) => cache.get(key).await,
        }
    }

    async fn set(&self, key: &str, value: &str, ttl: Duration) -> bool {
        match self {
            CacheProviderType::InMemory(cache) => cache.set(key, value, ttl).await,
            CacheProviderType::Redis(cache) => cache.set(key, value, ttl).await,
        }
    }

    async fn delete(&self, key: &str) -> bool {
        match self {
            CacheProviderType::InMemory(cache) => cache.delete(key).await,
            CacheProviderType::Redis(cache) => cache.delete(key).await,
        }
    }

    async fn exists(&self, key: &str) -> bool {
        match self {
            CacheProviderType::InMemory(cache) => cache.exists(key).await,
            CacheProviderType::Redis(cache) => cache.exists(key).await,
        }
    }

    async fn clear(&self) -> bool {
        match self {
            CacheProviderType::InMemory(cache) => cache.clear().await,
            CacheProviderType::Redis(cache) => cache.clear().await,
        }
    }

    async fn stats(&self) -> CacheStats {
        match self {
            CacheProviderType::InMemory(cache) => cache.stats().await,
            CacheProviderType::Redis(cache) => cache.stats().await,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CacheConfig {

    pub max_entries: usize,

    pub default_ttl: Duration,

    pub compression_enabled: bool,

    pub cleanup_interval: Duration,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_entries: 10000,
            default_ttl: Duration::from_secs(3600), // 1 hour
            compression_enabled: false,
            cleanup_interval: Duration::from_secs(300), // 5 minutes
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {

    pub hits: u64,

    pub misses: u64,

    pub hit_rate: f64,

    pub entries: u64,

    pub memory_usage_bytes: u64,
}

impl Default for CacheStats {
    fn default() -> Self {
        Self {
            hits: 0,
            misses: 0,
            hit_rate: 0.0,
            entries: 0,
            memory_usage_bytes: 0,
        }
    }
}

impl CacheStats {

    pub fn record_hit(&mut self) {
        self.hits += 1;
        self.update_hit_rate();
    }

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

#[derive(Debug, Clone)]
struct CacheEntry {
    value: String,
    expires_at: u64,
}

impl CacheEntry {
    fn new(value: &str, ttl: Duration) -> Self {
        let expires_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_secs()
            + ttl.as_secs();
        
        Self { value, expires_at }
    }

    fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_secs();
        now >= self.expires_at
    }
}

#[derive(Clone)]
pub struct InMemoryCache {

    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,

    stats: Arc<RwLock<CacheStats>>,

    #[allow(dead_code)] // Will be used for advanced cache configuration
    config: CacheConfig,
}

impl InMemoryCache {

    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            stats: Arc::new(RwLock::new(CacheStats::default())),
            config: CacheConfig::default(),
        }
    }

    async fn cleanup_expired(&self) {
        let mut data = self.cache.write().await;
        let initial_count = data.len();
        data.retain(|_, entry| !entry.is_expired());
        let removed_count = initial_count - data.len();
        
        if removed_count > 0 {
            debug!("Cache cleanup: removed {} expired entries", removed_count);

            let mut stats = self.stats.write().await;
            stats.entries = data.len() as u64;
            stats.memory_usage_bytes = data.len() as u64 * 64; // Rough estimate
        }
    }
}

impl EnhancedCacheProvider for InMemoryCache {
    async fn get(&self, key: &str) -> Option<String> {
        let data = self.cache.read().await;
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
        let mut data = self.cache.write().await;
        let entry = CacheEntry::new(value.to_string(), ttl);
        data.insert(key.to_string(), entry);
        debug!("Cache SET: {} (TTL: {:?})", key, ttl);
        true
    }

    async fn delete(&self, key: &str) -> bool {
        let mut data = self.cache.write().await;
        let removed = data.remove(key).is_some();
        if removed {

            let mut stats = self.stats.write().await;
            stats.entries = data.len() as u64;
            stats.memory_usage_bytes = data.len() as u64 * 64; // Rough estimate
            debug!("Cache DEL: {}", key);
        }
        removed
    }

    async fn exists(&self, key: &str) -> bool {
        let data = self.cache.read().await;
        if let Some(entry) = data.get(key) {
            !entry.is_expired()
        } else {
            false
        }
    }

    async fn clear(&self) -> bool {
        let mut data = self.cache.write().await;
        let mut stats = self.stats.write().await;
        
        let cleared_count = data.len();
        data.clear();

        stats.entries = 0;
        stats.memory_usage_bytes = 0;
        warn!("⚠️  In-memory cache cleared ({} entries)", cleared_count);
        true
    }

    async fn stats(&self) -> CacheStats {

        self.cleanup_expired().await;
        self.stats.read().await.clone()
    }
}

#[derive(Clone)]
pub struct RedisCache {

    fallback_cache: Arc<InMemoryCache>,

    stats: Arc<RwLock<CacheStats>>,

    redis_available: Arc<AtomicBool>,
}

impl RedisCache {

    pub async fn new() -> Result<RedisCache, Box<dyn std::error::Error + Send + Sync>> {
        info!("🔄 Initializing Redis cache with in-memory fallback");

        let redis_available = Arc::new(AtomicBool::new(false));

        let fallback_cache = Arc::new(InMemoryCache::new(1000).await?);
        
        warn!("⚠️ Redis not available - using in-memory cache fallback");
        
        Ok(RedisCache {
            fallback_cache,
            stats: Arc::new(RwLock::new(CacheStats::default())),
            redis_available,
        })
    }

    async fn ensure_redis_connection(&self) -> bool {

        false
    }
}

impl EnhancedCacheProvider for RedisCache {
    async fn get(&self, key: &str) -> Option<String> {
        if self.ensure_redis_connection().await {

            None
        } else {

            self.fallback_cache.get(key).await
        }
    }

    async fn set(&self, key: &str, value: &str, ttl: Duration) -> bool {
        if self.ensure_redis_connection().await {

            false
        } else {

            self.fallback_cache.set(key, value, ttl).await
        }
    }

    async fn delete(&self, key: &str) -> bool {
        if self.ensure_redis_connection().await {

            false
        } else {

            self.fallback_cache.delete(key).await
        }
    }

    async fn exists(&self, key: &str) -> bool {
        if self.ensure_redis_connection().await {

            false
        } else {

            self.fallback_cache.exists(key).await
        }
    }

    async fn clear(&self) -> bool {
        if self.ensure_redis_connection().await {

            false
        } else {

            self.fallback_cache.clear().await
        }
    }

    async fn stats(&self) -> CacheStats {

        self.fallback_cache.stats().await
    }
}

pub struct CacheKeyBuilder {

    prefix: String,
}

impl CacheKeyBuilder {

    pub fn new(service: &str) -> Self {
        Self {
            prefix: format!("beardog:{service}:"),
        }
    }

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

    pub fn user_session(&self, user_id: &str) -> String {
        format_args!("{}session:{}", self.prefix, user_id).to_string()
    }

    pub fn threat_analysis(&self, event_hash: &str) -> String {
        format_args!("{}threat:{}", self.prefix, event_hash).to_string()
    }

    pub fn compliance_report(&self, report_type: &str, filters_hash: &str) -> String {
        format_args!("{}compliance:{}:{}", self.prefix, report_type, filters_hash).to_string()
    }

    pub fn node_status(&self, node_id: &str) -> String {
        format_args!("{}node:{}", self.prefix, node_id).to_string()
    }
}

pub use beardog_types::constants::unified::cache::ttl;
