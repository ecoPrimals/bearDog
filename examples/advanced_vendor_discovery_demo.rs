

use beardog_errors::BearDogResult;
use std::collections::HashMap;
use std::env;
use tracing::{info, warn};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityOperation {
    KeyGeneration,
    Encryption,
    Signing,
    Attestation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CryptoOperationType {
    Symmetric,
    Asymmetric,
    Hashing,
    KeyDerivation,
}

#[derive(Debug, Clone)]
pub struct UniversalAdapterConfig {
    pub scan_environment: bool,
    pub scan_hardware: bool,
    pub scan_cloud: bool,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone)]
pub struct UniversalVendorRequest {
    pub operation: CapabilityOperation,
    pub crypto_type: CryptoOperationType,
    pub data: Vec<u8>,
    pub parameters: HashMap<String, String>,
}

pub use beardog_types::canonical::hsm::traits::VendorInfo;

pub struct UniversalVendorAdapter {
    config: UniversalAdapterConfig,
    discovered_vendors: Vec<VendorInfo>,
}

impl UniversalVendorAdapter {
    #[must_use] pub const fn new(config: UniversalAdapterConfig) -> Self {
        Self {
            config,
            discovered_vendors: Vec::new(),
        }
    }

    pub async fn discover_vendors(&mut self) -> BearDogResult<Vec<VendorInfo>> {
        let mut vendors = Vec::new();

        if self.config.scan_environment {
            vendors.extend(self.scan_environment_vendors().await?);
        }

        if self.config.scan_hardware {
            vendors.extend(self.scan_hardware_vendors().await?);
        }

        if self.config.scan_cloud {
            vendors.extend(self.scan_cloud_vendors().await?);
        }

        self.discovered_vendors = vendors.clone();
        Ok(vendors)
    }

    async fn scan_environment_vendors(&self) -> BearDogResult<Vec<VendorInfo>> {
        info!("🔍 Scanning environment variables for vendor configurations...");

        let mut vendors = Vec::new();

        if env::var("PKCS11_LIBRARY").is_ok() {
            vendors.push(VendorInfo {
                name: "PKCS#11 Provider".to_string(),
                version: "detected".to_string(),
                capabilities: vec![
                    CapabilityOperation::KeyGeneration,
                    CapabilityOperation::Signing,
                ],
                crypto_types: vec![
                    CryptoOperationType::Asymmetric,
                    CryptoOperationType::Symmetric,
                ],
                detection_method: "Environment Variable".to_string(),
            });
        }

        if env::var("AWS_KMS_KEY_ID").is_ok() {
            vendors.push(VendorInfo {
                name: "AWS KMS".to_string(),
                version: "cloud".to_string(),
                capabilities: vec![
                    CapabilityOperation::Encryption,
                    CapabilityOperation::KeyGeneration,
                ],
                crypto_types: vec![
                    CryptoOperationType::Symmetric,
                    CryptoOperationType::Asymmetric,
                ],
                detection_method: "Environment Variable".to_string(),
            });
        }

        if env::var("AZURE_KEYVAULT_URL").is_ok() {
            vendors.push(VendorInfo {
                name: "Azure Key Vault".to_string(),
                version: "cloud".to_string(),
                capabilities: vec![
                    CapabilityOperation::Encryption,
                    CapabilityOperation::Attestation,
                ],
                crypto_types: vec![
                    CryptoOperationType::Asymmetric,
                    CryptoOperationType::Hashing,
                ],
                detection_method: "Environment Variable".to_string(),
            });
        }

        info!("   Found {} environment-based vendors", vendors.len());
        Ok(vendors)
    }

    #[allow(clippy::vec_init_then_push)]
    async fn scan_hardware_vendors(&self) -> BearDogResult<Vec<VendorInfo>> {
        info!("🔧 Scanning hardware for HSM and security devices...");

        let mut vendors = Vec::new();

        #[cfg(target_os = "android")]
        {
            vendors.push(VendorInfo {
                name: "Android StrongBox".to_string(),
                version: "hardware".to_string(),
                capabilities: vec![
                    CapabilityOperation::KeyGeneration,
                    CapabilityOperation::Attestation,
                ],
                crypto_types: vec![
                    CryptoOperationType::Asymmetric,
                    CryptoOperationType::Hashing,
                ],
                detection_method: "Hardware Scan".to_string(),
            });
        }

        #[cfg(target_os = "ios")]
        {
            vendors.push(VendorInfo {
                name: "iOS Secure Enclave".to_string(),
                version: "hardware".to_string(),
                capabilities: vec![
                    CapabilityOperation::KeyGeneration,
                    CapabilityOperation::Signing,
                ],
                crypto_types: vec![CryptoOperationType::Asymmetric],
                detection_method: "Hardware Scan".to_string(),
            });
        }

        vendors.push(VendorInfo {
            name: "Rust Crypto Provider".to_string(),
            version: "software".to_string(),
            capabilities: vec![
                CapabilityOperation::KeyGeneration,
                CapabilityOperation::Encryption,
                CapabilityOperation::Signing,
            ],
            crypto_types: vec![
                CryptoOperationType::Symmetric,
                CryptoOperationType::Asymmetric,
                CryptoOperationType::Hashing,
                CryptoOperationType::KeyDerivation,
            ],
            detection_method: "Hardware Scan".to_string(),
        });

        info!("   Found {} hardware-based vendors", vendors.len());
        Ok(vendors)
    }

    async fn scan_cloud_vendors(&self) -> BearDogResult<Vec<VendorInfo>> {
        info!("☁️  Scanning cloud services for available HSM providers...");

        let mut vendors = Vec::new();

        if self.check_aws_availability().await {
            vendors.push(VendorInfo {
                name: "AWS CloudHSM".to_string(),
                version: "cloud".to_string(),
                capabilities: vec![
                    CapabilityOperation::KeyGeneration,
                    CapabilityOperation::Encryption,
                    CapabilityOperation::Signing,
                    CapabilityOperation::Attestation,
                ],
                crypto_types: vec![
                    CryptoOperationType::Symmetric,
                    CryptoOperationType::Asymmetric,
                    CryptoOperationType::Hashing,
                ],
                detection_method: "Cloud Scan".to_string(),
            });
        }

        if self.check_azure_availability().await {
            vendors.push(VendorInfo {
                name: "Azure Dedicated HSM".to_string(),
                version: "cloud".to_string(),
                capabilities: vec![
                    CapabilityOperation::KeyGeneration,
                    CapabilityOperation::Encryption,
                ],
                crypto_types: vec![
                    CryptoOperationType::Asymmetric,
                    CryptoOperationType::Symmetric,
                ],
                detection_method: "Cloud Scan".to_string(),
            });
        }

        info!("   Found {} cloud-based vendors", vendors.len());
        Ok(vendors)
    }

    async fn check_aws_availability(&self) -> bool {

        env::var("AWS_ACCESS_KEY_ID").is_ok() && env::var("AWS_SECRET_ACCESS_KEY").is_ok()
    }

    async fn check_azure_availability(&self) -> bool {

        env::var("AZURE_CLIENT_ID").is_ok() && env::var("AZURE_TENANT_ID").is_ok()
    }

    #[must_use] pub fn get_vendors_by_capability(&self, capability: CapabilityOperation) -> Vec<&VendorInfo> {
        self.discovered_vendors
            .iter()
            .filter(|vendor| vendor.capabilities.contains(&capability))
            .collect()
    }

    #[must_use] pub fn get_vendors_by_crypto_type(&self, crypto_type: CryptoOperationType) -> Vec<&VendorInfo> {
        self.discovered_vendors
            .iter()
            .filter(|vendor| vendor.crypto_types.contains(&crypto_type))
            .collect()
    }
}

#[tokio::main]
async fn main() -> BearDogResult<()> {

    println!("🔧 Initializing BearDog Advanced Vendor Discovery Demo...");

    info!("🚀 BearDog Advanced Vendor Discovery Demo");
    info!("==========================================");

    let config = UniversalAdapterConfig {
        scan_environment: true,
        scan_hardware: true,
        scan_cloud: true,
        timeout_seconds: 30,
    };

    let mut adapter = UniversalVendorAdapter::new(config);

    demonstrate_comprehensive_discovery(&mut adapter).await?;

    demonstrate_capability_filtering(&adapter).await?;

    demonstrate_crypto_type_filtering(&adapter).await?;

    demonstrate_vendor_selection(&adapter).await?;

    info!("✅ Advanced Vendor Discovery Demo completed successfully!");
    Ok(())
}

async fn demonstrate_comprehensive_discovery(
    adapter: &mut UniversalVendorAdapter,
) -> BearDogResult<()> {
    info!("\n📊 Phase 1: Comprehensive Vendor Discovery");
    info!("------------------------------------------");

    let vendors = adapter.discover_vendors().await?;

    info!("🔍 Discovery Summary:");
    info!("   Total vendors found: {}", vendors.len());

    for (index, vendor) in vendors.iter().enumerate() {
        info!("   {}. {} ({})", index + 1, vendor.name, vendor.version);
        info!("      Detection: {}", vendor.detection_method);
        info!("      Capabilities: {:?}", vendor.capabilities);
        info!("      Crypto Types: {:?}", vendor.crypto_types);
    }

    Ok(())
}

async fn demonstrate_capability_filtering(adapter: &UniversalVendorAdapter) -> BearDogResult<()> {
    info!("\n🎯 Phase 2: Capability-Based Filtering");
    info!("--------------------------------------");

    let capabilities = vec![
        CapabilityOperation::KeyGeneration,
        CapabilityOperation::Encryption,
        CapabilityOperation::Signing,
        CapabilityOperation::Attestation,
    ];

    for capability in capabilities {
        let matching_vendors = adapter.get_vendors_by_capability(capability.clone());
        info!(
            "🔑 {:?} capable vendors: {}",
            capability,
            matching_vendors.len()
        );

        for vendor in matching_vendors {
            info!("   - {} ({})", vendor.name, vendor.detection_method);
        }
    }

    Ok(())
}

async fn demonstrate_crypto_type_filtering(adapter: &UniversalVendorAdapter) -> BearDogResult<()> {
    info!("\n🔐 Phase 3: Crypto-Type Filtering");
    info!("----------------------------------");

    let crypto_types = vec![
        CryptoOperationType::Symmetric,
        CryptoOperationType::Asymmetric,
        CryptoOperationType::Hashing,
        CryptoOperationType::KeyDerivation,
    ];

    for crypto_type in crypto_types {
        let matching_vendors = adapter.get_vendors_by_crypto_type(crypto_type.clone());
        info!(
            "🔒 {:?} supporting vendors: {}",
            crypto_type,
            matching_vendors.len()
        );

        for vendor in matching_vendors {
            info!("   - {} ({})", vendor.name, vendor.detection_method);
        }
    }

    Ok(())
}

async fn demonstrate_vendor_selection(adapter: &UniversalVendorAdapter) -> BearDogResult<()> {
    info!("\n⚡ Phase 4: Intelligent Vendor Selection");
    info!("---------------------------------------");

    let scenarios = vec![
        (
            "High Security Key Generation",
            CapabilityOperation::KeyGeneration,
        ),
        (
            "Fast Encryption Operations",
            CapabilityOperation::Encryption,
        ),
        ("Digital Signatures", CapabilityOperation::Signing),
        ("Hardware Attestation", CapabilityOperation::Attestation),
    ];

    for (scenario, capability) in scenarios {
        info!("📋 Scenario: {}", scenario);

        let candidates = adapter.get_vendors_by_capability(capability);
        if candidates.is_empty() {
            warn!("   ⚠️  No vendors available for this capability");
            continue;
        }

        let selected = select_best_vendor(&candidates)?;
        info!(
            "   ✅ Selected: {} ({})",
            selected.name, selected.detection_method
        );
        info!("   📊 Reasoning: {}", get_selection_reasoning(selected));
    }

    Ok(())
}

fn select_best_vendor<'a>(
    candidates: &'a [&'a VendorInfo],
) -> Result<&'a VendorInfo, beardog_errors::BearDogError> {

    candidates
        .iter()
        .min_by_key(|vendor| match vendor.detection_method.as_str() {
            "Hardware Scan" => 0,
            "Cloud Scan" => 1,
            "Environment Variable" => 2,
            _ => 3,
        })
        .copied() // Convert &&VendorInfo to &VendorInfo
        .ok_or_else(|| {
            tracing::error!("No vendors available for selection");
            beardog_errors::BearDogError::internal("No vendors available for selection".to_string())
        })
}

fn get_selection_reasoning(vendor: &VendorInfo) -> String {
    match vendor.detection_method.as_str() {
        "Hardware Scan" => "Hardware-based security provides highest trust level".to_string(),
        "Cloud Scan" => "Cloud HSM offers enterprise-grade security with scalability".to_string(),
        "Environment Variable" => "Configured provider offers flexibility and control".to_string(),
        _ => "Software fallback ensures operation continuity".to_string(),
    }
}
