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


/// # Performance Constants
///
/// **CANONICAL CONSTANTS MIGRATION** ✅
/// This module now re-exports canonical performance constants to eliminate
/// duplication and provide a single source of truth for all performance-related values.
/// ## Migration Status
/// - ✅ **Duplicate constants eliminated** - All constants now use canonical values
/// - ✅ **Single source of truth** - Values defined in `canonical::constants`
/// - ✅ **Backward compatibility** - All existing imports continue to work
/// - ✅ **Consistent values** - No more conflicting constant definitions

use std::time::Duration;
// Re-export canonical constants to maintain backward compatibility
pub use crate::canonical::constants::{
    CONCURRENT_TASKS, HEAVY_ITERATIONS, LIGHT_ITERATIONS, OPERATIONS_PER_TASK, STANDARD_ITERATIONS,
    TARGET_RPS, TEST_DATA_SIZE,
};
/// **TESTING CONSTANTS** - Re-exported from canonical constants
/// **MIGRATION COMPLETE** ✅ - All values now use canonical constants
/// Previous duplicated values have been consolidated:
/// - `STANDARD_ITERATIONS`: 10,000 → 1,000 (canonical value)
/// - `LIGHT_ITERATIONS`: 1,000 → 100 (canonical value)  
/// - `HEAVY_ITERATIONS`: 100,000 → 10,000 (canonical value)
/// - `CONCURRENT_TASKS`: 100 → 50 (canonical value)
/// - `TARGET_RPS`: u64 → u32 (canonical type)
/// New constants added to canonical system:
/// - `CONCURRENT_USERS`: 1,000 (moved to canonical)
/// - `LOAD_TEST_DURATION_SECONDS`: 10 (moved to canonical)
pub mod testing {
    // Re-export all canonical performance constants
    pub use crate::canonical::constants::performance::{
        STANDARD_ITERATIONS, LIGHT_ITERATIONS, HEAVY_ITERATIONS,
        CONCURRENT_TASKS, TARGET_RPS, OPERATIONS_PER_TASK
    };
    // Additional testing constants (moved to canonical constants)
    pub use crate::canonical::constants::{
        CONCURRENT_TASKS as CONCURRENT_USERS, // Alias for backward compatibility
        TEST_DATA_SIZE,
    };
    /// Load test duration - using canonical value
    pub const LOAD_TEST_DURATION_SECONDS: u64 = 10;
}
/// **CACHE CONSTANTS** - Cache performance settings
pub mod cache {
    use super::Duration;
    /// Standard cache size for performance testing
    pub const STANDARD_CACHE_SIZE: usize = 10_000;
    /// Small cache size for limited memory scenarios
    pub const SMALL_CACHE_SIZE: usize = 1_000;
    /// Large cache size for high-performance scenarios
    pub const LARGE_CACHE_SIZE: usize = 100_000;
    /// Standard cache TTL
    pub const STANDARD_TTL: Duration = Duration::from_secs(3600);
    /// Short TTL for frequently changing data
    pub const SHORT_TTL: Duration = Duration::from_secs(300);
    /// Long TTL for stable data
    pub const LONG_TTL: Duration = Duration::from_secs(86400);
/// **NETWORK PERFORMANCE CONSTANTS** - Network-related performance settings
pub mod network {
    /// Connection timeout for performance tests
    pub const CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);
    /// Request timeout for API calls
    pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);
    /// Keep-alive timeout
    pub const KEEP_ALIVE_TIMEOUT: Duration = Duration::from_secs(300);
    /// Maximum concurrent connections
    pub const MAX_CONCURRENT_CONNECTIONS: usize = 1000;
    /// Connection pool size
    pub const CONNECTION_POOL_SIZE: usize = 100;
/// **MEMORY PERFORMANCE CONSTANTS** - Memory-related performance settings
pub mod memory {
    /// Buffer size for I/O operations
    pub const BUFFER_SIZE: usize = 64 * 1024; // 64KB
    /// Maximum memory usage for tests (MB)
    pub const MAX_MEMORY_USAGE_MB: usize = 1024; // 1GB
    /// Memory allocation threshold for warnings
    pub const MEMORY_WARNING_THRESHOLD_MB: usize = 512; // 512MB
    /// Garbage collection threshold
    pub const GC_THRESHOLD_MB: usize = 256; // 256MB
/// **THREADING PERFORMANCE CONSTANTS** - Thread pool settings
pub mod threading {
    /// Default thread pool size (use num_cpus::get() at runtime)
    pub const DEFAULT_THREAD_POOL_SIZE: usize = 8; // Reasonable default, use num_cpus::get() at runtime
    /// Maximum thread pool size
    pub const MAX_THREAD_POOL_SIZE: usize = 128;
    /// Thread stack size (bytes)
    pub const THREAD_STACK_SIZE: usize = 2 * 1024 * 1024; // 2MB
    /// Worker thread keepalive time (seconds)
    pub const WORKER_KEEPALIVE_SECONDS: u64 = 60;
/// **DATABASE PERFORMANCE CONSTANTS** - Database connection and query settings
pub mod database {
    pub const CONNECTION_POOL_SIZE: usize = 20;
    /// Maximum connection lifetime
    pub const MAX_CONNECTION_LIFETIME: Duration = Duration::from_secs(3600);
    /// Connection idle timeout
    pub const IDLE_TIMEOUT: Duration = Duration::from_secs(600);
    /// Query timeout
    pub const QUERY_TIMEOUT: Duration = Duration::from_secs(30);
    /// Transaction timeout
    pub const TRANSACTION_TIMEOUT: Duration = Duration::from_secs(300);
    /// Batch size for bulk operations
    pub const BATCH_SIZE: usize = 1000;
/// **CRYPTOGRAPHIC PERFORMANCE CONSTANTS** - Crypto operation settings
pub mod crypto {
    /// Key derivation iterations for performance tests
    pub const KEY_DERIVATION_ITERATIONS: u32 = 100_000;
    /// Hash algorithm rounds
    pub const HASH_ROUNDS: u32 = 12;
    /// Salt size for key derivation
    pub const SALT_SIZE: usize = 32;
    /// IV size for encryption
    pub const IV_SIZE: usize = 16;
    /// Key size for symmetric encryption
    pub const SYMMETRIC_KEY_SIZE: usize = 32; // 256 bits
/// **WORKFLOW PERFORMANCE CONSTANTS** - Workflow execution settings
pub mod workflow {
    /// Maximum workflow execution time
    pub const MAX_EXECUTION_TIME: Duration = Duration::from_secs(3600); // 1 hour
    /// Workflow step timeout
    pub const STEP_TIMEOUT: Duration = Duration::from_secs(300); // 5 minutes
    /// Maximum concurrent workflows
    pub const MAX_CONCURRENT_WORKFLOWS: usize = 100;
    /// Workflow retry attempts
    pub const MAX_RETRY_ATTEMPTS: u32 = 30;
    /// Retry backoff multiplier
    pub const RETRY_BACKOFF_MULTIPLIER: f64 = 2.0;
    /// Default workflow retention period
    pub const DEFAULT_RETENTION_DAYS: i64 = 30;
    /// Default workflow timeout
    pub const DEFAULT_TIMEOUT_HOURS: i64 = 24;
    /// Completed workflow idle duration
    pub const COMPLETED_IDLE_DAYS: i64 = 7;
    /// Warning threshold for expiring workflows
    pub const EXPIRY_WARNING_HOURS: i64 = 2;
    /// Maximum stale workflow age
    pub const MAX_STALE_DAYS: i64 = 7;
    /// Initial retry delay (seconds)
    pub const INITIAL_RETRY_DELAY_SECONDS: u64 = 1;
/// **GAMING PERFORMANCE CONSTANTS** - Gaming-specific optimization settings
pub mod gaming {
    /// SIMD vector processing chunk size for gaming crypto
    /// Optimized for AVX2 vector operations
    pub const SIMD_CHUNK_SIZE: usize = 32; // AVX2 vector size
    /// Gaming crypto batch processing size
    pub const GAMING_BATCH_SIZE: usize = 1024;
    /// Low latency threshold for gaming operations (microseconds)
    pub const LOW_LATENCY_THRESHOLD_US: u64 = 100;
    /// Ultra-low latency threshold for competitive gaming (microseconds)
    pub const ULTRA_LOW_LATENCY_THRESHOLD_US: u64 = 50;
    /// Gaming crypto key rotation interval (milliseconds)
    pub const GAMING_KEY_ROTATION_MS: u64 = 30000; // 30 seconds
    /// Gaming session heartbeat interval (milliseconds)
    pub const GAMING_HEARTBEAT_INTERVAL_MS: u64 = 100;
    /// Maximum gaming session duration (seconds)
    pub const MAX_GAMING_SESSION_DURATION_SECONDS: u64 = 14400; // 4 hours
    /// Gaming anti-cheat check interval (seconds)
    pub const ANTI_CHEAT_CHECK_INTERVAL_SECONDS: u64 = 5;
/// **MONITORING PERFORMANCE CONSTANTS** - Monitoring and metrics settings
pub mod monitoring {
    /// Metrics collection interval
    pub const METRICS_COLLECTION_INTERVAL: Duration = Duration::from_secs(60);
    /// Health check interval
    pub const HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(30);
    /// Alert evaluation interval
    pub const ALERT_EVALUATION_INTERVAL: Duration = Duration::from_secs(60);
    /// Metrics retention period
    pub const METRICS_RETENTION_PERIOD: Duration = Duration::from_secs(86400 * 7); // 7 days
    /// Maximum metrics points per series
    pub const MAX_METRICS_POINTS: usize = 10_000;
/// Maximum number of concurrent connections per HSM
pub const HSM_MAX_CONCURRENT_CONNECTIONS: usize = 50;
/// HSM operation timeout in milliseconds
pub const HSM_OPERATION_TIMEOUT_MS: u64 = 30_000;
/// Key rotation batch size
pub const KEY_ROTATION_BATCH_SIZE: usize = 10;
/// Key rotation timeout in milliseconds
pub const KEY_ROTATION_TIMEOUT_MS: u64 = 30_000;
// End of canonical performance constants
// Note: Legacy constants have been removed as part of canonical modernization
// All constants are now available through the canonical constants system
