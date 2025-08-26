

use std::time::Duration;

pub use crate::canonical::constants::{
    CONCURRENT_TASKS, HEAVY_ITERATIONS, LIGHT_ITERATIONS, OPERATIONS_PER_TASK, STANDARD_ITERATIONS,
    TARGET_RPS, TEST_DATA_SIZE,
};

pub mod testing {

    pub use crate::canonical::constants::performance::{
        STANDARD_ITERATIONS, LIGHT_ITERATIONS, HEAVY_ITERATIONS,
        CONCURRENT_TASKS, TARGET_RPS, OPERATIONS_PER_TASK
    };

    pub use crate::canonical::constants::{
        CONCURRENT_TASKS as CONCURRENT_USERS, // Alias for backward compatibility
        TEST_DATA_SIZE,
    };

    pub const LOAD_TEST_DURATION_SECONDS: u64 = 10;
}

pub mod cache {
    use super::Duration;

    pub const STANDARD_CACHE_SIZE: usize = 10_000;

    pub const SMALL_CACHE_SIZE: usize = 1_000;

    pub const LARGE_CACHE_SIZE: usize = 100_000;

    pub const STANDARD_TTL: Duration = Duration::from_secs(3600);

    pub const SHORT_TTL: Duration = Duration::from_secs(300);

    pub const LONG_TTL: Duration = Duration::from_secs(86400);

pub mod network {

    pub const CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);

    pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);

    pub const KEEP_ALIVE_TIMEOUT: Duration = Duration::from_secs(300);

    pub const MAX_CONCURRENT_CONNECTIONS: usize = 1000;

    pub const CONNECTION_POOL_SIZE: usize = 100;

pub mod memory {

    pub const BUFFER_SIZE: usize = 64 * 1024; // 64KB

    pub const MAX_MEMORY_USAGE_MB: usize = 1024; // 1GB

    pub const MEMORY_WARNING_THRESHOLD_MB: usize = 512; // 512MB

    pub const GC_THRESHOLD_MB: usize = 256; // 256MB

pub mod threading {

    pub const DEFAULT_THREAD_POOL_SIZE: usize = 8; // Reasonable default, use num_cpus::get() at runtime

    pub const MAX_THREAD_POOL_SIZE: usize = 128;

    pub const THREAD_STACK_SIZE: usize = 2 * 1024 * 1024; // 2MB

    pub const WORKER_KEEPALIVE_SECONDS: u64 = 60;

pub mod database {
    pub const CONNECTION_POOL_SIZE: usize = 20;

    pub const MAX_CONNECTION_LIFETIME: Duration = Duration::from_secs(3600);

    pub const IDLE_TIMEOUT: Duration = Duration::from_secs(600);

    pub const QUERY_TIMEOUT: Duration = Duration::from_secs(30);

    pub const TRANSACTION_TIMEOUT: Duration = Duration::from_secs(300);

    pub const BATCH_SIZE: usize = 1000;

pub mod crypto {

    pub const KEY_DERIVATION_ITERATIONS: u32 = 100_000;

    pub const HASH_ROUNDS: u32 = 12;

    pub const SALT_SIZE: usize = 32;

    pub const IV_SIZE: usize = 16;

    pub const SYMMETRIC_KEY_SIZE: usize = 32; // 256 bits

pub mod workflow {

    pub const MAX_EXECUTION_TIME: Duration = Duration::from_secs(3600); // 1 hour

    pub const STEP_TIMEOUT: Duration = Duration::from_secs(300); // 5 minutes

    pub const MAX_CONCURRENT_WORKFLOWS: usize = 100;

    pub const MAX_RETRY_ATTEMPTS: u32 = 30;

    pub const RETRY_BACKOFF_MULTIPLIER: f64 = 2.0;

    pub const DEFAULT_RETENTION_DAYS: i64 = 30;

    pub const DEFAULT_TIMEOUT_HOURS: i64 = 24;

    pub const COMPLETED_IDLE_DAYS: i64 = 7;

    pub const EXPIRY_WARNING_HOURS: i64 = 2;

    pub const MAX_STALE_DAYS: i64 = 7;

    pub const INITIAL_RETRY_DELAY_SECONDS: u64 = 1;

pub mod gaming {

    pub const SIMD_CHUNK_SIZE: usize = 32; // AVX2 vector size

    pub const GAMING_BATCH_SIZE: usize = 1024;

    pub const LOW_LATENCY_THRESHOLD_US: u64 = 100;

    pub const ULTRA_LOW_LATENCY_THRESHOLD_US: u64 = 50;

    pub const GAMING_KEY_ROTATION_MS: u64 = 30000; // 30 seconds

    pub const GAMING_HEARTBEAT_INTERVAL_MS: u64 = 100;

    pub const MAX_GAMING_SESSION_DURATION_SECONDS: u64 = 14400; // 4 hours

    pub const ANTI_CHEAT_CHECK_INTERVAL_SECONDS: u64 = 5;

pub mod monitoring {

    pub const METRICS_COLLECTION_INTERVAL: Duration = Duration::from_secs(60);

    pub const HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(30);

    pub const ALERT_EVALUATION_INTERVAL: Duration = Duration::from_secs(60);

    pub const METRICS_RETENTION_PERIOD: Duration = Duration::from_secs(86400 * 7); // 7 days

    pub const MAX_METRICS_POINTS: usize = 10_000;

pub const HSM_MAX_CONCURRENT_CONNECTIONS: usize = 50;

pub const HSM_OPERATION_TIMEOUT_MS: u64 = 30_000;

pub const KEY_ROTATION_BATCH_SIZE: usize = 10;

pub const KEY_ROTATION_TIMEOUT_MS: u64 = 30_000;

