// SPDX-License-Identifier: AGPL-3.0-or-later

//! Service discovery, registry, bootstrap, Kubernetes, and ecosystem integration environment variable keys.

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

// ── Discovery (unprefixed aliases) ───────────────────────────────────

/// Legacy unprefixed discovery endpoint URL.
pub const ENV_DISCOVERY_ENDPOINT_UNPREFIXED: &str = "DISCOVERY_ENDPOINT";
/// Legacy unprefixed discovery host.
pub const ENV_DISCOVERY_HOST_UNPREFIXED: &str = "DISCOVERY_HOST";
/// Legacy unprefixed discovery port.
pub const ENV_DISCOVERY_PORT_UNPREFIXED: &str = "DISCOVERY_PORT";
/// Ecosystem-wide discovery endpoint URL.
pub const ENV_ECOSYSTEM_DISCOVERY_ENDPOINT: &str = "ECOSYSTEM_DISCOVERY_ENDPOINT";
/// Local-only discovery endpoint URL.
pub const ENV_LOCAL_DISCOVERY_ENDPOINT: &str = "LOCAL_DISCOVERY_ENDPOINT";
/// Enable mDNS-based discovery.
pub const ENV_MDNS_DISCOVERY: &str = "BEARDOG_MDNS_DISCOVERY";
/// mDNS poll interval (seconds).
pub const ENV_MDNS_POLL_INTERVAL_SECS: &str = "BEARDOG_MDNS_POLL_INTERVAL_SECS";
/// HTTP discovery poll interval (seconds).
pub const ENV_HTTP_DISCOVERY_POLL_INTERVAL_SECS: &str = "BEARDOG_HTTP_DISCOVERY_POLL_INTERVAL_SECS";
/// Environment variable check interval (seconds).
pub const ENV_ENV_CHECK_INTERVAL_SECS: &str = "BEARDOG_ENV_CHECK_INTERVAL_SECS";
/// Mesh discovery poll interval (seconds).
pub const ENV_MESH_DISCOVERY_INTERVAL_SECS: &str = "BEARDOG_MESH_DISCOVERY_INTERVAL_SECS";
/// Ecosystem listener HTTP timeout (seconds).
pub const ENV_ECOSYSTEM_LISTENER_INTERVAL_SECS: &str = "BEARDOG_ECOSYSTEM_LISTENER_INTERVAL_SECS";
/// Enable unified discovery subsystem.
pub const ENV_DISCOVERY_ENABLED: &str = "BEARDOG_DISCOVERY_ENABLED";
/// Discovery service identifier override.
pub const ENV_DISCOVERY_SERVICE_ID: &str = "BEARDOG_DISCOVERY_SERVICE_ID";
/// Comma-separated discovery port list.
pub const ENV_DISCOVERY_PORTS: &str = "BEARDOG_DISCOVERY_PORTS";
/// Enable discovery result caching.
pub const ENV_DISCOVERY_CACHE_ENABLED: &str = "BEARDOG_DISCOVERY_CACHE_ENABLED";
/// Discovery cache maximum entries.
pub const ENV_DISCOVERY_CACHE_SIZE: &str = "BEARDOG_DISCOVERY_CACHE_SIZE";
/// Discovery cache TTL (seconds).
pub const ENV_DISCOVERY_CACHE_TTL_SECS: &str = "BEARDOG_DISCOVERY_CACHE_TTL_SECS";
/// Enable quantum discovery features.
pub const ENV_QUANTUM_DISCOVERY_ENABLED: &str = "BEARDOG_QUANTUM_DISCOVERY_ENABLED";
/// Quantum coherence time (milliseconds).
pub const ENV_QUANTUM_COHERENCE_TIME_MS: &str = "BEARDOG_QUANTUM_COHERENCE_TIME_MS";
/// Enable discovery security layer.
pub const ENV_DISCOVERY_SECURITY_ENABLED: &str = "BEARDOG_DISCOVERY_SECURITY_ENABLED";
/// Require authentication for discovery.
pub const ENV_DISCOVERY_AUTH_REQUIRED: &str = "BEARDOG_DISCOVERY_AUTH_REQUIRED";
/// Require encryption for discovery traffic.
pub const ENV_DISCOVERY_ENCRYPTION_REQUIRED: &str = "BEARDOG_DISCOVERY_ENCRYPTION_REQUIRED";
/// Fallback discovery host when primary resolution fails.
pub const ENV_DISCOVERY_HOST_FALLBACK: &str = "BEARDOG_DISCOVERY_HOST_FALLBACK";
/// Announce this node via mDNS.
pub const ENV_MDNS_ANNOUNCE: &str = "BEARDOG_MDNS_ANNOUNCE";
/// Legacy unprefixed discovery service URL.
pub const ENV_DISCOVERY_URL: &str = "DISCOVERY_URL";
/// Comma-separated discovery endpoint list.
pub const ENV_DISCOVERY_ENDPOINTS: &str = "BEARDOG_DISCOVERY_ENDPOINTS";
/// Maximum discovery retry attempts.
pub const ENV_DISCOVERY_MAX_ATTEMPTS: &str = "BEARDOG_DISCOVERY_MAX_ATTEMPTS";
/// Capability registry endpoint URL.
pub const ENV_CAPABILITY_REGISTRY: &str = "BEARDOG_CAPABILITY_REGISTRY";
/// Node discovery service port.
pub const ENV_NODE_DISCOVERY_PORT: &str = "BEARDOG_NODE_DISCOVERY_PORT";
/// Cluster coordination port.
pub const ENV_CLUSTER_PORT: &str = "BEARDOG_CLUSTER_PORT";

// ── Service registry / Consul ────────────────────────────────────────

/// `BearDog` service registry endpoint URL.
pub const ENV_SERVICE_REGISTRY_ENDPOINT: &str = "BEARDOG_SERVICE_REGISTRY_ENDPOINT";
/// Legacy unprefixed service registry endpoint URL.
pub const ENV_SERVICE_REGISTRY_ENDPOINT_UNPREFIXED: &str = "SERVICE_REGISTRY_ENDPOINT";
/// Discovery service HTTP endpoint URL.
pub const ENV_DISCOVERY_SERVICE_ENDPOINT: &str = "DISCOVERY_SERVICE_ENDPOINT";
/// `BearDog` Consul endpoint URL.
pub const ENV_CONSUL_ENDPOINT: &str = "BEARDOG_CONSUL_ENDPOINT";
/// Consul HTTP address (unprefixed, `host:port`).
pub const ENV_CONSUL_HTTP_ADDR: &str = "CONSUL_HTTP_ADDR";
/// Service registry host (unprefixed).
pub const ENV_SERVICE_REGISTRY_HOST: &str = "SERVICE_REGISTRY_HOST";
/// Consul host (unprefixed).
pub const ENV_CONSUL_HOST: &str = "CONSUL_HOST";
/// Service registry port (unprefixed).
pub const ENV_SERVICE_REGISTRY_PORT: &str = "SERVICE_REGISTRY_PORT";
/// Consul HTTP port (unprefixed).
pub const ENV_CONSUL_PORT_UNPREFIXED: &str = "CONSUL_PORT";
/// Service registry backend type.
pub const ENV_REGISTRY_BACKEND: &str = "BEARDOG_REGISTRY_BACKEND";
/// Comma-separated registry endpoint list.
pub const ENV_REGISTRY_ENDPOINTS: &str = "BEARDOG_REGISTRY_ENDPOINTS";
/// Registry host (unprefixed).
pub const ENV_REGISTRY_HOST: &str = "REGISTRY_HOST";
/// Default registry host when no env override is set (empty = not configured).
pub const DEFAULT_REGISTRY_HOST: &str = "";
/// Registry port (unprefixed).
pub const ENV_REGISTRY_PORT: &str = "REGISTRY_PORT";
/// Registry service TTL (seconds).
pub const ENV_REGISTRY_SERVICE_TTL_SECS: &str = "BEARDOG_REGISTRY_SERVICE_TTL_SECS";
/// Registry health check interval (seconds).
pub const ENV_REGISTRY_HEALTH_CHECK_INTERVAL_SECS: &str =
    "BEARDOG_REGISTRY_HEALTH_CHECK_INTERVAL_SECS";
/// Registry cleanup interval (seconds).
pub const ENV_REGISTRY_CLEANUP_INTERVAL_SECS: &str = "BEARDOG_REGISTRY_CLEANUP_INTERVAL_SECS";
/// Service registry URL override.
pub const ENV_SERVICE_REGISTRY_URL: &str = "BEARDOG_SERVICE_REGISTRY_URL";

// ── Bootstrap ────────────────────────────────────────────────────────

/// Bootstrap discovery timeout (milliseconds).
pub const ENV_DISCOVERY_TIMEOUT_MS: &str = "BEARDOG_DISCOVERY_TIMEOUT_MS";
/// Maximum bootstrap discovery attempts.
pub const ENV_MAX_DISCOVERY_ATTEMPTS: &str = "BEARDOG_MAX_DISCOVERY_ATTEMPTS";
/// Minimum required capabilities threshold.
pub const ENV_MIN_CAPABILITIES: &str = "BEARDOG_MIN_CAPABILITIES";
/// mDNS bootstrap timeout (milliseconds).
pub const ENV_MDNS_TIMEOUT_MS: &str = "BEARDOG_MDNS_TIMEOUT_MS";
/// HTTP bootstrap timeout (milliseconds).
pub const ENV_HTTP_TIMEOUT_MS: &str = "BEARDOG_HTTP_TIMEOUT_MS";
/// Environment bootstrap timeout (milliseconds).
pub const ENV_ENV_TIMEOUT_MS: &str = "BEARDOG_ENV_TIMEOUT_MS";
/// Mesh bootstrap timeout (milliseconds).
pub const ENV_MESH_TIMEOUT_MS: &str = "BEARDOG_MESH_TIMEOUT_MS";
/// Container bootstrap timeout (milliseconds).
pub const ENV_CONTAINER_TIMEOUT_MS: &str = "BEARDOG_CONTAINER_TIMEOUT_MS";
/// Bootstrap discovery port override.
pub const ENV_BOOTSTRAP_DISCOVERY_PORT: &str = "BEARDOG_BOOTSTRAP_DISCOVERY_PORT";
/// Bootstrap buffer size (bytes).
pub const ENV_BOOTSTRAP_BUFFER_SIZE: &str = "BEARDOG_BOOTSTRAP_BUFFER_SIZE";
/// Bootstrap cache duration (seconds).
pub const ENV_BOOTSTRAP_CACHE_DURATION_SECS: &str = "BEARDOG_BOOTSTRAP_CACHE_DURATION_SECS";
/// Pattern max age (seconds).
pub const ENV_PATTERN_MAX_AGE_SECS: &str = "BEARDOG_PATTERN_MAX_AGE_SECS";
/// Pattern consolidation interval (seconds).
pub const ENV_PATTERN_CONSOLIDATION_INTERVAL_SECS: &str =
    "BEARDOG_PATTERN_CONSOLIDATION_INTERVAL_SECS";

// ── IPC (unprefixed aliases) ─────────────────────────────────────────────

/// Primal IPC socket path (unprefixed).
pub const ENV_IPC_SOCKET: &str = "IPC_SOCKET";
/// Discovery socket path (unprefixed).
pub const ENV_DISCOVERY_SOCKET: &str = "DISCOVERY_SOCKET";
/// Development discovery socket path override.
pub const ENV_DEV_DISCOVERY_SOCKET: &str = "BEARDOG_DEV_DISCOVERY_SOCKET";
/// Development discovery socket directory override.
pub const ENV_DEV_DISCOVERY_SOCKET_DIR: &str = "BEARDOG_DEV_DISCOVERY_SOCKET_DIR";
/// biomeOS IPC namespace override.
pub const ENV_BIOMEOS_IPC_NAMESPACE: &str = "BIOMEOS_IPC_NAMESPACE";
/// Override the IPC resolve target parameter key name.
pub const ENV_IPC_RESOLVE_TARGET_PARAM_KEY: &str = "BEARDOG_IPC_RESOLVE_TARGET_PARAM_KEY";
/// TLS key log file path (Wireshark debugging).
pub const ENV_SSLKEYLOGFILE: &str = "SSLKEYLOGFILE";

// ── Kubernetes ───────────────────────────────────────────────────────────

/// `BearDog` Kubernetes namespace override.
pub const ENV_K8S_NAMESPACE: &str = "BEARDOG_K8S_NAMESPACE";
/// `BearDog` Kubernetes API server URL override.
pub const ENV_K8S_API_SERVER: &str = "BEARDOG_K8S_API_SERVER";
/// In-cluster Kubernetes API host (unprefixed).
pub const ENV_KUBERNETES_SERVICE_HOST: &str = "KUBERNETES_SERVICE_HOST";
/// In-cluster Kubernetes API port (unprefixed).
pub const ENV_KUBERNETES_SERVICE_PORT: &str = "KUBERNETES_SERVICE_PORT";
/// Kubeconfig file path (unprefixed).
pub const ENV_KUBECONFIG: &str = "KUBECONFIG";
/// Kubernetes pod namespace (unprefixed).
pub const ENV_KUBERNETES_NAMESPACE: &str = "KUBERNETES_NAMESPACE";

// ── Primal discovery (unprefixed) ────────────────────────────────────────

/// Primal discovery method selector.
pub const ENV_PRIMAL_DISCOVERY_METHOD: &str = "PRIMAL_DISCOVERY_METHOD";
/// mDNS service type for discovery.
pub const ENV_MDNS_SERVICE_TYPE: &str = "MDNS_SERVICE_TYPE";
/// DNS-SD discovery domain.
pub const ENV_DNSSD_DOMAIN: &str = "DNSSD_DOMAIN";
/// Discovery cache TTL (seconds, unprefixed).
pub const ENV_DISCOVERY_CACHE_TTL_SECS_UNPREFIXED: &str = "DISCOVERY_CACHE_TTL_SECS";

// ── Zero-knowledge bootstrap ─────────────────────────────────────────────

/// Zero-knowledge discovery timeout (milliseconds).
pub const ENV_ZK_DISCOVERY_TIMEOUT_MS: &str = "BEARDOG_ZK_DISCOVERY_TIMEOUT_MS";
/// Zero-knowledge max discovery attempts.
pub const ENV_ZK_MAX_DISCOVERY_ATTEMPTS: &str = "BEARDOG_ZK_MAX_DISCOVERY_ATTEMPTS";
/// Network listen interface override.
pub const ENV_LISTEN_INTERFACE: &str = "BEARDOG_LISTEN_INTERFACE";
/// Zero-knowledge max cache TTL (milliseconds).
pub const ENV_ZK_MAX_CACHE_TTL_MS: &str = "BEARDOG_ZK_MAX_CACHE_TTL_MS";
/// Zero-knowledge max concurrent tasks.
pub const ENV_ZK_MAX_CONCURRENT_TASKS: &str = "BEARDOG_ZK_MAX_CONCURRENT_TASKS";
/// Zero-knowledge target discovery time (milliseconds).
pub const ENV_ZK_TARGET_DISCOVERY_TIME_MS: &str = "BEARDOG_ZK_TARGET_DISCOVERY_TIME_MS";
/// Enable zero-knowledge profiling.
pub const ENV_ZK_ENABLE_PROFILING: &str = "BEARDOG_ZK_ENABLE_PROFILING";
/// Identity cache TTL (seconds).
pub const ENV_IDENTITY_CACHE_TTL_SECS: &str = "BEARDOG_IDENTITY_CACHE_TTL_SECS";
/// Capability health check interval (seconds).
pub const ENV_CAPABILITY_HEALTH_CHECK_INTERVAL_SECS: &str =
    "BEARDOG_CAPABILITY_HEALTH_CHECK_INTERVAL_SECS";
/// Capability health check timeout (seconds).
pub const ENV_CAPABILITY_HEALTH_CHECK_TIMEOUT_SECS: &str =
    "BEARDOG_CAPABILITY_HEALTH_CHECK_TIMEOUT_SECS";

// ── Universal discovery (network tuning) ───────────────────────────────

/// Discovery bind address override.
pub const ENV_DISCOVERY_BIND_ADDRESS: &str = "BEARDOG_DISCOVERY_BIND_ADDRESS";
/// Discovery multicast port.
pub const ENV_DISCOVERY_MULTICAST_PORT: &str = "BEARDOG_DISCOVERY_MULTICAST_PORT";
/// Discovery port range lower bound.
pub const ENV_DISCOVERY_PORT_START: &str = "BEARDOG_DISCOVERY_PORT_START";
/// Discovery port range upper bound.
pub const ENV_DISCOVERY_PORT_END: &str = "BEARDOG_DISCOVERY_PORT_END";
/// Discovery maximum UDP packet size (bytes).
pub const ENV_DISCOVERY_MAX_PACKET_SIZE: &str = "BEARDOG_DISCOVERY_MAX_PACKET_SIZE";
/// Discovery socket read timeout (milliseconds).
pub const ENV_DISCOVERY_READ_TIMEOUT_MS: &str = "BEARDOG_DISCOVERY_READ_TIMEOUT_MS";
/// Enable IPv6 for discovery traffic.
pub const ENV_DISCOVERY_ENABLE_IPV6: &str = "BEARDOG_DISCOVERY_ENABLE_IPV6";
/// Network interface for discovery binding.
pub const ENV_DISCOVERY_INTERFACE: &str = "BEARDOG_DISCOVERY_INTERFACE";
/// Host portion of bind address (without port).
pub const ENV_BIND_HOST: &str = "BEARDOG_BIND_HOST";
/// Maximum registered discovery services.
pub const ENV_MAX_SERVICES: &str = "BEARDOG_MAX_SERVICES";

// ── Ecosystem integration ────────────────────────────────────────────────

/// Ecosystem integration endpoint URL.
pub const ENV_ECOSYSTEM_ENDPOINT: &str = "BEARDOG_ECOSYSTEM_ENDPOINT";
/// Ecosystem base URL (unprefixed alias).
pub const ENV_ECOSYSTEM_BASE_URL: &str = "ECOSYSTEM_BASE_URL";
/// Storage replication factor.
pub const ENV_REPLICATION_FACTOR: &str = "BEARDOG_REPLICATION_FACTOR";
/// Storage backup interval (seconds).
pub const ENV_BACKUP_INTERVAL_SECS: &str = "BEARDOG_BACKUP_INTERVAL_SECS";
/// Storage cache maximum entries.
pub const ENV_STORAGE_CACHE_MAX_ENTRIES: &str = "BEARDOG_STORAGE_CACHE_MAX_ENTRIES";
