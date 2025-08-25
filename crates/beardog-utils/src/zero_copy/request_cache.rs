// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Request/Response Cache for Zero-Copy Operations
///
/// Caches frequently used request and response objects to avoid
/// repeated cloning in API handlers and processing pipelines.

use beardog_types::canonical::configuration::CacheConfig; // Use canonical cache config
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{debug, trace};
/// Cache for request/response objects
pub struct RequestCache<T> {
    /// Cached items with expiration
    cache: Arc<RwLock<HashMap<RequestCacheKey, CachedItem<T>>>>,
    /// Cache configuration
    config: RequestCacheConfig,
    /// Statistics
    stats: RequestCacheStats,
}
// RequestCacheConfig moved to canonical beardog-types::config::CacheConfig
/// Statistics for request cache
#[derive(Debug, Default)]
pub struct RequestCacheStats {
    /// Number of cache hits
    pub hits: std::sync::atomic::AtomicU64,
    /// Number of cache misses
    pub misses: std::sync::atomic::AtomicU64,
    /// Number of items evicted
    pub evictions: std::sync::atomic::AtomicU64,
    /// Number of items expired
    pub expirations: std::sync::atomic::AtomicU64,
/// Cache key for request objects
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RequestCacheKey {
    /// Method (GET, POST, etc.)
    pub method: String,
    /// Path
    pub path: String,
    /// Query parameters hash
    pub query_hash: u64,
    /// Request body hash (for POST/PUT requests)
    pub body_hash: Option<u64>,}


impl RequestCacheKey {
    /// Create from HTTP request components}


    pub fn from_http_request(
        method: &str,
        path: &str,
        query: Option<&str>,
        body: Option<&[u8]>,
    ) -> Self {
        let query_hash = query.map_or(0, |q| {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            q.hash(&mut hasher);
            hasher.finish()
        });
        let body_hash = body.map(|b| {
            b.hash(&mut hasher);
        Self {
            method: method.to_string(),
            path: path.to_string(),
            query_hash,
            body_hash,
        }
    }
    /// Create from API request ID and parameters
    pub fn from_api_request(request_id: &str, params: &[(&str, &str)]) -> Self {
        let mut query_hash_input = String::new();
        for (key, value) in params {
            query_hash_input.push_str(key);
            query_hash_input.push('=');
            query_hash_input.push_str(value);
            query_hash_input.push('&');
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        query_hash_input.hash(&mut hasher);
        let query_hash = hasher.finish();
            method: "API".to_string(),
            path: request_id.to_string(),
            body_hash: None,
/// Cached item with metadata
#[derive(Debug, Clone)]
struct CachedItem<T> {
    /// The cached data
    data: Arc<T>,
    /// When this item was cached
    cached_at: Instant,
    /// How many times this item has been accessed
    access_count: u64,
    /// Last access time
    last_accessed: Instant,
impl<T> CachedItem<T> {}


    fn new(data: T) -> Self {
        let now = Instant::now();
            data: Arc::new(data),
            cached_at: now,
            access_count: 0,
            last_accessed: now,}


    fn is_expired(&self, ttl: Duration) -> bool {
        self.cached_at.elapsed() > ttl
    fn access(&mut self) -> Arc<T> {
        self.access_count += 1;
        self.last_accessed = Instant::now();
        self.data.clone()
impl<T: Clone + Send + Sync + 'static> RequestCache<T> {
    /// Create a new request cache}


    pub fn new(config: RequestCacheConfig) -> Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            config,
            stats: RequestCacheStats::default(),
    /// Get an item from cache}


    pub fn get(&self, key: &RequestCacheKey) -> Option<Arc<T>> {
        if !self.config.enabled {
            return None;
        let mut cache = self.cache.write();
        if let Some(item) = cache.get_mut(key) {
            if item.is_expired(self.config.ttl) {
                cache.remove(key);
                self.stats
                    .expirations
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    .misses
                trace!("Cache item expired for key: {:?}", key);
                return None;
            }
            self.stats
                .hits
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            trace!("Cache hit for key: {:?}", key);
            return Some(item.access());
        self.stats
            .misses
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        trace!("Cache miss for key: {:?}", key);
        None
    /// Put an item in cache
    pub fn put(&self, key: RequestCacheKey, value: T) -> Arc<T> {
            return Arc::new(value);
        let item = CachedItem::new(value);
        let result = item.data.clone();
        // Evict items if cache is full
        if cache.len() >= self.config.max_items {
            self.evict_lru(&mut cache);
        let key_clone = key.clone();
        cache.insert(key, item);
        trace!("Cached item with key: {:?}", key_clone);
        result
    /// Get or create an item
    pub fn get_or_create<F>(&self, key: RequestCacheKey, factory: F) -> Arc<T>
    where
        F: FnOnce() -> T,
    {
        if let Some(cached) = self.get(&key) {
            return cached;
        let value = factory();
        self.put(key, value)
    /// Clear expired items}


    pub fn cleanup_expired(&self) {
        let initial_size = cache.len();
        cache.retain(|_key, item| {
                false
            } else {
                true
        let removed = initial_size - cache.len();
        if removed > 0 {
            debug!("Cleaned up {} expired cache items", removed);
    /// Clear all cached items
    pub fn clear(&self) {
        let cleared = cache.len();
        cache.clear();
        if cleared > 0 {
            debug!("Cleared {} cache items", cleared);
    /// Get cache statistics
    pub fn get_stats(&self) -> &RequestCacheStats {
        &self.stats
    /// Get cache size}


    pub fn size(&self) -> usize {
        self.cache.read().len()
    /// Get cache hit rate
    pub fn hit_rate(&self) -> f64 {
        let hits = self.stats.hits.load(std::sync::atomic::Ordering::Relaxed);
        let misses = self.stats.misses.load(std::sync::atomic::Ordering::Relaxed);
        let total = hits + misses;
        if total == 0 {
            0.0
        } else {
            hits as f64 / total as f64
    /// Evict least recently used item
    fn evict_lru(&self, cache: &mut HashMap<RequestCacheKey, CachedItem<T>>) {
        if let Some((lru_key, _)) = cache
            .iter()
            .min_by_key(|(_, item)| item.last_accessed)
            .map(|(k, v)| (k.clone(), v.clone()))
        {
            cache.remove(&lru_key);
                .evictions
            trace!("Evicted LRU item: {:?}", lru_key);
impl<T: Clone + Send + Sync + 'static> Default for RequestCache<T> {}


    fn default() -> Self {
        Self::new(RequestCacheConfig::default())
/// Specialized cache for common request types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRequest {
    pub request_id: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,}


impl ApiRequest {}


    pub fn cache_key(&self) -> RequestCacheKey {
        RequestCacheKey::from_http_request(
            &self.method,
            &self.path,
            None, // We'll use headers for query-like functionality
            self.body.as_deref(),
        )
pub struct ApiResponse {
    pub status: u16,
    pub body: Vec<u8>,
    pub generated_at: std::time::SystemTime,}


impl ApiResponse {}


    pub fn new(status: u16, body: Vec<u8>) -> Self {
            status,
            headers: HashMap::new(),
            body,
            generated_at: std::time::SystemTime::now(),}


    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
/// Global request cache instances
static REQUEST_CACHE: std::sync::OnceLock<RequestCache<ApiRequest>> = std::sync::OnceLock::new();
static RESPONSE_CACHE: std::sync::OnceLock<RequestCache<ApiResponse>> = std::sync::OnceLock::new();
/// Get the global request cache
pub fn global_request_cache() -> &'static RequestCache<ApiRequest> {
    REQUEST_CACHE.get_or_init(|| RequestCache::new(RequestCacheConfig::default()))
/// Get the global response cache}


pub fn global_response_cache() -> &'static RequestCache<ApiResponse> {
    RESPONSE_CACHE.get_or_init(|| RequestCache::new(RequestCacheConfig::default()))
/// Cache a request
pub fn cache_request(request: ApiRequest) -> Arc<ApiRequest> {
    let key = request.cache_key();
    global_request_cache().put(key, request)
/// Get a cached request}


pub fn get_cached_request(key: &RequestCacheKey) -> Option<Arc<ApiRequest>> {
    global_request_cache().get(key)
/// Cache a response
pub fn cache_response(key: RequestCacheKey, response: ApiResponse) -> Arc<ApiResponse> {
    global_response_cache().put(key, response)
/// Get a cached response}


pub fn get_cached_response(key: &RequestCacheKey) -> Option<Arc<ApiResponse>> {
    global_response_cache().get(key)
