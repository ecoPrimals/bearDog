

use beardog_core::zero_cost_architecture::{ZeroCostBearDog, ZeroCostCache, ZeroCostSecurity};
use beardog_errors::BearDogResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant, SystemTime};
use parking_lot::RwLock;

pub trait ZeroCostApiCache {
    type Key: Hash + Eq + Clone;
    type Value: Clone;

    async fn get_response(&self, key: &Self::Key) -> Option<Self::Value>;

    async fn set_response(&self, key: Self::Key, value: Self::Value, ttl: Duration) -> BearDogResult<()>;

    async fn remove_response(&self, key: &Self::Key) -> BearDogResult<bool>;

    async fn response_exists(&self, key: &Self::Key) -> bool;

    async fn cache_stats(&self) -> ApiCacheStats;
}

pub trait ZeroCostRateLimit {
    type ClientId: Hash + Eq + Clone;

    async fn check_limit(&self, client_id: &Self::ClientId) -> bool;

    async fn check_endpoint_limit(&self, client_id: &Self::ClientId, endpoint: &str) -> bool;

    async fn get_quota(&self, client_id: &Self::ClientId) -> RateQuota;

    async fn reset_client(&self, client_id: &Self::ClientId) -> BearDogResult<bool>;

    async fn rate_limit_stats(&self) -> RateLimitStats;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCacheStats {
    pub hits: u64,
    pub misses: u64,
    pub total_requests: u64,
    pub hit_rate: f64,
    pub cache_size: usize,
    pub memory_usage_bytes: u64,

pub struct RateQuota {
    pub remaining: u32,
    pub limit: u32,
    pub reset_time_seconds: u64,
    pub throttled: bool,

pub struct RateLimitStats {
    pub blocked_requests: u64,
    pub active_clients: usize,
    pub average_quota_usage: f64,

pub struct ZeroCostApiMemoryCache<K, V, const SIZE: usize = { beardog_types::constants::cache::STANDARD_CACHE_SIZE }, const TTL_SECONDS: u64 = { beardog_types::constants::cache::STANDARD_TTL.as_secs() }>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    data: RwLock<HashMap<K, CachedApiResponse<V>>>,
    access_order: RwLock<Vec<K>>,
    hits: AtomicU64,
    misses: AtomicU64,
    _phantom: PhantomData<(K, V)>,

#[derive(Debug, Clone)]
struct CachedApiResponse<V> {
    value: V,
    cached_at: Instant,
    ttl: Duration,
    access_count: u64,
impl<V> CachedApiResponse<V> {}

    fn new(value: V, ttl: Duration) -> Self {
        Self {
            value,
            cached_at: Instant::now(),
            ttl,
            access_count: 0,
        }
    }
    fn is_expired(&self) -> bool {
        self.cached_at.elapsed() > self.ttl
impl<K, V, const SIZE: usize, const TTL_SECONDS: u64> ZeroCostApiMemoryCache<K, V, SIZE, TTL_SECONDS>
    pub const fn new() -> Self {
            data: RwLock::new(HashMap::with_capacity(16)),
            access_order: RwLock::new(Vec::new()),
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
            _phantom: PhantomData,}

    fn evict_if_needed(&self) {
        let mut data = self.data.write();
        let mut access_order = self.access_order.write();

        data.retain(|k, entry| {
            if entry.is_expired() {
                access_order.retain(|access_k| access_k != k);
                false
            } else {
                true
            }
        });

        while data.len() >= SIZE && !access_order.is_empty() {
            if let Some(oldest_key) = access_order.remove(0) {
                data.remove(&oldest_key);
    fn update_access_order(&self, key: &K) {
        access_order.retain(|k| k != key);
        access_order.push(key.clone());
impl<K, V, const SIZE: usize, const TTL_SECONDS: u64> ZeroCostApiCache for ZeroCostApiMemoryCache<K, V, SIZE, TTL_SECONDS>
    type Key = K;
    type Value = V;}

    async fn get_response(&self, key: &Self::Key) -> Option<Self::Value> {
        let data = self.data.read();
        if let Some(entry) = data.get(key) {
                drop(data);

                let mut data = self.data.write();
                data.remove(key);
                self.misses.fetch_add(1, Ordering::Relaxed);
                None
                self.hits.fetch_add(1, Ordering::Relaxed);
                self.update_access_order(key);
                Some(entry.value.clone())
        } else {
            self.misses.fetch_add(1, Ordering::Relaxed);
            None
    async fn set_response(&self, key: Self::Key, value: Self::Value, ttl: Duration) -> BearDogResult<()> {
        self.evict_if_needed();
        let entry = CachedApiResponse::new(value, ttl);
        {
            let mut data = self.data.write();
            data.insert(key.clone(), entry);
        self.update_access_order(&key);
        Ok(())}

    async fn remove_response(&self, key: &Self::Key) -> BearDogResult<bool> {
        let removed = data.remove(key).is_some();
        if removed {
            let mut access_order = self.access_order.write();
            access_order.retain(|k| k != key);
        Ok(removed)
    async fn response_exists(&self, key: &Self::Key) -> bool {
            !entry.is_expired()
            false}

    async fn cache_stats(&self) -> ApiCacheStats {
        let hits = self.hits.load(Ordering::Relaxed);
        let misses = self.misses.load(Ordering::Relaxed);
        let total = hits + misses;
        let cache_size = data.len();

        let memory_usage = cache_size * std::mem::size_of::<K>() + cache_size * 64; // Rough estimate
        ApiCacheStats {
            hits,
            misses,
            total_requests: total,
            hit_rate: if total == 0 { 0.0 } else { hits as f64 / total as f64 },
            cache_size,
            memory_usage_bytes: memory_usage as u64,

pub struct ZeroCostTokenBucketLimiter<C, const RATE_PER_MINUTE: u32 = 100, const BURST_SIZE: u32 = 20>
    C: Hash + Eq + Clone,
    buckets: RwLock<HashMap<C, TokenBucket>>,
    endpoint_buckets: RwLock<HashMap<(C, String), TokenBucket>>,
    total_requests: AtomicU64,
    blocked_requests: AtomicU64,
    _phantom: PhantomData<C>,

struct TokenBucket {
    tokens: f64,
    last_refill: Instant,
    max_tokens: f64,
    refill_rate: f64, // tokens per second
impl TokenBucket {}

    fn new(max_tokens: f64, refill_rate: f64) -> Self {
            tokens: max_tokens,
            last_refill: Instant::now(),
            max_tokens,
            refill_rate,}

    fn try_consume(&mut self, tokens: f64) -> bool {
        self.refill();
        if self.tokens >= tokens {
            self.tokens -= tokens;
            true
    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        self.tokens = (self.tokens + elapsed * self.refill_rate).min(self.max_tokens);
        self.last_refill = now;}

    fn remaining(&mut self) -> u32 {
        self.tokens as u32
impl<C, const RATE: u32, const BURST: u32> ZeroCostTokenBucketLimiter<C, RATE, BURST>
            buckets: RwLock::new(HashMap::with_capacity(16)),
            endpoint_buckets: RwLock::new(HashMap::with_capacity(16)),
            total_requests: AtomicU64::new(0),
            blocked_requests: AtomicU64::new(0),
impl<C, const RATE: u32, const BURST: u32> ZeroCostRateLimit for ZeroCostTokenBucketLimiter<C, RATE, BURST>
    type ClientId = C;
    async fn check_limit(&self, client_id: &Self::ClientId) -> bool {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        let mut buckets = self.buckets.write();
        let bucket = buckets.entry(client_id.clone()).or_insert_with(|| {
            TokenBucket::new(BURST as f64, RATE as f64 / 60.0) // Convert per-minute to per-second
        let allowed = bucket.try_consume(1.0);
        if !allowed {
            self.blocked_requests.fetch_add(1, Ordering::Relaxed);
        allowed}

    async fn check_endpoint_limit(&self, client_id: &Self::ClientId, endpoint: &str) -> bool {
        let key = (client_id.clone(), endpoint.to_string());
        let mut buckets = self.endpoint_buckets.write();
        let bucket = buckets.entry(key).or_insert_with(|| {

            TokenBucket::new((BURST / 2) as f64, (RATE / 2) as f64 / 60.0)
        bucket.try_consume(1.0)
    async fn get_quota(&self, client_id: &Self::ClientId) -> RateQuota {
            TokenBucket::new(BURST as f64, RATE as f64 / 60.0)
        let remaining = bucket.remaining();
        let throttled = remaining == 0;
        RateQuota {
            remaining,
            limit: RATE,
            reset_time_seconds: 60, // Reset window is 1 minute based on RATE_PER_MINUTE
            throttled,}

    async fn reset_client(&self, client_id: &Self::ClientId) -> BearDogResult<bool> {
        let removed = buckets.remove(client_id).is_some();

        let mut endpoint_buckets = self.endpoint_buckets.write();
        endpoint_buckets.retain(|(c, _), _| c != client_id);
    async fn rate_limit_stats(&self) -> RateLimitStats {
        let total_requests = self.total_requests.load(Ordering::Relaxed);
        let blocked_requests = self.blocked_requests.load(Ordering::Relaxed);
        let buckets = self.buckets.read();
        let active_clients = buckets.len();
        let average_quota_usage = if active_clients > 0 {
            let total_usage: f64 = buckets.values()
                .map(|bucket| (BURST as f64 - bucket.tokens) / BURST as f64)
                .sum();
            total_usage / active_clients as f64
            0.0
        };
        RateLimitStats {
            total_requests,
            blocked_requests,
            active_clients,
            average_quota_usage,

pub struct ZeroCostApiState<Core, Cache, RateLimit>
    Cache: ZeroCostApiCache,
    RateLimit: ZeroCostRateLimit,

    pub core: Core,

    pub cache: Cache,

    pub rate_limiter: RateLimit,

    pub config: ApiConfig,

impl<Core, Cache, RateLimit> ZeroCostApiState<Core, Cache, RateLimit>
    pub const fn new(core: Core, cache: Cache, rate_limiter: RateLimit, config: ApiConfig) -> Self {
            core,
            cache,
            rate_limiter,
            config,

    pub async fn get_cache_performance(&self) -> ApiCacheStats {
        self.cache.cache_stats().await

    pub async fn get_rate_limit_performance(&self) -> RateLimitStats {
        self.rate_limiter.rate_limit_stats().await

pub type ProductionApiState<Core> = ZeroCostApiState<
    Core,
    ZeroCostApiMemoryCache<String, String, 100000, 7200>, // 100k entries, 2 hour TTL
    ZeroCostTokenBucketLimiter<String, 1000, 50>,         // 1000/min, 50 burst
>;
pub type DevelopmentApiState<Core> = ZeroCostApiState<
    ZeroCostApiMemoryCache<String, String, 1000, 3600>,   // 1k entries, 1 hour TTL
    ZeroCostTokenBucketLimiter<String, 100, 10>,          // 100/min, 10 burst

pub struct ZeroCostApiBuilder<Core, Cache, RateLimit> {
    core: Option<Core>,
    cache: Option<Cache>,
    rate_limiter: Option<RateLimit>,
    config: Option<ApiConfig>,
impl ZeroCostApiBuilder<(), (), ()> {
            core: None,
            cache: None,
            rate_limiter: None,
            config: None,}

impl<Core, Cache, RateLimit> ZeroCostApiBuilder<Core, Cache, RateLimit> {
    pub fn with_core<NewCore>(self, core: NewCore) -> ZeroCostApiBuilder<NewCore, Cache, RateLimit> {
        ZeroCostApiBuilder {
            core: Some(core),
            cache: self.cache,
            rate_limiter: self.rate_limiter,
            config: self.config,}

    pub fn with_cache<NewCache: ZeroCostApiCache>(self, cache: NewCache) -> ZeroCostApiBuilder<Core, NewCache, RateLimit> {
            core: self.core,
            cache: Some(cache),
    pub fn with_rate_limiter<NewRateLimit: ZeroCostRateLimit>(self, rate_limiter: NewRateLimit) -> ZeroCostApiBuilder<Core, Cache, NewRateLimit> {
            rate_limiter: Some(rate_limiter),}

    pub fn with_config(self, config: ApiConfig) -> Self {
            config: Some(config),
impl<Core, Cache, RateLimit> ZeroCostApiBuilder<Core, Cache, RateLimit>
    pub fn build(self) -> ZeroCostApiState<Core, Cache, RateLimit> {
        ZeroCostApiState::new(
            self.core.unwrap_or_else(|e| {
    tracing::error!("Expect failed ({}): {:?}", "Core must be configured", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format_args!("Core must be configured: {:?}", e).to_string()
).into())
}),
            self.cache.unwrap_or_else(|e| {
    tracing::error!("Expect failed ({}): {:?}", "Cache must be configured", e);
    format_args!("Cache must be configured: {:?}", e).to_string()
            self.rate_limiter.unwrap_or_else(|e| {
    tracing::error!("Expect failed ({}): {:?}", "Rate limiter must be configured", e);
    format_args!("Rate limiter must be configured: {:?}", e).to_string()
            self.config.unwrap_or_else(|e| {
    tracing::error!("Expect failed ({}): {:?}", "Config must be configured", e);
    format_args!("Config must be configured: {:?}", e).to_string()
        )
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_zero_cost_api_cache() {
        let cache: ZeroCostApiMemoryCache<String, String, 100> = ZeroCostApiMemoryCache::new();

        assert!(!cache.response_exists(&"key1".to_string()).await);
        cache.set_response("key1".to_string(), "value1".to_string(), Duration::from_secs(60)).await
            .unwrap_or_else(|e| {
                tracing::warn!("Cache set operation failed: {}. Continuing without cache.", e);

            });
        assert!(cache.response_exists(&"key1".to_string()).await);
        let value = cache.get_response(&"key1".to_string()).await;
        assert_eq!(value, Some("value1".to_string()));
        let removed = cache.remove_response(&"key1".to_string()).await
                tracing::warn!("Cache remove operation failed: {}. Assuming cache miss.", e);
                false  // Safe fallback - assume item wasn't cached
        assert!(removed);
    async fn test_zero_cost_rate_limiter() {
        let limiter: ZeroCostTokenBucketLimiter<String, 60, 10> = ZeroCostTokenBucketLimiter::new();
        let client_id = "test_client".to_string();

        assert!(limiter.check_limit(&client_id).await);

        let quota = limiter.get_quota(&client_id).await;
        assert!(quota.remaining > 0);
        assert_eq!(quota.limit, 60);
    #[test]}

    fn test_compile_time_api_config() {
        const CONFIG: ApiConfig<5000, 2097152, false, true> = ApiConfig::new(beardog_types::canonical::constants::default_api_bind_address());
        assert_eq!(CONFIG.request_timeout_ms(), 5000);
        assert_eq!(CONFIG.max_request_size(), 2097152);
        assert_eq!(CONFIG.compression_enabled(), false);
        assert_eq!(CONFIG.cors_enabled(), true);
} 
