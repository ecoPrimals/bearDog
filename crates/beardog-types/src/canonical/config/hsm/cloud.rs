// Cloud HSM Configuration

use super::HsmConfigValidation;
use crate::canonical::capabilities::CapabilityType;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

///
/// allowing `BearDog` to integrate with multiple cloud security services.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedCloudHsmConfig {
    /// Whether cloud HSM integration is enabled
    /// Whether feature is enabled
    pub enabled: bool,

    /// Modern capability-based discovery configuration
    /// Optional required capabilities
    pub required_capabilities: Option<Vec<CapabilityType>>,

    /// Enable `universal_cloud` Key Management Service integration
    #[deprecated(
        since = "3.0.0",
        note = "Use capability_discovery with CapabilityType::KeyManagement. This hardcoded vendor flag violates primal sovereignty - primals should only know themselves and discover others dynamically."
    )]
    /// Universal Kms
    /// Whether `universal_kms` is enabled
    pub universal_kms: bool,

    /// Enable `universal_cloud` Key Vault integration
    #[deprecated(
        since = "3.0.0",
        note = "Use capability_discovery with CapabilityType::KeyManagement. This hardcoded vendor flag violates primal sovereignty - primals should only know themselves and discover others dynamically."
    )]
    /// Universal Secrets
    /// Whether `universal_secrets` is enabled
    pub universal_secrets: bool,

    #[deprecated(
        since = "3.0.0",
        note = "Use capability_discovery with CapabilityType::KeyManagement. This hardcoded vendor flag violates primal sovereignty - primals should only know themselves and discover others dynamically."
    )]
    /// Universal Cloud Kms
    /// Whether `universal_cloud_kms` is enabled
    pub universal_cloud_kms: bool,
}

impl Default for UnifiedCloudHsmConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            required_capabilities: Some(vec![
                CapabilityType::KeyManagement,
                CapabilityType::HardwareSecurityModule,
            ]),
            #[allow(deprecated)]
            universal_kms: false,
            #[allow(deprecated)]
            universal_secrets: false,
            #[allow(deprecated)]
            universal_cloud_kms: false,
        }
    }
}

impl HsmConfigValidation for UnifiedCloudHsmConfig {
    /// Validates input
    fn validate(&self) -> Result<(), BearDogError> {
        // Warn about deprecated usage
        #[allow(deprecated)]
        if self.universal_kms || self.universal_secrets || self.universal_cloud_kms {
            tracing::warn!(
                "🚨 DEPRECATED: Using hardcoded vendor flags (universal_kms, universal_secrets, universal_kms). \
                 Migrate to capability_discovery for true primal sovereignty. \
                 See migration guide: docs/guides/UNIVERSAL_ADAPTER_USAGE_GUIDE.md"
            );
        }

        if self.enabled && self.required_capabilities.is_none() {
            #[allow(deprecated)]
            if !self.universal_kms && !self.universal_secrets && !self.universal_cloud_kms {
                return Err(BearDogError::validation(
                    "Cloud HSM enabled but no discovery configuration or legacy flags provided",
                ));
            }
        }

        Ok(())
    }

    /// Checks if compatible with
    fn is_compatible_with(&self, _other_version: u32) -> bool {
        true
    }
}

impl UnifiedCloudHsmConfig {
    /// Create modern capability-based configuration
    #[must_use]
    /// Creates instance with required capabilities
    pub fn with_required_capabilities(capabilities: Vec<CapabilityType>) -> Self {
        Self {
            enabled: true,
            required_capabilities: Some(capabilities),
            #[allow(deprecated)]
            universal_kms: false,
            #[allow(deprecated)]
            universal_secrets: false,
            #[allow(deprecated)]
            universal_cloud_kms: false,
        }
    }

    /// Check if using deprecated vendor-specific flags
    #[must_use]
    /// Checks if using deprecated flags
    /// Checks if using deprecated flags
    pub fn is_using_deprecated_flags(&self) -> bool {
        #[allow(deprecated)]
        {
            self.universal_kms || self.universal_secrets || self.universal_cloud_kms
        }
    }

    /// Get required capabilities based on configuration
    #[must_use]
    /// Gets `required_capabilities`
    /// Gets `required_capabilities`
    pub fn get_required_capabilities(&self) -> Vec<CapabilityType> {
        if let Some(capabilities) = &self.required_capabilities {
            capabilities.clone()
        } else {
            // Fallback for deprecated configuration
            vec![
                CapabilityType::KeyManagement,
                CapabilityType::HardwareSecurityModule,
            ]
        }
    }
}
