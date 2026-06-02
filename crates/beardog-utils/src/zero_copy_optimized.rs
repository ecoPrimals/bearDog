// SPDX-License-Identifier: AGPL-3.0-or-later

//! Higher-level zero-copy manager with [`std::sync::Weak`]-backed string cache and [`OptimizedString`](crate::zero_copy_optimized::OptimizedString)/[`OptimizedBytes`](crate::zero_copy_optimized::OptimizedBytes) views.

use beardog_config::env_keys;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Weak};
use std::time::{Duration, Instant};
use tracing::{debug, info, trace};

/// Atomic counters for clone avoidance, cache effectiveness, and applied transforms.
#[derive(Debug, Default)]
pub struct ZeroCopyStats {
    /// Number of clones avoided through optimization
    pub clones_avoided: std::sync::atomic::AtomicU64,
    /// Bytes of memory saved
    /// The memory saved value
    pub memory_saved: std::sync::atomic::AtomicU64,
    /// The cache hits value
    pub cache_hits: std::sync::atomic::AtomicU64,
    /// Cache misses
    /// The cache misses value
    pub cache_misses: std::sync::atomic::AtomicU64,
    /// Number of optimizations applied
    /// The optimizations applied value
    pub optimizations_applied: std::sync::atomic::AtomicU64,
}

/// Zero-copy optimization manager
pub struct ZeroCopyManager {
    stats: Arc<ZeroCopyStats>,
    /// Shared string cache to avoid duplicates
    string_cache: Arc<RwLock<HashMap<String, Weak<str>>>>,
    /// Configuration cache
    #[expect(
        dead_code,
        reason = "Config JSON cache for future optimize_config path"
    )]
    config_cache: Arc<RwLock<HashMap<String, Arc<serde_json::Value>>>>,
    /// Last cleanup time
    last_cleanup: Arc<RwLock<Instant>>,
}

impl ZeroCopyManager {
    /// Fresh caches with default capacities and logging.
    pub fn new() -> Self {
        info!("🚀 Initializing Zero-Copy Optimization Manager");
        Self {
            stats: Arc::new(ZeroCopyStats::default()),
            string_cache: Arc::new(RwLock::new(HashMap::with_capacity(256))),
            config_cache: Arc::new(RwLock::new(HashMap::with_capacity(64))),
            last_cleanup: Arc::new(RwLock::new(Instant::now())),
        }
    }

    /// Returns a strong [`Arc<str>`], upgrading weak cache entries when alive.
    pub fn get_shared_string<S: AsRef<str>>(&self, s: S) -> Arc<str> {
        let s_ref = s.as_ref();

        // Check cache first
        {
            let cache = self.string_cache.read();
            if let Some(weak_str) = cache.get(s_ref)
                && let Some(arc_str) = weak_str.upgrade()
            {
                self.stats
                    .cache_hits
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                self.stats
                    .clones_avoided
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                self.stats
                    .memory_saved
                    .fetch_add(s_ref.len() as u64, std::sync::atomic::Ordering::Relaxed);
                trace!("Zero-copy string cache hit: {}", s_ref);
                return arc_str;
            }
        }

        // Create new shared string
        self.stats
            .cache_misses
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let arc_str: Arc<str> = Arc::from(s_ref);
        let weak_str = Arc::downgrade(&arc_str);

        {
            let mut cache = self.string_cache.write();
            cache.insert(s_ref.to_string(), weak_str);
        }

        trace!("Created new shared string: {}", s_ref);
        arc_str
    }

    /// Picks [`OptimizedString::Shared`] for common/large strings, else an owned copy.
    #[must_use]
    pub fn optimize_string(&self, s: &str) -> OptimizedString {
        if self.is_common_string(s) || s.len() > 1024 {
            // Use shared reference for common or large strings
            OptimizedString::Shared(self.get_shared_string(s))
        } else {
            // Keep small, uncommon strings as owned
            OptimizedString::Owned(s.to_string())
        }
    }

    /// Uses [`Arc<[u8]>`] for payloads larger than 4 KiB.
    #[must_use]
    pub fn optimize_bytes(&self, data: &[u8]) -> OptimizedBytes {
        if data.len() > 4096 {
            // Use Arc for large byte arrays to enable sharing
            OptimizedBytes::Shared(Arc::from(data))
        } else {
            // Keep small arrays as owned
            OptimizedBytes::Owned(data.to_vec())
        }
    }

    /// Check if a string is commonly used and worth caching
    /// Checks if common string
    fn is_common_string(&self, s: &str) -> bool {
        use beardog_types::constants::domains::network::addresses::{
            DEFAULT_LOCALHOST_IPV4_STR, DEFAULT_LOCALHOST_IPV6_STR, DEFAULT_WILDCARD_IPV4_STR,
        };
        use beardog_types::constants::domains::network::config::DEFAULT_LOCALHOST_NAME_STR;

        let is_reserved_literal_host = s == DEFAULT_LOCALHOST_NAME_STR
            || s == DEFAULT_LOCALHOST_IPV4_STR
            || s == DEFAULT_WILDCARD_IPV4_STR
            || s == DEFAULT_LOCALHOST_IPV6_STR;

        is_reserved_literal_host
            || matches!(
                s,
                // HTTP methods
                "GET" | "POST" | "PUT" | "DELETE" | "PATCH" | "HEAD" | "OPTIONS" |
            // Content types
            "application/json" | "text/plain" | "application/octet-stream" |
            "text/html" | "application/xml" | "multipart/form-data" |
            // Common endpoints
            "api" | "metrics" | "health" | "admin" | "status" | "ping" |
            // Security
            "bearer" | "jwt" | "authorization" | "x-api-key" |
            // BearDog specific
            "beardog " | "capability" | "discovery" | "security" | "hsm" |
            "compute" | "storage" | "network" | "monitoring" |
            // Status values
            "success " | "error " | "pending" | "completed " | "failed " |
            "healthy" | "unhealthy" | "degraded" | "unknown" |
            // Boolean strings
            "true" | "false" | "null" | "undefined"
            )
            || s.len() <= 2
            || s.starts_with("id_")
            || s.ends_with("_id")
            || s.starts_with("bearer_")
            || std::path::Path::new(s)
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("local"))
    }

    /// Drops dead weak entries no more often than `BEARDOG_CACHE_CLEANUP_INTERVAL_SECS` (default 300s).
    pub fn cleanup_expired(&self) {
        let mut last_cleanup = self.last_cleanup.write();
        let now = Instant::now();

        let cleanup_interval_secs =
            beardog_errors::process_env::var(env_keys::ENV_CACHE_CLEANUP_INTERVAL_SECS)
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(300);
        if now.duration_since(*last_cleanup) < Duration::from_secs(cleanup_interval_secs) {
            return; // Cleanup at most once per interval
        }

        let mut removed_count = 0;
        {
            let mut cache = self.string_cache.write();
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
            debug!("🧹 Cleaned up {} expired string references", removed_count);
        }

        *last_cleanup = now;
    }

    /// Shared [`Arc`] to the live statistics bundle.
    #[must_use]
    pub fn get_stats(&self) -> Arc<ZeroCopyStats> {
        self.stats.clone()
    }

    /// Emits a human-readable summary via `tracing::info!`.
    pub fn print_optimization_report(&self) {
        let clones_avoided = self
            .stats
            .clones_avoided
            .load(std::sync::atomic::Ordering::Relaxed);
        let memory_saved = self
            .stats
            .memory_saved
            .load(std::sync::atomic::Ordering::Relaxed);
        let cache_hits = self
            .stats
            .cache_hits
            .load(std::sync::atomic::Ordering::Relaxed);
        let cache_misses = self
            .stats
            .cache_misses
            .load(std::sync::atomic::Ordering::Relaxed);
        let optimizations = self
            .stats
            .optimizations_applied
            .load(std::sync::atomic::Ordering::Relaxed);

        info!("📊 Zero-Copy Optimization Report:");
        info!("   🎯 Clones avoided: {}", clones_avoided);
        info!("   💾 Memory saved: {} bytes", memory_saved);
        info!(
            "   ⚡ Cache hit rate: {:.1}%",
            if cache_hits + cache_misses > 0 {
                #[expect(
                    clippy::cast_precision_loss,
                    reason = "Percentage from integer hit counts"
                )]
                {
                    (cache_hits as f64 / (cache_hits + cache_misses) as f64) * 100.0
                }
            } else {
                0.0
            }
        );
        info!("   🔧 Optimizations applied: {}", optimizations);

        if clones_avoided > 0 {
            #[expect(
                clippy::cast_precision_loss,
                reason = "Heuristic gain estimate from clone count"
            )]
            let estimated_performance_gain = (clones_avoided as f64 * 0.1).min(30.0);
            info!(
                "   🚀 Estimated performance improvement: {:.1}%",
                estimated_performance_gain
            );
        }
    }
}

impl Default for ZeroCopyManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Either a shared [`Arc<str>`] or a private [`String`] for small/uncommon text.
#[derive(Debug, Clone)]
pub enum OptimizedString {
    /// Shared reference to avoid clones
    Shared(Arc<str>),
    /// Unique heap copy for small/uncommon literals.
    Owned(String),
}

impl OptimizedString {
    /// Borrows the active representation without allocating.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Shared(s) => s,
            Self::Owned(s) => s,
        }
    }

    /// Convert to String (may clone if necessary)
    #[must_use]
    pub fn into_string(self) -> String {
        match self {
            Self::Shared(s) => s.to_string(),
            Self::Owned(s) => s,
        }
    }

    /// True when backed by [`OptimizedString::Shared`].
    #[must_use]
    pub const fn is_optimized(&self) -> bool {
        matches!(self, Self::Shared(_))
    }
}

impl fmt::Display for OptimizedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Shared(s) => write!(f, "{s}"),
            Self::Owned(s) => write!(f, "{s}"),
        }
    }
}

/// Byte payload as [`Arc<[u8]>`] or an owned [`Vec<u8>`] under the size threshold.
#[derive(Debug, Clone)]
pub enum OptimizedBytes {
    /// Shared slice for large blobs.
    Shared(Arc<[u8]>),
    /// Inline vector for small payloads.
    Owned(Vec<u8>),
}

impl OptimizedBytes {
    /// Borrows the active byte representation.
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        match self {
            Self::Shared(bytes) => bytes,
            Self::Owned(bytes) => bytes,
        }
    }

    /// Materializes an owned copy, cloning shared data when needed.
    #[must_use]
    pub fn to_vec(&self) -> Vec<u8> {
        match self {
            Self::Shared(bytes) => bytes.to_vec(),
            Self::Owned(bytes) => bytes.clone(),
        }
    }

    /// True when backed by [`OptimizedBytes::Shared`].
    #[must_use]
    pub const fn is_optimized(&self) -> bool {
        matches!(self, Self::Shared(_))
    }
}

/// Hooks for routing values through [`ZeroCopyManager`] heuristics.
pub trait ZeroCopyOptimized {
    /// Returns a (possibly) interned or shared clone of `self`.
    #[must_use]
    fn optimize(&self, manager: &ZeroCopyManager) -> Self;

    /// Heuristic used by [`ZeroCopyBuilder`] before calling [`Self::optimize`].
    fn is_optimized(&self) -> bool;
}

impl ZeroCopyOptimized for String {
    fn optimize(&self, manager: &ZeroCopyManager) -> Self {
        manager.optimize_string(self).to_string()
    }

    fn is_optimized(&self) -> bool {
        // Simple heuristic: common strings are likely optimized
        self.len() <= 64
    }
}

impl ZeroCopyOptimized for Vec<u8> {
    fn optimize(&self, manager: &ZeroCopyManager) -> Self {
        manager.optimize_bytes(self).to_vec()
    }

    fn is_optimized(&self) -> bool {
        // Large vectors benefit from sharing
        self.len() <= 1024
    }
}

/// Global zero-copy manager instance
static GLOBAL_ZERO_COPY_MANAGER: std::sync::OnceLock<ZeroCopyManager> = std::sync::OnceLock::new();

/// Get the global zero-copy manager
pub fn global_zero_copy_manager() -> &'static ZeroCopyManager {
    GLOBAL_ZERO_COPY_MANAGER.get_or_init(ZeroCopyManager::new)
}

/// Convenience function to get a shared string
pub fn shared_string<S: AsRef<str>>(s: S) -> Arc<str> {
    global_zero_copy_manager().get_shared_string(s)
}

/// Convenience function to optimize a string
#[must_use]
pub fn optimize_string(s: &str) -> OptimizedString {
    global_zero_copy_manager().optimize_string(s)
}

/// Convenience function to optimize bytes
#[must_use]
pub fn optimize_bytes(data: &[u8]) -> OptimizedBytes {
    global_zero_copy_manager().optimize_bytes(data)
}

/// [`optimize_string`] on a `format!`-built literal or template.
#[macro_export]
macro_rules! zero_copy_format {
    ($template:literal) => {
        $crate::zero_copy_optimized::optimize_string($template)
    };
    ($template:literal, $($args:expr),+) => {
        $crate::zero_copy_optimized::optimize_string(&format!($template, $($args),+))
    };
}

/// Fluent helper that optionally runs [`ZeroCopyOptimized::optimize`] once.
pub struct ZeroCopyBuilder<T> {
    inner: T,
    optimized: bool,
}

impl<T> ZeroCopyBuilder<T> {
    /// Wraps `inner` before optional optimization.
    pub const fn new(inner: T) -> Self {
        Self {
            inner,
            optimized: false,
        }
    }

    /// Apply optimizations
    #[must_use]
    pub fn optimize(mut self) -> Self
    where
        T: ZeroCopyOptimized,
    {
        if !self.inner.is_optimized() {
            self.inner = self.inner.optimize(global_zero_copy_manager());
            self.optimized = true;
            global_zero_copy_manager()
                .stats
                .optimizations_applied
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        self
    }

    /// Consumes the builder and returns the (possibly optimized) value.
    pub fn build(self) -> T {
        self.inner
    }

    /// Whether [`Self::optimize`] mutated `inner`.
    pub const fn is_optimized(&self) -> bool {
        self.optimized
    }
}

#[cfg(test)]
#[path = "zero_copy_optimized_tests.rs"]
mod tests;
