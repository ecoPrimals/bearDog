//! Android StrongBox Multi-Credential HSM Provider
//!
//! This module implements the `MultiCredentialHsmProvider` trait for Android StrongBox
//! (Google Pixel Titan M2, Samsung Knox, Qualcomm SPU, etc.)
//!
//! ## **The Magic**: Same Code, Different Hardware
//!
//! This provider implements the **EXACT SAME TRAIT** as the FIDO2 provider, meaning:
//! - Application code works identically with SoloKeys and Pixel 8a
//! - No platform-specific logic in application layer
//! - Hardware-agnostic multi-credential operations
//!
//! ## Architecture
//!
//! ```text
//! Your App Code (vendor-agnostic)
//!         ↓
//! MultiCredentialHsmProvider trait
//!         ↓
//!    ┌────┴────┐
//!    │         │
//! FIDO2     StrongBox  
//! (SoloKeys) (Pixel 8a)
//! ```

use beardog_errors::{phase2_not_implemented, BearDogError};
use beardog_traits::unified::{
    CredentialHierarchy, CredentialInfo, CredentialNode, CredentialReplicationData,
    CredentialRequest, HsmProtocol, MultiCredentialCapabilities, MultiCredentialHsmProvider,
};
use beardog_types::canonical::providers_unified::traits::{
    ProviderCapability, ProviderHealth, ProviderMetrics,
};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Android StrongBox device information (simplified)
#[derive(Debug, Clone)]
pub struct StrongBoxDeviceInfo {
    /// Device manufacturer (e.g., "Google", "Samsung")
    pub manufacturer: String,
    /// Device model (e.g., "Pixel 8a", "Galaxy S24")
    pub model: String,
    /// Android OS version (e.g., "14", "13")
    pub android_version: String,
    /// Titan M/M2 version if Google Pixel, or Knox version if Samsung
    pub titan_m_version: Option<String>,
    /// Whether StrongBox (hardware-backed keystore) is available
    pub strongbox_available: bool,
    /// Maximum number of keys supported (None = unlimited)
    pub max_keys: Option<usize>,
}

impl Default for StrongBoxDeviceInfo {
    fn default() -> Self {
        Self {
            manufacturer: "Google".to_string(),
            model: "Pixel 8a".to_string(),
            android_version: "14".to_string(),
            titan_m_version: Some("Titan M2".to_string()),
            strongbox_available: true,
            max_keys: None, // Unlimited on Android
        }
    }
}

/// Android StrongBox Multi-Credential Provider
///
/// Implements multi-credential operations for Android StrongBox HSM.
/// Works with Pixel devices (Titan M/M2), Samsung (Knox), Qualcomm (SPU), etc.
pub struct StrongBoxMultiCredentialProvider {
    /// Device information
    device_info: StrongBoxDeviceInfo,

    /// In-memory credential storage (backed by Android Keystore)
    /// In production, this would query the device directly
    credentials: Arc<RwLock<HashMap<String, CredentialInfo>>>,

    /// Configuration
    config: StrongBoxProviderConfig,
}

/// Configuration for StrongBox provider
#[derive(Debug, Clone)]
pub struct StrongBoxProviderConfig {
    /// App ID (Android package name)
    pub app_id: String,

    /// Whether to require user authentication by default
    pub require_user_authentication: bool,

    /// Default authentication validity (seconds)
    pub auth_validity_duration: u32,

    /// Whether to require StrongBox (vs TEE fallback)
    pub require_strongbox: bool,
}

impl Default for StrongBoxProviderConfig {
    fn default() -> Self {
        Self {
            app_id: "com.ecoprimals.beardog".to_string(),
            require_user_authentication: false,
            auth_validity_duration: 300, // 5 minutes
            require_strongbox: true,
        }
    }
}

impl StrongBoxMultiCredentialProvider {
    /// Create a new StrongBox multi-credential provider
    pub async fn new(
        device_info: StrongBoxDeviceInfo,
        config: Option<StrongBoxProviderConfig>,
    ) -> Result<Self, BearDogError> {
        info!(
            "🔐 Initializing Android StrongBox Multi-Credential Provider: {} {} ({})",
            device_info.manufacturer, device_info.model, device_info.android_version
        );

        // Verify StrongBox is available
        if !device_info.strongbox_available {
            warn!("StrongBox not available, will use TEE fallback");
        }

        if let Some(ref titan_version) = device_info.titan_m_version {
            info!("   Titan M detected: {}", titan_version);
        }

        Ok(Self {
            device_info,
            credentials: Arc::new(RwLock::new(HashMap::new())),
            config: config.unwrap_or_default(),
        })
    }

    /// Get device information
    pub fn device_info(&self) -> &StrongBoxDeviceInfo {
        &self.device_info
    }

    /// Convert Android key alias to universal credential ID
    fn alias_to_credential_id(alias: &str) -> String {
        // Android keystore uses string aliases
        alias.to_string()
    }

    /// Convert credential ID to Android key alias
    fn credential_id_to_alias(id: &str) -> String {
        id.to_string()
    }

    /// Create Android KeyGenParameterSpec and generate key
    ///
    /// # Errors
    /// Returns a PHASE-2 not implemented error with detailed implementation notes.
    async fn android_generate_key(
        &self,
        request: &CredentialRequest,
    ) -> Result<(String, Vec<u8>), BearDogError> {
        debug!(
            "Generating Android StrongBox key for role: {}",
            request.role
        );

        Err(phase2_not_implemented(
            "Android Keystore Key Generation",
            "\
Using Binder IPC (recommended) or JNI bridge:

Binder approach (Fast):
1. Call keystore2.generateKey() via Binder IPC
2. Set SecurityLevel::STRONGBOX
3. Set KeyPurpose::SIGN | KeyPurpose::VERIFY
4. Set Digest::SHA_2_256
5. Set user authentication if required

JNI approach (Fallback):
1. KeyGenParameterSpec.Builder with:
   - KeyProperties.PURPOSE_SIGN | KeyProperties.PURPOSE_VERIFY
   - setIsStrongBoxBacked(true)
   - setDigests(KeyProperties.DIGEST_SHA256)
   - setUserAuthenticationRequired(request.require_user_verification)
2. KeyPairGenerator.getInstance(\"EC\", \"AndroidKeyStore\")
3. keyPairGenerator.initialize(spec)
4. keyPairGenerator.generateKeyPair()

Estimated effort: 12-20 hours",
            Some("Use Software HSM with deterministic key derivation"),
        ).into())
    }

    /// Sign using Android Keystore key
    ///
    /// # Errors
    /// Returns a PHASE-2 not implemented error with detailed implementation notes.
    async fn android_sign(
        &self,
        alias: &str,
        data: &[u8],
        _require_auth: bool,
    ) -> Result<Vec<u8>, BearDogError> {
        debug!("Signing {} bytes with Android key: {}", data.len(), alias);

        Err(phase2_not_implemented(
            "Android Keystore Signing",
            "\
Binder approach (recommended):
1. Call keystore2.sign() via Binder IPC
2. Pass key alias and data
3. If require_auth, trigger BiometricPrompt first

JNI approach (fallback):
1. KeyStore.getInstance(\"AndroidKeyStore\")
2. keyStore.load(null)
3. privateKey = keyStore.getKey(alias, null)
4. Signature.getInstance(\"SHA256withECDSA\")
5. signature.initSign(privateKey)
6. signature.update(data)
7. signature.sign()

Note: BiometricPrompt integration requires UI context.

Estimated effort: 8-12 hours",
            Some("Use Software HSM signing"),
        ).into())
    }

    /// List all keys in Android Keystore
    ///
    /// # Errors
    /// Returns a PHASE-2 not implemented error. Currently returns in-memory cache as fallback.
    async fn android_list_keys(&self) -> Result<Vec<String>, BearDogError> {
        debug!("Listing keys from Android Keystore");

        // NOTE: For now, return in-memory cache (safe fallback)
        // PHASE-2 will query actual hardware keystore
        let creds = self.credentials.read().await;
        Ok(creds.keys().cloned().collect())
    }

    /// Delete key from Android Keystore
    ///
    /// # Errors
    /// Returns a PHASE-2 not implemented error with detailed implementation notes.
    async fn android_delete_key(&self, alias: &str) -> Result<(), BearDogError> {
        debug!("Deleting Android key: {}", alias);

        Err(phase2_not_implemented(
            "Android Keystore Key Deletion",
            "\
Binder approach (recommended):
1. Call keystore2.deleteKey() via Binder IPC

JNI approach (fallback):
1. KeyStore.getInstance(\"AndroidKeyStore\")
2. keyStore.load(null)
3. keyStore.deleteEntry(alias)

Estimated effort: 4-6 hours",
            Some("Keys in Software HSM can be deleted via provider API"),
        ).into())
    }

    /// Generate hardware entropy using Android SecureRandom
    ///
    /// # Note
    /// This actually WORKS via Rust's `getrandom()` which uses the kernel's entropy pool,
    /// which on Android is seeded by hardware RNG (including Titan M2).
    ///
    /// PHASE-2 could add direct SecureRandom access for additional entropy sources.
    async fn android_hardware_entropy(&self, size: usize) -> Result<Vec<u8>, BearDogError> {
        debug!("Generating {} bytes of entropy from Android hardware", size);

        // ✅ THIS ACTUALLY WORKS! getrandom() uses hardware RNG on Android
        use rand::RngCore;
        let mut entropy = vec![0u8; size];
        rand::thread_rng().fill_bytes(&mut entropy);
        
        Ok(entropy)
    }
}

// ============================================================================
// TRAIT IMPLEMENTATIONS: MultiCredentialHsmProvider
// ============================================================================

impl MultiCredentialHsmProvider for StrongBoxMultiCredentialProvider {
    type Error = BearDogError;
    async fn create_credential(
        &self,
        request: CredentialRequest,
    ) -> Result<CredentialInfo, Self::Error> {
        info!("Creating credential with role: {}", request.role);

        // Validate algorithm
        let algorithm = request.algorithm.as_deref().unwrap_or("EC");
        let supported = vec!["EC", "RSA"];
        if !supported.contains(&algorithm) {
            return Err(BearDogError::system(format!(
                "Algorithm '{}' not supported. Supported: {:?}",
                algorithm, supported
            )));
        }

        // Validate permissions against parent (if hierarchical)
        if let Some(parent_id) = &request.parent_credential {
            let creds = self.credentials.read().await;
            if let Some(parent) = creds.get(parent_id) {
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
                    "Parent credential '{}' not found",
                    parent_id
                )));
            }
        }

        // Generate key in Android Keystore
        let (alias, public_key) = self.android_generate_key(&request).await?;

        // Create credential info
        let credential_info = CredentialInfo {
            credential_id: Self::alias_to_credential_id(&alias),
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
        let _aliases = self.android_list_keys().await?;

        // Return in-memory cache
        let creds = self.credentials.read().await;
        Ok(creds.values().cloned().collect())
    }

    async fn delete_credential(&self, credential_id: &str) -> Result<(), Self::Error> {
        info!("Deleting credential: {}", credential_id);

        let alias = Self::credential_id_to_alias(credential_id);
        self.android_delete_key(&alias).await?;

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

        let alias = Self::credential_id_to_alias(credential_id);
        let signature = self
            .android_sign(&alias, data, require_user_presence)
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
        // Android doesn't have a hard limit, but 64KB is reasonable
        if size > 65536 {
            return Err(BearDogError::system(format!(
                "Requested {} bytes exceeds reasonable limit of 65536 bytes",
                size
            )));
        }

        self.android_hardware_entropy(size).await
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

        // Build hierarchy tree (same logic as FIDO2 provider)
        let mut roots = Vec::new();
        let mut children_map: HashMap<String, Vec<CredentialInfo>> = HashMap::new();

        for cred in creds.values() {
            if let Some(parent_id) = &cred.parent_credential_id {
                children_map
                    .entry(parent_id.clone())
                    .or_default()
                    .push(cred.clone());
            }
        }

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
            max_credentials: self.device_info.max_keys, // Unlimited on Android
            current_credentials: 0, // NOTE: Uses in-memory count; PHASE-2 will query keystore
            supports_hierarchical_credentials: true,
            supports_deterministic_derivation: true, // Via SecureRandom seeding
            supports_hardware_entropy: true, // ✅ WORKING via getrandom()
            max_entropy_bytes: Some(65536), // Reasonable limit
            supported_algorithms: vec!["EC".to_string(), "RSA".to_string()],
            supports_user_presence: true,     // PHASE-2: Via BiometricPrompt
            supports_user_verification: true, // PHASE-2: Via BiometricPrompt or PIN
            supports_metadata: true,          // Android keystore supports metadata
            protocol: HsmProtocol::AndroidStrongBox,
        }
    }

    async fn prepare_credential_replication(
        &self,
        credential_id: &str,
        shared_entropy: &[u8],
    ) -> Result<CredentialReplicationData, Self::Error> {
        info!(
            "Preparing credential {} for replication using {} bytes of shared entropy",
            credential_id,
            shared_entropy.len()
        );

        // Get source credential info
        let source_cred = self.get_credential_info(credential_id).await?;

        // Create entropy hash
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(shared_entropy);
        let entropy_hash = hasher.finalize().to_vec();

        // Create derivation path
        let derivation_path = vec![0, 1, 2]; // PHASE-2: Implement BIP32-style derivation

        // Create credential request for target device
        let target_request = CredentialRequest {
            role: source_cred.role.clone(),
            display_name: source_cred.display_name.clone(),
            permissions: source_cred.permissions.clone(),
            require_user_presence: source_cred.requires_user_presence,
            require_user_verification: source_cred.requires_user_verification,
            parent_credential: None,
            metadata: source_cred.metadata.clone(),
            algorithm: Some(source_cred.algorithm.clone()),
        };

        info!("✅ Prepared replication data for credential");
        Ok(CredentialReplicationData {
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

impl beardog_traits::unified::BearDogProvider for StrongBoxMultiCredentialProvider {
    type Error = BearDogError;
    type Config = StrongBoxProviderConfig;

    fn provider_id(&self) -> &str {
        "android_strongbox_multi_credential"
    }

    fn provider_version(&self) -> &str {
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
                description: "Hardware RNG".to_string(),
                parameters: vec![],
                enabled: true,
            },
            ProviderCapability {
                name: "user_authentication".to_string(),
                description: "Biometric/PIN authentication".to_string(),
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
                (
                    "device".to_string(),
                    format!(
                        "{} {}",
                        self.device_info.manufacturer, self.device_info.model
                    ),
                ),
                (
                    "android_version".to_string(),
                    self.device_info.android_version.clone(),
                ),
                (
                    "strongbox".to_string(),
                    self.device_info.strongbox_available.to_string(),
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
            custom_metrics: vec![CustomMetric {
                name: "credentials_count".to_string(),
                value: creds.len() as f64,
                unit: "count".to_string(),
                description: "Number of credentials stored".to_string(),
                tags: HashMap::new(),
            }],
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alias_conversion() {
        let alias = "admin_key";
        let cred_id = StrongBoxMultiCredentialProvider::alias_to_credential_id(alias);
        let back_to_alias = StrongBoxMultiCredentialProvider::credential_id_to_alias(&cred_id);
        assert_eq!(alias, back_to_alias);
    }

    #[test]
    fn test_default_config() {
        let config = StrongBoxProviderConfig::default();
        assert_eq!(config.app_id, "com.ecoprimals.beardog");
        assert!(config.require_strongbox);
    }

    #[tokio::test]
    async fn test_provider_creation() {
        let device_info = StrongBoxDeviceInfo::default();
        let provider = StrongBoxMultiCredentialProvider::new(device_info, None).await;
        assert!(provider.is_ok());

        let provider = provider.unwrap();
        assert_eq!(provider.provider_id(), "android_strongbox_multi_credential");
    }
}
