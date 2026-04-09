// SPDX-License-Identifier: AGPL-3.0-or-later

//! Cloud HSM Discoverer
//!
//! Provides discovery functionality for cloud-based HSMs
//!
//! This module discovers cloud HSM services from major providers:
//! - AWS KMS (Key Management Service) and CloudHSM
//! - Azure Key Vault and Managed HSM
//! - Google Cloud KMS and Cloud HSM
//! - Other cloud providers

use super::super::*;
use beardog_errors::BearDogError;
use chrono::Utc;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use tracing::{debug, info, warn};

// Re-export canonical CloudProvider from beardog-types
pub use beardog_types::canonical::hsm_unified::CloudProvider;

/// Cloud region information
#[derive(Debug, Clone)]
pub struct CloudRegion {
    /// Provider
    pub provider: CloudProvider,
    /// Region identifier (e.g., "us-east-1", "eastus", "us-central1")
    pub region_id: String,
    /// Display name
    pub display_name: String,
}

/// Cloud HSM configuration
#[derive(Debug, Clone)]
pub struct CloudHsmConfig {
    /// Provider
    pub provider: CloudProvider,
    /// Region
    pub region: String,
    /// Endpoint URL (if custom)
    pub endpoint: Option<String>,
    /// Service type (KMS, CloudHSM, etc.)
    pub service_type: String,
}

/// Cloud HSM discoverer
#[derive(Debug, Clone)]
pub struct CloudDiscoverer {
    /// Whether to check AWS
    enable_aws: bool,
    /// Whether to check Azure
    enable_azure: bool,
    /// Whether to check GCP
    enable_gcp: bool,
    /// Whether to check other providers
    enable_other_providers: bool,
    /// Custom cloud configurations
    custom_configs: Vec<CloudHsmConfig>,
}

impl CloudDiscoverer {
    /// Create new cloud discoverer
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            enable_aws: true,
            enable_azure: true,
            enable_gcp: true,
            enable_other_providers: false,
            custom_configs: Vec::new(),
        })
    }

    /// Add custom cloud configuration
    pub fn add_custom_config(&mut self, config: CloudHsmConfig) {
        self.custom_configs.push(config);
    }

    /// Discover cloud HSMs
    ///
    /// # Errors
    /// Returns an error if discovery fails
    pub async fn discover(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("🔍 Discovering cloud-based HSMs");
        
        let mut discovered = Vec::new();

        // Discover AWS KMS
        if self.enable_aws {
            discovered.extend(self.discover_aws_kms().await?);
        }

        // Discover Azure Key Vault
        if self.enable_azure {
            discovered.extend(self.discover_azure_key_vault().await?);
        }

        // Discover Google Cloud KMS
        if self.enable_gcp {
            discovered.extend(self.discover_gcp_kms().await?);
        }

        // Discover from custom configs
        for config in &self.custom_configs {
            if let Some(hsm) = self.create_hsm_from_config(config) {
                info!("✓ Found custom cloud HSM: {}", hsm.name);
                discovered.push(hsm);
            }
        }

        info!("✅ Cloud HSM discovery complete: {} HSMs found", discovered.len());
        Ok(discovered)
    }

    /// Discover AWS KMS instances
    ///
    /// # Errors
    /// Returns an error if discovery fails
    async fn discover_aws_kms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        debug!("Checking for AWS KMS configuration");

        let mut hsms = Vec::new();

        // Check for AWS credentials
        let has_credentials = self.check_aws_credentials();
        if !has_credentials {
            debug!("No AWS credentials found, skipping AWS KMS discovery");
            return Ok(hsms);
        }

        // Get configured region(s)
        let regions = self.get_aws_regions();
        
        for region in regions {
            // Create KMS HSM for this region
            hsms.push(self.create_aws_kms_hsm(&region));
            
            // Check for CloudHSM in this region
            if let Some(cloudhsm) = self.check_aws_cloudhsm(&region).await? {
                hsms.push(cloudhsm);
            }
        }

        if !hsms.is_empty() {
            info!("✓ Found {} AWS KMS/CloudHSM instance(s)", hsms.len());
        }

        Ok(hsms)
    }

    /// Check for AWS credentials
    fn check_aws_credentials(&self) -> bool {
        // Check environment variables
        if beardog_errors::process_env::var("AWS_ACCESS_KEY_ID").is_ok() && beardog_errors::process_env::var("AWS_SECRET_ACCESS_KEY").is_ok() {
            return true;
        }

        // Check for AWS credentials file
        if let Ok(home) = beardog_errors::process_env::var("HOME") {
            let aws_creds = Path::new(&home).join(".aws").join("credentials");
            if aws_creds.exists() {
                return true;
            }
        }

        // Check for AWS config file
        if let Ok(home) = beardog_errors::process_env::var("HOME") {
            let aws_config = Path::new(&home).join(".aws").join("config");
            if aws_config.exists() {
                return true;
            }
        }

        false
    }

    /// Get AWS regions to check
    fn get_aws_regions(&self) -> Vec<String> {
        let mut regions = Vec::new();

        // Check configured region
        if let Ok(region) = beardog_errors::process_env::var("AWS_REGION") {
            regions.push(region);
        } else if let Ok(region) = beardog_errors::process_env::var("AWS_DEFAULT_REGION") {
            regions.push(region);
        } else {
            // Default to us-east-1
            regions.push("us-east-1".to_string());
        }

        regions
    }

    /// Create AWS KMS HSM
    fn create_aws_kms_hsm(&self, region: &str) -> DiscoveredHsm {
        let now = Utc::now();
        
        DiscoveredHsm {
            name: format!("aws-kms-{region}"),
            hsm_type: HsmType::Cloud,
            endpoint: HsmEndpoint {
                host: format!("kms.{region}.amazonaws.com"),
                port: Some(443),
                protocol: "https".to_string(),
                secure: true,
            },
            capabilities: self.create_aws_kms_capabilities(),
            assigned_tier: HsmTier::Tier1,
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: now,
            last_health_check: now,
            integration_status: IntegrationStatus::Discovered,
        }
    }

    /// Check for AWS CloudHSM
    ///
    /// # Errors
    /// Returns an error if check fails
    async fn check_aws_cloudhsm(&self, region: &str) -> Result<Option<DiscoveredHsm>, BearDogError> {
        // In a full implementation, this would query AWS CloudHSM API
        // For now, we'll check if CLOUDHSM_CLUSTER_ID env var is set
        if let Ok(cluster_id) = beardog_errors::process_env::var("CLOUDHSM_CLUSTER_ID") {
            let now = Utc::now();
            
            Ok(Some(DiscoveredHsm {
                name: format!("aws-cloudhsm-{}-{}", region, &cluster_id[..8.min(cluster_id.len())]),
                hsm_type: HsmType::Cloud,
                endpoint: HsmEndpoint {
                    host: format!("cloudhsmv2.{region}.amazonaws.com"),
                    port: Some(443),
                    protocol: "https".to_string(),
                    secure: true,
                },
                capabilities: self.create_aws_cloudhsm_capabilities(),
                assigned_tier: HsmTier::Tier1,
                supports_human_entropy: false,
                health_status: HsmHealthStatus::Healthy,
                discovered_at: now,
                last_health_check: now,
                integration_status: IntegrationStatus::Discovered,
            }))
        } else {
            Ok(None)
        }
    }

    /// Discover Azure Key Vault instances
    ///
    /// # Errors
    /// Returns an error if discovery fails
    async fn discover_azure_key_vault(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        debug!("Checking for Azure Key Vault configuration");

        let mut hsms = Vec::new();

        // Check for Azure credentials
        let has_credentials = self.check_azure_credentials();
        if !has_credentials {
            debug!("No Azure credentials found, skipping Azure Key Vault discovery");
            return Ok(hsms);
        }

        // Check for configured vault names
        if let Ok(vault_name) = beardog_errors::process_env::var("AZURE_KEYVAULT_NAME") {
            hsms.push(self.create_azure_key_vault_hsm(&vault_name));
        }

        // Check for Managed HSM
        if let Ok(hsm_name) = beardog_errors::process_env::var("AZURE_MANAGEDHSM_NAME") {
            hsms.push(self.create_azure_managed_hsm(&hsm_name));
        }

        if !hsms.is_empty() {
            info!("✓ Found {} Azure Key Vault/Managed HSM instance(s)", hsms.len());
        }

        Ok(hsms)
    }

    /// Check for Azure credentials
    fn check_azure_credentials(&self) -> bool {
        // Check for service principal
        if beardog_errors::process_env::var("AZURE_CLIENT_ID").is_ok() 
            && beardog_errors::process_env::var("AZURE_CLIENT_SECRET").is_ok() 
            && beardog_errors::process_env::var("AZURE_TENANT_ID").is_ok() {
            return true;
        }

        // Check for managed identity
        if beardog_errors::process_env::var("AZURE_CLIENT_ID").is_ok() {
            return true;
        }

        // Check for Azure CLI authentication
        if let Ok(home) = beardog_errors::process_env::var("HOME") {
            let azure_dir = Path::new(&home).join(".azure");
            if azure_dir.exists() {
                return true;
            }
        }

        false
    }

    /// Create Azure Key Vault HSM
    fn create_azure_key_vault_hsm(&self, vault_name: &str) -> DiscoveredHsm {
        let now = Utc::now();
        
        DiscoveredHsm {
            name: format!("azure-keyvault-{vault_name}"),
            hsm_type: HsmType::Cloud,
            endpoint: HsmEndpoint {
                host: format!("{vault_name}.vault.azure.net"),
                port: Some(443),
                protocol: "https".to_string(),
                secure: true,
            },
            capabilities: self.create_azure_key_vault_capabilities(),
            assigned_tier: HsmTier::Tier1,
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: now,
            last_health_check: now,
            integration_status: IntegrationStatus::Discovered,
        }
    }

    /// Create Azure Managed HSM
    fn create_azure_managed_hsm(&self, hsm_name: &str) -> DiscoveredHsm {
        let now = Utc::now();
        
        DiscoveredHsm {
            name: format!("azure-managedhsm-{hsm_name}"),
            hsm_type: HsmType::Cloud,
            endpoint: HsmEndpoint {
                host: format!("{hsm_name}.managedhsm.azure.net"),
                port: Some(443),
                protocol: "https".to_string(),
                secure: true,
            },
            capabilities: self.create_azure_managed_hsm_capabilities(),
            assigned_tier: HsmTier::Tier1,
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: now,
            last_health_check: now,
            integration_status: IntegrationStatus::Discovered,
        }
    }

    /// Discover Google Cloud KMS instances
    ///
    /// # Errors
    /// Returns an error if discovery fails
    async fn discover_gcp_kms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        debug!("Checking for Google Cloud KMS configuration");

        let mut hsms = Vec::new();

        // Check for GCP credentials
        let has_credentials = self.check_gcp_credentials();
        if !has_credentials {
            debug!("No GCP credentials found, skipping Cloud KMS discovery");
            return Ok(hsms);
        }

        // Get project ID
        if let Ok(project_id) = beardog_errors::process_env::var("GCP_PROJECT_ID").or_else(|_| beardog_errors::process_env::var("GOOGLE_CLOUD_PROJECT")) {
            // Get regions
            let regions = self.get_gcp_regions();
            
            for region in regions {
                hsms.push(self.create_gcp_kms_hsm(&project_id, &region));
            }
        }

        if !hsms.is_empty() {
            info!("✓ Found {} Google Cloud KMS instance(s)", hsms.len());
        }

        Ok(hsms)
    }

    /// Check for GCP credentials
    fn check_gcp_credentials(&self) -> bool {
        // Check for service account key
        if let Ok(key_file) = beardog_errors::process_env::var("GOOGLE_APPLICATION_CREDENTIALS") {
            if Path::new(&key_file).exists() {
                return true;
            }
        }

        // Check for gcloud config
        if let Ok(home) = beardog_errors::process_env::var("HOME") {
            let gcloud_config = Path::new(&home).join(".config").join("gcloud");
            if gcloud_config.exists() {
                return true;
            }
        }

        false
    }

    /// Get GCP regions
    fn get_gcp_regions(&self) -> Vec<String> {
        if let Ok(region) = beardog_errors::process_env::var("GCP_REGION") {
            vec![region]
        } else {
            vec!["us-central1".to_string()]
        }
    }

    /// Create GCP KMS HSM
    fn create_gcp_kms_hsm(&self, project_id: &str, region: &str) -> DiscoveredHsm {
        let now = Utc::now();
        
        DiscoveredHsm {
            name: format!("gcp-kms-{}-{}", project_id, region),
            hsm_type: HsmType::Cloud,
            endpoint: HsmEndpoint {
                host: format!("cloudkms.googleapis.com"),
                port: Some(443),
                protocol: "https".to_string(),
                secure: true,
            },
            capabilities: self.create_gcp_kms_capabilities(),
            assigned_tier: HsmTier::Tier1,
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: now,
            last_health_check: now,
            integration_status: IntegrationStatus::Discovered,
        }
    }

    /// Create HSM from custom config
    fn create_hsm_from_config(&self, config: &CloudHsmConfig) -> Option<DiscoveredHsm> {
        let now = Utc::now();
        let provider_name = format!("{:?}", config.provider).to_lowercase();

        Some(DiscoveredHsm {
            name: format!("{}-{}-{}", provider_name, config.service_type, config.region),
            hsm_type: HsmType::Cloud,
            endpoint: HsmEndpoint {
                host: config.endpoint.clone().unwrap_or_else(|| format!("{}.{}.cloud", config.service_type, config.region)),
                port: Some(443),
                protocol: "https".to_string(),
                secure: true,
            },
            capabilities: self.create_generic_cloud_hsm_capabilities(),
            assigned_tier: HsmTier::Tier2,
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: now,
            last_health_check: now,
            integration_status: IntegrationStatus::Discovered,
        })
    }

    /// Create AWS KMS capabilities
    fn create_aws_kms_capabilities(&self) -> UniversalHsmCapabilities {
        use crate::tunnel::hsm::types::capability::*;

        UniversalHsmCapabilities {
            crypto_operations: CryptoOperationCapabilities {
                symmetric_encryption: vec!["AES-128".to_string(), "AES-256".to_string()],
                asymmetric_encryption: vec![
                    "RSA-2048".to_string(),
                    "RSA-3072".to_string(),
                    "RSA-4096".to_string(),
                    "ECC-P256".to_string(),
                    "ECC-P384".to_string(),
                    "ECC-P521".to_string(),
                ],
                signing: vec![
                    "RSA-PSS".to_string(),
                    "ECDSA".to_string(),
                    "RSASSA-PKCS1-V1_5".to_string(),
                ],
                hashing: vec!["SHA-256".to_string(), "SHA-384".to_string(), "SHA-512".to_string()],
                key_agreement: vec!["ECDH".to_string()],
            },
            key_management: KeyManagementCapabilities {
                key_generation: true,
                key_storage: true,
                key_rotation: true,
                key_backup: true,
                key_recovery: false, // AWS KMS doesn't export keys
            },
            key_generation: KeyGenerationCapabilities {
                rsa: vec![2048, 3072, 4096],
                ecc: vec![256, 384, 521],
                aes: vec![128, 256],
                supports_secure_random: true,
            },
            security: SecurityCapabilities {
                tamper_resistance: TamperResistance::Tier1,
                fips_140_2_level: Some(3), // AWS KMS uses FIPS 140-2 Level 3 HSMs
                common_criteria_eal: None,
                secure_boot: true,
                attestation: true,
            },
            performance: PerformanceCapabilities {
                max_operations_per_second: 100000,
                typical_latency_ms: 20.0,
                supports_parallel_operations: true,
                hardware_acceleration: true,
            },
            compliance: ComplianceCapabilities {
                fips_140_2: true,
                common_criteria: false,
                pci_dss: true,
                hipaa: true,
                gdpr: true,
            },
            api_support: ApiSupportCapabilities {
                pkcs11: false,
                tpm2: false,
                kmip: false,
                pkcs7: true,
            },
            advanced_features: AdvancedFeatureCapabilities {
                quantum_resistant: true, // AWS KMS supports post-quantum TLS
                multi_party_computation: false,
                threshold_cryptography: false,
                homomorphic_encryption: false,
            },
            human_entropy: HumanEntropyCapabilities {
                supported: false,
                methods: vec![],
                quality_score: 0.0,
            },
        }
    }

    /// Create AWS CloudHSM capabilities
    fn create_aws_cloudhsm_capabilities(&self) -> UniversalHsmCapabilities {
        use crate::tunnel::hsm::types::capability::*;

        let mut caps = self.create_aws_kms_capabilities();
        caps.api_support.pkcs11 = true;
        caps.performance.max_operations_per_second = 50000;
        caps.performance.typical_latency_ms = 5.0;
        caps.key_management.key_recovery = false;
        caps
    }

    /// Create Azure Key Vault capabilities
    fn create_azure_key_vault_capabilities(&self) -> UniversalHsmCapabilities {
        use crate::tunnel::hsm::types::capability::*;

        UniversalHsmCapabilities {
            crypto_operations: CryptoOperationCapabilities {
                symmetric_encryption: vec!["AES-128".to_string(), "AES-256".to_string()],
                asymmetric_encryption: vec![
                    "RSA-2048".to_string(),
                    "RSA-3072".to_string(),
                    "RSA-4096".to_string(),
                    "ECC-P256".to_string(),
                    "ECC-P384".to_string(),
                    "ECC-P521".to_string(),
                ],
                signing: vec!["RSA-PSS".to_string(), "ECDSA".to_string()],
                hashing: vec!["SHA-256".to_string(), "SHA-384".to_string(), "SHA-512".to_string()],
                key_agreement: vec!["ECDH".to_string()],
            },
            key_management: KeyManagementCapabilities {
                key_generation: true,
                key_storage: true,
                key_rotation: true,
                key_backup: true,
                key_recovery: true,
            },
            key_generation: KeyGenerationCapabilities {
                rsa: vec![2048, 3072, 4096],
                ecc: vec![256, 384, 521],
                aes: vec![128, 256],
                supports_secure_random: true,
            },
            security: SecurityCapabilities {
                tamper_resistance: TamperResistance::Tier1,
                fips_140_2_level: Some(2),
                common_criteria_eal: None,
                secure_boot: true,
                attestation: true,
            },
            performance: PerformanceCapabilities {
                max_operations_per_second: 50000,
                typical_latency_ms: 25.0,
                supports_parallel_operations: true,
                hardware_acceleration: true,
            },
            compliance: ComplianceCapabilities {
                fips_140_2: true,
                common_criteria: false,
                pci_dss: true,
                hipaa: true,
                gdpr: true,
            },
            api_support: ApiSupportCapabilities {
                pkcs11: false,
                tpm2: false,
                kmip: false,
                pkcs7: true,
            },
            advanced_features: AdvancedFeatureCapabilities {
                quantum_resistant: false,
                multi_party_computation: false,
                threshold_cryptography: false,
                homomorphic_encryption: false,
            },
            human_entropy: HumanEntropyCapabilities {
                supported: false,
                methods: vec![],
                quality_score: 0.0,
            },
        }
    }

    /// Create Azure Managed HSM capabilities
    fn create_azure_managed_hsm_capabilities(&self) -> UniversalHsmCapabilities {
        use crate::tunnel::hsm::types::capability::*;

        let mut caps = self.create_azure_key_vault_capabilities();
        caps.security.fips_140_2_level = Some(3);
        caps.performance.max_operations_per_second = 100000;
        caps.performance.typical_latency_ms = 15.0;
        caps
    }

    /// Create GCP KMS capabilities
    fn create_gcp_kms_capabilities(&self) -> UniversalHsmCapabilities {
        use crate::tunnel::hsm::types::capability::*;

        UniversalHsmCapabilities {
            crypto_operations: CryptoOperationCapabilities {
                symmetric_encryption: vec!["AES-128".to_string(), "AES-256".to_string()],
                asymmetric_encryption: vec![
                    "RSA-2048".to_string(),
                    "RSA-3072".to_string(),
                    "RSA-4096".to_string(),
                    "ECC-P256".to_string(),
                    "ECC-P384".to_string(),
                ],
                signing: vec![
                    "RSA-PSS".to_string(),
                    "ECDSA".to_string(),
                    "RSA-SIGN-PKCS1".to_string(),
                ],
                hashing: vec!["SHA-256".to_string(), "SHA-384".to_string(), "SHA-512".to_string()],
                key_agreement: vec!["ECDH".to_string()],
            },
            key_management: KeyManagementCapabilities {
                key_generation: true,
                key_storage: true,
                key_rotation: true,
                key_backup: true,
                key_recovery: false,
            },
            key_generation: KeyGenerationCapabilities {
                rsa: vec![2048, 3072, 4096],
                ecc: vec![256, 384],
                aes: vec![128, 256],
                supports_secure_random: true,
            },
            security: SecurityCapabilities {
                tamper_resistance: TamperResistance::Tier1,
                fips_140_2_level: Some(3),
                common_criteria_eal: None,
                secure_boot: true,
                attestation: true,
            },
            performance: PerformanceCapabilities {
                max_operations_per_second: 150000,
                typical_latency_ms: 15.0,
                supports_parallel_operations: true,
                hardware_acceleration: true,
            },
            compliance: ComplianceCapabilities {
                fips_140_2: true,
                common_criteria: false,
                pci_dss: true,
                hipaa: true,
                gdpr: true,
            },
            api_support: ApiSupportCapabilities {
                pkcs11: false,
                tpm2: false,
                kmip: false,
                pkcs7: true,
            },
            advanced_features: AdvancedFeatureCapabilities {
                quantum_resistant: false,
                multi_party_computation: false,
                threshold_cryptography: false,
                homomorphic_encryption: false,
            },
            human_entropy: HumanEntropyCapabilities {
                supported: false,
                methods: vec![],
                quality_score: 0.0,
            },
        }
    }

    /// Create generic cloud HSM capabilities
    fn create_generic_cloud_hsm_capabilities(&self) -> UniversalHsmCapabilities {
        use crate::tunnel::hsm::types::capability::*;

        UniversalHsmCapabilities {
            crypto_operations: CryptoOperationCapabilities {
                symmetric_encryption: vec!["AES-256".to_string()],
                asymmetric_encryption: vec!["RSA-2048".to_string(), "ECC-P256".to_string()],
                signing: vec!["RSA-PSS".to_string(), "ECDSA".to_string()],
                hashing: vec!["SHA-256".to_string()],
                key_agreement: vec!["ECDH".to_string()],
            },
            key_management: KeyManagementCapabilities {
                key_generation: true,
                key_storage: true,
                key_rotation: true,
                key_backup: true,
                key_recovery: false,
            },
            key_generation: KeyGenerationCapabilities {
                rsa: vec![2048],
                ecc: vec![256],
                aes: vec![256],
                supports_secure_random: true,
            },
            security: SecurityCapabilities {
                tamper_resistance: TamperResistance::Tier2,
                fips_140_2_level: Some(2),
                common_criteria_eal: None,
                secure_boot: true,
                attestation: false,
            },
            performance: PerformanceCapabilities {
                max_operations_per_second: 10000,
                typical_latency_ms: 50.0,
                supports_parallel_operations: true,
                hardware_acceleration: true,
            },
            compliance: ComplianceCapabilities {
                fips_140_2: true,
                common_criteria: false,
                pci_dss: false,
                hipaa: false,
                gdpr: true,
            },
            api_support: ApiSupportCapabilities {
                pkcs11: false,
                tpm2: false,
                kmip: false,
                pkcs7: false,
            },
            advanced_features: AdvancedFeatureCapabilities {
                quantum_resistant: false,
                multi_party_computation: false,
                threshold_cryptography: false,
                homomorphic_encryption: false,
            },
            human_entropy: HumanEntropyCapabilities {
                supported: false,
                methods: vec![],
                quality_score: 0.0,
            },
        }
    }
}

impl Default for CloudDiscoverer {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            enable_aws: true,
            enable_azure: true,
            enable_gcp: true,
            enable_other_providers: false,
            custom_configs: Vec::new(),
        })
    }
}

#[cfg(test)]
#[path = "cloud_discoverer_tests.rs"]
mod tests;
