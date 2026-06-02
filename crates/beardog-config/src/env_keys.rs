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
/// Default service host (alias: `BEARDOG_HOST`).
pub const ENV_SERVICE_HOST: &str = "BEARDOG_SERVICE_HOST";
/// Generic host override for network binding.
pub const ENV_HOST: &str = "BEARDOG_HOST";
/// Localhost override for self-discovery endpoints.
pub const ENV_LOCALHOST: &str = "BEARDOG_LOCALHOST";
/// Default host when discovery fails.
pub const ENV_DEFAULT_HOST: &str = "BEARDOG_DEFAULT_HOST";
/// Default URL protocol (e.g. `http`, `https`).
pub const ENV_DEFAULT_PROTOCOL: &str = "BEARDOG_DEFAULT_PROTOCOL";
/// Enable localhost fallback when service discovery fails.
pub const ENV_ENABLE_LOCALHOST_FALLBACK: &str = "BEARDOG_ENABLE_LOCALHOST_FALLBACK";

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
/// Debug/diagnostics port.
pub const ENV_DEBUG_PORT: &str = "BEARDOG_DEBUG_PORT";
/// WebSocket port.
pub const ENV_WS_PORT: &str = "BEARDOG_WS_PORT";
/// Compute service port.
pub const ENV_COMPUTE_PORT: &str = "BEARDOG_COMPUTE_PORT";
/// Compute service port range upper bound.
pub const ENV_COMPUTE_PORT_END: &str = "BEARDOG_COMPUTE_PORT_END";
/// Service mesh port.
pub const ENV_MESH_PORT: &str = "BEARDOG_MESH_PORT";
/// Service mesh bind address.
pub const ENV_MESH_BIND_ADDRESS: &str = "BEARDOG_MESH_BIND_ADDRESS";
/// AI / intelligence service port.
pub const ENV_AI_PORT: &str = "BEARDOG_AI_PORT";
/// Storage service port.
pub const ENV_STORAGE_PORT: &str = "BEARDOG_STORAGE_PORT";
/// Storage service port range lower bound.
pub const ENV_STORAGE_PORT_START: &str = "BEARDOG_STORAGE_PORT_START";
/// Storage service port range upper bound.
pub const ENV_STORAGE_PORT_END: &str = "BEARDOG_STORAGE_PORT_END";
/// Intelligence service port range lower bound.
pub const ENV_INTELLIGENCE_PORT_START: &str = "BEARDOG_INTELLIGENCE_PORT_START";
/// Intelligence service port range upper bound.
pub const ENV_INTELLIGENCE_PORT_END: &str = "BEARDOG_INTELLIGENCE_PORT_END";
/// Intelligence service default port.
pub const ENV_INTELLIGENCE_PORT: &str = "BEARDOG_INTELLIGENCE_PORT";
/// Security service port.
pub const ENV_SECURITY_PORT: &str = "BEARDOG_SECURITY_PORT";
/// Database port (`PostgreSQL`).
pub const ENV_DATABASE_PORT: &str = "BEARDOG_DATABASE_PORT";
/// Grafana dashboard port.
pub const ENV_GRAFANA_PORT: &str = "BEARDOG_GRAFANA_PORT";
/// Jaeger tracing port.
pub const ENV_JAEGER_PORT: &str = "BEARDOG_JAEGER_PORT";
/// gRPC server port.
pub const ENV_GRPC_PORT: &str = "BEARDOG_GRPC_PORT";
/// Consul service discovery port.
pub const ENV_CONSUL_PORT: &str = "BEARDOG_CONSUL_PORT";
/// Redis cache port.
pub const ENV_REDIS_PORT: &str = "BEARDOG_REDIS_PORT";
/// Compute health check port.
pub const ENV_COMPUTE_HEALTH_PORT: &str = "BEARDOG_COMPUTE_HEALTH_PORT";
/// Compute metrics port.
pub const ENV_COMPUTE_METRICS_PORT: &str = "BEARDOG_COMPUTE_METRICS_PORT";
/// Compute admin port.
pub const ENV_COMPUTE_ADMIN_PORT: &str = "BEARDOG_COMPUTE_ADMIN_PORT";
/// Storage health check port.
pub const ENV_STORAGE_HEALTH_PORT: &str = "BEARDOG_STORAGE_HEALTH_PORT";
/// Storage metrics port.
pub const ENV_STORAGE_METRICS_PORT: &str = "BEARDOG_STORAGE_METRICS_PORT";
/// Storage admin port.
pub const ENV_STORAGE_ADMIN_PORT: &str = "BEARDOG_STORAGE_ADMIN_PORT";
/// AI health check port.
pub const ENV_AI_HEALTH_PORT: &str = "BEARDOG_AI_HEALTH_PORT";
/// AI metrics port.
pub const ENV_AI_METRICS_PORT: &str = "BEARDOG_AI_METRICS_PORT";
/// AI admin port.
pub const ENV_AI_ADMIN_PORT: &str = "BEARDOG_AI_ADMIN_PORT";
/// Mesh health check port.
pub const ENV_MESH_HEALTH_PORT: &str = "BEARDOG_MESH_HEALTH_PORT";
/// Mesh metrics port.
pub const ENV_MESH_METRICS_PORT: &str = "BEARDOG_MESH_METRICS_PORT";
/// Mesh admin port.
pub const ENV_MESH_ADMIN_PORT: &str = "BEARDOG_MESH_ADMIN_PORT";
/// Discovery health check port.
pub const ENV_DISCOVERY_HEALTH_PORT: &str = "BEARDOG_DISCOVERY_HEALTH_PORT";
/// Discovery metrics port.
pub const ENV_DISCOVERY_METRICS_PORT: &str = "BEARDOG_DISCOVERY_METRICS_PORT";
/// Discovery admin port.
pub const ENV_DISCOVERY_ADMIN_PORT: &str = "BEARDOG_DISCOVERY_ADMIN_PORT";

// ── Network (misc) ───────────────────────────────────────────────────

/// Maximum API connections.
pub const ENV_API_MAX_CONNECTIONS: &str = "BEARDOG_API_MAX_CONNECTIONS";
/// Discovery interval (seconds).
pub const ENV_DISCOVERY_INTERVAL_SECS: &str = "BEARDOG_DISCOVERY_INTERVAL_SECS";
/// Admin bind address.
pub const ENV_ADMIN_BIND_ADDRESS: &str = "BEARDOG_ADMIN_BIND_ADDRESS";
/// Enable admin interface.
pub const ENV_ADMIN_ENABLED: &str = "BEARDOG_ADMIN_ENABLED";
/// Service discovery endpoint URL.
pub const ENV_DISCOVERY_ENDPOINT: &str = "BEARDOG_DISCOVERY_ENDPOINT";
/// Compute service primary endpoint URL.
pub const ENV_COMPUTE_ENDPOINT: &str = "BEARDOG_COMPUTE_ENDPOINT";
/// Compute service backup endpoints (comma-separated).
pub const ENV_COMPUTE_BACKUPS: &str = "BEARDOG_COMPUTE_BACKUPS";
/// Compute health check endpoint URL.
pub const ENV_COMPUTE_HEALTH_ENDPOINT: &str = "BEARDOG_COMPUTE_HEALTH_ENDPOINT";
/// Compute metrics endpoint URL.
pub const ENV_COMPUTE_METRICS_ENDPOINT: &str = "BEARDOG_COMPUTE_METRICS_ENDPOINT";
/// Storage service primary endpoint URL.
pub const ENV_STORAGE_ENDPOINT: &str = "BEARDOG_STORAGE_ENDPOINT";
/// Storage service backup endpoints (comma-separated).
pub const ENV_STORAGE_BACKUPS: &str = "BEARDOG_STORAGE_BACKUPS";
/// Storage health check endpoint URL.
pub const ENV_STORAGE_HEALTH_ENDPOINT: &str = "BEARDOG_STORAGE_HEALTH_ENDPOINT";
/// Storage metrics endpoint URL.
pub const ENV_STORAGE_METRICS_ENDPOINT: &str = "BEARDOG_STORAGE_METRICS_ENDPOINT";
/// AI service primary endpoint URL.
pub const ENV_AI_ENDPOINT: &str = "BEARDOG_AI_ENDPOINT";
/// AI service backup endpoints (comma-separated).
pub const ENV_AI_BACKUPS: &str = "BEARDOG_AI_BACKUPS";
/// AI health check endpoint URL.
pub const ENV_AI_HEALTH_ENDPOINT: &str = "BEARDOG_AI_HEALTH_ENDPOINT";
/// AI metrics endpoint URL.
pub const ENV_AI_METRICS_ENDPOINT: &str = "BEARDOG_AI_METRICS_ENDPOINT";
/// Service mesh primary endpoint URL.
pub const ENV_MESH_ENDPOINT: &str = "BEARDOG_MESH_ENDPOINT";
/// Service mesh backup endpoints (comma-separated).
pub const ENV_MESH_BACKUPS: &str = "BEARDOG_MESH_BACKUPS";
/// Mesh health check endpoint URL.
pub const ENV_MESH_HEALTH_ENDPOINT: &str = "BEARDOG_MESH_HEALTH_ENDPOINT";
/// Mesh metrics endpoint URL.
pub const ENV_MESH_METRICS_ENDPOINT: &str = "BEARDOG_MESH_METRICS_ENDPOINT";
/// Discovery service backup endpoints (comma-separated).
pub const ENV_DISCOVERY_BACKUPS: &str = "BEARDOG_DISCOVERY_BACKUPS";
/// Discovery health check endpoint URL.
pub const ENV_DISCOVERY_HEALTH_ENDPOINT: &str = "BEARDOG_DISCOVERY_HEALTH_ENDPOINT";
/// Discovery metrics endpoint URL.
pub const ENV_DISCOVERY_METRICS_ENDPOINT: &str = "BEARDOG_DISCOVERY_METRICS_ENDPOINT";
/// Capabilities API URL override.
pub const ENV_CAPABILITIES_URL: &str = "BEARDOG_CAPABILITIES_URL";
/// Health check URL override.
pub const ENV_HEALTH_URL: &str = "BEARDOG_HEALTH_URL";
/// Metrics URL override.
pub const ENV_METRICS_URL: &str = "BEARDOG_METRICS_URL";
/// Admin interface URL override.
pub const ENV_ADMIN_URL: &str = "BEARDOG_ADMIN_URL";
/// WebSocket URL override.
pub const ENV_WEBSOCKET_URL: &str = "BEARDOG_WEBSOCKET_URL";
/// Enable TLS for runtime network config.
pub const ENV_ENABLE_TLS: &str = "BEARDOG_ENABLE_TLS";
/// Enable TLS for network discovery.
pub const ENV_TLS_ENABLED: &str = "BEARDOG_TLS_ENABLED";
/// Verify TLS certificates in network discovery.
pub const ENV_VERIFY_CERTS: &str = "BEARDOG_VERIFY_CERTS";
/// CA certificate path for mTLS.
pub const ENV_CA_CERT: &str = "BEARDOG_CA_CERT";
/// Client certificate path for mTLS.
pub const ENV_CLIENT_CERT: &str = "BEARDOG_CLIENT_CERT";
/// Client private key path for mTLS.
pub const ENV_CLIENT_KEY: &str = "BEARDOG_CLIENT_KEY";

// ── Database ─────────────────────────────────────────────────────────

/// Standard `DATABASE_URL` connection string (unprefixed).
pub const ENV_DATABASE_URL: &str = "DATABASE_URL";
/// BearDog-prefixed database URL override.
pub const ENV_DATABASE_URL_PREFIXED: &str = "BEARDOG_DATABASE_URL";
/// Database pool maximum connections.
pub const ENV_DB_MAX_CONNECTIONS: &str = "DB_MAX_CONNECTIONS";
/// Database pool minimum connections.
pub const ENV_DB_MIN_CONNECTIONS: &str = "DB_MIN_CONNECTIONS";
/// Database connection establishment timeout (seconds).
pub const ENV_DB_CONNECTION_TIMEOUT: &str = "DB_CONNECTION_TIMEOUT";
/// Database idle connection timeout (seconds).
pub const ENV_DB_IDLE_TIMEOUT: &str = "DB_IDLE_TIMEOUT";
/// Enable SSL for database connections.
pub const ENV_DB_SSL_ENABLED: &str = "DB_SSL_ENABLED";
/// Database SSL mode (`require`, `prefer`, `disable`).
pub const ENV_DB_SSL_MODE: &str = "DB_SSL_MODE";
/// Database SSL certificate path.
pub const ENV_DB_SSL_CERT: &str = "DB_SSL_CERT";
/// Database SSL private key path.
pub const ENV_DB_SSL_KEY: &str = "DB_SSL_KEY";

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
/// HTTP request timeout (milliseconds).
pub const ENV_REQUEST_TIMEOUT_MS: &str = "BEARDOG_REQUEST_TIMEOUT_MS";
/// Connection establishment timeout (milliseconds).
pub const ENV_CONNECTION_TIMEOUT_MS: &str = "BEARDOG_CONNECTION_TIMEOUT_MS";
/// Keep-alive timeout (seconds, `_S` suffix variant).
pub const ENV_KEEPALIVE_TIMEOUT_S: &str = "BEARDOG_KEEPALIVE_TIMEOUT_S";
/// Global operation timeout (seconds, runtime config).
pub const ENV_TIMEOUT_SECONDS: &str = "BEARDOG_TIMEOUT_SECONDS";

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
/// Enable RBAC authorization.
pub const ENV_AUTHZ_RBAC_ENABLED: &str = "BEARDOG_AUTHZ_RBAC_ENABLED";
/// Enable ABAC authorization.
pub const ENV_AUTHZ_ABAC_ENABLED: &str = "BEARDOG_AUTHZ_ABAC_ENABLED";
/// Default authorization role for new principals.
pub const ENV_AUTHZ_DEFAULT_ROLE: &str = "BEARDOG_AUTHZ_DEFAULT_ROLE";
/// Permission cache TTL (seconds).
pub const ENV_AUTHZ_CACHE_TIMEOUT_SECS: &str = "BEARDOG_AUTHZ_CACHE_TIMEOUT_SECS";
/// Enable capability-based authorization.
pub const ENV_AUTHZ_CAPABILITY_ENABLED: &str = "BEARDOG_AUTHZ_CAPABILITY_ENABLED";
/// Enable legacy authentication integration.
pub const ENV_LEGACY_INTEGRATION_ENABLED: &str = "BEARDOG_LEGACY_INTEGRATION_ENABLED";
/// Legacy integration request timeout (seconds).
pub const ENV_LEGACY_TIMEOUT_SECS: &str = "BEARDOG_LEGACY_TIMEOUT_SECS";
/// Deadline for migrating off legacy auth (ISO date string).
pub const ENV_LEGACY_MIGRATION_DEADLINE: &str = "BEARDOG_LEGACY_MIGRATION_DEADLINE";
/// Enable security health monitoring.
pub const ENV_HEALTH_MONITORING_ENABLED: &str = "BEARDOG_HEALTH_MONITORING_ENABLED";
/// Enable automatic health remediation.
pub const ENV_HEALTH_AUTO_REMEDIATION_ENABLED: &str = "BEARDOG_HEALTH_AUTO_REMEDIATION_ENABLED";
/// Enable security event monitoring.
pub const ENV_SECURITY_MONITORING_ENABLED: &str = "BEARDOG_SECURITY_MONITORING_ENABLED";
/// Security monitoring poll interval (seconds).
pub const ENV_SECURITY_MONITORING_INTERVAL_SECS: &str = "BEARDOG_SECURITY_MONITORING_INTERVAL_SECS";
/// Security alert rate limit (alerts per hour).
pub const ENV_SECURITY_ALERT_RATE_LIMIT_PER_HOUR: &str =
    "BEARDOG_SECURITY_ALERT_RATE_LIMIT_PER_HOUR";
/// Enable trust score decay over time.
pub const ENV_TRUST_DECAY_ENABLED: &str = "BEARDOG_TRUST_DECAY_ENABLED";
/// Trust decay rate per interval (0.0–1.0).
pub const ENV_TRUST_DECAY_RATE: &str = "BEARDOG_TRUST_DECAY_RATE";
/// Trust decay interval (seconds).
pub const ENV_TRUST_DECAY_INTERVAL_SECS: &str = "BEARDOG_TRUST_DECAY_INTERVAL_SECS";
/// Minimum trust score floor (0.0–1.0).
pub const ENV_TRUST_MINIMUM: &str = "BEARDOG_TRUST_MINIMUM";
/// Trust evaluation algorithm name.
pub const ENV_TRUST_EVAL_ALGORITHM: &str = "BEARDOG_TRUST_EVAL_ALGORITHM";
/// Trust evaluation timeout (seconds).
pub const ENV_TRUST_EVAL_TIMEOUT_SECS: &str = "BEARDOG_TRUST_EVAL_TIMEOUT_SECS";
/// Enable compliance validation checks.
pub const ENV_COMPLIANCE_VALIDATION_ENABLED: &str = "BEARDOG_COMPLIANCE_VALIDATION_ENABLED";
/// Compliance validation frequency (hours).
pub const ENV_COMPLIANCE_VALIDATION_FREQUENCY_HOURS: &str =
    "BEARDOG_COMPLIANCE_VALIDATION_FREQUENCY_HOURS";
/// Compliance strictness level (`low`, `moderate`, `high`, `strict`).
pub const ENV_COMPLIANCE_STRICTNESS: &str = "BEARDOG_COMPLIANCE_STRICTNESS";
/// Enable automatic compliance remediation.
pub const ENV_COMPLIANCE_AUTO_REMEDIATION: &str = "BEARDOG_COMPLIANCE_AUTO_REMEDIATION";
/// Enable compliance reporting.
pub const ENV_COMPLIANCE_REPORTING_ENABLED: &str = "BEARDOG_COMPLIANCE_REPORTING_ENABLED";
/// Compliance report frequency (days).
pub const ENV_COMPLIANCE_REPORT_FREQUENCY_DAYS: &str = "BEARDOG_COMPLIANCE_REPORT_FREQUENCY_DAYS";
/// Enable data sovereignty validation.
pub const ENV_SOVEREIGNTY_VALIDATION_ENABLED: &str = "BEARDOG_SOVEREIGNTY_VALIDATION_ENABLED";
/// Sovereignty validation frequency (hours).
pub const ENV_SOVEREIGNTY_VALIDATION_FREQUENCY_HOURS: &str =
    "BEARDOG_SOVEREIGNTY_VALIDATION_FREQUENCY_HOURS";
/// Data sovereignty enforcement level.
pub const ENV_SOVEREIGNTY_ENFORCEMENT: &str = "BEARDOG_SOVEREIGNTY_ENFORCEMENT";
/// Cryptographic key rotation interval (seconds).
pub const ENV_KEY_ROTATION_INTERVAL_SECS: &str = "BEARDOG_KEY_ROTATION_INTERVAL_SECS";
/// Genetics integration strength (0.0–1.0).
pub const ENV_GENETICS_INTEGRATION_STRENGTH: &str = "BEARDOG_GENETICS_INTEGRATION_STRENGTH";
/// Genetics validation timeout (seconds).
pub const ENV_GENETICS_VALIDATION_TIMEOUT_SECS: &str = "BEARDOG_GENETICS_VALIDATION_TIMEOUT_SECS";
/// Development rate-limit max requests override.
pub const ENV_RATE_LIMITING_MAX_REQUESTS: &str = "BEARDOG_RATE_LIMITING_MAX_REQUESTS";
/// Development audit log retention (days).
pub const ENV_DEV_AUDIT_RETENTION_DAYS: &str = "BEARDOG_DEV_AUDIT_RETENTION_DAYS";
/// Production audit log retention (days).
pub const ENV_PRODUCTION_AUDIT_RETENTION_DAYS: &str = "BEARDOG_PRODUCTION_AUDIT_RETENTION_DAYS";

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
/// TPM device path (e.g. `/dev/tpm0`).
pub const ENV_TPM_DEVICE: &str = "BEARDOG_TPM_DEVICE";
/// Software HSM storage directory.
pub const ENV_HSM_STORAGE: &str = "BEARDOG_HSM_STORAGE";
/// Enable hardware HSM at runtime.
pub const ENV_ENABLE_HARDWARE_HSM: &str = "BEARDOG_ENABLE_HARDWARE_HSM";

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

// ── Identity ─────────────────────────────────────────────────────────

/// Ecosystem family identifier (unprefixed).
pub const ENV_FAMILY_ID: &str = "FAMILY_ID";
/// BearDog-prefixed family identifier.
pub const ENV_FAMILY_ID_PREFIXED: &str = "BEARDOG_FAMILY_ID";
/// Ecosystem family seed (unprefixed).
pub const ENV_FAMILY_SEED: &str = "FAMILY_SEED";
/// BearDog-prefixed family seed.
pub const ENV_FAMILY_SEED_PREFIXED: &str = "BEARDOG_FAMILY_SEED";
/// Ecosystem node identifier (unprefixed).
pub const ENV_NODE_ID: &str = "NODE_ID";
/// BearDog-prefixed node identifier.
pub const ENV_NODE_ID_PREFIXED: &str = "BEARDOG_NODE_ID";
/// Primal name for IPC path resolution (unprefixed).
pub const ENV_PRIMAL_NAME: &str = "PRIMAL_NAME";
/// BearDog-prefixed primal name override.
pub const ENV_PRIMAL_NAME_PREFIXED: &str = "BEARDOG_PRIMAL_NAME";
/// Primal type / role (unprefixed).
pub const ENV_PRIMAL_TYPE: &str = "PRIMAL_TYPE";
/// BearDog-prefixed primal type override.
pub const ENV_PRIMAL_TYPE_PREFIXED: &str = "BEARDOG_PRIMAL_TYPE";
/// Orchestrator identifier override.
pub const ENV_ORCHESTRATOR_ID: &str = "BEARDOG_ORCHESTRATOR_ID";
/// System hostname.
pub const ENV_HOSTNAME: &str = "HOSTNAME";
/// Windows hostname (`COMPUTERNAME`).
pub const ENV_COMPUTERNAME: &str = "COMPUTERNAME";
/// Service type alias for primal type detection.
pub const ENV_SERVICE_TYPE: &str = "SERVICE_TYPE";
/// Service display name (unprefixed).
pub const ENV_SERVICE_NAME: &str = "SERVICE_NAME";
/// Human-readable display name override.
pub const ENV_DISPLAY_NAME: &str = "BEARDOG_DISPLAY_NAME";
/// Deployment environment (`development`, `staging`, `production`).
pub const ENV_ENVIRONMENT: &str = "BEARDOG_ENVIRONMENT";
/// Real user id (Unix).
pub const ENV_UID: &str = "UID";
/// Effective user id (Unix).
pub const ENV_EUID: &str = "EUID";

// ── Socket / IPC ─────────────────────────────────────────────────────

/// Primal-specific Unix socket path (tier 1).
pub const ENV_SOCKET: &str = "BEARDOG_SOCKET";
/// Override root for tier-5 temp socket fallback.
pub const ENV_SOCKET_TMP_DIR: &str = "BEARDOG_SOCKET_TMP_DIR";
/// Comma-separated IPC capability domain stems for symlink creation.
pub const ENV_IPC_CAPABILITY_STEMS: &str = "BEARDOG_IPC_CAPABILITY_STEMS";
/// Windows named pipe path override.
pub const ENV_PIPE: &str = "BEARDOG_PIPE";
/// Neural registration instance override.
pub const ENV_NEURAL_REGISTRATION_INSTANCE: &str = "BEARDOG_NEURAL_REGISTRATION_INSTANCE";
/// Neural API socket path.
pub const ENV_NEURAL_API_SOCKET: &str = "NEURAL_API_SOCKET";
/// Legacy alias for [`ENV_NEURALS_SOCKET`] (typo-tolerant `NEURALS_SOCKET` env var).
pub const ENV_NEURAL_API_SOCKET_LEGACY: &str = "NEURALS_SOCKET";
/// Neurals socket path.
pub const ENV_NEURALS_SOCKET: &str = "NEURALS_SOCKET";
/// Override the Neural API socket filename (default: `neural-api.sock`).
pub const ENV_NEURAL_API_SOCKET_NAME: &str = "BEARDOG_NEURAL_API_SOCKET_NAME";

// ── Ecosystem (biomeOS) ──────────────────────────────────────────────

/// Disable BTSP production security (`1` or `true`).
pub const ENV_BIOMEOS_INSECURE: &str = "BIOMEOS_INSECURE";
/// biomeOS family label.
pub const ENV_BIOMEOS_FAMILY: &str = "BIOMEOS_FAMILY";
/// Generic orchestrator socket path (tier 2).
pub const ENV_BIOMEOS_SOCKET_PATH: &str = "BIOMEOS_SOCKET_PATH";
/// Generic orchestrator socket directory (tier 2).
pub const ENV_BIOMEOS_SOCKET_DIR: &str = "BIOMEOS_SOCKET_DIR";
/// Windows biomeOS named pipe directory.
pub const ENV_BIOMEOS_PIPE_DIR: &str = "BIOMEOS_PIPE_DIR";
