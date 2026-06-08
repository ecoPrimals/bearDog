// SPDX-License-Identifier: AGPL-3.0-or-later

//! Timeouts, limits, capacity, retry policy, production resources, and workflow environment variable keys.

// ── Timeouts ─────────────────────────────────────────────────────────

/// HSM operation timeout (seconds).
pub const ENV_HSM_OPERATION_TIMEOUT_SECS: &str = "BEARDOG_HSM_OPERATION_TIMEOUT_SECS";
/// HSM probe timeout (milliseconds).
pub const ENV_HSM_PROBE_TIMEOUT_MILLIS: &str = "BEARDOG_HSM_PROBE_TIMEOUT_MILLIS";
/// Database pool idle timeout (seconds).
pub const ENV_POOL_IDLE_TIMEOUT_SECS: &str = "BEARDOG_POOL_IDLE_TIMEOUT_SECS";
/// Maximum database connection age (seconds).
pub const ENV_MAX_CONNECTION_AGE_SECS: &str = "BEARDOG_MAX_CONNECTION_AGE_SECS";
/// Discovery timeout (seconds).
pub const ENV_DISCOVERY_TIMEOUT_SECS: &str = "BEARDOG_DISCOVERY_TIMEOUT_SECS";
/// AI decision timeout (seconds).
pub const ENV_DECISION_TIMEOUT_SECS: &str = "BEARDOG_DECISION_TIMEOUT_SECS";
/// AI request timeout (seconds).
pub const ENV_AI_REQUEST_TIMEOUT_SECS: &str = "BEARDOG_AI_REQUEST_TIMEOUT_SECS";
/// AI batch timeout (milliseconds).
pub const ENV_AI_BATCH_TIMEOUT_MS: &str = "BEARDOG_AI_BATCH_TIMEOUT_MS";
/// Connection establishment timeout (seconds).
pub const ENV_CONNECTION_TIMEOUT_SECS: &str = "BEARDOG_CONNECTION_TIMEOUT_SECS";
/// Network handshake timeout (seconds).
pub const ENV_HANDSHAKE_TIMEOUT_SECS: &str = "BEARDOG_HANDSHAKE_TIMEOUT_SECS";
/// TLS handshake timeout (seconds).
pub const ENV_TLS_HANDSHAKE_TIMEOUT_SECS: &str = "BEARDOG_TLS_HANDSHAKE_TIMEOUT_SECS";
/// Keep-alive timeout (seconds).
pub const ENV_KEEP_ALIVE_TIMEOUT_SECS: &str = "BEARDOG_KEEP_ALIVE_TIMEOUT_SECS";
/// Idle connection timeout (seconds).
pub const ENV_IDLE_CONNECTION_TIMEOUT_SECS: &str = "BEARDOG_IDLE_CONNECTION_TIMEOUT_SECS";
/// Read operation timeout (seconds).
pub const ENV_READ_TIMEOUT_SECS: &str = "BEARDOG_READ_TIMEOUT_SECS";
/// Write operation timeout (seconds).
pub const ENV_WRITE_TIMEOUT_SECS: &str = "BEARDOG_WRITE_TIMEOUT_SECS";
/// HTTP request timeout (seconds).
pub const ENV_HTTP_REQUEST_TIMEOUT_SECS: &str = "BEARDOG_HTTP_REQUEST_TIMEOUT_SECS";
/// HTTP response timeout (seconds).
pub const ENV_HTTP_RESPONSE_TIMEOUT_SECS: &str = "BEARDOG_HTTP_RESPONSE_TIMEOUT_SECS";
/// DNS resolution timeout (seconds).
pub const ENV_DNS_RESOLUTION_TIMEOUT_SECS: &str = "BEARDOG_DNS_RESOLUTION_TIMEOUT_SECS";
/// Retry timeout (milliseconds).
pub const ENV_RETRY_TIMEOUT_MILLIS: &str = "BEARDOG_RETRY_TIMEOUT_MILLIS";
/// Backoff timeout (milliseconds).
pub const ENV_BACKOFF_TIMEOUT_MILLIS: &str = "BEARDOG_BACKOFF_TIMEOUT_MILLIS";
/// Ping timeout (seconds).
pub const ENV_PING_TIMEOUT_SECS: &str = "BEARDOG_PING_TIMEOUT_SECS";
/// Heartbeat timeout (seconds).
pub const ENV_HEARTBEAT_TIMEOUT_SECS: &str = "BEARDOG_HEARTBEAT_TIMEOUT_SECS";
/// HTTP request timeout (milliseconds).
pub const ENV_REQUEST_TIMEOUT_MS: &str = "BEARDOG_REQUEST_TIMEOUT_MS";
/// Connection establishment timeout (milliseconds).
pub const ENV_CONNECTION_TIMEOUT_MS: &str = "BEARDOG_CONNECTION_TIMEOUT_MS";
/// Keep-alive timeout (seconds, `_S` suffix variant).
pub const ENV_KEEPALIVE_TIMEOUT_S: &str = "BEARDOG_KEEPALIVE_TIMEOUT_S";
/// Global operation timeout (seconds, runtime config).
pub const ENV_TIMEOUT_SECONDS: &str = "BEARDOG_TIMEOUT_SECONDS";
/// Connect timeout (seconds, zero-hardcoding).
pub const ENV_TIMEOUT_CONNECT: &str = "BEARDOG_TIMEOUT_CONNECT";
/// Request timeout (seconds, zero-hardcoding).
pub const ENV_TIMEOUT_REQUEST: &str = "BEARDOG_TIMEOUT_REQUEST";
/// Idle timeout (seconds, zero-hardcoding).
pub const ENV_TIMEOUT_IDLE: &str = "BEARDOG_TIMEOUT_IDLE";
/// Discovery timeout (seconds, zero-hardcoding).
pub const ENV_TIMEOUT_DISCOVERY: &str = "BEARDOG_TIMEOUT_DISCOVERY";
/// Shutdown timeout (seconds, zero-hardcoding).
pub const ENV_TIMEOUT_SHUTDOWN: &str = "BEARDOG_TIMEOUT_SHUTDOWN";
/// Health check timeout (seconds, zero-hardcoding).
pub const ENV_TIMEOUT_HEALTH: &str = "BEARDOG_TIMEOUT_HEALTH";
/// Database query timeout (seconds, zero-hardcoding).
pub const ENV_TIMEOUT_DB_QUERY: &str = "BEARDOG_TIMEOUT_DB_QUERY";
// ── Limits ───────────────────────────────────────────────────────────

/// I/O buffer size in bytes.
pub const ENV_BUFFER_SIZE: &str = "BEARDOG_BUFFER_SIZE";
/// Maximum concurrent connections.
pub const ENV_MAX_CONNECTIONS: &str = "BEARDOG_MAX_CONNECTIONS";
/// Maximum retry attempts.
pub const ENV_MAX_RETRIES: &str = "BEARDOG_MAX_RETRIES";
/// Backoff duration between retries (milliseconds).
pub const ENV_BACKOFF_MS: &str = "BEARDOG_BACKOFF_MS";
/// Maximum message size in bytes.
pub const ENV_MAX_MESSAGE_SIZE: &str = "BEARDOG_MAX_MESSAGE_SIZE";
/// Queue size for async operations.
pub const ENV_QUEUE_SIZE: &str = "BEARDOG_QUEUE_SIZE";
/// Thread pool size (0 = automatic based on CPU count).
pub const ENV_THREAD_POOL_SIZE: &str = "BEARDOG_THREAD_POOL_SIZE";
/// Default operation timeout (seconds).
pub const ENV_OPERATION_TIMEOUT_SECS: &str = "BEARDOG_OPERATION_TIMEOUT_SECS";

// ── Capacity ─────────────────────────────────────────────────────────

/// Default channel buffer size.
pub const ENV_CHANNEL_BUFFER: &str = "BEARDOG_CHANNEL_BUFFER";
/// Discovery queue capacity.
pub const ENV_DISCOVERY_QUEUE_SIZE: &str = "BEARDOG_DISCOVERY_QUEUE_SIZE";
/// Event bus capacity.
pub const ENV_EVENT_BUS_CAPACITY: &str = "BEARDOG_EVENT_BUS_CAPACITY";
/// Minimum idle connections in pool.
pub const ENV_MIN_IDLE_CONNECTIONS: &str = "BEARDOG_MIN_IDLE_CONNECTIONS";
/// Connection pool acquire timeout (seconds).
pub const ENV_CONNECTION_POOL_TIMEOUT_SECS: &str = "BEARDOG_CONNECTION_POOL_TIMEOUT_SECS";
/// Maximum message size in bytes (capacity domain).
pub const ENV_MAX_MESSAGE_SIZE_BYTES: &str = "BEARDOG_MAX_MESSAGE_SIZE_BYTES";
/// Buffer pool capacity.
pub const ENV_BUFFER_POOL_SIZE: &str = "BEARDOG_BUFFER_POOL_SIZE";
/// Maximum cache entries.
pub const ENV_CACHE_MAX_ENTRIES: &str = "BEARDOG_CACHE_MAX_ENTRIES";
/// General-purpose cache size (entries or MB depending on context).
pub const ENV_CACHE_SIZE: &str = "BEARDOG_CACHE_SIZE";
/// General-purpose cache TTL (seconds or duration string depending on context).
pub const ENV_CACHE_TTL: &str = "BEARDOG_CACHE_TTL";

// ── Retry policy (zero-hardcoding) ───────────────────────────────────────

/// Maximum retry attempts.
pub const ENV_RETRY_MAX_ATTEMPTS: &str = "BEARDOG_RETRY_MAX_ATTEMPTS";
/// Initial retry backoff (milliseconds).
pub const ENV_RETRY_INITIAL_BACKOFF_MS: &str = "BEARDOG_RETRY_INITIAL_BACKOFF_MS";
/// Maximum retry backoff (seconds).
pub const ENV_RETRY_MAX_BACKOFF_SECS: &str = "BEARDOG_RETRY_MAX_BACKOFF_SECS";
/// Retry backoff multiplier.
pub const ENV_RETRY_BACKOFF_MULTIPLIER: &str = "BEARDOG_RETRY_BACKOFF_MULTIPLIER";

// ── Tunnel ─────────────────────────────────────────────────────────────

/// Tunnel key storage path override.
pub const ENV_TUNNEL_KEY_STORAGE_PATH: &str = "BEARDOG_TUNNEL_KEY_STORAGE_PATH";
/// Gaming anti-cheat capability flag.
pub const ENV_GAMING_ANTI_CHEAT_CAPABILITY: &str = "BEARDOG_GAMING_ANTI_CHEAT_CAPABILITY";

// ── Production resources ─────────────────────────────────────────────

/// Production max connections.
pub const ENV_PROD_PRODUCTION_MAX_CONNECTIONS: &str = "BEARDOG_PROD_PRODUCTION_MAX_CONNECTIONS";
/// Production connection timeout (seconds).
pub const ENV_PROD_PRODUCTION_CONNECTION_TIMEOUT_SECS: &str =
    "BEARDOG_PROD_PRODUCTION_CONNECTION_TIMEOUT_SECS";
/// Production temp cleanup interval (seconds).
pub const ENV_PROD_PRODUCTION_TEMP_CLEANUP_INTERVAL_SECS: &str =
    "BEARDOG_PROD_PRODUCTION_TEMP_CLEANUP_INTERVAL_SECS";
/// Production log rotation size (MB).
pub const ENV_PRODUCTION_LOG_ROTATION_SIZE_MB: &str = "BEARDOG_PRODUCTION_LOG_ROTATION_SIZE_MB";
/// Production log retention (days).
pub const ENV_PROD_PRODUCTION_LOG_RETENTION_DAYS: &str =
    "BEARDOG_PROD_PRODUCTION_LOG_RETENTION_DAYS";
/// Production connection pool size.
pub const ENV_PROD_PRODUCTION_POOL_SIZE: &str = "BEARDOG_PROD_PRODUCTION_POOL_SIZE";
/// Production max idle connections.
pub const ENV_PROD_PRODUCTION_MAX_IDLE_CONNECTIONS: &str =
    "BEARDOG_PROD_PRODUCTION_MAX_IDLE_CONNECTIONS";
/// Production connection lifetime (seconds).
pub const ENV_PROD_PRODUCTION_CONNECTION_LIFETIME_SECS: &str =
    "BEARDOG_PROD_PRODUCTION_CONNECTION_LIFETIME_SECS";
/// Production health check interval (seconds).
pub const ENV_PROD_PRODUCTION_HEALTH_CHECK_INTERVAL_SECS: &str =
    "BEARDOG_PROD_PRODUCTION_HEALTH_CHECK_INTERVAL_SECS";
/// Production max disk usage (percent).
pub const ENV_PROD_PRODUCTION_MAX_DISK_USAGE_PERCENT: &str =
    "BEARDOG_PROD_PRODUCTION_MAX_DISK_USAGE_PERCENT";

// ── Production resources (generic) ───────────────────────────────────

/// Production max connections (generic profile).
pub const ENV_PROD_MAX_CONNECTIONS: &str = "BEARDOG_PROD_MAX_CONNECTIONS";
/// Production connection timeout (seconds, generic profile).
pub const ENV_PROD_CONNECTION_TIMEOUT_SECS: &str = "BEARDOG_PROD_CONNECTION_TIMEOUT_SECS";
/// Production read timeout (seconds, generic profile).
pub const ENV_PROD_READ_TIMEOUT_SECS: &str = "BEARDOG_PROD_READ_TIMEOUT_SECS";
/// Production write timeout (seconds, generic profile).
pub const ENV_PROD_WRITE_TIMEOUT_SECS: &str = "BEARDOG_PROD_WRITE_TIMEOUT_SECS";
/// Max disk usage (percent).
pub const ENV_MAX_DISK_USAGE_PERCENT: &str = "BEARDOG_MAX_DISK_USAGE_PERCENT";
/// Temp directory cleanup interval (seconds).
pub const ENV_TEMP_CLEANUP_INTERVAL_SECS: &str = "BEARDOG_TEMP_CLEANUP_INTERVAL_SECS";
/// Log rotation size (MB, generic profile).
pub const ENV_LOG_ROTATION_SIZE_MB: &str = "BEARDOG_LOG_ROTATION_SIZE_MB";
/// Log retention (days, generic profile).
pub const ENV_LOG_RETENTION_DAYS: &str = "BEARDOG_LOG_RETENTION_DAYS";
/// Connection pool size.
pub const ENV_CONNECTION_POOL_SIZE: &str = "BEARDOG_CONNECTION_POOL_SIZE";
/// Max idle connections.
pub const ENV_MAX_IDLE_CONNECTIONS: &str = "BEARDOG_MAX_IDLE_CONNECTIONS";
/// Connection lifetime (seconds).
pub const ENV_CONNECTION_LIFETIME_SECS: &str = "BEARDOG_CONNECTION_LIFETIME_SECS";
/// Connection health check interval (seconds).
pub const ENV_CONNECTION_HEALTH_CHECK_INTERVAL_SECS: &str =
    "BEARDOG_CONNECTION_HEALTH_CHECK_INTERVAL_SECS";
/// GC target pause (milliseconds).
pub const ENV_GC_TARGET_PAUSE_MS: &str = "BEARDOG_GC_TARGET_PAUSE_MS";
/// GC throughput target (percent).
pub const ENV_GC_THROUGHPUT_TARGET_PERCENT: &str = "BEARDOG_GC_THROUGHPUT_TARGET_PERCENT";

// ── Workflow ─────────────────────────────────────────────────────────────

/// Enable workflow engine.
pub const ENV_WORKFLOW_ENABLED: &str = "BEARDOG_WORKFLOW_ENABLED";
/// Workflow worker pool size.
pub const ENV_WORKFLOW_WORKER_POOL_SIZE: &str = "BEARDOG_WORKFLOW_WORKER_POOL_SIZE";
/// Workflow queue capacity.
pub const ENV_WORKFLOW_QUEUE_CAPACITY: &str = "BEARDOG_WORKFLOW_QUEUE_CAPACITY";
/// Workflow maximum concurrent executions.
pub const ENV_WORKFLOW_MAX_CONCURRENT: &str = "BEARDOG_WORKFLOW_MAX_CONCURRENT";
/// Enable workflow persistence.
pub const ENV_WORKFLOW_PERSISTENCE_ENABLED: &str = "BEARDOG_WORKFLOW_PERSISTENCE_ENABLED";
/// Workflow persistence backend selector.
pub const ENV_WORKFLOW_PERSISTENCE_BACKEND: &str = "BEARDOG_WORKFLOW_PERSISTENCE_BACKEND";
/// Workflow message TTL (seconds).
pub const ENV_WORKFLOW_MESSAGE_TTL_SECS: &str = "BEARDOG_WORKFLOW_MESSAGE_TTL_SECS";
/// Workflow default timeout (seconds).
pub const ENV_WORKFLOW_TIMEOUT_DEFAULT_SECS: &str = "BEARDOG_WORKFLOW_TIMEOUT_DEFAULT_SECS";
/// Workflow maximum timeout (seconds).
pub const ENV_WORKFLOW_TIMEOUT_MAXIMUM_SECS: &str = "BEARDOG_WORKFLOW_TIMEOUT_MAXIMUM_SECS";
/// Workflow connection timeout (seconds).
pub const ENV_WORKFLOW_TIMEOUT_CONNECTION_SECS: &str = "BEARDOG_WORKFLOW_TIMEOUT_CONNECTION_SECS";
/// Workflow read timeout (seconds).
pub const ENV_WORKFLOW_TIMEOUT_READ_SECS: &str = "BEARDOG_WORKFLOW_TIMEOUT_READ_SECS";
/// Workflow retry max attempts.
pub const ENV_WORKFLOW_RETRY_MAX_ATTEMPTS: &str = "BEARDOG_WORKFLOW_RETRY_MAX_ATTEMPTS";
/// Workflow persistence database URL.
pub const ENV_WORKFLOW_DB_URL: &str = "BEARDOG_WORKFLOW_DB_URL";
/// Workflow persistence database pool size.
pub const ENV_WORKFLOW_DB_POOL_SIZE: &str = "BEARDOG_WORKFLOW_DB_POOL_SIZE";
/// Workflow persistence database timeout (seconds).
pub const ENV_WORKFLOW_DB_TIMEOUT_SECS: &str = "BEARDOG_WORKFLOW_DB_TIMEOUT_SECS";
/// Workflow record retention period (seconds).
pub const ENV_WORKFLOW_RETENTION_PERIOD_SECS: &str = "BEARDOG_WORKFLOW_RETENTION_PERIOD_SECS";
/// Workflow cleanup interval (seconds).
pub const ENV_WORKFLOW_CLEANUP_INTERVAL_SECS: &str = "BEARDOG_WORKFLOW_CLEANUP_INTERVAL_SECS";
