// Zero-copy utilities for BearDog
//
// This module provides zero-copy abstractions using safe Rust patterns.

pub mod cow_string;
pub mod hyperoptimized_zero_copy;
pub mod id_manager;
pub mod request_cache;
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

#[derive(Debug, Default)]
pub struct ZeroCopyStats {
    /// The string cache hits value
    pub string_cache_hits: std::sync::atomic::AtomicUsize,
    /// The string cache misses value
    pub string_cache_misses: std::sync::atomic::AtomicUsize,
    pub config_cache_hits: std::sync::atomic::AtomicUsize,
    pub config_cache_misses: std::sync::atomic::AtomicUsize,
}

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

    /// Gets `shared_string`
    /// Gets `shared_string`
    pub fn get_shared_string<S: AsRef<str>>(&self, s: S) -> Arc<str> {
        let s_ref = s.as_ref();

        // Try to get from cache first
        {
            let cache = self.string_cache.read().unwrap();
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
            let mut cache = self.string_cache.write().unwrap();
            cache.insert(s_ref.to_string(), Arc::downgrade(&arc_str));
        }

        self.stats
            .string_cache_misses
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        debug!("Created new shared string: {}", s_ref);
        arc_str
    }

    /// Gets `shared_config`
    /// Gets `shared_config`
    pub fn get_shared_config<T, F>(&self, key: &str, factory: F) -> Arc<T>
    where
        T: Send + Sync + 'static,
        F: FnOnce() -> T,
    {
        let type_key = format!("{}::{}", std::any::type_name::<T>(), key);

        // Try to get from cache first
        {
            let cache = self.config_cache.read().unwrap();
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
            let mut cache = self.config_cache.write().unwrap();
            cache.insert(type_key, config.clone());
        }

        self.stats
            .config_cache_misses
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        debug!("Created new shared config: {}", key);
        config
    }

    /// Cleans up expired
    /// Cleans up expired
    pub fn cleanup_expired(&self) {
        let mut last_cleanup = self.last_cleanup.write().unwrap();
        let now = Instant::now();
        if now.duration_since(*last_cleanup) < Duration::from_secs(60) {
            return; // Cleanup at most once per minute
        }

        let mut removed_count = 0;
        {
            let mut cache = self.string_cache.write().unwrap();
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

    /// Gets stats
    /// Gets stats
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

pub fn global_zero_copy_manager() -> &'static ZeroCopyManager {
    GLOBAL_ZERO_COPY_MANAGER.get_or_init(ZeroCopyManager::new)
}

pub fn shared_string<S: AsRef<str>>(s: S) -> Arc<str> {
    global_zero_copy_manager().get_shared_string(s)
}

pub fn shared_config<T, F>(key: &str, factory: F) -> Arc<T>
where
    T: Send + Sync + 'static,
    F: FnOnce() -> T,
{
    global_zero_copy_manager().get_shared_config(key, factory)
}

/// Zero-copy optimization builder
pub struct ZeroCopyBuilder<T> {
    inner: T,
    optimized: bool,
}

impl<T> ZeroCopyBuilder<T> {
    /// Creates a new instance
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            optimized: false,
        }
    }

    pub fn optimize(mut self) -> Self
    where
        T: Clone,
    {
        // Placeholder for zero-copy optimization logic
        self.optimized = true;
        self
    }

    /// Builds component
    /// Builds component
    pub fn build(self) -> T {
        self.inner
    }

    /// Checks if optimized
    /// Checks if optimized
    pub fn is_optimized(&self) -> bool {
        self.optimized
    }
}
