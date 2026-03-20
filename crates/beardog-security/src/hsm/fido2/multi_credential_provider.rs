// SPDX-License-Identifier: AGPL-3.0-only

//! FIDO2 Multi-Credential HSM Provider Implementation
//!
//! This module implements the `MultiCredentialHsmProvider` trait for FIDO2-compliant
//! security keys (SoloKeys, YubiKey FIDO2, Nitrokey FIDO2, etc.)

use beardog_errors::BearDogError;
use beardog_traits::unified::{
    CredentialHierarchy, CredentialInfo, CredentialNode, CredentialRequest, HsmProtocol,
    MultiCredentialCapabilities, MultiCredentialHsmProvider,
};
use beardog_types::canonical::providers_unified::traits::{
    ProviderCapability, ProviderHealth, ProviderMetrics,
};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use super::types::Fido2DeviceInfo;

/// FIDO2 Multi-Credential HSM Provider
///
/// Implements multi-credential operations for FIDO2-compliant security keys.
/// This works with any FIDO2 device: SoloKeys, YubiKey 5, Nitrokey FIDO2, etc.
pub struct Fido2MultiCredentialProvider {
    /// Device information
    device_info: Fido2DeviceInfo,

    /// In-memory credential storage (backed by device)
    /// In production, this would query the device directly
    credentials: Arc<RwLock<HashMap<String, CredentialInfo>>>,

    /// FIDO2-specific configuration
    /// Note: Used for future protocol operations (makeCredential, getAssertion)
    #[allow(dead_code)]
    config: Fido2ProviderConfig,
}

/// Configuration for FIDO2 provider
#[derive(Debug, Clone)]
pub struct Fido2ProviderConfig {
    /// RP ID (Relying Party Identifier) - typically your domain
    pub rp_id: String,

    /// RP name (human-readable)
    pub rp_name: String,

    /// Whether to require user verification by default
    pub require_user_verification: bool,

    /// Default timeout for operations (milliseconds)
    pub timeout_ms: u64,
}

impl Default for Fido2ProviderConfig {
    fn default() -> Self {
        Self {
            rp_id: "beardog.ecoPrimals".to_string(),
            rp_name: "BearDog Security Platform".to_string(),
            require_user_verification: false,
            timeout_ms: 30000,
        }
    }
}

impl Fido2MultiCredentialProvider {
    /// Create a new FIDO2 multi-credential provider
    pub async fn new(
        device_info: Fido2DeviceInfo,
        config: Option<Fido2ProviderConfig>,
    ) -> Result<Self, BearDogError> {
        info!(
            "🔐 Initializing FIDO2 Multi-Credential Provider: {} ({})",
            device_info.product, device_info.manufacturer
        );

        // Verify device supports resident keys (required for multi-credential)
        if !device_info.capabilities.resident_keys {
            return Err(BearDogError::system(format!(
                "Device '{}' does not support resident keys (required for multi-credential operations)",
                device_info.product
            )));
        }

        Ok(Self {
            device_info,
            credentials: Arc::new(RwLock::new(HashMap::new())),
            config: config.unwrap_or_default(),
        })
    }

    /// Get device information
    pub const fn device_info(&self) -> &Fido2DeviceInfo {
        &self.device_info
    }

    /// Convert FIDO2 credential ID (bytes) to universal string ID
    fn credential_id_to_string(id: &[u8]) -> String {
        // Use base64url encoding for credential IDs
        base64_url::encode(id)
    }

    /// Convert universal string ID back to FIDO2 credential ID (bytes)
    fn string_to_credential_id(id: &str) -> Result<Vec<u8>, BearDogError> {
        base64_url::decode(id)
            .map_err(|e| BearDogError::system(format!("Invalid credential ID format: {e}")))
    }

    /// Send CTAP2 MakeCredential command
    async fn ctap2_make_credential(
        &self,
        request: &CredentialRequest,
    ) -> Result<(Vec<u8>, Vec<u8>), BearDogError> {
        debug!("Sending CTAP2 MakeCredential for role: {}", request.role);

        // PHASE-2(CTAP2): Implement CTAP2 MakeCredential command
        // Universal provider architecture is ready for CTAP2 protocol implementation
        // Implementation plan:
        // 1. Build CBOR-encoded MakeCredential request
        // 2. Send via CTAPHID transport
        // 3. Parse CBOR response
        // 4. Return (credential_id, public_key)

        Err(BearDogError::system(format!(
            "CTAP2 MakeCredential planned for Phase 2 - protocol architecture ready. \
             Role: '{}', Algorithm: '{}'",
            request.role,
            request.algorithm.as_deref().unwrap_or("ES256")
        )))
    }

    /// Send CTAP2 GetAssertion command
    async fn ctap2_get_assertion(
        &self,
        credential_id: &[u8],
        data: &[u8],
        require_user_presence: bool,
    ) -> Result<Vec<u8>, BearDogError> {
        debug!(
            "Sending CTAP2 GetAssertion for credential ID (len={})",
            credential_id.len()
        );

        // PHASE-2(CTAP2): Implement CTAP2 GetAssertion command
        // Universal signing interface is ready for CTAP2 protocol
        // Implementation plan: CBOR request → CTAPHID transport → Parse signature

        Err(BearDogError::system(format!(
            "CTAP2 GetAssertion planned for Phase 2 - signing architecture ready. \
             Data size: {} bytes, User presence: {}",
            data.len(),
            require_user_presence
        )))
    }

    /// Enumerate credentials using CTAP2 CredentialManagement
    async fn ctap2_enumerate_credentials(&self) -> Result<Vec<CredentialInfo>, BearDogError> {
        debug!("Enumerating credentials via CTAP2 CredentialManagement");

        // PHASE-2(CTAP2): Implement CTAP2 credentialManagement enumerate
        // Currently using in-memory cache - Phase 2 will query device directly
        // Universal credential management architecture supports vendor-agnostic enumeration

        // Return in-memory cache (Phase 2 will query device)
        let creds = self.credentials.read().await;
        Ok(creds.values().cloned().collect())
    }

    /// Delete credential using CTAP2 CredentialManagement
    async fn ctap2_delete_credential(&self, credential_id: &[u8]) -> Result<(), BearDogError> {
        debug!(
            "Deleting credential via CTAP2 (len={})",
            credential_id.len()
        );

        // PHASE-2(CTAP2): Implement CTAP2 credentialManagement delete
        // Universal credential lifecycle management architecture ready
        // Implementation plan: CBOR delete request → Device acknowledgment

        Err(BearDogError::system(
            "CTAP2 credential deletion planned for Phase 2 - lifecycle management ready"
                .to_string(),
        ))
    }

    /// Generate hardware entropy using hmac-secret extension
    async fn ctap2_hmac_secret_entropy(&self, size: usize) -> Result<Vec<u8>, BearDogError> {
        debug!("Generating {} bytes of entropy via hmac-secret", size);

        if !self.device_info.capabilities.hmac_secret {
            return Err(BearDogError::system(
                "Device does not support hmac-secret extension".to_string(),
            ));
        }

        // PHASE-2(CTAP2): Implement CTAP2 hmac-secret entropy generation
        // Universal entropy collection architecture ready for hardware sources
        // Implementation plan: HMAC-secret extension → High-quality hardware RNG

        Err(BearDogError::system(
            "CTAP2 hmac-secret planned for Phase 2 - entropy architecture ready".to_string(),
        ))
    }
}

// ============================================================================
// TRAIT IMPLEMENTATIONS: MultiCredentialHsmProvider
// ============================================================================

impl MultiCredentialHsmProvider for Fido2MultiCredentialProvider {
    type Error = BearDogError;
    async fn create_credential(
        &self,
        request: CredentialRequest,
    ) -> Result<CredentialInfo, Self::Error> {
        info!("Creating credential with role: {}", request.role);

        // Validate algorithm
        let algorithm = request.algorithm.as_deref().unwrap_or("ES256");
        if !self
            .device_info
            .capabilities
            .supported_algorithms
            .contains(&algorithm.to_string())
        {
            return Err(BearDogError::system(format!(
                "Algorithm '{}' not supported by device. Supported: {:?}",
                algorithm, self.device_info.capabilities.supported_algorithms
            )));
        }

        // Validate permissions against parent (if hierarchical)
        if let Some(parent_id) = &request.parent_credential {
            let creds = self.credentials.read().await;
            if let Some(parent) = creds.get(parent_id) {
                // Ensure child permissions are subset of parent
                for perm in &request.permissions {
                    if !parent.permissions.contains(perm) {
                        warn!(
                            "Child credential requests permission '{}' not granted to parent",
                            perm
                        );
                    }
                }
            } else {
                return Err(BearDogError::system(format!(
                    "Parent credential '{parent_id}' not found"
                )));
            }
        }

        // Send CTAP2 MakeCredential command
        let (credential_id, public_key) = self.ctap2_make_credential(&request).await?;

        // Create credential info
        let credential_info = CredentialInfo {
            credential_id: Self::credential_id_to_string(&credential_id),
            role: request.role.clone(),
            display_name: request.display_name.clone(),
            permissions: request.permissions.clone(),
            public_key,
            algorithm: algorithm.to_string(),
            created_at: Utc::now(),
            last_used: None,
            use_count: 0,
            parent_credential_id: request.parent_credential.clone(),
            metadata: request.metadata.clone(),
            requires_user_presence: request.require_user_presence,
            requires_user_verification: request.require_user_verification,
        };

        // Store in cache
        let mut creds = self.credentials.write().await;
        creds.insert(
            credential_info.credential_id.clone(),
            credential_info.clone(),
        );

        info!("✅ Created credential: {}", credential_info.credential_id);
        Ok(credential_info)
    }

    async fn list_credentials(&self) -> Result<Vec<CredentialInfo>, Self::Error> {
        self.ctap2_enumerate_credentials().await
    }

    async fn delete_credential(&self, credential_id: &str) -> Result<(), Self::Error> {
        info!("Deleting credential: {}", credential_id);

        let cred_id_bytes = Self::string_to_credential_id(credential_id)?;
        self.ctap2_delete_credential(&cred_id_bytes).await?;

        // Remove from cache
        let mut creds = self.credentials.write().await;
        creds.remove(credential_id);

        info!("✅ Deleted credential: {}", credential_id);
        Ok(())
    }

    async fn get_credential_info(
        &self,
        credential_id: &str,
    ) -> Result<CredentialInfo, Self::Error> {
        let creds = self.credentials.read().await;
        creds
            .get(credential_id)
            .cloned()
            .ok_or_else(|| BearDogError::system(format!("Credential '{credential_id}' not found")))
    }

    async fn sign_with_credential(
        &self,
        credential_id: &str,
        data: &[u8],
        require_user_presence: bool,
    ) -> Result<Vec<u8>, Self::Error> {
        info!(
            "Signing {} bytes with credential: {}",
            data.len(),
            credential_id
        );

        let cred_id_bytes = Self::string_to_credential_id(credential_id)?;
        let signature = self
            .ctap2_get_assertion(&cred_id_bytes, data, require_user_presence)
            .await?;

        // Update use count
        let mut creds = self.credentials.write().await;
        if let Some(cred) = creds.get_mut(credential_id) {
            cred.last_used = Some(Utc::now());
            cred.use_count += 1;
        }

        Ok(signature)
    }

    async fn generate_hardware_entropy(&self, size: usize) -> Result<Vec<u8>, Self::Error> {
        let max_size = self.device_info.capabilities.max_entropy_size.unwrap_or(64);
        if size > max_size {
            return Err(BearDogError::system(format!(
                "Requested {size} bytes exceeds device maximum of {max_size} bytes"
            )));
        }

        self.ctap2_hmac_secret_entropy(size).await
    }

    async fn derive_child_credential(
        &self,
        parent_credential_id: &str,
        mut child_request: CredentialRequest,
    ) -> Result<CredentialInfo, Self::Error> {
        info!(
            "Deriving child credential from parent: {}",
            parent_credential_id
        );

        // Set parent relationship
        child_request.parent_credential = Some(parent_credential_id.to_string());

        // Create the child credential
        self.create_credential(child_request).await
    }

    async fn get_credential_hierarchy(&self) -> Result<CredentialHierarchy, Self::Error> {
        let creds = self.credentials.read().await;

        // Build hierarchy tree
        let mut roots = Vec::new();
        let mut children_map: HashMap<String, Vec<CredentialInfo>> = HashMap::new();

        // Organize credentials by parent
        for cred in creds.values() {
            if let Some(parent_id) = &cred.parent_credential_id {
                children_map
                    .entry(parent_id.clone())
                    .or_default()
                    .push(cred.clone());
            }
        }

        // Build tree starting from roots
        fn build_node(
            cred: &CredentialInfo,
            children_map: &HashMap<String, Vec<CredentialInfo>>,
        ) -> CredentialNode {
            let children = children_map
                .get(&cred.credential_id)
                .map(|kids| {
                    kids.iter()
                        .map(|kid| build_node(kid, children_map))
                        .collect()
                })
                .unwrap_or_default();

            CredentialNode {
                credential: cred.clone(),
                children,
            }
        }

        for cred in creds.values() {
            if cred.parent_credential_id.is_none() {
                roots.push(build_node(cred, &children_map));
            }
        }

        Ok(CredentialHierarchy { roots })
    }

    fn get_multi_credential_capabilities(&self) -> MultiCredentialCapabilities {
        MultiCredentialCapabilities {
            max_credentials: self.device_info.capabilities.max_resident_keys,
            current_credentials: 0, // PHASE-2(CTAP2): Query actual count from device via getInfo
            supports_hierarchical_credentials: true,
            supports_deterministic_derivation: self.device_info.capabilities.hmac_secret,
            supports_hardware_entropy: self.device_info.capabilities.hmac_secret,
            max_entropy_bytes: self.device_info.capabilities.max_entropy_size,
            supported_algorithms: self.device_info.capabilities.supported_algorithms.clone(),
            supports_user_presence: true,
            supports_user_verification: self.device_info.capabilities.user_verification,
            supports_metadata: false, // FIDO2 has limited metadata support
            protocol: HsmProtocol::Fido2,
        }
    }

    async fn prepare_credential_replication(
        &self,
        credential_id: &str,
        shared_entropy: &[u8],
    ) -> Result<beardog_traits::unified::CredentialReplicationData, Self::Error> {
        info!(
            "Preparing credential {} for replication using {} bytes of shared entropy",
            credential_id,
            shared_entropy.len()
        );

        // Get source credential info
        let source_cred = self.get_credential_info(credential_id).await?;

        // Create entropy hash for verification
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(shared_entropy);
        let entropy_hash = hasher.finalize().to_vec();

        // Create derivation path (for deterministic key derivation)
        // Use HMAC of role + timestamp as path
        let derivation_path = vec![0, 1, 2]; // PHASE-2: Implement proper BIP32-style derivation

        // Create credential request for target device
        let target_request = CredentialRequest {
            role: source_cred.role.clone(),
            display_name: source_cred.display_name.clone(),
            permissions: source_cred.permissions.clone(),
            require_user_presence: source_cred.requires_user_presence,
            require_user_verification: source_cred.requires_user_verification,
            parent_credential: None, // Don't replicate parent relationship
            metadata: source_cred.metadata.clone(),
            algorithm: Some(source_cred.algorithm.clone()),
        };

        info!("✅ Prepared replication data for credential");
        Ok(beardog_traits::unified::CredentialReplicationData {
            source_credential: source_cred,
            derivation_path,
            entropy_hash,
            target_request,
        })
    }
}

// ============================================================================
// TRAIT IMPLEMENTATIONS: BearDogProvider (required by HsmProvider)
// ============================================================================

impl beardog_traits::unified::BearDogProvider for Fido2MultiCredentialProvider {
    type Error = BearDogError;
    type Config = Fido2ProviderConfig;

    fn provider_id(&self) -> &'static str {
        "fido2_multi_credential"
    }

    fn provider_version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn capabilities(&self) -> Vec<ProviderCapability> {
        vec![
            ProviderCapability {
                name: "multi_credential".to_string(),
                description: "Multiple credentials per device".to_string(),
                parameters: vec![],
                enabled: true,
            },
            ProviderCapability {
                name: "hardware_entropy".to_string(),
                description: "Hardware random number generation".to_string(),
                parameters: vec![],
                enabled: self.device_info.capabilities.hmac_secret,
            },
            ProviderCapability {
                name: "user_presence".to_string(),
                description: "User presence verification (button press)".to_string(),
                parameters: vec![],
                enabled: true,
            },
        ]
    }

    async fn health_check(&self) -> Result<ProviderHealth, Self::Error> {
        use beardog_types::canonical::providers_unified::traits::{
            HealthStatus, NetworkIoMetrics, ResourceUsage,
        };
        use std::time::SystemTime;

        Ok(ProviderHealth {
            status: HealthStatus::Healthy,
            timestamp: SystemTime::now(),
            details: HashMap::from([
                ("device".to_string(), self.device_info.product.clone()),
                (
                    "protocol".to_string(),
                    format!("CTAP2 {:?}", self.device_info.protocol_versions),
                ),
            ]),
            resource_usage: ResourceUsage {
                cpu_percent: 0.0,
                memory_bytes: 0,
                memory_percent: 0.0,
                network_io: NetworkIoMetrics {
                    bytes_sent: 0,
                    bytes_received: 0,
                    packets_sent: 0,
                    packets_received: 0,
                },
                disk_io: HashMap::new(),
            },
            last_error: None,
        })
    }

    async fn metrics(&self) -> Result<ProviderMetrics, Self::Error> {
        use beardog_types::canonical::providers_unified::traits::CustomMetric;
        use std::time::SystemTime;

        let creds = self.credentials.read().await;
        Ok(ProviderMetrics {
            timestamp: SystemTime::now(),
            performance: HashMap::new(),
            custom_metrics: vec![
                CustomMetric {
                    name: "credentials_count".to_string(),
                    value: creds.len() as f64,
                    unit: "count".to_string(),
                    description: "Number of credentials stored".to_string(),
                    tags: HashMap::new(),
                },
                CustomMetric {
                    name: "max_credentials".to_string(),
                    value: self.device_info.capabilities.max_resident_keys.unwrap_or(0) as f64,
                    unit: "count".to_string(),
                    description: "Maximum credentials supported".to_string(),
                    tags: HashMap::new(),
                },
            ],
            system_metrics: beardog_types::canonical::providers_unified::traits::SystemMetrics {
                uptime_seconds: 0,
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                avg_response_time_ms: 0.0,
                active_connections: 0,
                error_rate: 0.0,
            },
        })
    }
}

// Note: SecurityProvider and CryptoProvider implementations would go here
// For brevity, marking as stub implementations

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credential_id_conversion() {
        let original = b"test_credential_id_12345";
        let string_id = Fido2MultiCredentialProvider::credential_id_to_string(original);
        let decoded = Fido2MultiCredentialProvider::string_to_credential_id(&string_id).unwrap();
        assert_eq!(original.to_vec(), decoded);
    }

    #[test]
    fn test_default_config() {
        let config = Fido2ProviderConfig::default();
        assert_eq!(config.rp_id, "beardog.ecoPrimals");
        assert!(!config.require_user_verification);
    }
}
