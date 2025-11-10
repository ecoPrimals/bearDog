//! Universal HSM Discovery Module
//!
//! Provides functionality for discovering and classifying Hardware Security Modules
//! across different platforms and providers.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

// Re-export capability types
pub use crate::tunnel::hsm::types::capability::{
    AdvancedFeatureCapabilities, ApiSupportCapabilities, ComplianceCapabilities,
    CryptoOperationCapabilities, HsmCapabilities as UniversalHsmCapabilities,
    HumanEntropyCapabilities, KeyGenerationCapabilities, KeyManagementCapabilities,
    PerformanceCapabilities, SecurityCapabilities, TamperResistanceLevel as TamperResistance,
};
pub use crate::tunnel::hsm::types::{status::HsmHealthStatus, tier::HsmTier};

// Submodules
pub mod capability_detection;
pub mod discovery;
pub mod human_entropy_classifier;
pub mod tier_manager;

#[cfg(test)]
mod comprehensive_core_tests;

#[cfg(test)]
mod edge_cases_and_fault_tests;

#[cfg(test)]
mod capability_detection_comprehensive_tests;

#[cfg(test)]
mod universal_adapter_comprehensive_tests;

#[cfg(test)]
mod health_monitoring_comprehensive_tests;

#[cfg(test)]
mod configuration_management_comprehensive_tests;

#[cfg(test)]
mod workflow_integration_comprehensive_tests;

#[cfg(test)]
mod chaos_engineering_comprehensive_tests;

#[cfg(test)]
mod e2e_scenarios_comprehensive_tests;

#[cfg(test)]
mod property_based_comprehensive_tests;

#[cfg(test)]
mod performance_benchmarks_comprehensive_tests;

/// HSM endpoint information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmEndpoint {
    /// Hostname or address
    pub host: String,
    /// Optional port
    pub port: Option<u16>,
    /// Protocol
    pub protocol: String,
    /// Whether secure connection is required
    pub secure: bool,
}

/// Human entropy methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HumanEntropyMethod {
    /// Touch patterns with advanced pressure sensitivity
    TouchPatternsAdvanced { pressure_sensitive: bool },
    /// Biometric variation
    BiometricVariation,
    /// Keyboard timing
    KeyboardTiming,
    /// Device motion
    DeviceMotion,
    /// Environmental sensors
    EnvironmentalSensors { sensor_types: Vec<String> },
}

/// HSM type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HsmType {
    /// Hardware HSM
    Hardware,
    /// Software HSM
    Software,
    /// Cloud HSM
    Cloud,
    /// Smartphone (iOS/Android)
    Smartphone,
    /// TPM (Trusted Platform Module)
    Tpm,
    /// PKCS#11 provider
    Pkcs11,
}

/// Integration status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntegrationStatus {
    /// Discovered but not configured
    Discovered,
    /// Currently configuring
    Configuring,
    /// Ready for use
    Ready,
    /// Integration failed
    Failed,
    /// Disabled
    Disabled,
}

/// Discovered HSM information
#[derive(Debug, Clone)]
pub struct DiscoveredHsm {
    /// Unique identifier
    pub name: String,
    /// HSM type
    pub hsm_type: HsmType,
    /// Connection endpoint
    pub endpoint: HsmEndpoint,
    /// Detected capabilities
    pub capabilities: UniversalHsmCapabilities,
    /// Assigned tier
    pub assigned_tier: HsmTier,
    /// Whether supports human entropy
    pub supports_human_entropy: bool,
    /// Health status
    pub health_status: HsmHealthStatus,
    /// When discovered
    pub discovered_at: chrono::DateTime<chrono::Utc>,
    /// Last health check time
    pub last_health_check: chrono::DateTime<chrono::Utc>,
    /// Integration status
    pub integration_status: IntegrationStatus,
}

// Use canonical HSM discovery configuration
pub use beardog_types::canonical::hsm::discovery::HsmDiscoveryConfig as DiscoveryConfig;

// NOTE: Original local DiscoveryConfig had these fields (now in HsmDiscoveryConfig):
// - enable_cloud_discovery → enable_cloud_kms
// - enable_pkcs11_discovery → enable_pkcs11_discovery
// - enable_smartphone_discovery → enable_mobile_hsm
// - discovery_timeout_seconds → base.timeout
// - enable_capability_detection → enable_capability_detection

// Removed: Local Default implementation (now using canonical HsmDiscoveryConfig)

/// Universal HSM discovery engine
#[derive(Clone)]
pub struct UniversalHsmDiscovery {
    /// Discovered HSMs registry
    discovered_hsms: Arc<RwLock<HashMap<String, DiscoveredHsm>>>,
    /// Tier manager
    tier_manager: Arc<tier_manager::TierManager>,
    /// Entropy classifier
    entropy_classifier: Arc<human_entropy_classifier::HumanEntropyClassifier>,
    /// Discovery configuration
    config: DiscoveryConfig,
}

impl UniversalHsmDiscovery {
    /// Create new discovery engine
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub async fn new(config: DiscoveryConfig) -> Result<Self, BearDogError> {
        info!("Creating Universal HSM Discovery engine");

        Ok(Self {
            discovered_hsms: Arc::new(RwLock::new(HashMap::new())),
            tier_manager: Arc::new(tier_manager::TierManager::new().await?),
            entropy_classifier: Arc::new(
                human_entropy_classifier::HumanEntropyClassifier::new().await?,
            ),
            config,
        })
    }

    /// Discover all available HSMs
    ///
    /// # Errors
    /// Returns an error if discovery fails
    pub async fn discover_all(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("Starting HSM discovery");

        let mut discovered = Vec::new();

        // Cloud HSM discovery
        if self.config.enable_cloud_discovery {
            debug!("Discovering cloud HSMs");
            match self.discover_cloud_hsms().await {
                Ok(mut cloud_hsms) => {
                    info!("Discovered {} cloud HSMs", cloud_hsms.len());
                    discovered.append(&mut cloud_hsms);
                }
                Err(e) => {
                    warn!("Cloud HSM discovery failed: {}", e);
                }
            }
        }

        // PKCS#11 discovery
        if self.config.enable_pkcs11_discovery {
            debug!("Discovering PKCS#11 HSMs");
            match self.discover_pkcs11_hsms().await {
                Ok(mut pkcs11_hsms) => {
                    info!("Discovered {} PKCS#11 HSMs", pkcs11_hsms.len());
                    discovered.append(&mut pkcs11_hsms);
                }
                Err(e) => {
                    warn!("PKCS#11 HSM discovery failed: {}", e);
                }
            }
        }

        // Smartphone discovery
        if self.config.enable_smartphone_discovery {
            debug!("Discovering smartphone HSMs");
            match self.discover_smartphone_hsms().await {
                Ok(mut mobile_hsms) => {
                    info!("Discovered {} smartphone HSMs", mobile_hsms.len());
                    discovered.append(&mut mobile_hsms);
                }
                Err(e) => {
                    warn!("Smartphone HSM discovery failed: {}", e);
                }
            }
        }

        // Store discovered HSMs
        let mut hsm_map = self.discovered_hsms.write().await;
        for hsm in &discovered {
            hsm_map.insert(hsm.name.clone(), hsm.clone());
        }

        info!("Discovered {} HSMs total", discovered.len());
        Ok(discovered)
    }

    /// Discover cloud HSMs (AWS KMS, Azure Key Vault, Google Cloud KMS, etc.)
    ///
    /// # Errors
    /// Returns an error if cloud discovery fails
    async fn discover_cloud_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        debug!("Starting cloud HSM discovery");
        let mut cloud_hsms = Vec::new();

        // Try to discover AWS KMS
        if let Ok(aws_hsm) = self.discover_aws_kms().await {
            cloud_hsms.push(aws_hsm);
        }

        // Try to discover Azure Key Vault
        if let Ok(azure_hsm) = self.discover_azure_key_vault().await {
            cloud_hsms.push(azure_hsm);
        }

        // Try to discover Google Cloud KMS
        if let Ok(gcp_hsm) = self.discover_gcp_kms().await {
            cloud_hsms.push(gcp_hsm);
        }

        Ok(cloud_hsms)
    }

    /// Discover AWS KMS HSM
    async fn discover_aws_kms(&self) -> Result<DiscoveredHsm, BearDogError> {
        // Check for AWS credentials via environment
        if std::env::var("AWS_ACCESS_KEY_ID").is_err() {
            return Err(BearDogError::not_found(
                "AWS credentials not configured".to_string(),
            ));
        }

        Ok(DiscoveredHsm {
            name: "aws-kms".to_string(),
            tier: HsmTier::CloudHsm,
            capabilities: vec![
                "sign".to_string(),
                "verify".to_string(),
                "encrypt".to_string(),
                "decrypt".to_string(),
            ],
            metadata: HashMap::from([
                ("provider".to_string(), "aws".to_string()),
                ("type".to_string(), "cloud".to_string()),
                (
                    "region".to_string(),
                    std::env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
                ),
            ]),
        })
    }

    /// Discover Azure Key Vault HSM
    async fn discover_azure_key_vault(&self) -> Result<DiscoveredHsm, BearDogError> {
        // Check for Azure credentials via environment
        if std::env::var("AZURE_CLIENT_ID").is_err() {
            return Err(BearDogError::not_found(
                "Azure credentials not configured".to_string(),
            ));
        }

        Ok(DiscoveredHsm {
            name: "azure-keyvault".to_string(),
            tier: HsmTier::CloudHsm,
            capabilities: vec![
                "sign".to_string(),
                "verify".to_string(),
                "encrypt".to_string(),
                "decrypt".to_string(),
            ],
            metadata: HashMap::from([
                ("provider".to_string(), "azure".to_string()),
                ("type".to_string(), "cloud".to_string()),
                (
                    "vault_url".to_string(),
                    std::env::var("AZURE_KEY_VAULT_URL")
                        .unwrap_or_else(|_| "https://vault.azure.net".to_string()),
                ),
            ]),
        })
    }

    /// Discover Google Cloud KMS HSM
    async fn discover_gcp_kms(&self) -> Result<DiscoveredHsm, BearDogError> {
        // Check for GCP credentials via environment
        if std::env::var("GOOGLE_APPLICATION_CREDENTIALS").is_err() {
            return Err(BearDogError::not_found(
                "GCP credentials not configured".to_string(),
            ));
        }

        Ok(DiscoveredHsm {
            name: "gcp-kms".to_string(),
            tier: HsmTier::CloudHsm,
            capabilities: vec![
                "sign".to_string(),
                "verify".to_string(),
                "encrypt".to_string(),
                "decrypt".to_string(),
            ],
            metadata: HashMap::from([
                ("provider".to_string(), "gcp".to_string()),
                ("type".to_string(), "cloud".to_string()),
                (
                    "project_id".to_string(),
                    std::env::var("GCP_PROJECT_ID").unwrap_or_else(|_| "default".to_string()),
                ),
            ]),
        })
    }

    /// Discover PKCS#11 HSMs (YubiKey, network HSMs, etc.)
    ///
    /// # Errors
    /// Returns an error if PKCS#11 discovery fails
    async fn discover_pkcs11_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        debug!("Starting PKCS#11 HSM discovery");
        let mut pkcs11_hsms = Vec::new();

        // Common PKCS#11 library paths to check
        let library_paths = vec![
            "/usr/lib/softhsm/libsofthsm2.so",
            "/usr/local/lib/softhsm/libsofthsm2.so",
            "/usr/lib/x86_64-linux-gnu/softhsm/libsofthsm2.so",
            "/usr/lib/pkcs11/yubihsm_pkcs11.so",
            "/usr/local/lib/libyubihsm_pkcs11.dylib",
        ];

        for path in library_paths {
            if std::path::Path::new(path).exists() {
                debug!("Found PKCS#11 library at {}", path);

                let name = std::path::Path::new(path)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .trim_start_matches("lib")
                    .to_string();

                pkcs11_hsms.push(DiscoveredHsm {
                    name: format!("pkcs11-{}", name),
                    tier: HsmTier::Hardware,
                    capabilities: vec![
                        "sign".to_string(),
                        "verify".to_string(),
                        "encrypt".to_string(),
                        "decrypt".to_string(),
                        "generate_key".to_string(),
                    ],
                    metadata: HashMap::from([
                        ("provider".to_string(), "pkcs11".to_string()),
                        ("library_path".to_string(), path.to_string()),
                        ("type".to_string(), "hardware".to_string()),
                    ]),
                });
            }
        }

        Ok(pkcs11_hsms)
    }

    /// Discover smartphone HSMs (Android StrongBox, iOS Secure Enclave)
    ///
    /// # Errors
    /// Returns an error if smartphone discovery fails
    async fn discover_smartphone_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        debug!("Starting smartphone HSM discovery");
        let mut mobile_hsms = Vec::new();

        // Detect Android StrongBox
        #[cfg(target_os = "android")]
        {
            if let Ok(strongbox) = self.discover_android_strongbox().await {
                mobile_hsms.push(strongbox);
            }
        }

        // Detect iOS Secure Enclave
        #[cfg(target_os = "ios")]
        {
            if let Ok(secure_enclave) = self.discover_ios_secure_enclave().await {
                mobile_hsms.push(secure_enclave);
            }
        }

        // For non-mobile platforms, return software HSM as fallback
        #[cfg(not(any(target_os = "android", target_os = "ios")))]
        {
            debug!("Not running on mobile platform, checking for software HSM");
            if let Ok(software_hsm) = self.discover_software_hsm().await {
                mobile_hsms.push(software_hsm);
            }
        }

        Ok(mobile_hsms)
    }

    /// Discover Android StrongBox HSM
    #[cfg(target_os = "android")]
    async fn discover_android_strongbox(&self) -> Result<DiscoveredHsm, BearDogError> {
        Ok(DiscoveredHsm {
            name: "android-strongbox".to_string(),
            tier: HsmTier::Hardware,
            capabilities: vec![
                "sign".to_string(),
                "verify".to_string(),
                "encrypt".to_string(),
                "decrypt".to_string(),
                "generate_key".to_string(),
                "hardware_backed".to_string(),
            ],
            metadata: HashMap::from([
                ("provider".to_string(), "android".to_string()),
                ("type".to_string(), "strongbox".to_string()),
                ("platform".to_string(), "android".to_string()),
            ]),
        })
    }

    /// Discover iOS Secure Enclave HSM
    #[cfg(target_os = "ios")]
    async fn discover_ios_secure_enclave(&self) -> Result<DiscoveredHsm, BearDogError> {
        Ok(DiscoveredHsm {
            name: "ios-secure-enclave".to_string(),
            tier: HsmTier::Hardware,
            capabilities: vec![
                "sign".to_string(),
                "verify".to_string(),
                "encrypt".to_string(),
                "decrypt".to_string(),
                "generate_key".to_string(),
                "hardware_backed".to_string(),
                "biometric".to_string(),
            ],
            metadata: HashMap::from([
                ("provider".to_string(), "ios".to_string()),
                ("type".to_string(), "secure_enclave"),
                ("platform".to_string(), "ios".to_string()),
            ]),
        })
    }

    /// Discover software HSM (fallback for non-mobile platforms)
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    async fn discover_software_hsm(&self) -> Result<DiscoveredHsm, BearDogError> {
        Ok(DiscoveredHsm {
            name: "software-hsm".to_string(),
            tier: HsmTier::Software,
            capabilities: vec![
                "sign".to_string(),
                "verify".to_string(),
                "encrypt".to_string(),
                "decrypt".to_string(),
                "generate_key".to_string(),
            ],
            metadata: HashMap::from([
                ("provider".to_string(), "software".to_string()),
                ("type".to_string(), "software".to_string()),
                ("platform".to_string(), std::env::consts::OS.to_string()),
            ]),
        })
    }

    /// Get discovered HSM by name
    ///
    /// # Errors
    /// Returns an error if HSM not found
    pub async fn get_hsm(&self, name: &str) -> Result<DiscoveredHsm, BearDogError> {
        let hsms = self.discovered_hsms.read().await;
        hsms.get(name)
            .cloned()
            .ok_or_else(|| BearDogError::not_found(format!("HSM not found: {}", name)))
    }

    /// List all discovered HSMs
    ///
    /// # Errors
    /// Returns an error if retrieval fails
    pub async fn list_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        let hsms = self.discovered_hsms.read().await;
        Ok(hsms.values().cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_discovery_creation() -> Result<(), BearDogError> {
        let config = DiscoveryConfig::default();
        let discovery = UniversalHsmDiscovery::new(config).await?;
        assert!(discovery.discovered_hsms.read().await.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_default_config() {
        let config = DiscoveryConfig::default();
        assert!(config.enable_cloud_discovery);
        assert!(config.enable_pkcs11_discovery);
        assert!(config.enable_smartphone_discovery);
        assert_eq!(config.discovery_timeout_seconds, 30);
        assert!(config.enable_capability_detection);
    }
}
