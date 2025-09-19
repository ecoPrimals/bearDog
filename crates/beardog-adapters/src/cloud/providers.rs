// Cloud Provider Abstractions - DEPRECATED
//
// This module provides deprecated cloud provider abstractions.
// All functionality has been migrated to universal capability-based discovery.

// MODERNIZATION NOTE: This file contains vendor-specific references that should be migrated
// to universal adapter patterns. See migration guide: docs/guides/UNIVERSAL_ADAPTER_USAGE_GUIDE.md
// Target: Replace with capability-based discovery for vendor/primal agnosticism
use beardog_errors::BearDogError;
use beardog_types::canonical::capabilities::CapabilityType;

/// Cloud provider enumeration - DEPRECATED
///
/// This enum represents hardcoded cloud providers and violates the principle
/// that "each primal only knows itself and discovers others via universal adapter".
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[deprecated(
    since = "3.0.0",
    note = "Use CapabilityType with dynamic discovery instead. This hardcoded enum violates primal sovereignty. Migrate to universal capability discovery patterns."
)]
pub enum CloudProvider {
    /// Amazon Web Services - DEPRECATED
    #[deprecated(note = "Use CapabilityType::KeyManagement with discovery")]
    /// Represents aws variant
    Aws,
    /// Microsoft universal_cloud - DEPRECATED  
    #[deprecated(note = "Use CapabilityType::KeyManagement with discovery")]
    universal_cloud,
    #[deprecated(note = "Use CapabilityType::KeyManagement with discovery")]
    /// Represents gcp variant
    Gcp,
    /// HashiCorp Vault - DEPRECATED
    #[deprecated(note = "Use CapabilityType::KeyManagement with discovery")]
    /// Represents vault variant
    Vault,
    /// Generic cloud provider - DEPRECATED
    #[deprecated(note = "Use CapabilityType with discovery")]
    /// Represents generic variant
    Generic(String),
}

/// Cloud integration trait - DEPRECATED
///
/// This trait represents hardcoded cloud integration patterns.
/// Use universal adapter patterns instead.
#[deprecated(
    since = "3.0.0",
    note = "Use universal adapter with capability discovery instead of hardcoded cloud integrations"
)]
pub trait CloudIntegration: Send + Sync {
    /// Connect to cloud provider - DEPRECATED
    fn connect(&self) -> Result<(), BearDogError>;
    /// Disconnect from cloud provider - DEPRECATED  
    fn disconnect(&self) -> Result<(), BearDogError>;
    fn health_check(&self) -> Result<bool, BearDogError>;
    /// Get cloud provider type - DEPRECATED
    #[allow(deprecated)]
    fn provider(&self) -> CloudProvider;
}

pub struct CloudProviderMigrationHelper;

impl CloudProviderMigrationHelper {
    /// Convert deprecated cloud provider to capability type
    #[allow(deprecated)]
    pub fn migrate_to_capability(provider: CloudProvider) -> CapabilityType {
        match provider {
            CloudProvider::Aws
            | CloudProvider::universal_cloud
            | CloudProvider::Gcp
            | CloudProvider::Vault => CapabilityType::KeyManagement,
            CloudProvider::Generic(_) => CapabilityType::Custom("unknown_capability".to_string()),
        }
    }

    /// Get migration guidance message
    /// Gets migration_guidance
    /// Gets migration_guidance
    pub fn get_migration_guidance() -> &'static str {
        "🚨 MIGRATION REQUIRED: Replace hardcoded cloud providers with universal capability discovery.\n\
         \n\
         BEFORE (deprecated):\n\
         let provider = CloudProvider::Aws;\n\
         let integration = AwsIntegration::new();\n\
         \n\
         AFTER (modern):\n\
         let adapter = UniversalAdapter::new()?;\n\
         let capability = adapter.discover_capability(CapabilityType::KeyManagement)?;\n\
         \n\
         See: docs/guides/UNIVERSAL_ADAPTER_USAGE_GUIDE.md"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migration_helper() {
        let guidance = CloudProviderMigrationHelper::get_migration_guidance();
        assert!(guidance.contains("MIGRATION REQUIRED"));
    }
}
