// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model registry, versioning, and authentication types.

use beardog_config::env_keys;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Model versioning strategies for tracking model evolution
///
/// Different approaches to assigning version identifiers to trained models
/// for tracking, comparison, and rollback purposes.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum VersioningStrategy {
    /// Semantic versioning following MAJOR.MINOR.PATCH format (e.g., 1.2.3)
    Semantic,
    /// Timestamp-based versioning using creation time (e.g., `20251011_143022`)
    Timestamp,
    /// Content-based hash versioning using model weights (e.g., SHA-256)
    Hash,
    /// Simple incremental integer versioning (e.g., v1, v2, v3)
    Incremental,
}

/// Authentication configuration for secure registry access
///
/// Configures authentication credentials and method for accessing
/// protected model registries and services.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Type of authentication mechanism to use (API key, bearer token, basic auth, `OAuth2`, or none)
    pub auth_type: AuthType,
    /// Authentication credentials as key-value pairs (e.g., "`api_key"`: "...", "username": "...", "password": "...")
    pub credentials: HashMap<String, String>,
}

/// Authentication types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Types of auth
pub enum AuthType {
    /// No authentication
    None,
    /// API key authentication
    ApiKey,
    /// Bearer token authentication
    BearerToken,
    /// Basic authentication
    Basic,
    /// `OAuth2` authentication
    OAuth2,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
/// Types of registry
pub enum RegistryType {
    /// Local file system registry
    Local,
    /// Remote HTTP registry
    Remote,
    /// Database registry
    Database,
    /// Cloud storage registry
    CloudStorage,
}

/// Registry configuration for AI model storage and catalog
///
/// Configures where and how trained models are stored, versioned, and retrieved.
/// Renamed from `RegistryConfig` to `AIRegistryConfig` for clarity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRegistryConfig {
    /// Type of registry storage backend (local filesystem, remote HTTP, database, or cloud storage)
    pub registry_type: RegistryType,
    /// Network address or path where the registry is accessible
    pub endpoint: String,
    /// Optional authentication credentials required to access the registry
    pub auth: Option<AuthConfig>,
}

/// Backward compatibility alias
#[deprecated(since = "3.2.0", note = "Use AIRegistryConfig instead")]
pub type RegistryConfig = AIRegistryConfig;

impl Default for AIRegistryConfig {
    fn default() -> Self {
        let network_config = beardog_types::canonical::config::network::NetworkConfig::default();
        Self {
            registry_type: RegistryType::Local,
            endpoint: format!(
                "{}:{}",
                network_config.default_host, network_config.service_ports.ai_port
            ),
            auth: None,
        }
    }
}

impl AIRegistryConfig {
    /// Prefer `BEARDOG_AI_REGISTRY_ENDPOINT` when set; otherwise same as [`Default::default`].
    #[must_use]
    pub fn from_env() -> Self {
        let network_config = beardog_types::canonical::config::network::NetworkConfig::default();
        Self {
            registry_type: RegistryType::Local,
            endpoint: std::env::var(env_keys::ENV_AI_REGISTRY_ENDPOINT).unwrap_or_else(|_| {
                format!(
                    "{}:{}",
                    network_config.default_host, network_config.service_ports.ai_port
                )
            }),
            auth: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_types::canonical::config::network::NetworkConfig;

    #[test]
    fn ai_registry_config_default() {
        let c = AIRegistryConfig::default();
        assert!(matches!(c.registry_type, RegistryType::Local));
        assert!(c.auth.is_none());
        let net = NetworkConfig::default();
        assert_eq!(
            c.endpoint,
            format!("{}:{}", net.default_host, net.service_ports.ai_port)
        );
    }

    #[test]
    fn ai_registry_config_from_env() {
        let c = AIRegistryConfig::from_env();
        assert!(matches!(c.registry_type, RegistryType::Local));
        assert!(c.auth.is_none());
        let net = NetworkConfig::default();
        assert_eq!(
            c.endpoint,
            format!("{}:{}", net.default_host, net.service_ports.ai_port)
        );
    }
}
