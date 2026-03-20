// SPDX-License-Identifier: AGPL-3.0-only

// HSM Providers Configuration - MODERNIZED
//
// This module provides HSM provider configuration with universal capability discovery.
// Hardcoded cloud providers are deprecated in favor of dynamic discovery.

use crate::canonical::capabilities::CapabilityType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Types of hsm interface
pub enum HsmInterfaceType {
    /// PKCS#11 interface
    Pkcs11 {
        /// Path to the PKCS#11 library
        library_path: String,
    },
    /// Network interface
    Network {
        /// Network endpoint URL
        endpoint: String,
    },
    /// Cloud API interface
    CloudApi {
        /// Cloud API endpoint URL
        endpoint: String,
    },
    /// Platform-native cryptographic APIs (e.g. OS keystore) without PKCS#11 or network.
    Native,
}

/// Observed or advertised HSM throughput and latency characteristics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HsmPerformanceProfile {
    /// Operations per second
    /// The ops per second value
    pub ops_per_second: f64,
    /// Average latency in milliseconds
    /// The avg latency ms value
    pub avg_latency_ms: f64,
}

impl Default for HsmPerformanceProfile {
    fn default() -> Self {
        Self {
            ops_per_second: 100.0,
            avg_latency_ms: 10.0,
        }
    }
}

/// HSM capability discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmCapabilityDiscovery {
    /// Required capability type
    /// The capability type value
    pub capability_type: CapabilityType,
    /// Discovery endpoint (optional)
    /// Optional discovery endpoint
    pub discovery_endpoint: Option<String>,
    /// Provider preferences
    /// Collection of preferences
    pub preferences: Vec<String>,
}

/// HSM provider configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HsmProviderConfig {
    /// Provider name
    /// Name of the item
    pub name: String,
    /// Provider type
    pub provider_type: HsmProviderType,
    /// Provider endpoint (optional - can be discovered dynamically)
    /// Optional endpoint
    pub endpoint: Option<String>,
    /// Provider enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Modern capability-based configuration (preferred)
    /// Optional capability discovery
    pub capability_discovery: Option<HsmCapabilityDiscovery>,
}

/// Modern capability-based HSM provider identification
/// Instead of hardcoding vendor names, we identify by security capabilities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HsmSecurityCapability {
    /// Hardware-backed cryptographic operations
    HardwareCrypto,
    /// FIPS 140-2 Level 2 compliance
    Fips140Level2,
    /// FIPS 140-2 Level 3 compliance  
    Fips140Level3,
    /// Common Criteria EAL4+ certification
    CommonCriteriaEal4Plus,
    /// High-speed bulk encryption
    BulkEncryption,
    /// Secure key generation with true random number generation
    SecureKeyGeneration,
    /// Tamper-resistant hardware
    TamperResistant,
    /// Network-attached security module
    NetworkAttached,
    /// USB cryptographic token
    UsbToken,
    /// Smart card with secure element
    SmartCard,
    /// Cloud-based HSM service
    CloudBased,
    /// Software-only crypto provider (no dedicated HSM hardware)
    SoftwareBased,
}

/// Modern HSM provider based on capabilities, not vendor names
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModernHsmProvider {
    /// Stable identifier for this discovered or configured provider instance
    pub provider_id: String,
    /// Security capabilities provided by this HSM
    /// Collection of security capabilities
    pub security_capabilities: Vec<HsmSecurityCapability>,
    /// The interface type value
    pub interface_type: HsmInterfaceType,
    /// Throughput and latency profile for this provider
    pub performance_profile: HsmPerformanceProfile,
    /// Trust level based on successful operations
    /// The trust level value
    pub trust_level: f64,
}

/// Modern capability-based HSM provider types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
/// Types of hsm provider
pub enum HsmProviderType {
    #[default]
    /// Represents software variant
    Software,

    /// Hardware-based HSM (capability-discovered)
    Hardware {
        /// Security capabilities provided
        capabilities: Vec<HsmSecurityCapability>,
    },

    /// Network-attached HSM (capability-discovered)
    Network {
        /// Security capabilities provided
        capabilities: Vec<HsmSecurityCapability>,
    },

    /// Cloud-based HSM service (capability-discovered)
    Cloud {
        /// Security capabilities provided
        capabilities: Vec<HsmSecurityCapability>,
    },

    /// Universal HSM discovered via capability discovery (preferred)
    Universal {
        /// Discovered provider ID
        provider_id: String,
        /// Provider capability type
        capability_type: CapabilityType,
        /// Security capabilities provided
        capabilities: Vec<HsmSecurityCapability>,
    },
}

impl HsmProviderType {
    /// Check if provider type is deprecated
    #[must_use]
    pub const fn is_deprecated(&self) -> bool {
        // All hardcoded cloud providers have been removed
        // Only legacy patterns would be deprecated now
        false
    }

    /// Returns a modern provider type that supersedes this one, if any migration is defined.
    #[must_use]
    pub const fn get_modern_replacement(&self) -> Option<Self> {
        // All providers are now modern (capability-based)
        // Return None as no replacement needed
        None
    }

    /// Human-readable migration advice when this provider pattern is legacy or deprecated.
    #[must_use]
    pub const fn get_migration_guidance(&self) -> Option<&'static str> {
        // All hardcoded cloud providers have been removed
        // Migration is complete - all providers now use capability discovery
        None
    }

    #[must_use]
    /// Converts to capability type
    pub fn to_capability_type(&self) -> CapabilityType {
        match self {
            Self::Universal {
                capability_type, ..
            } => capability_type.clone(),
            _ => CapabilityType::HardwareSecurityModule,
        }
    }

    /// Check if provider supports cloud-based operations
    #[must_use]
    /// Checks if cloud based
    /// Checks if cloud based
    pub fn is_cloud_based(&self) -> bool {
        match self {
            Self::Cloud { .. } => true,
            Self::Universal { capabilities, .. } => {
                capabilities.contains(&HsmSecurityCapability::CloudBased)
            }
            _ => false,
        }
    }

    /// Check if provider is hardware-based
    #[must_use]
    /// Checks if hardware based
    /// Checks if hardware based
    pub fn is_hardware_based(&self) -> bool {
        match self {
            Self::Hardware { capabilities }
            | Self::Network { capabilities }
            | Self::Universal { capabilities, .. } => {
                capabilities.contains(&HsmSecurityCapability::HardwareCrypto)
            }
            Self::Software | Self::Cloud { .. } => false, // Cloud HSMs are hardware-backed but accessed remotely
        }
    }

    /// Checks if the HSM provider supports hardware-based cryptographic operations
    ///
    /// Returns `true` if the provider has hardware crypto capabilities, `false` otherwise.
    pub fn supports_hardware_crypto(&self) -> bool {
        match self {
            Self::Hardware { capabilities }
            | Self::Network { capabilities }
            | Self::Universal { capabilities, .. } => {
                capabilities.contains(&HsmSecurityCapability::HardwareCrypto)
            }
            Self::Software | Self::Cloud { .. } => false,
        }
    }
}

impl HsmProviderConfig {
    /// Create a new modern HSM provider config with capability discovery
    #[must_use]
    /// Creates a new instance
    /// Creates a new instance
    pub fn new_with_capability_discovery(
        name: String,
        capability_type: CapabilityType,
        discovery_endpoint: Option<String>,
    ) -> Self {
        Self {
            name,
            provider_type: HsmProviderType::Universal {
                provider_id: "discovered".to_string(),
                capability_type: capability_type.clone(),
                capabilities: vec![HsmSecurityCapability::HardwareCrypto],
            },
            endpoint: None, // Will be discovered dynamically
            enabled: true,
            capability_discovery: Some(HsmCapabilityDiscovery {
                capability_type,
                discovery_endpoint,
                preferences: vec![],
            }),
        }
    }

    /// Check if this config uses deprecated patterns
    #[must_use]
    pub const fn uses_deprecated_patterns(&self) -> bool {
        // All providers now use modern capability-based discovery
        false
    }

    #[must_use]
    /// Gets `migration_recommendation`
    /// Gets `migration_recommendation`
    pub fn get_migration_recommendation(&self) -> Option<String> {
        if self.uses_deprecated_patterns() {
            Some(format!(
                "Migrate {} to use capability discovery instead of hardcoded provider type. \
                 Use HsmProviderConfig::new_with_capability_discovery() for modern patterns.",
                &self.name
            ))
        } else {
            // No provider available
            None
        }
    }
}

/// Static helpers documenting migration from legacy hardcoded HSM provider enums.
pub struct HsmProviderMigrationHelper;

impl HsmProviderMigrationHelper {
    /// Full migration guide string for operators and config authors.
    #[must_use]
    pub const fn get_migration_guidance() -> &'static str {
        "🚨 HSM PROVIDER MIGRATION GUIDE:\n\
         \n\
         DEPRECATED PATTERNS:\n\
         ❌ HsmProviderType::AwsCloudHsm\n\
         ❌ HsmProviderType::UniversalCloudDedicatedHsm\n\
         ❌ HsmProviderType::GoogleCloudHsm\n\
         \n\
         MODERN PATTERNS:\n\
         ✅ HsmProviderType::Universal with capability discovery\n\
         ✅ Dynamic provider discovery via CapabilityType::HardwareSecurityModule\n\
         ✅ Automatic failover and provider selection\n\
         \n\
         MIGRATION EXAMPLE:\n\
         \n\
                 BEFORE (hardcoded - now removed):\n\
        // Hardcoded cloud providers have been eliminated\n\
        // for sovereignty compliance\n\
         \n\
         AFTER (modern):\n\
         let config = HsmProviderConfig::new_with_capability_discovery(\n\
             \"universal_hsm\".to_string(),\n\
             CapabilityType::HardwareSecurityModule,\n\
             None, // Will be discovered automatically\n\
         );\n\
         \n\
         Benefits:\n\
         ✅ Works with ANY HSM provider (discovered dynamically at runtime)\n\
         ✅ Dynamic discovery - no hardcoded endpoints\n\
         ✅ Automatic failover between providers\n\
         ✅ Future-proof architecture\n\
         \n\
         See: ARCHITECTURE.md for capability-based patterns"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modern_hsm_config() {
        let config = HsmProviderConfig::new_with_capability_discovery(
            "test_hsm".to_string(),
            CapabilityType::HardwareSecurityModule,
            // No specific endpoint
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            None,
        );

        assert!(!config.uses_deprecated_patterns());
        assert!(config.capability_discovery.is_some());
        assert!(matches!(
            config.provider_type,
            HsmProviderType::Universal { .. }
        ));
    }

    #[test]
    fn test_modern_provider_patterns() {
        // All providers are now modern - no deprecated patterns remain
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let universal_provider = HsmProviderType::Universal {
            provider_id: "test_provider".to_string(),
            capability_type: CapabilityType::HardwareSecurityModule,
            capabilities: vec![],
        };
        assert!(!universal_provider.is_deprecated());
        assert!(universal_provider.get_migration_guidance().is_none());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_migration_helper() {
        let guidance = HsmProviderMigrationHelper::get_migration_guidance();
        assert!(guidance.contains("MIGRATION GUIDE"));
        assert!(guidance.contains("MODERN PATTERNS"));
    }
}
