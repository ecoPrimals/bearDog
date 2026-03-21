// SPDX-License-Identifier: AGPL-3.0-only

/// **CONFIG PERFORMANCE METRICS** - Performance tracking for consolidation
#[derive(Debug, Clone)]
pub struct ConfigPerformanceMetrics {
    /// Estimated speedup or cost reduction from sharing merged config views (implementation-defined scale).
    pub consolidation_benefit: f64,
    /// Heap saved by deduplicating config blobs versus per-consumer copies (MB).
    pub memory_reduction_mb: f64,
    /// Measured nanoseconds of overhead for a typical read through the consolidated accessor path.
    pub function_call_overhead_ns: u64,
    /// Fraction of reads satisfied from cache without re-parsing or locking (`0.0`–`1.0`).
    pub cache_hit_rate: f64,
    /// Sustained config read/update operations per second observed in benchmarks.
    pub config_operations_per_second: f64,
    /// Number of distinct logical configs currently materialized in the shared manager.
    pub shared_configs_active: usize,
}

/// Shared configuration statistics
#[derive(Debug, Clone)]
pub struct SharedConfigStats {
    /// Count of configs tracked as “active” by the shared manager.
    pub active_configs: usize,
    /// Rough working-set estimate for those configs (KB), for telemetry only.
    pub memory_usage_estimate_kb: usize,
}
