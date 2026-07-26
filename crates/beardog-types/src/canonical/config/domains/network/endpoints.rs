// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Endpoints Configuration Module
//!
//! This module contains endpoint configuration and management.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Endpoints configuration - consolidates endpoint management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointsConfiguration {
    /// API base URL
    pub api_base_url: String,
    /// Discovery endpoint URL
    pub discovery_url: String,
    /// Health check endpoint URL
    pub health_url: String,
    /// Metrics endpoint URL
    pub metrics_url: String,
    /// Custom endpoints
    pub custom_endpoints: HashMap<String, String>,
    /// Enable endpoint versioning
    pub enable_versioning: bool,
    /// Default API version
    pub default_api_version: String,
}

impl Default for EndpointsConfiguration {
    fn default() -> Self {
        Self {
            api_base_url: format!(
                "http://{}:{}",
                crate::constants::domains::network::addresses::LOCALHOST_IPV4,
                crate::constants::domains::network::defaults::default_api_port()
            ),
            discovery_url: crate::constants::domains::network::config::default_discovery_endpoint(),
            health_url: format!(
                "{}:{}/health",
                crate::constants::domains::network::addresses::LOCALHOST_IPV4,
                crate::constants::domains::network::defaults::default_health_port()
            ),
            metrics_url: format!(
                "{}:{}/metrics",
                crate::constants::domains::network::addresses::LOCALHOST_IPV4,
                crate::constants::domains::network::defaults::default_metrics_port()
            ),
            custom_endpoints: HashMap::new(),
            enable_versioning: true,
            default_api_version: crate::constants::domains::system::versions::API_VERSION
                .to_string(),
        }
    }
}

impl EndpointsConfiguration {
    /// Validate endpoints configuration
    ///
    /// # Errors
    ///
    /// Returns an error if any required URL is empty or versioning is enabled without an API version.
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.api_base_url.is_empty() {
            return Err(BearDogError::configuration("API base URL cannot be empty"));
        }

        if self.discovery_url.is_empty() {
            return Err(BearDogError::configuration("Discovery URL cannot be empty"));
        }

        if self.health_url.is_empty() {
            return Err(BearDogError::configuration("Health URL cannot be empty"));
        }

        if self.metrics_url.is_empty() {
            return Err(BearDogError::configuration("Metrics URL cannot be empty"));
        }

        if self.enable_versioning && self.default_api_version.is_empty() {
            return Err(BearDogError::configuration(
                "Default API version cannot be empty when versioning is enabled",
            ));
        }

        Ok(())
    }

    /// Add a custom endpoint
    pub fn add_custom_endpoint(&mut self, name: String, url: String) {
        self.custom_endpoints.insert(name, url);
    }

    /// Get a custom endpoint URL
    #[must_use]
    pub fn get_custom_endpoint(&self, name: &str) -> Option<&String> {
        self.custom_endpoints.get(name)
    }

    /// Get versioned API URL
    #[must_use]
    pub fn get_versioned_api_url(&self, path: &str) -> String {
        if self.enable_versioning {
            format!(
                "{}/v{}/{}",
                self.api_base_url,
                self.default_api_version,
                path.trim_start_matches('/')
            )
        } else {
            format!("{}/{}", self.api_base_url, path.trim_start_matches('/'))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_endpoints_config() {
        let config = EndpointsConfiguration::default();
        assert!(config.validate().is_ok());
        assert!(!config.api_base_url.is_empty());
        assert!(!config.discovery_url.is_empty());
        assert!(config.enable_versioning);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_custom_endpoints() {
        let mut config = EndpointsConfiguration::default();
        config.add_custom_endpoint("custom".to_string(), "http://example.com".to_string());

        assert_eq!(
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            config.get_custom_endpoint("custom"),
            Some(&"http://example.com".to_string())
        );
        assert_eq!(config.get_custom_endpoint("nonexistent"), None);
    }

    #[test]
    fn test_versioned_api_url() {
        let config = EndpointsConfiguration::default();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let url = config.get_versioned_api_url("/users");
        assert!(url.contains(&config.default_api_version));
        assert!(url.contains("/users"));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: important
    #[test]
    fn test_invalid_endpoints_config() {
        let mut config = EndpointsConfiguration::default();
        config.api_base_url = String::new();
        assert!(config.validate().is_err());
    }
}
