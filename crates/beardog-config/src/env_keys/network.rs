// SPDX-License-Identifier: AGPL-3.0-or-later

//! Network addresses, ports, database, bind, and application runtime environment variable keys.

// ── Network addresses ────────────────────────────────────────────────

/// Structured transport endpoint (JSON). Tier 0 override for all transport
/// resolution — the launcher/orchestrator injects this so the primal never
/// self-selects a transport.
///
/// Format: `{"transport":"uds","path":"..."} | {"transport":"tcp","host":"...","port":N}`
pub const ENV_TRANSPORT_ENDPOINT: &str = "TRANSPORT_ENDPOINT";

/// guideStone P1/P4: Ecosystem-standard bind mode for primal startup contract.
///
/// Values: `auto` (default), `filesystem`, `abstract`, `tcp`.
/// Unprefixed — shared across all primals. Replaces per-primal transport
/// flags (`--abstract`, `--no-unix`, `--no-uds`).
pub const ENV_PRIMAL_BIND_MODE: &str = "PRIMAL_BIND_MODE";

/// API host address.
pub const ENV_API_HOST: &str = "BEARDOG_API_HOST";
/// Listen / bind address (alias: `BEARDOG_BIND_ADDRESS`).
pub const ENV_LISTEN_ADDR: &str = "BEARDOG_LISTEN_ADDR";
/// Bind address (canonical form).
pub const ENV_BIND_ADDRESS: &str = "BEARDOG_BIND_ADDRESS";
/// Bind address (short form).
pub const ENV_BIND_ADDR: &str = "BEARDOG_BIND_ADDR";
/// Network probe target address for non-loopback interface detection.
pub const ENV_NETWORK_PROBE_TARGET: &str = "BEARDOG_NETWORK_PROBE_TARGET";
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
/// Default discovery host when no env override is set (empty = not configured).
pub const DEFAULT_DISCOVERY_HOST: &str = "";
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
/// HTTP port.
pub const ENV_HTTP_PORT: &str = "BEARDOG_HTTP_PORT";
/// HTTPS port.
pub const ENV_HTTPS_PORT: &str = "BEARDOG_HTTPS_PORT";
/// Upstream HTTP backend for the ACME TLS gateway (e.g., songBird `http.proxy`).
/// Format: `host:port` or unix socket path prefixed with `unix:`.
pub const ENV_GATEWAY_UPSTREAM: &str = "BEARDOG_GATEWAY_UPSTREAM";
/// Gatehouse mode: activates bearDog as the sovereign external gateway
/// (`:443` TLS + `:80` ACME/redirect). Equivalent to `BEARDOG_TLS_MODE=acme`.
pub const ENV_GATEHOUSE_MODE: &str = "BEARDOG_GATEHOUSE_MODE";
/// RPC port.
pub const ENV_RPC_PORT: &str = "BEARDOG_RPC_PORT";
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
/// Production max connections override.
pub const ENV_PRODUCTION_MAX_CONNECTIONS: &str = "BEARDOG_PRODUCTION_MAX_CONNECTIONS";
/// Production per-IP requests-per-minute rate limit.
pub const ENV_PRODUCTION_PER_IP_RPM: &str = "BEARDOG_PRODUCTION_PER_IP_RPM";
/// TCP listen backlog size.
pub const ENV_SERVER_BACKLOG_SIZE: &str = "BEARDOG_SERVER_BACKLOG_SIZE";
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
/// Database connection establishment timeout (seconds, canonical config).
pub const ENV_DB_CONNECTION_TIMEOUT_SECS: &str = "BEARDOG_DB_CONNECTION_TIMEOUT_SECS";
/// Database query execution timeout (seconds).
pub const ENV_DB_QUERY_TIMEOUT_SECS: &str = "BEARDOG_DB_QUERY_TIMEOUT_SECS";

// ── Network bind (combined host:port) ────────────────────────────────────

/// API bind address (combined host:port).
pub const ENV_API_BIND: &str = "BEARDOG_API_BIND";
/// Metrics bind address (combined host:port).
pub const ENV_METRICS_BIND: &str = "BEARDOG_METRICS_BIND";
/// Health check bind address (combined host:port).
pub const ENV_HEALTH_BIND: &str = "BEARDOG_HEALTH_BIND";
/// Comma-separated DNS server list.
pub const ENV_DNS_SERVERS: &str = "BEARDOG_DNS_SERVERS";
/// Network bind address override.
pub const ENV_NETWORK_BIND_ADDRESS: &str = "BEARDOG_NETWORK_BIND_ADDRESS";
/// Network port override.
pub const ENV_NETWORK_PORT: &str = "BEARDOG_NETWORK_PORT";
/// Default service port for discovery fallbacks.
pub const ENV_DEFAULT_SERVICE_PORT: &str = "BEARDOG_DEFAULT_SERVICE_PORT";
/// Redis connection URL (unprefixed).
pub const ENV_REDIS_URL: &str = "REDIS_URL";
/// BearDog-prefixed Redis connection URL override.
pub const ENV_REDIS_URL_PREFIXED: &str = "BEARDOG_REDIS_URL";
/// Redis port (unprefixed).
pub const ENV_REDIS_PORT_UNPREFIXED: &str = "REDIS_PORT";

// ── Application / runtime ──────────────────────────────────────────────

/// Primary listen port.
pub const ENV_PORT: &str = "BEARDOG_PORT";
/// Worker thread pool size.
pub const ENV_WORKER_THREADS: &str = "BEARDOG_WORKER_THREADS";
/// Database host override.
pub const ENV_DB_HOST: &str = "BEARDOG_DB_HOST";
/// Database port override.
pub const ENV_DB_PORT: &str = "BEARDOG_DB_PORT";
/// Application name override.
pub const ENV_APP_NAME: &str = "BEARDOG_APP_NAME";
/// Application version string.
pub const ENV_APP_VERSION: &str = "BEARDOG_APP_VERSION";
/// Application description.
pub const ENV_APP_DESCRIPTION: &str = "BEARDOG_APP_DESCRIPTION";
/// Instance identifier.
pub const ENV_INSTANCE_ID: &str = "BEARDOG_INSTANCE_ID";
/// Service display name override.
pub const ENV_NAME: &str = "BEARDOG_NAME";
/// Blocking thread pool size.
pub const ENV_BLOCKING_THREADS: &str = "BEARDOG_BLOCKING_THREADS";
/// Maximum file descriptors limit.
pub const ENV_MAX_FILE_DESCRIPTORS: &str = "BEARDOG_MAX_FILE_DESCRIPTORS";
/// Maximum concurrent connections.
pub const ENV_SYSTEM_MAX_CONNECTIONS: &str = "BEARDOG_SYSTEM_MAX_CONNECTIONS";
/// System monitoring interval in seconds.
pub const ENV_SYSTEM_MONITORING_INTERVAL_SECS: &str = "BEARDOG_SYSTEM_MONITORING_INTERVAL_SECS";
/// Self-advertised endpoint URL.
pub const ENV_ENDPOINT: &str = "BEARDOG_ENDPOINT";
/// Self-discovery endpoint URL (unprefixed).
pub const ENV_SELF_DISCOVERY_ENDPOINT: &str = "SELF_DISCOVERY_ENDPOINT";
/// Comma-separated advertised capability list.
pub const ENV_ADVERTISED_CAPABILITIES: &str = "BEARDOG_ADVERTISED_CAPABILITIES";
/// Generic host override (unprefixed).
pub const ENV_HOST_UNPREFIXED: &str = "HOST";
/// Fallback family label when family ID is unset.
pub const ENV_FAMILY_UNKNOWN_LABEL: &str = "BEARDOG_FAMILY_UNKNOWN_LABEL";
/// License key for certificate issuance.
pub const ENV_LICENSE_KEY: &str = "BEARDOG_LICENSE_KEY";
/// Genesis mode selector.
pub const ENV_GENESIS_MODE: &str = "BEARDOG_GENESIS_MODE";
/// Local Unix socket directory for CLI clients.
pub const ENV_LOCAL_SOCKET_DIR: &str = "BEARDOG_LOCAL_SOCKET_DIR";
