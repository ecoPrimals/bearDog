// SPDX-License-Identifier: AGPL-3.0-only

//! Host/port helpers and industry default ports for services.

/// IPv4 localhost address
pub const LOCALHOST_IPV4: &str = "127.0.0.1";
/// IPv6 localhost address  
pub const LOCALHOST_IPV6: &str = "::1";
/// Standard localhost name
pub const LOCALHOST_NAME: &str = "localhost";

// ✅ REMOVED DEPRECATED CONSTANTS - Use config system instead:
// - Use beardog_config::global::BEARDOG_CONFIG.network.api.port (not DEFAULT_HTTP_PORT)
// - Use ports::HTTPS_PORT for standard HTTPS port (not DEFAULT_HTTPS_PORT)
// - Use beardog_config::global::BEARDOG_CONFIG.network.api.bind_address (not DEFAULT_API_BIND)

/// Default `PostgreSQL` port (industry standard)
/// Note: This is a well-known industry port, not configuration
pub const DEFAULT_POSTGRES_PORT: u16 = 5432;

/// Default Grafana port (industry standard)
/// Note: This is a well-known industry port, not configuration
pub const DEFAULT_GRAFANA_PORT: u16 = 3000;

/// Get the default service host from environment or fallback
///
/// ✅ MIGRATED: Now uses centralized BEARDOG_CONFIG
pub fn default_service_host() -> String {
    use beardog_config::global::BEARDOG_CONFIG;
    BEARDOG_CONFIG.network.api.bind_address.to_string()
}

/// Get the default service port from environment or fallback
///
/// ✅ MIGRATED: Now uses centralized BEARDOG_CONFIG instead of reading env vars directly
pub fn default_service_port() -> u16 {
    use beardog_config::global::BEARDOG_CONFIG;
    BEARDOG_CONFIG.network.api.port
}

/// Get the default database URL from environment or fallback
pub fn default_database_url() -> String {
    std::env::var("DATABASE_URL")
        .or_else(|_| std::env::var("BEARDOG_DATABASE_URL"))
        .unwrap_or_else(|_| {
            format!(
                "postgresql://{}:{}/beardog",
                default_service_host(),
                DEFAULT_POSTGRES_PORT
            )
        })
}

/// Get the default discovery endpoint from environment or fallback
pub fn default_discovery_endpoint() -> String {
    std::env::var("BEARDOG_DISCOVERY_ENDPOINT")
        .or_else(|_| std::env::var("DISCOVERY_URL"))
        .unwrap_or_else(|_| {
            format!(
                "http://{}:{}/discovery",
                default_service_host(),
                default_service_port()
            )
        })
}

/// Get the default compute endpoint from environment or fallback
pub fn default_compute_endpoint() -> String {
    std::env::var("BEARDOG_COMPUTE_ENDPOINT").unwrap_or_else(|_| {
        format!(
            "http://{}:{}/compute",
            default_service_host(),
            default_service_port()
        )
    })
}

/// Get the default storage endpoint from environment or fallback  
pub fn default_storage_endpoint() -> String {
    std::env::var("BEARDOG_STORAGE_ENDPOINT").unwrap_or_else(|_| {
        format!(
            "http://{}:{}/storage",
            default_service_host(),
            default_service_port()
        )
    })
}
