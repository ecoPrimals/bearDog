// High-Performance Capability Discovery Cache
//
// This module provides advanced caching capabilities for the universal discovery system,
// dramatically reducing discovery latency and improving overall system performance.

use beardog_types::canonical::capabilities::{CapabilityType, HealthStatus, UniversalCapability};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use tokio::sync::broadcast;

pub struct CapabilityDiscoveryCache {
    /// Primary capability cache
    cache: Arc<RwLock<HashMap<String, CachedCapability>>>,

    type_index: Arc<RwLock<HashMap<CapabilityType, Vec<String>>>>,

    metrics: Arc<RwLock<CacheMetrics>>,

    /// Cache configuration
    config: CacheConfig,

    /// Invalidation broadcast channel
    invalidation_tx: broadcast::Sender<InvalidationEvent>,
}

/// Cached capability with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedCapability {
    /// The capability itself
    /// The capability value
    pub capability: UniversalCapability,

    /// Cache timestamp
    /// The cached at value
    pub cached_at: Instant,

    /// Last health check
    /// Optional last health check
    pub last_health_check: Option<Instant>,

    /// Number of access
    pub access_count: u64,

    /// Last accessed timestamp
    /// The last accessed value
    pub last_accessed: Instant,

    /// Cache priority (higher = more important)
    /// The priority value
    pub priority: CachePriority,
}

/// Cache priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CachePriority {
    /// Represents low = 1 variant
    Low = 1,
    /// Represents normal = 2 variant
    Normal = 2,
    /// Represents high = 3 variant
    High = 3,
    /// Represents critical = 4 variant
    Critical = 4,
}

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum number of cached capabilities
    /// Number of max_entries
    pub max_entries: usize,

    /// The default ttl value
    pub default_ttl: Duration,

    /// Health check interval
    /// The health check interval value
    pub health_check_interval: Duration,

    /// Background cleanup interval
    /// The cleanup interval value
    pub cleanup_interval: Duration,

    /// Whether enable_optimization is enabled
    pub enable_optimization: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CacheMetrics {
    /// Total cache hits
    /// Number of hits
    pub hits: u64,

    /// Total cache misses
    /// Number of misses
    pub misses: u64,

    /// Cache evictions
    /// Number of evictions
    pub evictions: u64,

    /// Average lookup time (microseconds)
    pub avg_lookup_time_us: f64,

    /// Cache size
    /// Number of current_size
    pub current_size: usize,

    /// Memory usage estimate (bytes)
    /// Number of memory_usage_bytes
    pub memory_usage_bytes: usize,
}

/// Cache invalidation events
#[derive(Debug, Clone)]
pub enum InvalidationEvent {
    /// Invalidate specific capability
    Capability(String),

    /// Invalidate all capabilities of a type
    CapabilityType(CapabilityType),

    /// Invalidate all unhealthy capabilities
    UnhealthyCapabilities,

    /// Full cache clear
    FullClear,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_entries: std::env::var("BEARDOG_CAPABILITY_CACHE_MAX_ENTRIES")
                .ok()
                .and_then(|e| e.parse().ok())
                .unwrap_or(1000), // 1000 entries default
            default_ttl: Duration::from_secs(
                std::env::var("BEARDOG_DISCOVERY_CACHE_DEFAULT_TTL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(300) // 5 minutes default
            ),
            health_check_interval: Duration::from_secs(
                std::env::var("BEARDOG_DISCOVERY_CACHE_HEALTH_CHECK_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30)
            ),
            cleanup_interval: Duration::from_secs(
                std::env::var("BEARDOG_DISCOVERY_CACHE_CLEANUP_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(60)
            ),
            enable_optimization: true,
        }
    }
}

impl CapabilityDiscoveryCache {
    /// Create a new capability discovery cache
    /// Creates a new instance
    pub fn new(config: CacheConfig) -> Self {
        let (invalidation_tx, _) = broadcast::channel(100);

        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            type_index: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(CacheMetrics::default())),
            config,
            invalidation_tx,
        }
    }

    /// Get capability from cache
    /// Gets value
    /// Gets value
    pub fn get(&self, capability_id: &str) -> Option<UniversalCapability> {
        let start = Instant::now();

        let result = {
            let cache = self.cache.read().ok()?;
            let cached = cache.get(capability_id)?;

            // Check TTL
            if cached.cached_at.elapsed() > self.config.default_ttl {
                return None; // Expired
            }

            Some(cached.capability.clone())
        };

        // Update metrics
        if let Ok(mut metrics) = self.metrics.write() {
            if result.is_some() {
                metrics.hits += 1;

                // Update access count
                if let Ok(mut cache) = self.cache.write() {
                    if let Some(cached) = cache.get_mut(capability_id) {
                        cached.access_count += 1;
                        cached.last_accessed = Instant::now();
                    }
                }
            } else {
                metrics.misses += 1;
            }

            // Update average lookup time
            let lookup_time = start.elapsed().as_micros() as f64;
            metrics.avg_lookup_time_us = (metrics.avg_lookup_time_us + lookup_time) / 2.0;
        }

        result
    }

    /// Get capabilities by type (optimized lookup)
    /// Gets by_type
    /// Gets by_type
    pub fn get_by_type(&self, capability_type: &CapabilityType) -> Vec<UniversalCapability> {
        let start = Instant::now();

        let capability_ids = {
            let type_index = self.type_index.read().ok()?;
            type_index.get(capability_type)?.clone()
        };

        let mut capabilities = Vec::new();

        if let Ok(cache) = self.cache.read() {
            for id in capability_ids {
                if let Some(cached) = cache.get(&id) {
                    // Check TTL and health
                    if cached.cached_at.elapsed() <= self.config.default_ttl
                        && cached.capability.health_status != HealthStatus::Unhealthy
                    {
                        capabilities.push(cached.capability.clone());
                    }
                }
            }
        }

        // Update metrics
        if let Ok(mut metrics) = self.metrics.write() {
            let lookup_time = start.elapsed().as_micros() as f64;
            metrics.avg_lookup_time_us = (metrics.avg_lookup_time_us + lookup_time) / 2.0;
        }

        capabilities
    }

    /// Insert capability into cache
    pub fn insert(&self, capability: UniversalCapability, priority: CachePriority) {
        let capability_id = capability.provider.provider_id.clone();
        let capability_type = capability.capability_type.clone();

        let cached_capability = CachedCapability {
            capability,
            cached_at: Instant::now(),
            last_health_check: None,
            access_count: 1,
            last_accessed: Instant::now(),
            priority,
        };

        // Insert into main cache
        if let Ok(mut cache) = self.cache.write() {
            // Check if we need to evict
            if cache.len() >= self.config.max_entries {
                self.evict_lru(&mut cache);
            }

            cache.insert(capability_id.clone(), cached_capability);
        }

        // Update type index
        if let Ok(mut type_index) = self.type_index.write() {
            type_index
                .entry(capability_type)
                .or_insert_with(Vec::new)
                .push(capability_id);
        }

        // Update metrics
        if let Ok(mut metrics) = self.metrics.write() {
            metrics.current_size = self.get_cache_size();
            metrics.memory_usage_bytes = self.estimate_memory_usage();
        }
    }

    /// Bulk insert capabilities (optimized)
    pub fn bulk_insert(&self, capabilities: Vec<(UniversalCapability, CachePriority)>) {
        let mut cache_guard = match self.cache.write() {
            Ok(guard) => guard,
            Err(_) => return,
        };

        let mut type_index_guard = match self.type_index.write() {
            Ok(guard) => guard,
            Err(_) => return,
        };

        for (capability, priority) in capabilities {
            let capability_id = capability.provider.provider_id.clone();
            let capability_type = capability.capability_type.clone();

            // Check if we need to evict
            if cache_guard.len() >= self.config.max_entries {
                self.evict_lru(&mut cache_guard);
            }

            let cached_capability = CachedCapability {
                capability,
                cached_at: Instant::now(),
                last_health_check: None,
                access_count: 1,
                last_accessed: Instant::now(),
                priority,
            };

            cache_guard.insert(capability_id.clone(), cached_capability);

            type_index_guard
                .entry(capability_type)
                .or_insert_with(Vec::new)
                .push(capability_id);
        }

        // Update metrics
        if let Ok(mut metrics) = self.metrics.write() {
            metrics.current_size = cache_guard.len();
            metrics.memory_usage_bytes = self.estimate_memory_usage();
        }
    }

    /// Invalidate capability
    pub fn invalidate(&self, capability_id: &str) {
        if let Ok(mut cache) = self.cache.write() {
            if let Some(cached) = cache.remove(capability_id) {
                // Remove from type index
                if let Ok(mut type_index) = self.type_index.write() {
                    if let Some(ids) = type_index.get_mut(&cached.capability.capability_type) {
                        ids.retain(|id| id != capability_id);
                    }
                }
            }
        }

        // Broadcast invalidation event
        let _ = self
            .invalidation_tx
            .send(InvalidationEvent::Capability(capability_id.to_string()));
    }

    /// Invalidate all capabilities of a type
    pub fn invalidate_type(&self, capability_type: &CapabilityType) {
        let capability_ids = {
            let mut type_index = match self.type_index.write() {
                Ok(guard) => guard,
                Err(_) => return,
            };

            type_index.remove(capability_type).unwrap_or_default()
        };

        if let Ok(mut cache) = self.cache.write() {
            for id in capability_ids {
                cache.remove(&id);
            }
        }

        // Broadcast invalidation event
        let _ = self
            .invalidation_tx
            .send(InvalidationEvent::CapabilityType(capability_type.clone()));
    }

    /// Clear all cache entries
    pub fn clear(&self) {
        if let Ok(mut cache) = self.cache.write() {
            cache.clear();
        }

        if let Ok(mut type_index) = self.type_index.write() {
            type_index.clear();
        }

        // Broadcast invalidation event
        let _ = self.invalidation_tx.send(InvalidationEvent::FullClear);
    }

    /// Get cache metrics
    /// Gets metrics
    /// Gets metrics
    pub fn get_metrics(&self) -> CacheMetrics {
        self.metrics.read().map(|m| m.clone()).unwrap_or_default()
    }

    /// Start background maintenance tasks
    /// Starts maintenance
    /// Starts maintenance
    pub fn start_maintenance(&self) -> tokio::task::JoinHandle<()> {
        let cache = self.cache.clone();
        let type_index = self.type_index.clone();
        let metrics = self.metrics.clone();
        let config = self.config.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(config.cleanup_interval);

            loop {
                interval.tick();

                // Cleanup expired entries
                Self::cleanup_expired(&cache, &type_index, &metrics, &config);

                // Health check critical capabilities
                Self::health_check_critical(&cache, &config);
            }
        })
    }

    /// LRU eviction strategy
    fn evict_lru(&self, cache: &mut HashMap<String, CachedCapability>) {
        if cache.is_empty() {
            return;
        }

        // Find LRU entry with lowest priority
        let mut lru_key = None;
        let mut lru_time = Instant::now();
        let mut lru_priority = CachePriority::Critical;

        for (key, cached) in cache.iter() {
            if cached.priority < lru_priority
                || (cached.priority == lru_priority && cached.last_accessed < lru_time)
            {
                lru_key = Some(key.clone());
                lru_time = cached.last_accessed;
                lru_priority = cached.priority;
            }
        }

        if let Some(key) = lru_key {
            cache.remove(&key);

            // Update metrics
            if let Ok(mut metrics) = self.metrics.write() {
                metrics.evictions += 1;
            }
        }
    }

    /// Cleanup expired entries
    /// Cleans up expired
    fn cleanup_expired(
        cache: &Arc<RwLock<HashMap<String, CachedCapability>>>,
        type_index: &Arc<RwLock<HashMap<CapabilityType, Vec<String>>>>,
        metrics: &Arc<RwLock<CacheMetrics>>,
        config: &CacheConfig,
    ) {
        let expired_keys: Vec<String> = {
            let cache_guard = match cache.read() {
                Ok(guard) => guard,
                Err(_) => return,
            };

            cache_guard
                .iter()
                .filter_map(|(key, cached)| {
                    if cached.cached_at.elapsed() > config.default_ttl {
                        Some(key.clone())
                    } else {
                        None
                    }
                })
                .collect()
        };

        if expired_keys.is_empty() {
            return;
        }

        // Remove expired entries
        if let Ok(mut cache_guard) = cache.write() {
            for key in &expired_keys {
                if let Some(cached) = cache_guard.remove(key) {
                    // Remove from type index
                    if let Ok(mut type_index_guard) = type_index.write() {
                        if let Some(ids) =
                            type_index_guard.get_mut(&cached.capability.capability_type)
                        {
                            ids.retain(|id| id != key);
                        }
                    }
                }
            }
        }

        // Update metrics
        if let Ok(mut metrics_guard) = metrics.write() {
            metrics_guard.current_size = cache.read().map(|c| c.len()).unwrap_or(0);
        }
    }

    /// Health check critical capabilities
    fn health_check_critical(
        cache: &Arc<RwLock<HashMap<String, CachedCapability>>>,
        config: &CacheConfig,
    ) {
        let critical_capabilities: Vec<(String, String)> = {
            let cache_guard = match cache.read() {
                Ok(guard) => guard,
                Err(_) => return,
            };

            cache_guard
                .iter()
                .filter_map(|(key, cached)| {
                    if cached.priority >= CachePriority::High
                        && cached
                            .last_health_check
                            .map(|t| t.elapsed() > config.health_check_interval)
                            .unwrap_or(true)
                    {
                        Some((key.clone(), cached.capability.provider.endpoint.clone()))
                    } else {
                        None
                    }
                })
                .collect()
        };

        // Perform health checks
        for (key, endpoint) in critical_capabilities {
            let health_status = Self::check_capability_health(&endpoint);

            // Update health status in cache
            if let Ok(mut cache_guard) = cache.write() {
                if let Some(cached) = cache_guard.get_mut(&key) {
                    cached.last_health_check = Some(Instant::now());
                    cached.capability.health_status = health_status;
                }
            }
        }
    }

    /// Check capability health
    fn check_capability_health(endpoint: &str) -> HealthStatus {
        match reqwest::get(&format!("{}/health", endpoint)) {
            Ok(response) if response.status().is_success() => HealthStatus::Healthy,
            Ok(_) => HealthStatus::Degraded,
            Err(_) => HealthStatus::Unhealthy,
        }
    }

    /// Get current cache size
    /// Gets cache_size
    fn get_cache_size(&self) -> usize {
        self.cache.read().map(|c| c.len()).unwrap_or(0)
    }

    /// Estimate memory usage
    fn estimate_memory_usage(&self) -> usize {
        // Rough estimate: 1KB per cached capability
        self.get_cache_size() * 1024
    }
}

impl Default for CapabilityDiscoveryCache {
    fn default() -> Self {
        Self::new(CacheConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_types::canonical::capabilities::{PerformanceMetrics, ProviderInfo};

    /// Creates test_capability
    fn create_test_capability(id: &str, cap_type: CapabilityType) -> UniversalCapability {
        UniversalCapability {
            capability_type: cap_type,
            provider: ProviderInfo {
                provider_id: id.to_string(),
                provider_name: format!("Test Provider {}", id),
                endpoint: format!("http://test-{}.example.com", id),
                region: "test-region".to_string(),
                availability_zone: Some("test-az".to_string()),
            },
            health_status: HealthStatus::Healthy,
            performance: PerformanceMetrics {
                avg_response_time_ms: 100.0,
                success_rate: 0.99,
                throughput_rps: 1000.0,
            },
            // ... other fields with default values
            ..Default::default()
        }
    }

    #[test]
    fn test_cache_basic_operations() {
        let cache = CapabilityDiscoveryCache::default();
        let capability = create_test_capability("test-1", CapabilityType::KeyManagement);
        let capability_id = capability.provider.provider_id.clone();

        // Insert and retrieve
        cache.insert(capability.clone(), CachePriority::Normal);
        let retrieved = cache.get(&capability_id);
 // TEST_CATEGORY: unit
 // TEST_DOMAIN: adapters
 // TEST_PRIORITY: normal

        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().provider.provider_id, capability_id);
    }

    #[test]
    fn test_cache_type_based_lookup() {
        let cache = CapabilityDiscoveryCache::default();

        // Insert multiple capabilities of same type
        for i in 1..=3 {
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: adapters
            // TEST_PRIORITY: normal
            let capability =
                create_test_capability(&format!("kms-{}", i), CapabilityType::KeyManagement);
            cache.insert(capability, CachePriority::Normal);
        }

        // Retrieve by type
        let kms_capabilities = cache.get_by_type(&CapabilityType::KeyManagement);
        assert_eq!(kms_capabilities.len(), 3);
    }

    #[test]
    fn test_cache_eviction() {
        let config = CacheConfig {
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: adapters
            // TEST_PRIORITY: normal
            max_entries: 2,
            ..Default::default()
        };
        let cache = CapabilityDiscoveryCache::new(config);

        // Insert 3 capabilities (should trigger eviction)
        for i in 1..=3 {
            let capability =
                create_test_capability(&format!("test-{}", i), CapabilityType::KeyManagement);
            cache.insert(capability, CachePriority::Low);
        }

        // Should only have 2 entries
        assert_eq!(cache.get_cache_size(), 2);

        // First entry should be evicted (LRU)
        assert!(cache.get("test-1").is_none());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    #[test]
    fn test_cache_metrics() {
        let cache = CapabilityDiscoveryCache::default();
        let capability = create_test_capability("test-1", CapabilityType::KeyManagement);
        let capability_id = capability.provider.provider_id.clone();

        cache.insert(capability, CachePriority::Normal);

        // Generate hits and misses
        let _ = cache.get(&capability_id); // Hit
        let _ = cache.get("nonexistent"); // Miss

        let metrics = cache.get_metrics();
        assert_eq!(metrics.hits, 1);
        assert_eq!(metrics.misses, 1);
        assert_eq!(metrics.current_size, 1);
    }
}
