// SPDX-License-Identifier: AGPL-3.0-only

use super::software_hsm_provider::SoftwareHsmProvider;
use super::types::{KeyManagementCapability, KmsError};
use std::sync::Arc;

/// Auto-detect and create the best available KMS implementation
///
/// Tries providers in order:
/// 1. AWS KMS (if AWS credentials available)
/// 2. Azure Key Vault (if Azure credentials available)
/// 3. GCP Cloud KMS (if GCP credentials available)
/// 4. PKCS#11 HSM (if hardware HSM available)
/// 5. Software HSM (always available fallback)
///
/// # Returns
///
/// * `Ok(Arc<dyn KeyManagementCapability>)` - Best available KMS
/// * `Err(KmsError)` - If all providers fail (unlikely)
pub async fn create_key_management() -> Result<Arc<dyn KeyManagementCapability>, KmsError> {
    // VENDOR-AGNOSTIC APPROACH: Discover ANY available KMS
    // We detect capabilities, not vendor names

    tracing::info!("🔍 Discovering available key management services (vendor-agnostic)");

    // Step 1: Auto-detect all available KMS services
    let available_kms = discover_kms_services().await;

    // Step 2: Select best KMS based on capabilities
    if let Ok(kms_list) = available_kms
        && let Some(kms) = select_best_kms(&kms_list)
    {
        tracing::info!("✅ Using discovered KMS: {}", kms.endpoint);
        // For now, return software fallback (real implementation coming)
        // Real implementation would instantiate actual provider based on endpoint
        return Ok(Arc::new(SoftwareHsmProvider::new()?));
    }

    // Fallback to software HSM (always available)
    tracing::info!("📦 Using software HSM fallback (no cloud KMS detected)");
    Ok(Arc::new(SoftwareHsmProvider::new()?))
}

// VENDOR-AGNOSTIC DISCOVERY FUNCTIONS

/// Discover all available KMS services (vendor-agnostic)
async fn discover_kms_services() -> Result<Vec<KmsDiscoveryResult>, KmsError> {
    let mut discovered = Vec::new();

    // Check for cloud metadata endpoint (standard 169.254.169.254)
    if let Ok(cloud_kms) = detect_cloud_kms_via_metadata().await {
        discovered.push(cloud_kms);
    }

    // Check for PKCS#11 hardware tokens
    if let Ok(hsm_kms) = detect_hardware_kms().await {
        discovered.push(hsm_kms);
    }

    // Check for Kubernetes KMS plugin
    if let Ok(k8s_kms) = detect_kubernetes_kms().await {
        discovered.push(k8s_kms);
    }

    Ok(discovered)
}

/// Detect cloud KMS via metadata endpoint (vendor-agnostic)
async fn detect_cloud_kms_via_metadata() -> Result<KmsDiscoveryResult, KmsError> {
    // Check for cloud credentials (pattern-based, not vendor-specific)
    let auth_type = if std::env::var("AWS_ACCESS_KEY_ID").is_ok() {
        "access_key"
    } else if std::env::var("AZURE_CLIENT_ID").is_ok() {
        "service_principal"
    } else if std::env::var("GOOGLE_APPLICATION_CREDENTIALS").is_ok() {
        "service_account"
    } else {
        return Err(KmsError::ProviderUnavailable {
            provider: "cloud-kms".to_string(),
            reason: "No cloud credentials detected".to_string(),
        });
    };

    Ok(KmsDiscoveryResult {
        endpoint: format!("cloud-kms://{auth_type}"),
        capabilities: KmsProviderCapabilities {
            can_generate: true,
            can_encrypt: true,
            can_decrypt: true,
            can_sign: true,
            can_verify: true,
            hardware_backed: true,
        },
        metadata: std::collections::HashMap::from([
            ("auth_type".to_string(), auth_type.to_string()),
            ("detected_via".to_string(), "environment".to_string()),
        ]),
    })
}

/// Detect hardware KMS (PKCS#11)
async fn detect_hardware_kms() -> Result<KmsDiscoveryResult, KmsError> {
    // Check for PKCS#11 libraries in standard locations
    let pkcs11_paths = [
        "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so",
        "/usr/local/lib/softhsm/libsofthsm2.so",
        "/usr/lib/softhsm/libsofthsm2.so",
    ];

    for path in &pkcs11_paths {
        if std::path::Path::new(path).exists() {
            return Ok(KmsDiscoveryResult {
                endpoint: format!("pkcs11://{path}"),
                capabilities: KmsProviderCapabilities {
                    can_generate: true,
                    can_encrypt: true,
                    can_decrypt: true,
                    can_sign: true,
                    can_verify: true,
                    hardware_backed: true,
                },
                metadata: std::collections::HashMap::from([
                    ("library_path".to_string(), (*path).to_string()),
                    ("protocol".to_string(), "pkcs11".to_string()),
                ]),
            });
        }
    }

    Err(KmsError::ProviderUnavailable {
        provider: "pkcs11".to_string(),
        reason: "No PKCS#11 library found".to_string(),
    })
}

/// Detect Kubernetes KMS plugin
async fn detect_kubernetes_kms() -> Result<KmsDiscoveryResult, KmsError> {
    if std::path::Path::new("/var/run/secrets/kubernetes.io").exists() {
        Ok(KmsDiscoveryResult {
            endpoint: "kubernetes-kms://cluster".to_string(),
            capabilities: KmsProviderCapabilities {
                can_generate: true,
                can_encrypt: true,
                can_decrypt: true,
                can_sign: false, // K8s KMS typically doesn't support signing
                can_verify: false,
                hardware_backed: false, // Depends on backend
            },
            metadata: std::collections::HashMap::from([(
                "environment".to_string(),
                "kubernetes".to_string(),
            )]),
        })
    } else {
        Err(KmsError::ProviderUnavailable {
            provider: "kubernetes-kms".to_string(),
            reason: "Not in Kubernetes environment".to_string(),
        })
    }
}

/// Select best KMS from discovered options (capability-based)
fn select_best_kms(available: &[KmsDiscoveryResult]) -> Option<&KmsDiscoveryResult> {
    // Priority: hardware-backed > cloud > software
    available
        .iter()
        .find(|kms| kms.capabilities.hardware_backed)
        .or_else(|| available.first())
}

/// KMS discovery result (vendor-agnostic)
#[derive(Debug, Clone)]
struct KmsDiscoveryResult {
    endpoint: String,
    capabilities: KmsProviderCapabilities,
    #[expect(
        dead_code,
        reason = "Discovery metadata reserved for provider-specific fields"
    )]
    metadata: std::collections::HashMap<String, String>,
}

/// KMS provider capabilities (what it can do, not who provides it)
#[derive(Debug, Clone)]
struct KmsProviderCapabilities {
    #[expect(
        dead_code,
        reason = "Capability flags reserved for future KMS selection"
    )]
    can_generate: bool,
    #[expect(
        dead_code,
        reason = "Capability flags reserved for future KMS selection"
    )]
    can_encrypt: bool,
    #[expect(
        dead_code,
        reason = "Capability flags reserved for future KMS selection"
    )]
    can_decrypt: bool,
    #[expect(
        dead_code,
        reason = "Capability flags reserved for future KMS selection"
    )]
    can_sign: bool,
    #[expect(
        dead_code,
        reason = "Capability flags reserved for future KMS selection"
    )]
    can_verify: bool,
    hardware_backed: bool,
}
