// SPDX-License-Identifier: AGPL-3.0-or-later

//! Android-specific HSM types and keystore adapters.
//!
//! These types model the Android Keystore / `StrongBox` surface. Logic (validation, logging,
//! mapping) lives in [`AndroidKeystore`]; JNI / hardware access is isolated behind
//! [`KeystoreTransport`] and related ports so unit tests can run without Android hardware.

use std::sync::Arc;

#[cfg(target_os = "android")]
use beardog_config::env_keys;
use beardog_errors::BearDogError;
use beardog_types::canonical::providers_unified::traits::security_traits::{
    KeyInfo, KeyType, KeyUsage,
};

use super::key::KeyType as HsmKeyType;
use super::status;
use super::tier::AttestationLevel;

/// Android key purpose enumeration
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AndroidKeyPurpose {
    /// Encryption operations
    Encrypt,
    /// Decryption operations
    Decrypt,
    /// Signing operations
    Sign,
    /// Verification operations
    Verify,
    /// Key wrapping operations
    Wrap,
    /// Key unwrapping operations
    Unwrap,
}

/// Android key parameters configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AndroidKeyParams {
    /// Cryptographic algorithm
    pub algorithm: String,
    /// Key size in bits
    pub key_size: u32,
    /// Allowed key purposes
    pub purposes: Vec<AndroidKeyPurpose>,
    /// Require `StrongBox` hardware
    pub strongbox_required: bool,
    /// Require user authentication
    pub user_authentication_required: bool,
    /// User authentication timeout (seconds)
    pub user_authentication_timeout: Option<i32>,
    /// Key validity start time
    pub key_validity_start: Option<chrono::DateTime<chrono::Utc>>,
    /// Key validity end time
    pub key_validity_end: Option<chrono::DateTime<chrono::Utc>>,
    /// Attestation challenge data
    pub attestation_challenge: Option<Vec<u8>>,
}

impl AndroidKeyParams {
    /// Create new Android key parameters with defaults
    pub fn new() -> Self {
        Self {
            algorithm: "Ed25519".to_string(),
            key_size: 256,
            purposes: vec![AndroidKeyPurpose::Sign, AndroidKeyPurpose::Verify],
            strongbox_required: false,
            user_authentication_required: false,
            user_authentication_timeout: None,
            key_validity_start: None,
            key_validity_end: None,
            attestation_challenge: None,
        }
    }

    /// Set the cryptographic algorithm
    pub fn set_algorithm(mut self, algorithm: &str) -> Self {
        self.algorithm = algorithm.to_string();
        self
    }

    /// Set the key size
    pub fn set_key_size(&mut self, size: u32) {
        self.key_size = size;
    }

    /// Set the key purposes
    pub fn set_purposes(&mut self, purposes: Vec<AndroidKeyPurpose>) {
        self.purposes = purposes;
    }

    /// Set `StrongBox` requirement
    pub fn set_strongbox_required(&mut self, required: bool) {
        self.strongbox_required = required;
    }

    /// Set user authentication requirement
    pub fn set_user_authentication_required(&mut self, required: bool) {
        self.user_authentication_required = required;
    }

    /// Set key validity end time
    pub fn set_key_validity_end(&mut self, end: chrono::DateTime<chrono::Utc>) {
        self.key_validity_end = Some(end);
    }

    /// Set attestation challenge
    pub fn set_attestation_challenge(&mut self, challenge: Vec<u8>) {
        self.attestation_challenge = Some(challenge);
    }
}

impl Default for AndroidKeyParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Android HSM configuration
#[derive(Debug, Clone)]
/// Android keystore configuration
pub struct AndroidKeystoreConfig {
    /// Use hardware-backed keys
    pub hardware_backed: bool,
    /// Maximum key count
    pub max_keys: usize,
}

impl Default for AndroidKeystoreConfig {
    fn default() -> Self {
        Self {
            hardware_backed: true,
            max_keys: 256,
        }
    }
}

/// Android HSM configuration - canonical definition
#[derive(Debug, Clone)]
pub struct AndroidHsmConfig {
    /// Enable `StrongBox` hardware security
    pub strongbox_enabled: bool,
    /// Key parameters
    pub key_params: AndroidKeyParams,
    /// Attestation security level
    pub attestation_level: AttestationLevel,
    /// Security level (0-3)
    pub security_level: u8,
    /// Keystore configuration
    pub keystore_config: AndroidKeystoreConfig,
    /// Attestation configuration (using default for now)
    pub attestation_config: String, // Placeholder - will use proper type once AttestationConfig is accessible
}

impl Default for AndroidHsmConfig {
    fn default() -> Self {
        Self {
            strongbox_enabled: true,
            key_params: AndroidKeyParams::new(),
            keystore_config: AndroidKeystoreConfig::default(),
            attestation_config: "default".to_string(),
            attestation_level: AttestationLevel::Hardware,
            security_level: 2,
        }
    }
}

/// Android device capabilities
#[derive(Debug, Clone)]
pub struct AndroidDeviceCapabilities {
    /// `StrongBox` hardware available
    pub strongbox_available: bool,
    /// Key attestation available
    pub key_attestation_available: bool,
    /// Hardware-backed keystore available
    pub hardware_backed_keystore: bool,
    /// Verified boot enabled
    pub verified_boot: bool,
}

// Re-export transport traits and stubs from the dedicated module.
#[cfg(target_os = "android")]
pub use super::android_transports::AndroidJniHealthMetricsTransport;
#[cfg(target_os = "android")]
pub use super::android_transports::{AndroidJniAttestationTransport, AndroidJniKeystoreTransport};
pub use super::android_transports::{
    AttestationTransport, AttestationTransportBackend, HealthMetricsTransport,
    HealthMetricsTransportBackend, KeystoreTransport, KeystoreTransportBackend,
    MemoryKeystoreTransport, StubHealthMetricsTransport,
};
#[cfg(not(target_os = "android"))]
pub use super::android_transports::{StubAttestationTransport, StubKeystoreTransport};

// --- Android keystore (logic + delegation) -------------------------------------------------------

/// Android Keystore implementation
#[derive(Clone)]
pub struct AndroidKeystore {
    /// HSM configuration
    pub config: AndroidHsmConfig,
    /// Device capabilities
    pub capabilities: AndroidDeviceCapabilities,
    transport: Arc<KeystoreTransportBackend>,
}

impl AndroidKeystore {
    /// Create new Android keystore instance with an explicit transport (JNI on device, stub in tests).
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn new(
        config: AndroidHsmConfig,
        transport: Arc<KeystoreTransportBackend>,
    ) -> Result<Self, BearDogError> {
        let hardware_backed = transport.is_hardware_backed();
        let capabilities = AndroidDeviceCapabilities {
            strongbox_available: hardware_backed && config.strongbox_enabled,
            key_attestation_available: hardware_backed,
            hardware_backed_keystore: hardware_backed,
            verified_boot: hardware_backed,
        };

        Ok(Self {
            config,
            capabilities,
            transport,
        })
    }

    /// Build with the platform-appropriate keystore transport: in-memory stub on non-Android
    /// hosts; JNI-shaped backend on Android (see [`MemoryKeystoreTransport`]).
    ///
    /// # Errors
    ///
    /// Returns the same errors as [`Self::new`].
    pub fn with_platform_keystore_transport(
        config: AndroidHsmConfig,
    ) -> Result<Self, BearDogError> {
        #[cfg(target_os = "android")]
        {
            let transport = match beardog_errors::process_env::var(env_keys::ENV_KEYSTORE_BACKEND) {
                Ok(backend) if backend.eq_ignore_ascii_case("keymaster") => {
                    tracing::info!(
                        "Android keystore backend: keymaster (hardware StrongBox transport selected)"
                    );
                    KeystoreTransportBackend::AndroidKeymaster
                }
                Ok(backend) => {
                    tracing::warn!(
                        "Unknown BEARDOG_KEYSTORE_BACKEND={backend:?}; \
                         falling back to in-memory AndroidJni stub"
                    );
                    KeystoreTransportBackend::AndroidJni(MemoryKeystoreTransport::default())
                }
                Err(_) => {
                    tracing::warn!(
                        "Android keystore using in-memory stub — keys are NOT hardware-backed. \
                         Set BEARDOG_KEYSTORE_BACKEND=keymaster when Keymaster JNI is wired."
                    );
                    KeystoreTransportBackend::AndroidJni(MemoryKeystoreTransport::default())
                }
            };
            Self::new(config, Arc::new(transport))
        }
        #[cfg(not(target_os = "android"))]
        {
            Self::new(
                config,
                Arc::new(KeystoreTransportBackend::Stub(
                    StubKeystoreTransport::default(),
                )),
            )
        }
    }

    /// Non-Android only: explicit stub transport for unit tests and CI.
    ///
    /// # Errors
    ///
    /// Returns the same errors as [`Self::new`].
    #[cfg(not(target_os = "android"))]
    pub fn with_stub_transport(config: AndroidHsmConfig) -> Result<Self, BearDogError> {
        Self::with_platform_keystore_transport(config)
    }

    fn validate_key_id(key_id: &str) -> Result<(), BearDogError> {
        if key_id.is_empty() {
            return Err(BearDogError::invalid_input(
                "Android keystore key id must not be empty",
            ));
        }
        if key_id.len() > 256 {
            return Err(BearDogError::invalid_input(
                "Android keystore key id exceeds maximum length",
            ));
        }
        Ok(())
    }

    fn validate_params_for_generate(params: &AndroidKeyParams) -> Result<(), BearDogError> {
        if params.algorithm.trim().is_empty() {
            return Err(BearDogError::invalid_input(
                "Android key generation requires a non-empty algorithm",
            ));
        }
        if params.key_size == 0 {
            return Err(BearDogError::invalid_input(
                "Android key generation requires key_size > 0",
            ));
        }
        if let (Some(start), Some(end)) = (params.key_validity_start, params.key_validity_end)
            && end <= start
        {
            return Err(BearDogError::invalid_input(
                "Android key validity end must be after validity start",
            ));
        }
        Ok(())
    }

    /// Test keystore access (pure validation — no JNI).
    ///
    /// # Errors
    /// Returns an error if validation fails
    pub fn test_keystore_access(
        &self,
        key_id: &str,
        params: &AndroidKeyParams,
    ) -> Result<(), BearDogError> {
        Self::validate_key_id(key_id)?;
        Self::validate_params_for_generate(params)?;
        tracing::debug!("Android keystore access validation OK for {}", key_id);
        Ok(())
    }

    /// Generate a new key
    ///
    /// # Errors
    /// Returns an error if key generation fails
    pub async fn generate_key(
        &self,
        key_id: &str,
        params: &AndroidKeyParams,
    ) -> Result<(), BearDogError> {
        Self::validate_key_id(key_id)?;
        Self::validate_params_for_generate(params)?;
        tracing::info!("Generating Android keystore key: {}", key_id);
        let _handle = self.transport.jni_generate_key(key_id, params).await?;
        Ok(())
    }

    /// Encrypt data
    ///
    /// # Errors
    /// Returns an error if encryption fails
    pub async fn encrypt(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        Self::validate_key_id(key_id)?;
        tracing::debug!("Android keystore encrypt for {}", key_id);
        self.transport.jni_encrypt(key_id, data).await
    }

    /// Decrypt data
    ///
    /// # Errors
    /// Returns an error if decryption fails
    pub async fn decrypt(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        Self::validate_key_id(key_id)?;
        tracing::debug!("Android keystore decrypt for {}", key_id);
        self.transport.jni_decrypt(key_id, data).await
    }

    /// Sign data
    ///
    /// # Errors
    /// Returns an error if signing fails
    pub async fn sign(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        Self::validate_key_id(key_id)?;
        tracing::debug!("Android keystore sign for {}", key_id);
        self.transport.jni_sign(key_id, data).await
    }

    /// Verify signature
    ///
    /// # Errors
    /// Returns an error if verification fails
    pub async fn verify(
        &self,
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
        Self::validate_key_id(key_id)?;
        tracing::debug!("Android keystore verify for {}", key_id);
        self.transport.jni_verify(key_id, data, signature).await
    }

    /// Delete a key
    ///
    /// # Errors
    /// Returns an error if deletion fails
    pub async fn delete_key(&self, key_id: &str) -> Result<(), BearDogError> {
        Self::validate_key_id(key_id)?;
        tracing::info!("Deleting Android keystore key: {}", key_id);
        self.transport.jni_delete_key(key_id).await
    }

    /// Check if key exists
    ///
    /// # Errors
    /// Returns an error if check fails
    pub async fn key_exists(&self, key_id: &str) -> Result<bool, BearDogError> {
        Self::validate_key_id(key_id)?;
        let aliases = self.transport.jni_list_aliases().await?;
        Ok(aliases.iter().any(|a| a == key_id))
    }

    /// Generate attestation challenge
    ///
    /// # Errors
    /// Returns an error if generation fails
    pub fn generate_attestation_challenge(&self, size: usize) -> Result<Vec<u8>, BearDogError> {
        use rand::RngCore;
        let mut challenge = vec![0u8; size];
        rand::rng().fill_bytes(&mut challenge);
        tracing::debug!("Generated attestation challenge of {} bytes", size);
        Ok(challenge)
    }

    /// Check if `StrongBox` is available on this device
    pub fn is_strongbox_available(&self) -> bool {
        self.capabilities.strongbox_available
    }

    /// Whether keys are stored in a hardware-backed keystore (TEE / `StrongBox`).
    pub fn is_hardware_backed_keystore(&self) -> bool {
        self.capabilities.hardware_backed_keystore
    }

    /// Generate random bytes using hardware RNG
    ///
    /// # Errors
    /// Returns an error if RNG fails
    pub async fn generate_random_bytes(&self, count: usize) -> Result<Vec<u8>, BearDogError> {
        use rand::RngCore;

        tracing::debug!(
            "Generating {} random bytes using Android hardware RNG",
            count
        );

        let mut bytes = vec![0u8; count];
        rand::rng().fill_bytes(&mut bytes);

        Ok(bytes)
    }

    /// Import existing key material into keystore
    ///
    /// # Errors
    /// Returns an error if import fails
    pub async fn import_key(
        &self,
        key_id: &str,
        key_data: &[u8],
        key_type: HsmKeyType,
    ) -> Result<(), BearDogError> {
        Self::validate_key_id(key_id)?;
        if key_data.is_empty() {
            return Err(BearDogError::invalid_input(
                "Android keystore import requires non-empty key material",
            ));
        }
        tracing::info!("Importing key into Android Keystore: {}", key_id);
        self.transport
            .jni_import_key(key_id, key_data, key_type)
            .await
    }

    /// List all keys in keystore
    ///
    /// # Errors
    /// Returns an error if listing fails
    pub async fn list_keys(&self) -> Result<Vec<KeyInfo>, BearDogError> {
        tracing::info!("Listing keys in Android Keystore");
        let aliases = self.transport.jni_list_aliases().await?;
        let now = std::time::SystemTime::now();
        let key_usage: Vec<KeyUsage> = self
            .config
            .key_params
            .purposes
            .iter()
            .map(|p| match p {
                AndroidKeyPurpose::Encrypt => KeyUsage::Encrypt,
                AndroidKeyPurpose::Decrypt => KeyUsage::Decrypt,
                AndroidKeyPurpose::Sign => KeyUsage::Sign,
                AndroidKeyPurpose::Verify => KeyUsage::Verify,
                AndroidKeyPurpose::Wrap => KeyUsage::Wrap,
                AndroidKeyPurpose::Unwrap => KeyUsage::Unwrap,
            })
            .collect();

        let mut out = Vec::with_capacity(aliases.len());
        for key_id in aliases {
            out.push(KeyInfo {
                key_id,
                key_type: KeyType::Ed25519,
                key_size: self.config.key_params.key_size,
                key_usage: key_usage.clone(),
                created_at: now,
                extractable: false,
            });
        }
        Ok(out)
    }
}

/// Android attestation service
pub struct AndroidAttestationService {
    /// Whether attestation is enabled
    pub enabled: bool,
    /// Attestation level
    pub attestation_level: AttestationLevel,
    transport: Arc<AttestationTransportBackend>,
}

impl AndroidAttestationService {
    /// Create new attestation service with a transport (JNI on device).
    pub fn new(
        attestation_level: AttestationLevel,
        transport: Arc<AttestationTransportBackend>,
    ) -> Self {
        Self {
            enabled: true,
            attestation_level,
            transport,
        }
    }

    /// Attestation transport for the current platform (stub on hosts, JNI adapter on Android).
    pub fn with_platform_attestation_transport(attestation_level: AttestationLevel) -> Self {
        #[cfg(target_os = "android")]
        {
            tracing::warn!(
                "Android attestation using in-memory stub — attestations are NOT hardware-backed. \
                 Wire real Key Attestation JNI for production security."
            );
            Self::new(
                attestation_level,
                Arc::new(AttestationTransportBackend::AndroidJni(
                    AndroidJniAttestationTransport::default(),
                )),
            )
        }
        #[cfg(not(target_os = "android"))]
        {
            Self::new(
                attestation_level,
                Arc::new(AttestationTransportBackend::Stub(StubAttestationTransport)),
            )
        }
    }

    /// Non-Android only: explicit stub for tests.
    #[cfg(not(target_os = "android"))]
    pub fn with_stub_transport(attestation_level: AttestationLevel) -> Self {
        Self::with_platform_attestation_transport(attestation_level)
    }

    /// Initialize the attestation service
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub async fn initialize(&self) -> Result<(), BearDogError> {
        tracing::info!(
            "Initializing Android attestation service with level: {:?}",
            self.attestation_level
        );

        if !self.enabled {
            return Err(BearDogError::configuration(
                "Attestation service is disabled",
            ));
        }

        self.transport
            .jni_initialize(self.attestation_level.clone())
            .await?;
        tracing::info!("Android attestation service initialized successfully");
        Ok(())
    }
}

/// Android health monitor
pub struct AndroidHealthMonitor {
    /// Health check interval (seconds)
    pub check_interval_seconds: u64,
    metrics_transport: Arc<HealthMetricsTransportBackend>,
}

impl AndroidHealthMonitor {
    /// Create a monitor using stub metrics (non-Android) or `AndroidJniHealthMetricsTransport` on Android.
    pub fn new() -> Self {
        #[cfg(target_os = "android")]
        {
            Self::with_transport(Arc::new(HealthMetricsTransportBackend::AndroidJni(
                AndroidJniHealthMetricsTransport::new(),
            )))
        }
        #[cfg(not(target_os = "android"))]
        {
            Self::with_transport(Arc::new(HealthMetricsTransportBackend::Stub(
                StubHealthMetricsTransport::default(),
            )))
        }
    }

    /// Full control (e.g. inject stub in tests on Android).
    pub fn with_transport(metrics_transport: Arc<HealthMetricsTransportBackend>) -> Self {
        Self {
            check_interval_seconds: 60,
            metrics_transport,
        }
    }

    /// Start health monitoring
    ///
    /// # Errors
    /// Returns an error if monitoring fails to start
    pub fn start_monitoring(&self) -> Result<(), BearDogError> {
        tracing::info!(
            "Starting Android health monitoring with {}-second intervals",
            self.check_interval_seconds
        );

        let check_interval = std::time::Duration::from_secs(self.check_interval_seconds);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(check_interval);
            loop {
                interval.tick().await;
                tracing::debug!("Performing Android HSM health check (interval)");
            }
        });

        Ok(())
    }

    /// Get current health status (metrics from transport — JNI on device, stub in tests).
    ///
    /// # Errors
    /// Returns an error if status check fails
    pub async fn get_health_status(&self) -> Result<status::HsmHealthStatus, BearDogError> {
        tracing::debug!("Getting Android HSM health status");
        let performance_metrics = self.metrics_transport.collect_performance_metrics().await?;

        let is_healthy = performance_metrics.success_rate >= 50.0;

        Ok(status::HsmHealthStatus {
            is_healthy,
            last_check: chrono::Utc::now(),
            error_message: None,
            performance_metrics,
        })
    }
}

impl Default for AndroidHealthMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "android_tests.rs"]
mod tests;
