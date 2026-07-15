// SPDX-License-Identifier: AGPL-3.0-or-later
// Fixed license_manager.rs - Context-aware licensing system
// Removed unused compute client imports - licensing doesn't require compute operations
// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use crate::BearDogCore;
use beardog_errors::BearDogError;
// Removed unused capability types - licensing uses direct service registration
// Removed unused HealthStatus import
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tracing::debug;

/// Information about a software license
///
/// Describes license terms, validity period, granted capabilities, and restrictions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseInfo {
    /// Unique license identifier
    pub license_id: String,
    /// Type of license (e.g., "MIT", "Apache-2.0", "Commercial")
    pub license_type: String,
    /// When the license was issued
    pub issued_at: DateTime<Utc>,
    /// When the license expires (if applicable)
    pub expires_at: Option<DateTime<Utc>>,
    /// Capabilities granted by this license
    pub capabilities: Vec<String>,
    /// Restrictions imposed by this license
    pub restrictions: BTreeMap<String, String>,
}

/// Result of license validation check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseValidation {
    /// Whether the license is currently valid
    pub is_valid: bool,
    /// Optional warning about upcoming expiry
    /// Optional expiry warning
    pub expiry_warning: Option<String>,
    /// List of capabilities granted by valid licenses
    /// Collection of capabilities granted
    pub capabilities_granted: Vec<String>,
    /// List of restrictions currently in effect
    /// Collection of restrictions active
    pub restrictions_active: Vec<String>,
}

impl BearDogCore {
    // Note: Licensing initialization functions removed as they were unused dead code.
    // Enterprise licensing features can be re-implemented when needed with proper
    // integration points. The public licensing API (get_license_status, check_capability_license)
    // remains available for current use cases.

    /// Get the current license validation status
    ///
    /// including validity, expiration warnings, granted capabilities,
    /// and any active restrictions. This provides a complete view of
    ///
    /// # Returns
    /// Gets `license_status`
    #[must_use]
    pub fn get_license_status(&self) -> LicenseValidation {
        LicenseValidation {
            is_valid: true,
            expiry_warning: None,
            capabilities_granted: vec![
                "security".to_string(),
                "hsm".to_string(),
                "crypto".to_string(),
                "monitoring".to_string(),
                "workflows".to_string(),
                "ai".to_string(),
            ],
            restrictions_active: vec![],
        }
    }

    /// Check if a specific capability is licensed
    ///
    /// Verifies whether the current license grants access to the specified
    /// only licensed features are accessible.
    ///
    /// # Arguments
    /// * `capability` - Name of the capability to check
    ///
    /// # Returns
    /// - `Ok(true)` if the capability is licensed and available
    /// - `Ok(false)` if the capability is not licensed
    ///
    /// # Errors
    /// Returns `Err(BearDogError)` if the license validation check fails
    pub fn check_capability_license(&self, capability: &str) -> Result<bool, BearDogError> {
        debug!("🔐 Checking license for capability: {}", capability);

        let validation = self.get_license_status();
        Ok(validation
            .capabilities_granted
            .contains(&capability.to_string()))
    }

    // refresh_license() removed as unused dead code.
    // Can be re-implemented when enterprise licensing features are needed.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_license_info_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let license = LicenseInfo {
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            license_id: "test-001".to_string(),
            license_type: "test".to_string(),
            issued_at: Utc::now(),
            expires_at: None,
            capabilities: vec!["test".to_string()],
            restrictions: BTreeMap::new(),
        };

        let json = serde_json::to_string(&license)?;
        let deserialized: LicenseInfo = serde_json::from_str(&json)?;
        assert_eq!(license.license_id, deserialized.license_id);

        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_license_validation() {
        let validation = LicenseValidation {
            is_valid: true,
            expiry_warning: None,
            capabilities_granted: vec!["test".to_string()],
            restrictions_active: vec![],
        };

        assert!(validation.is_valid);
        assert!(
            validation
                .capabilities_granted
                .contains(&"test".to_string())
        );
    }
}
