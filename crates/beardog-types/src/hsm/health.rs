// SPDX-License-Identifier: AGPL-3.0-only

//! HSM Provider Health and Information Types
//!
//! Provides types for tracking HSM provider status, health, and capabilities.

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Provider information
///
/// Describes an HSM provider's identity and capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderInfo {
    /// Unique provider identifier
    pub provider_id: String,

    /// Provider type (e.g., "hardware", "software", "cloud")
    pub provider_type: String,

    /// Whether the provider is currently available
    pub is_available: bool,

    /// List of provider capabilities
    pub capabilities: Vec<String>,
}

impl ProviderInfo {
    /// Creates new provider info
    #[must_use]
    pub fn new(provider_id: impl Into<String>, provider_type: impl Into<String>) -> Self {
        Self {
            provider_id: provider_id.into(),
            provider_type: provider_type.into(),
            is_available: true,
            capabilities: Vec::new(),
        }
    }

    /// Sets availability
    #[must_use]
    pub const fn with_availability(mut self, available: bool) -> Self {
        self.is_available = available;
        self
    }

    /// Adds a capability
    #[must_use]
    pub fn with_capability(mut self, capability: impl Into<String>) -> Self {
        self.capabilities.push(capability.into());
        self
    }

    /// Adds multiple capabilities
    #[must_use]
    pub fn with_capabilities(
        mut self,
        capabilities: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.capabilities
            .extend(capabilities.into_iter().map(Into::into));
        self
    }
}

impl Default for ProviderInfo {
    fn default() -> Self {
        Self {
            provider_id: "unknown".to_string(),
            provider_type: "unknown".to_string(),
            is_available: false,
            capabilities: Vec::new(),
        }
    }
}

/// Provider health status
///
/// Tracks the operational health of an HSM provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderHealth {
    /// Whether the provider is healthy
    pub is_healthy: bool,

    /// Timestamp of last health check
    pub last_check: SystemTime,

    /// Number of errors since last reset
    pub error_count: u64,
}

impl ProviderHealth {
    /// Creates a new healthy provider status
    #[must_use]
    pub fn healthy() -> Self {
        Self {
            is_healthy: true,
            last_check: SystemTime::now(),
            error_count: 0,
        }
    }

    /// Creates a new unhealthy provider status
    #[must_use]
    pub fn unhealthy(error_count: u64) -> Self {
        Self {
            is_healthy: false,
            last_check: SystemTime::now(),
            error_count,
        }
    }

    /// Updates the health status
    pub fn update(&mut self, is_healthy: bool) {
        self.is_healthy = is_healthy;
        self.last_check = SystemTime::now();
        if !is_healthy {
            self.error_count += 1;
        }
    }

    /// Resets the error count
    pub fn reset_errors(&mut self) {
        self.error_count = 0;
        self.last_check = SystemTime::now();
    }
}

impl Default for ProviderHealth {
    fn default() -> Self {
        Self::healthy()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_info_creation() {
        let info = ProviderInfo::new("hsm-1", "hardware");
        assert_eq!(info.provider_id, "hsm-1");
        assert_eq!(info.provider_type, "hardware");
        assert!(info.is_available);
        assert!(info.capabilities.is_empty());
    }

    #[test]
    fn test_provider_info_with_capabilities() {
        let info = ProviderInfo::new("hsm-1", "hardware")
            .with_capability("encrypt")
            .with_capability("decrypt")
            .with_capability("sign");

        assert_eq!(info.capabilities.len(), 3);
        assert!(info.capabilities.contains(&"encrypt".to_string()));
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_provider_info_with_capabilities_bulk() {
        let caps = vec!["encrypt", "decrypt", "sign", "verify"];
        let info = ProviderInfo::new("hsm-1", "hardware").with_capabilities(caps);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal

        assert_eq!(info.capabilities.len(), 4);
    }

    #[test]
    fn test_provider_health_healthy() {
        let health = ProviderHealth::healthy();
        assert!(health.is_healthy);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(health.error_count, 0);
    }

    #[test]
    fn test_provider_health_unhealthy() {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let health = ProviderHealth::unhealthy(5);
        assert!(!health.is_healthy);
        assert_eq!(health.error_count, 5);
    }
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal

    #[test]
    fn test_provider_health_update() {
        let mut health = ProviderHealth::healthy();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        health.update(false);
        assert!(!health.is_healthy);
        assert_eq!(health.error_count, 1);

        health.update(false);
        assert_eq!(health.error_count, 2);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_provider_health_reset() {
        let mut health = ProviderHealth::unhealthy(10);
        health.reset_errors();
        assert_eq!(health.error_count, 0);
    }
}
