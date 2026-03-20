// SPDX-License-Identifier: AGPL-3.0-only

// Cloud HSM Configuration

use super::HsmConfigValidation;
use crate::canonical::capabilities::CapabilityType;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

/// Unified Cloud HSM Configuration
///
/// Vendor-agnostic cloud HSM configuration using capability-based discovery.
/// This configuration works with ANY cloud provider (AWS, Azure, GCP, etc.)
/// without hardcoding vendor-specific flags.
///
/// # Migration from Deprecated Flags
///
/// Previously deprecated flags (`universal_kms`, `universal_secrets`, `universal_cloud_kms`)
/// have been REMOVED as of v3.1.0. Use `required_capabilities` instead.
///
/// ## Before (Deprecated):
/// ```ignore
/// UnifiedCloudHsmConfig {
///     universal_kms: true,  // ❌ Removed - violated sovereignty
/// }
/// ```
///
/// ## After (Vendor-Agnostic):
/// ```ignore
/// UnifiedCloudHsmConfig {
///     required_capabilities: Some(vec![CapabilityType::KeyManagement]),  // ✅
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedCloudHsmConfig {
    /// Whether cloud HSM integration is enabled
    pub enabled: bool,

    /// Modern capability-based discovery configuration
    ///
    /// Specify required capabilities instead of hardcoded vendor flags.
    /// System will auto-discover providers with these capabilities.
    ///
    /// # Example
    /// ```ignore
    /// required_capabilities: Some(vec![
    ///     CapabilityType::KeyManagement,
    ///     CapabilityType::HardwareSecurityModule,
    /// ])
    /// ```
    pub required_capabilities: Option<Vec<CapabilityType>>,
}

impl Default for UnifiedCloudHsmConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            required_capabilities: Some(vec![
                CapabilityType::KeyManagement,
                CapabilityType::HardwareSecurityModule,
            ]),
        }
    }
}

impl HsmConfigValidation for UnifiedCloudHsmConfig {
    /// Validates vendor-agnostic cloud HSM configuration
    fn validate(&self) -> Result<(), BearDogError> {
        // Enforce capability-based configuration
        if self.enabled && self.required_capabilities.is_none() {
            return Err(BearDogError::validation(
                "Cloud HSM enabled but no required_capabilities specified. \
                 Use capability-based discovery (e.g., CapabilityType::KeyManagement) \
                 for vendor-agnostic configuration.",
            ));
        }

        Ok(())
    }

    /// Checks if compatible with other configuration version
    fn is_compatible_with(&self, _other_version: u32) -> bool {
        true
    }
}

impl UnifiedCloudHsmConfig {
    /// Create vendor-agnostic capability-based configuration
    ///
    /// # Example
    /// ```ignore
    /// let config = UnifiedCloudHsmConfig::with_required_capabilities(vec![
    ///     CapabilityType::KeyManagement,
    ///     CapabilityType::HardwareSecurityModule,
    /// ]);
    /// // Works with AWS, Azure, GCP, or any provider with these capabilities!
    /// ```
    #[must_use]
    pub const fn with_required_capabilities(capabilities: Vec<CapabilityType>) -> Self {
        Self {
            enabled: true,
            required_capabilities: Some(capabilities),
        }
    }

    /// Get required capabilities for cloud HSM discovery
    ///
    /// Returns the configured capabilities or sensible defaults.
    #[must_use]
    pub fn get_required_capabilities(&self) -> Vec<CapabilityType> {
        self.required_capabilities.clone().unwrap_or_else(|| {
            vec![
                CapabilityType::KeyManagement,
                CapabilityType::HardwareSecurityModule,
            ]
        })
    }

    /// Check if cloud HSM is enabled
    #[must_use]
    pub const fn is_enabled(&self) -> bool {
        self.enabled
    }
}
