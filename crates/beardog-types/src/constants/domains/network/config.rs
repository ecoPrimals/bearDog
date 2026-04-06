// SPDX-License-Identifier: AGPL-3.0-or-later

//! Host/port helpers and industry default ports for services.

pub use super::addresses::{
    DEFAULT_LOCALHOST_IPV4_STR as LOCALHOST_IPV4, DEFAULT_LOCALHOST_IPV6_STR as LOCALHOST_IPV6,
};

/// **Fallback** localhost hostname (override with `BEARDOG_EXTERNAL_HOST` / config).
pub const DEFAULT_LOCALHOST_NAME_STR: &str = "localhost";

/// Back-compat alias for [`DEFAULT_LOCALHOST_NAME_STR`].
pub const LOCALHOST_NAME: &str = DEFAULT_LOCALHOST_NAME_STR;

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
/// ✅ MIGRATED: Now uses centralized `BEARDOG_CONFIG`
pub fn default_service_host() -> String {
    use beardog_config::global::BEARDOG_CONFIG;
    BEARDOG_CONFIG.network.api.bind_address.to_string()
}

/// Get the default service port from environment or fallback
///
/// ✅ MIGRATED: Now uses centralized `BEARDOG_CONFIG` instead of reading env vars directly
pub fn default_service_port() -> u16 {
    use beardog_config::global::BEARDOG_CONFIG;
    BEARDOG_CONFIG.network.api.port
}

/// Default database URL from centralized config (no environment reads).
pub fn default_database_url() -> String {
    format!(
        "postgresql://{}:{}/beardog",
        default_service_host(),
        DEFAULT_POSTGRES_PORT
    )
}

/// `DATABASE_URL` / `BEARDOG_DATABASE_URL`, falling back to [`default_database_url`].
pub fn default_database_url_from_env() -> String {
    std::env::var("DATABASE_URL")
        .or_else(|_| std::env::var("BEARDOG_DATABASE_URL"))
        .unwrap_or_else(|_| default_database_url())
}

/// Default discovery URL from centralized config (no environment reads).
pub fn default_discovery_endpoint() -> String {
    format!(
        "http://{}:{}/discovery",
        default_service_host(),
        default_service_port()
    )
}

/// Discovery endpoint from env (`BEARDOG_DISCOVERY_ENDPOINT` / `DISCOVERY_URL`), falling back to [`default_discovery_endpoint`].
pub fn default_discovery_endpoint_from_env() -> String {
    std::env::var("BEARDOG_DISCOVERY_ENDPOINT")
        .or_else(|_| std::env::var("DISCOVERY_URL"))
        .unwrap_or_else(|_| default_discovery_endpoint())
}

/// Default compute URL (no environment reads).
pub fn default_compute_endpoint() -> String {
    format!(
        "http://{}:{}/compute",
        default_service_host(),
        default_service_port()
    )
}

/// Compute endpoint from `BEARDOG_COMPUTE_ENDPOINT`, falling back to [`default_compute_endpoint`].
pub fn default_compute_endpoint_from_env() -> String {
    std::env::var("BEARDOG_COMPUTE_ENDPOINT").unwrap_or_else(|_| default_compute_endpoint())
}

/// Default storage URL (no environment reads).
pub fn default_storage_endpoint() -> String {
    format!(
        "http://{}:{}/storage",
        default_service_host(),
        default_service_port()
    )
}

/// Storage endpoint from `BEARDOG_STORAGE_ENDPOINT`, falling back to [`default_storage_endpoint`].
pub fn default_storage_endpoint_from_env() -> String {
    std::env::var("BEARDOG_STORAGE_ENDPOINT").unwrap_or_else(|_| default_storage_endpoint())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn localhost_aliases_and_default_ports() {
        assert_eq!(LOCALHOST_NAME, DEFAULT_LOCALHOST_NAME_STR);
        assert_eq!(DEFAULT_POSTGRES_PORT, 5432);
        assert_eq!(DEFAULT_GRAFANA_PORT, 3000);
        assert!(!LOCALHOST_IPV4.is_empty());
        assert!(!LOCALHOST_IPV6.is_empty());
    }

    #[test]
    fn default_endpoints_include_host_and_port() {
        let h = default_service_host();
        let p = default_service_port();
        assert!(!h.is_empty());
        assert!(p > 0);

        let db = default_database_url();
        assert!(db.contains("postgresql://"));
        assert!(db.contains(&format!(":{DEFAULT_POSTGRES_PORT}/")));

        let disc = default_discovery_endpoint();
        assert!(disc.contains("/discovery"));
        let comp = default_compute_endpoint();
        assert!(comp.contains("/compute"));
        let stor = default_storage_endpoint();
        assert!(stor.contains("/storage"));
    }

    #[test]
    fn default_urls_from_env_fallback() {
        let _ = default_database_url_from_env();
        let _ = default_discovery_endpoint_from_env();
        let _ = default_compute_endpoint_from_env();
        let _ = default_storage_endpoint_from_env();
    }
}
