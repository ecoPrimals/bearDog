// SPDX-License-Identifier: AGPL-3.0-or-later

//! Centralized environment variable key constants.
//!
//! All `BEARDOG_*` env var names used across the codebase should be defined
//! here. Callers use these constants instead of inline string literals,
//! enabling rename refactors and grep-based auditing.
//!
//! Organized by domain, matching the `beardog-config/src/domains/` structure.

// ── Paths ────────────────────────────────────────────────────────────

/// Override the configuration directory.
pub const ENV_CONFIG_DIR: &str = "BEARDOG_CONFIG_DIR";
/// Override the data directory.
pub const ENV_DATA_DIR: &str = "BEARDOG_DATA_DIR";
/// Override the log directory.
pub const ENV_LOG_DIR: &str = "BEARDOG_LOG_DIR";
/// PKCS#11 library path override.
pub const ENV_PKCS11_LIBRARY: &str = "BEARDOG_PKCS11_LIBRARY";
/// PKCS#11 search paths (colon-separated).
pub const ENV_PKCS11_SEARCH_PATHS: &str = "BEARDOG_PKCS11_SEARCH_PATHS";

// ── Network addresses ────────────────────────────────────────────────

/// API host address.
pub const ENV_API_HOST: &str = "BEARDOG_API_HOST";
/// Listen / bind address (alias: `BEARDOG_BIND_ADDRESS`).
pub const ENV_LISTEN_ADDR: &str = "BEARDOG_LISTEN_ADDR";
/// Bind address (canonical form).
pub const ENV_BIND_ADDRESS: &str = "BEARDOG_BIND_ADDRESS";
/// External host for public-facing URLs.
pub const ENV_EXTERNAL_HOST: &str = "BEARDOG_EXTERNAL_HOST";
/// Multicast address for discovery.
pub const ENV_MULTICAST_ADDRESS: &str = "BEARDOG_MULTICAST_ADDRESS";
/// API bind address (combined host:port).
pub const ENV_API_BIND_ADDRESS: &str = "BEARDOG_API_BIND_ADDRESS";
/// Infrastructure host fallback.
pub const ENV_INFRASTRUCTURE_HOST_FALLBACK: &str = "BEARDOG_INFRASTRUCTURE_HOST_FALLBACK";
/// Localhost IPv4 address override.
pub const ENV_LOCALHOST_IPV4: &str = "BEARDOG_LOCALHOST_IPV4";
/// Localhost IPv6 address override.
pub const ENV_LOCALHOST_IPV6: &str = "BEARDOG_LOCALHOST_IPV6";
/// Wildcard IPv4 address override (all interfaces).
pub const ENV_WILDCARD_IPV4: &str = "BEARDOG_WILDCARD_IPV4";

// ── Network hosts ────────────────────────────────────────────────────

/// Client host address.
pub const ENV_CLIENT_HOST: &str = "BEARDOG_CLIENT_HOST";
/// Discovery host address.
pub const ENV_DISCOVERY_HOST: &str = "BEARDOG_DISCOVERY_HOST";
/// Database host address.
pub const ENV_DATABASE_HOST: &str = "BEARDOG_DATABASE_HOST";
/// Redis host address.
pub const ENV_REDIS_HOST: &str = "BEARDOG_REDIS_HOST";
/// Metrics host address.
pub const ENV_METRICS_HOST: &str = "BEARDOG_METRICS_HOST";

// ── Network ports ────────────────────────────────────────────────────

/// API server port.
pub const ENV_API_PORT: &str = "BEARDOG_API_PORT";
/// Discovery service port.
pub const ENV_DISCOVERY_PORT: &str = "BEARDOG_DISCOVERY_PORT";
/// Admin interface port.
pub const ENV_ADMIN_PORT: &str = "BEARDOG_ADMIN_PORT";
/// HTTPS port.
pub const ENV_HTTPS_PORT: &str = "BEARDOG_HTTPS_PORT";
/// Metrics / Prometheus port.
pub const ENV_METRICS_PORT: &str = "BEARDOG_METRICS_PORT";
/// Health check port.
pub const ENV_HEALTH_PORT: &str = "BEARDOG_HEALTH_PORT";
/// TCP IPC port (opt-in; absent = UDS-only mode).
pub const ENV_TCP_IPC_PORT: &str = "BEARDOG_TCP_IPC_PORT";
/// UPA URL (combined host + port).
pub const ENV_UPA_URL: &str = "BEARDOG_UPA_URL";

// ── Network (misc) ───────────────────────────────────────────────────

/// Maximum API connections.
pub const ENV_API_MAX_CONNECTIONS: &str = "BEARDOG_API_MAX_CONNECTIONS";
/// Discovery interval (seconds).
pub const ENV_DISCOVERY_INTERVAL_SECS: &str = "BEARDOG_DISCOVERY_INTERVAL_SECS";
/// Admin bind address.
pub const ENV_ADMIN_BIND_ADDRESS: &str = "BEARDOG_ADMIN_BIND_ADDRESS";
/// Enable admin interface.
pub const ENV_ADMIN_ENABLED: &str = "BEARDOG_ADMIN_ENABLED";

// ── Port discovery ───────────────────────────────────────────────────

/// Discovered primal ports (comma-separated).
pub const ENV_DISCOVERED_PRIMAL_PORTS: &str = "BEARDOG_DISCOVERED_PRIMAL_PORTS";
/// Port probe bind address.
pub const ENV_PORT_PROBE_BIND: &str = "BEARDOG_PORT_PROBE_BIND";
/// Port discovery minimum port (scan range lower bound).
pub const ENV_PORT_DISCOVERY_MIN: &str = "BEARDOG_PORT_DISCOVERY_MIN";
/// Port discovery maximum port (scan range upper bound).
pub const ENV_PORT_DISCOVERY_MAX: &str = "BEARDOG_PORT_DISCOVERY_MAX";
/// Port discovery timeout (milliseconds).
pub const ENV_PORT_DISCOVERY_TIMEOUT_MS: &str = "BEARDOG_PORT_DISCOVERY_TIMEOUT_MS";
/// Ports excluded from discovery.
pub const ENV_PORT_DISCOVERY_EXCLUDE: &str = "BEARDOG_PORT_DISCOVERY_EXCLUDE";

// ── Monitoring ───────────────────────────────────────────────────────

/// Log level (e.g. `debug`, `info`, `warn`).
pub const ENV_LOG_LEVEL: &str = "BEARDOG_LOG_LEVEL";
/// Log format (e.g. `json`, `pretty`).
pub const ENV_LOG_FORMAT: &str = "BEARDOG_LOG_FORMAT";
/// Tracing sample rate (0.0–1.0).
pub const ENV_TRACING_SAMPLE_RATE: &str = "BEARDOG_TRACING_SAMPLE_RATE";

// ── Health checks ────────────────────────────────────────────────────

/// Global health check timeout (seconds).
pub const ENV_HEALTH_GLOBAL_TIMEOUT_SECS: &str = "BEARDOG_HEALTH_GLOBAL_TIMEOUT_SECS";
/// Health check interval (seconds).
pub const ENV_HEALTH_CHECK_INTERVAL_SECS: &str = "BEARDOG_HEALTH_CHECK_INTERVAL_SECS";
/// Consecutive failures before unhealthy.
pub const ENV_HEALTH_FAILURE_THRESHOLD: &str = "BEARDOG_HEALTH_FAILURE_THRESHOLD";
/// Consecutive successes before healthy.
pub const ENV_HEALTH_SUCCESS_THRESHOLD: &str = "BEARDOG_HEALTH_SUCCESS_THRESHOLD";
/// HTTP health check timeout (seconds).
pub const ENV_HTTP_HEALTH_CHECK_TIMEOUT_SECS: &str = "BEARDOG_HTTP_HEALTH_CHECK_TIMEOUT_SECS";
/// TCP health check timeout (seconds).
pub const ENV_TCP_HEALTH_CHECK_TIMEOUT_SECS: &str = "BEARDOG_TCP_HEALTH_CHECK_TIMEOUT_SECS";
/// Database health check timeout (seconds).
pub const ENV_DB_HEALTH_CHECK_TIMEOUT_SECS: &str = "BEARDOG_DB_HEALTH_CHECK_TIMEOUT_SECS";
/// Service discovery interval (seconds).
pub const ENV_SERVICE_DISCOVERY_INTERVAL_SECS: &str = "BEARDOG_SERVICE_DISCOVERY_INTERVAL_SECS";
/// Auto-recovery max attempts.
pub const ENV_AUTO_RECOVERY_ATTEMPTS: &str = "BEARDOG_AUTO_RECOVERY_ATTEMPTS";
/// Recovery delay (seconds).
pub const ENV_HEALTH_RECOVERY_DELAY_SECS: &str = "BEARDOG_HEALTH_RECOVERY_DELAY_SECS";
/// Recovery backoff multiplier.
pub const ENV_HEALTH_RECOVERY_BACKOFF: &str = "BEARDOG_HEALTH_RECOVERY_BACKOFF";
/// Maximum recovery delay (seconds).
pub const ENV_HEALTH_MAX_RECOVERY_DELAY_SECS: &str = "BEARDOG_HEALTH_MAX_RECOVERY_DELAY_SECS";
/// Health check timeout (seconds) — used by timeout builder.
pub const ENV_HEALTH_CHECK_TIMEOUT_SECS: &str = "BEARDOG_HEALTH_CHECK_TIMEOUT_SECS";

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

// ── Security ─────────────────────────────────────────────────────────

/// Minimum TLS version (e.g. `1.2`, `1.3`).
pub const ENV_MIN_TLS_VERSION: &str = "BEARDOG_MIN_TLS_VERSION";
/// TLS mode for server startup.
pub const ENV_TLS_MODE: &str = "BEARDOG_TLS_MODE";
/// PEM certificate chain path for TLS termination.
pub const ENV_TLS_CERT_PATH: &str = "BEARDOG_TLS_CERT_PATH";
/// PEM private key path for TLS termination.
pub const ENV_TLS_KEY_PATH: &str = "BEARDOG_TLS_KEY_PATH";
/// Enable strict security mode.
pub const ENV_STRICT_MODE: &str = "BEARDOG_STRICT_MODE";
/// Require mutual TLS for connections.
pub const ENV_REQUIRE_MTLS: &str = "BEARDOG_REQUIRE_MTLS";
/// Allow loopback connections without authentication.
pub const ENV_ALLOW_LOCALHOST_BYPASS: &str = "BEARDOG_ALLOW_LOCALHOST_BYPASS";
/// Enable security audit logging.
pub const ENV_ENABLE_AUDIT_LOG: &str = "BEARDOG_ENABLE_AUDIT_LOG";
/// Enable request rate limiting.
pub const ENV_ENABLE_RATE_LIMITING: &str = "BEARDOG_ENABLE_RATE_LIMITING";
/// Per-IP max connections per rate-limit window.
pub const ENV_RATE_LIMIT_MAX_CONN: &str = "BEARDOG_RATE_LIMIT_MAX_CONN";
/// Rate-limit sliding window duration (seconds).
pub const ENV_RATE_LIMIT_WINDOW_SECS: &str = "BEARDOG_RATE_LIMIT_WINDOW_SECS";
/// Global max concurrent connections before rate limiting rejects.
pub const ENV_RATE_LIMIT_MAX_TOTAL: &str = "BEARDOG_RATE_LIMIT_MAX_TOTAL";
/// Automatically block suspicious source IPs.
pub const ENV_AUTO_BLOCK_SUSPICIOUS_IPS: &str = "BEARDOG_AUTO_BLOCK_SUSPICIOUS_IPS";
/// Require authentication for all endpoints.
pub const ENV_REQUIRE_AUTHENTICATION: &str = "BEARDOG_REQUIRE_AUTHENTICATION";
/// `MethodGate` enforcement mode (`permissive` or `enforced`).
pub const ENV_AUTH_MODE: &str = "BEARDOG_AUTH_MODE";
/// BTSP `BirdSong` HSM key label.
pub const ENV_BTSP_BIRDSONG_KEY_LABEL: &str = "BEARDOG_BTSP_BIRDSONG_KEY_LABEL";
/// BTSP lineage root prefix.
pub const ENV_BTSP_LINEAGE_ROOT_PREFIX: &str = "BEARDOG_BTSP_LINEAGE_ROOT_PREFIX";
/// BTSP maximum lineage depth.
pub const ENV_BTSP_LINEAGE_MAX_DEPTH: &str = "BEARDOG_BTSP_LINEAGE_MAX_DEPTH";

// ── Crypto ───────────────────────────────────────────────────────────

/// RSA key size (bits).
pub const ENV_RSA_KEY_SIZE: &str = "BEARDOG_RSA_KEY_SIZE";
/// Elliptic curve name.
pub const ENV_EC_CURVE: &str = "BEARDOG_EC_CURVE";
/// AES key size (bits).
pub const ENV_AES_KEY_SIZE: &str = "BEARDOG_AES_KEY_SIZE";
/// Hash algorithm name.
pub const ENV_HASH_ALGORITHM: &str = "BEARDOG_HASH_ALGORITHM";
/// PBKDF2 iteration count.
pub const ENV_PBKDF2_ITERATIONS: &str = "BEARDOG_PBKDF2_ITERATIONS";

// ── HSM ──────────────────────────────────────────────────────────────

/// Enable HSM auto-detection at startup.
pub const ENV_HSM_AUTO_DETECT: &str = "BEARDOG_HSM_AUTO_DETECT";
/// Prefer hardware HSM over software fallback.
pub const ENV_HSM_PREFER_HARDWARE: &str = "BEARDOG_HSM_PREFER_HARDWARE";
/// Enable `SoftHSM` backend.
pub const ENV_HSM_ENABLE_SOFTHSM: &str = "BEARDOG_HSM_ENABLE_SOFTHSM";
/// Enable `YubiHSM` backend.
pub const ENV_HSM_ENABLE_YUBIHSM: &str = "BEARDOG_HSM_ENABLE_YUBIHSM";
/// Enable TPM backend.
pub const ENV_HSM_ENABLE_TPM: &str = "BEARDOG_HSM_ENABLE_TPM";
/// Enable Android `StrongBox` backend.
pub const ENV_HSM_ENABLE_STRONGBOX: &str = "BEARDOG_HSM_ENABLE_STRONGBOX";
/// `YubiHSM` connector URL.
pub const ENV_YUBIHSM_CONNECTOR: &str = "BEARDOG_YUBIHSM_CONNECTOR";

// ── ACME ─────────────────────────────────────────────────────────────

/// ACME directory URL.
pub const ENV_ACME_DIRECTORY: &str = "BEARDOG_ACME_DIRECTORY";
/// ACME email contact.
pub const ENV_ACME_EMAIL: &str = "BEARDOG_ACME_EMAIL";
/// ACME domains (comma-separated).
pub const ENV_ACME_DOMAINS: &str = "BEARDOG_ACME_DOMAINS";
/// ACME HTTP-01 challenge bind port.
pub const ENV_ACME_CHALLENGE_PORT: &str = "BEARDOG_ACME_CHALLENGE_PORT";
/// Days before certificate expiry to trigger ACME renewal.
pub const ENV_ACME_RENEWAL_DAYS: &str = "BEARDOG_ACME_RENEWAL_DAYS";

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
