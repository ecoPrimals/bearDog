// SPDX-License-Identifier: AGPL-3.0-only

//! Shared [`std::sync::Arc`] strings/configs with weak-cache eviction and capability helpers.
//!
//! **Ownership:** [`ZeroCopyManager`](crate::zero_copy::ZeroCopyManager) stores [`std::sync::Weak`] handles to strings so dropping all
//! strong refs allows cleanup; typed configs use `Arc<dyn Any + Send + Sync>` clones.

/// Copy-on-write string utilities layered on shared buffers.
pub mod cow_string;
/// Experimental higher-throughput paths built on this module’s primitives.
pub mod hyperoptimized_zero_copy;
/// Identifier pools that reuse allocations for hot IDs.
pub mod id_manager;
/// Request-scoped caches that avoid cloning response bodies.
pub mod request_cache;
/// Sharable configuration snapshots with stable [`Arc`] handles.
pub mod shared_config;

pub use cow_string::*;
pub use hyperoptimized_zero_copy::*;
pub use id_manager::*;
pub use request_cache::*;
pub use shared_config::*;

use std::collections::HashMap;
use std::sync::{Arc, RwLock, Weak};
use std::time::{Duration, Instant};
use tracing::{debug, trace};

#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]
#[cfg(test)]
mod tests_comprehensive;

/// Simple capability validation
/// Checks if valid service capability
#[must_use]
pub fn is_valid_service_capability(capability: &str) -> bool {
    matches!(
        capability,
        "communication_mesh"
            | "storage_services"
            | "compute_orchestration"
            | "ai_intelligence"
            | "security_provider"
            | "system_integration"
            | "hsm"
            | "key_management"
            | "secure_enclave"
    )
}

/// Get all standard capabilities
/// Gets `all_standard_capabilities`
#[must_use]
pub fn get_all_standard_capabilities() -> Vec<&'static str> {
    vec![
        "communication_mesh",
        "storage_services",
        "compute_orchestration",
        "ai_intelligence",
        "security_provider",
        "system_integration",
        "hsm",
        "key_management",
        "secure_enclave",
    ]
}

/// Atomic counters exposed for metrics exporters.
#[derive(Debug, Default)]
pub struct ZeroCopyStats {
    /// The string cache hits value
    pub string_cache_hits: std::sync::atomic::AtomicUsize,
    /// The string cache misses value
    pub string_cache_misses: std::sync::atomic::AtomicUsize,
    /// Successful typed-config lookups.
    pub config_cache_hits: std::sync::atomic::AtomicUsize,
    /// Typed-config factory invocations.
    pub config_cache_misses: std::sync::atomic::AtomicUsize,
}

/// Process-wide intern tables guarded by `RwLock`s; safe for concurrent readers.
pub struct ZeroCopyManager {
    string_cache: RwLock<HashMap<String, Weak<str>>>,
    config_cache: RwLock<HashMap<String, Arc<dyn std::any::Any + Send + Sync>>>,
    stats: Arc<ZeroCopyStats>,
    last_cleanup: RwLock<Instant>,
}

impl ZeroCopyManager {
    /// Creates a new instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            string_cache: RwLock::new(HashMap::new()),
            config_cache: RwLock::new(HashMap::new()),
            stats: Arc::new(ZeroCopyStats::default()),
            last_cleanup: RwLock::new(Instant::now()),
        }
    }

    /// Returns a strong `Arc<str>`, reusing an existing allocation when possible.
    pub fn get_shared_string<S: AsRef<str>>(&self, s: S) -> Arc<str> {
        let s_ref = s.as_ref();

        // Try to get from cache first
        {
            let cache = self.string_cache.read().unwrap_or_else(|poisoned| {
                tracing::warn!("String cache lock poisoned on read, recovering");
                poisoned.into_inner()
            });
            if let Some(weak_str) = cache.get(s_ref) {
                if let Some(arc_str) = weak_str.upgrade() {
                    self.stats
                        .string_cache_hits
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    trace!("String cache hit for: {}", s_ref);
                    return arc_str;
                }
            }
        }

        // Create new shared string
        let arc_str: Arc<str> = Arc::from(s_ref);
        {
            let mut cache = self.string_cache.write().unwrap_or_else(|poisoned| {
                tracing::warn!("String cache lock poisoned on write, recovering");
                poisoned.into_inner()
            });
            cache.insert(s_ref.to_string(), Arc::downgrade(&arc_str));
        }

        self.stats
            .string_cache_misses
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        debug!("Created new shared string: {}", s_ref);
        arc_str
    }

    /// Memoizes `factory()` per `(T::type_name, key)` and hands out `Arc<T>` clones.
    pub fn get_shared_config<T, F>(&self, key: &str, factory: F) -> Arc<T>
    where
        T: Send + Sync + 'static,
        F: FnOnce() -> T,
    {
        let type_key = format!("{}::{}", std::any::type_name::<T>(), key);

        // Try to get from cache first
        {
            let cache = self.config_cache.read().unwrap_or_else(|poisoned| {
                tracing::warn!("Config cache lock poisoned on read, recovering");
                poisoned.into_inner()
            });
            if let Some(any_config) = cache.get(&type_key) {
                if let Ok(typed_config) = any_config.clone().downcast::<T>() {
                    self.stats
                        .config_cache_hits
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    trace!("Config cache hit for: {}", key);
                    return typed_config;
                }
            }
        }

        // Create new config if not found
        let config = Arc::new(factory());
        {
            let mut cache = self.config_cache.write().unwrap_or_else(|poisoned| {
                tracing::warn!("Config cache lock poisoned on write, recovering");
                poisoned.into_inner()
            });
            cache.insert(type_key, config.clone());
        }

        self.stats
            .config_cache_misses
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        debug!("Created new shared config: {}", key);
        config
    }

    /// Removes dead weak string entries no more often than once per minute.
    pub fn cleanup_expired(&self) {
        let mut last_cleanup = self.last_cleanup.write().unwrap_or_else(|poisoned| {
            tracing::warn!("Last cleanup lock poisoned on write, recovering");
            poisoned.into_inner()
        });
        let now = Instant::now();
        if now.duration_since(*last_cleanup) < Duration::from_secs(60) {
            return; // Cleanup at most once per minute
        }

        let mut removed_count = 0;
        {
            let mut cache = self.string_cache.write().unwrap_or_else(|poisoned| {
                tracing::warn!("String cache lock poisoned on cleanup, recovering");
                poisoned.into_inner()
            });
            cache.retain(|_k, weak_str| {
                if weak_str.strong_count() == 0 {
                    removed_count += 1;
                    false
                } else {
                    true
                }
            });
        }

        if removed_count > 0 {
            debug!("Cleaned up {} expired string references", removed_count);
        }
        *last_cleanup = now;
    }

    /// Shared pointer to the live statistics bundle.
    pub fn get_stats(&self) -> Arc<ZeroCopyStats> {
        self.stats.clone()
    }
}

impl Default for ZeroCopyManager {
    fn default() -> Self {
        Self::new()
    }
}

static GLOBAL_ZERO_COPY_MANAGER: std::sync::OnceLock<ZeroCopyManager> = std::sync::OnceLock::new();

/// Lazily constructs the singleton [`ZeroCopyManager`] for crate-wide reuse.
pub fn global_zero_copy_manager() -> &'static ZeroCopyManager {
    GLOBAL_ZERO_COPY_MANAGER.get_or_init(ZeroCopyManager::new)
}

/// [`ZeroCopyManager::get_shared_string`] on the global singleton.
pub fn shared_string<S: AsRef<str>>(s: S) -> Arc<str> {
    global_zero_copy_manager().get_shared_string(s)
}

/// [`ZeroCopyManager::get_shared_config`] on the global singleton.
pub fn shared_config<T, F>(key: &str, factory: F) -> Arc<T>
where
    T: Send + Sync + 'static,
    F: FnOnce() -> T,
{
    global_zero_copy_manager().get_shared_config(key, factory)
}

/// Fluent wrapper marking whether placeholder optimizations ran (API compatibility shim).
pub struct ZeroCopyBuilder<T> {
    inner: T,
    optimized: bool,
}

impl<T> ZeroCopyBuilder<T> {
    /// Wraps `inner` with `optimized = false`.
    pub const fn new(inner: T) -> Self {
        Self {
            inner,
            optimized: false,
        }
    }

    /// Marks the builder as having applied best-effort optimizations (no clone elision yet).
    pub const fn optimize(mut self) -> Self
    where
        T: Clone,
    {
        // Placeholder for zero-copy optimization logic
        self.optimized = true;
        self
    }

    /// Consumes the builder and returns the wrapped value.
    pub fn build(self) -> T {
        self.inner
    }

    /// Whether [`Self::optimize`] has been called.
    pub const fn is_optimized(&self) -> bool {
        self.optimized
    }
}

#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_copy_manager_creation() {
        let manager = ZeroCopyManager::new();
        let stats = manager.get_stats();

        assert_eq!(
            stats
                .string_cache_hits
                .load(std::sync::atomic::Ordering::Relaxed),
            0
        );
        assert_eq!(
            stats
                .string_cache_misses
                .load(std::sync::atomic::Ordering::Relaxed),
            0
        );
    }

    #[test]
    fn test_zero_copy_manager_default() {
        let manager = ZeroCopyManager::default();
        let stats = manager.get_stats();

        assert_eq!(
            stats
                .string_cache_hits
                .load(std::sync::atomic::Ordering::Relaxed),
            0
        );
    }

    #[test]
    fn test_shared_string_cache_miss() {
        let manager = ZeroCopyManager::new();
        let str1 = manager.get_shared_string("test_string");

        assert_eq!(str1.as_ref(), "test_string");

        let stats = manager.get_stats();
        assert_eq!(
            stats
                .string_cache_misses
                .load(std::sync::atomic::Ordering::Relaxed),
            1
        );
    }

    #[test]
    fn test_shared_string_cache_hit() {
        let manager = ZeroCopyManager::new();

        // First call - cache miss
        let str1 = manager.get_shared_string("test_string");
        // Second call - cache hit
        let str2 = manager.get_shared_string("test_string");

        assert_eq!(str1.as_ref(), str2.as_ref());
        assert!(Arc::ptr_eq(&str1, &str2));

        let stats = manager.get_stats();
        assert_eq!(
            stats
                .string_cache_hits
                .load(std::sync::atomic::Ordering::Relaxed),
            1
        );
        assert_eq!(
            stats
                .string_cache_misses
                .load(std::sync::atomic::Ordering::Relaxed),
            1
        );
    }

    #[test]
    fn test_multiple_different_strings() {
        let manager = ZeroCopyManager::new();

        let str1 = manager.get_shared_string("string1");
        let str2 = manager.get_shared_string("string2");
        let str3 = manager.get_shared_string("string3");

        assert_eq!(str1.as_ref(), "string1");
        assert_eq!(str2.as_ref(), "string2");
        assert_eq!(str3.as_ref(), "string3");

        let stats = manager.get_stats();
        assert_eq!(
            stats
                .string_cache_misses
                .load(std::sync::atomic::Ordering::Relaxed),
            3
        );
    }

    #[test]
    fn test_shared_config_cache() {
        let manager = ZeroCopyManager::new();

        #[derive(Debug, Clone, PartialEq)]
        struct TestConfig {
            value: u32,
        }

        // First call - cache miss
        let config1 = manager.get_shared_config("test_config", || TestConfig { value: 42 });

        // Second call - cache hit
        let config2 =
            manager.get_shared_config::<TestConfig, _>("test_config", || TestConfig { value: 99 });

        assert_eq!(config1.value, 42);
        assert_eq!(config2.value, 42); // Should get cached value, not 99
        assert!(Arc::ptr_eq(&config1, &config2));

        let stats = manager.get_stats();
        assert_eq!(
            stats
                .config_cache_hits
                .load(std::sync::atomic::Ordering::Relaxed),
            1
        );
        assert_eq!(
            stats
                .config_cache_misses
                .load(std::sync::atomic::Ordering::Relaxed),
            1
        );
    }

    #[test]
    fn test_cleanup_expired() {
        let manager = ZeroCopyManager::new();

        // Add some strings
        {
            let _str1 = manager.get_shared_string("temp1");
            let _str2 = manager.get_shared_string("temp2");
        }
        // Strings are now dropped, should be cleaned up

        manager.cleanup_expired();

        // Should run without errors
    }

    #[test]
    fn test_cleanup_rate_limiting() {
        let manager = ZeroCopyManager::new();

        // First cleanup
        manager.cleanup_expired();

        // Immediate second cleanup should be rate-limited
        manager.cleanup_expired();

        // Should complete successfully
    }

    #[test]
    fn test_global_zero_copy_manager() {
        let manager1 = global_zero_copy_manager();
        let manager2 = global_zero_copy_manager();

        // Should be the same instance
        assert!(std::ptr::eq(manager1, manager2));
    }

    #[test]
    fn test_global_shared_string() {
        let str1 = shared_string("global_test");
        let str2 = shared_string("global_test");

        assert!(Arc::ptr_eq(&str1, &str2));
    }

    #[test]
    fn test_global_shared_config() {
        #[derive(Debug, Clone)]
        struct Config {
            val: i32,
        }

        let cfg1 = shared_config("test_cfg", || Config { val: 123 });
        let cfg2 = shared_config::<Config, _>("test_cfg", || Config { val: 456 });

        assert_eq!(cfg1.val, 123);
        assert_eq!(cfg2.val, 123);
    }

    #[test]
    fn test_is_valid_service_capability() {
        assert!(is_valid_service_capability("communication_mesh"));
        assert!(is_valid_service_capability("storage_services"));
        assert!(is_valid_service_capability("compute_orchestration"));
        assert!(is_valid_service_capability("ai_intelligence"));
        assert!(is_valid_service_capability("security_provider"));
        assert!(is_valid_service_capability("system_integration"));
        assert!(is_valid_service_capability("hsm"));
        assert!(is_valid_service_capability("key_management"));
        assert!(is_valid_service_capability("secure_enclave"));

        assert!(!is_valid_service_capability("invalid_capability"));
        assert!(!is_valid_service_capability(""));
    }

    #[test]
    fn test_get_all_standard_capabilities() {
        let capabilities = get_all_standard_capabilities();

        assert_eq!(capabilities.len(), 9);
        assert!(capabilities.contains(&"communication_mesh"));
        assert!(capabilities.contains(&"hsm"));
        assert!(capabilities.contains(&"secure_enclave"));
    }

    #[test]
    fn test_zero_copy_builder_new() {
        let builder = ZeroCopyBuilder::new(42);

        assert_eq!(builder.inner, 42);
        assert!(!builder.is_optimized());
    }

    #[test]
    fn test_zero_copy_builder_optimize() {
        let builder = ZeroCopyBuilder::new(42);
        let builder = builder.optimize();

        assert!(builder.is_optimized());
    }

    #[test]
    fn test_zero_copy_builder_build() {
        let builder = ZeroCopyBuilder::new(42);
        let value = builder.build();

        assert_eq!(value, 42);
    }

    #[test]
    fn test_zero_copy_builder_chain() {
        let value = ZeroCopyBuilder::new(100).optimize().build();

        assert_eq!(value, 100);
    }

    #[test]
    fn test_builder_with_string() {
        let builder = ZeroCopyBuilder::new("test".to_string());
        let result = builder.optimize().build();

        assert_eq!(result, "test");
    }

    #[test]
    fn test_concurrent_string_caching() {
        use std::sync::Arc;
        use std::thread;

        let manager = Arc::new(ZeroCopyManager::new());
        let mut handles = vec![];

        for i in 0..4 {
            let mgr = Arc::clone(&manager);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    let _ = mgr.get_shared_string(format!("test_{}", i));
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let stats = manager.get_stats();
        // Verify that concurrent operations completed successfully
        // Each thread called get_shared_string 100 times with the same string
        // So we should have hits (99 per thread) + misses (1 per thread) = 400 total operations
        let total_ops = stats
            .string_cache_hits
            .load(std::sync::atomic::Ordering::Relaxed)
            + stats
                .string_cache_misses
                .load(std::sync::atomic::Ordering::Relaxed);
        assert!(total_ops >= 300); // At least most operations should be tracked
    }

    #[test]
    fn test_empty_string() {
        let manager = ZeroCopyManager::new();
        let str1 = manager.get_shared_string("");

        assert_eq!(str1.as_ref(), "");
    }

    #[test]
    fn test_long_string() {
        let manager = ZeroCopyManager::new();
        let long_str = "a".repeat(10000);
        let str1 = manager.get_shared_string(&long_str);
        let str2 = manager.get_shared_string(&long_str);

        assert!(Arc::ptr_eq(&str1, &str2));
    }

    #[test]
    fn test_stats_structure() {
        let stats = ZeroCopyStats::default();

        assert_eq!(
            stats
                .string_cache_hits
                .load(std::sync::atomic::Ordering::Relaxed),
            0
        );
        assert_eq!(
            stats
                .config_cache_hits
                .load(std::sync::atomic::Ordering::Relaxed),
            0
        );
    }

    #[test]
    fn test_multiple_config_types() {
        let manager = ZeroCopyManager::new();

        #[derive(Debug)]
        struct ConfigA {
            a: i32,
        }

        #[derive(Debug)]
        struct ConfigB {
            b: String,
        }

        let cfg_a = manager.get_shared_config("test", || ConfigA { a: 1 });
        let cfg_b = manager.get_shared_config("test", || ConfigB {
            b: "hello".to_string(),
        });

        // Different types should have different cache entries
        assert_eq!(cfg_a.a, 1);
        assert_eq!(cfg_b.b, "hello");
    }
}
