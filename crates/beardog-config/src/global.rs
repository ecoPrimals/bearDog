//! Global Configuration Singleton
//!
//! Provides thread-safe access to the global BearDog configuration.
//! Uses `once_cell` for lazy initialization that happens exactly once.
//!
//! ## Usage Pattern
//!
//! ```no_run
//! use beardog_config::global::BEARDOG_CONFIG;
//!
//! // Access configuration anywhere in your code
//! let port = BEARDOG_CONFIG.network.api.port;
//! let timeout = BEARDOG_CONFIG.limits.operation_timeout_secs;
//! ```
//!
//! ## Design Rationale
//!
//! - **Thread-safe**: Uses `Lazy<T>` from `once_cell` for safe concurrent access
//! - **Lazy initialization**: Configuration is loaded only when first accessed
//! - **Environment-aware**: Automatically loads from environment variables
//! - **Fallback to defaults**: Always works even without configuration file
//!
//! ## Environment Variable Support
//!
//! Configuration is loaded from environment variables with this precedence:
//! 1. `BEARDOG_CONFIG_PATH` - explicit config file path
//! 2. `BEARDOG_*` environment variables (e.g., `BEARDOG_API_PORT`)
//! 3. Secure defaults from `BearDogConfig::default()`

use crate::BearDogConfig;
use once_cell::sync::Lazy;
use std::sync::Arc;
use tracing::{info, warn};

/// Global BearDog configuration singleton
///
/// This is initialized exactly once on first access, loading configuration from:
/// 1. Config file (if `BEARDOG_CONFIG_PATH` is set)
/// 2. Environment variables (prefixed with `BEARDOG_`)
/// 3. Secure defaults
///
/// # Examples
///
/// ```no_run
/// use beardog_config::global::BEARDOG_CONFIG;
///
/// // Access configuration values
/// let api_port = BEARDOG_CONFIG.network.api.port;
/// println!("API running on port {}", api_port);
/// ```
pub static BEARDOG_CONFIG: Lazy<Arc<BearDogConfig>> = Lazy::new(|| {
    info!("🔧 Initializing global BearDog configuration");

    let config = load_global_config();

    // Validate configuration
    if let Err(e) = config.validate() {
        warn!("⚠️ Configuration validation failed: {}. Using defaults.", e);
        return Arc::new(BearDogConfig::default());
    }

    info!("✅ Global configuration loaded successfully");
    Arc::new(config)
});

/// Load configuration with fallback chain
///
/// Priority order:
/// 1. Config file (if `BEARDOG_CONFIG_PATH` is set)
/// 2. Environment variables
/// 3. Defaults
fn load_global_config() -> BearDogConfig {
    // Try loading from explicit config file path
    if let Ok(config_path) = std::env::var("BEARDOG_CONFIG_PATH") {
        info!("📂 Loading configuration from: {}", config_path);

        match BearDogConfig::from_file(&config_path) {
            Ok(config) => {
                info!("✅ Configuration loaded from file: {}", config_path);
                return config;
            }
            Err(e) => {
                warn!(
                    "⚠️ Failed to load config from {}: {}. Falling back to environment.",
                    config_path, e
                );
            }
        }
    }

    // Load from environment variables + defaults
    info!("🌍 Loading configuration from environment variables");
    BearDogConfig::from_env()
}

/// Get a reference to the global configuration
///
/// This is a convenience function that returns a reference to the global config.
/// Most code should use `BEARDOG_CONFIG` directly instead.
#[must_use]
pub fn config() -> &'static Arc<BearDogConfig> {
    &BEARDOG_CONFIG
}

/// Get the current API port
///
/// Convenience function for the most commonly accessed config value.
///
/// # Examples
///
/// ```no_run
/// use beardog_config::global::api_port;
///
/// let port = api_port();
/// println!("API port: {}", port);
/// ```
#[must_use]
#[inline]
pub fn api_port() -> u16 {
    BEARDOG_CONFIG.network.api.port
}

/// Get the current discovery port
///
/// Convenience function for service discovery configuration.
#[must_use]
#[inline]
pub fn discovery_port() -> u16 {
    BEARDOG_CONFIG.network.discovery.port
}

/// Get the current admin port
///
/// Convenience function for admin interface configuration.
#[must_use]
#[inline]
pub fn admin_port() -> u16 {
    BEARDOG_CONFIG.network.admin.port
}

// ═══════════════════════════════════════════════════════════════════════════
// Centralized Port Access (Preferred)
// ═══════════════════════════════════════════════════════════════════════════

/// Get metrics port from centralized configuration
///
/// **Preferred over individual port configurations.**
///
/// # Examples
///
/// ```no_run
/// use beardog_config::global::metrics_port;
///
/// let port = metrics_port();
/// println!("Metrics port: {}", port);
/// ```
#[must_use]
#[inline]
pub fn metrics_port() -> u16 {
    BEARDOG_CONFIG.network.ports.metrics_port
}

/// Get health check port from centralized configuration
///
/// **Preferred over individual port configurations.**
#[must_use]
#[inline]
pub fn health_port() -> u16 {
    BEARDOG_CONFIG.network.ports.health_port
}

/// Get HTTPS port from centralized configuration
///
/// **Preferred over individual port configurations.**
#[must_use]
#[inline]
pub fn https_port() -> u16 {
    BEARDOG_CONFIG.network.ports.https_port
}

// ═══════════════════════════════════════════════════════════════════════════
// Centralized Address Access (Preferred)
// ═══════════════════════════════════════════════════════════════════════════

/// Get API host from centralized configuration
///
/// **Preferred over hardcoded addresses.**
///
/// # Examples
///
/// ```no_run
/// use beardog_config::global::api_host;
///
/// let host = api_host();
/// println!("API host: {}", host);
/// ```
#[must_use]
#[inline]
pub fn api_host() -> String {
    BEARDOG_CONFIG.network.addresses.api_host.clone()
}

/// Get bind address from centralized configuration
///
/// **Preferred over hardcoded addresses.**
///
/// # Examples
///
/// ```no_run
/// use beardog_config::global::bind_address;
///
/// let addr = bind_address();
/// println!("Bind address: {}", addr);
/// ```
#[must_use]
#[inline]
pub fn bind_address() -> String {
    BEARDOG_CONFIG.network.addresses.bind_address.clone()
}

/// Get external host from centralized configuration
///
/// **Preferred over hardcoded hostnames.**
#[must_use]
#[inline]
pub fn external_host() -> String {
    BEARDOG_CONFIG.network.addresses.external_host.clone()
}

// ═══════════════════════════════════════════════════════════════════════════
// Centralized Timeout Access (Phase 3 - November 21, 2025)
// ═══════════════════════════════════════════════════════════════════════════

/// Get connection timeout in seconds
///
/// **Preferred over hardcoded timeouts.**
#[must_use]
#[inline]
pub fn connection_timeout_secs() -> u64 {
    BEARDOG_CONFIG.timeouts.connection_timeout_secs
}

/// Get HTTP request timeout in seconds
///
/// **Preferred over hardcoded timeouts.**
#[must_use]
#[inline]
pub fn http_request_timeout_secs() -> u64 {
    BEARDOG_CONFIG.timeouts.request_timeout_secs
}

/// Get DNS resolution timeout in seconds
///
/// **Preferred over hardcoded timeouts.**
#[must_use]
#[inline]
pub fn dns_resolution_timeout_secs() -> u64 {
    BEARDOG_CONFIG.timeouts.dns_resolution_timeout_secs
}

/// Get health check timeout in seconds
///
/// **Preferred over hardcoded timeouts.**
#[must_use]
#[inline]
pub fn health_check_timeout_secs() -> u64 {
    BEARDOG_CONFIG.timeouts.health_check_secs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_config_initialization() {
        // Access config triggers initialization
        let config = config();
        assert!(config.network.api.port > 0);
    }

    #[test]
    fn test_convenience_functions() {
        let port = api_port();
        assert!(port > 0);
        assert_eq!(port, BEARDOG_CONFIG.network.api.port);
    }

    #[test]
    fn test_config_is_singleton() {
        let config1 = config();
        let config2 = config();

        // Should be the exact same Arc
        assert!(Arc::ptr_eq(config1, config2));
    }

    #[test]
    fn test_default_ports() {
        // Even without environment variables, should have valid ports
        assert!(api_port() > 0);
        assert!(discovery_port() > 0);
        assert!(admin_port() > 0);
    }
}
