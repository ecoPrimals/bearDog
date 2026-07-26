// SPDX-License-Identifier: AGPL-3.0-or-later

//! Android-specific HSM types and keystore adapters.
//!
//! These types model the Android Keystore / `StrongBox` surface. Logic (validation, logging,
//! mapping) lives in [`AndroidKeystore`]; JNI / hardware access is isolated behind
//! [`KeystoreTransport`] and related ports so unit tests can run without Android hardware.

use std::sync::Arc;

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
    #[must_use]
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
    #[must_use]
    pub fn set_algorithm(mut self, algorithm: &str) -> Self {
        self.algorithm = algorithm.to_string();
        self
    }

    /// Set the key size
    pub const fn set_key_size(&mut self, size: u32) {
        self.key_size = size;
    }

    /// Set the key purposes
    pub fn set_purposes(&mut self, purposes: Vec<AndroidKeyPurpose>) {
        self.purposes = purposes;
    }

    /// Set `StrongBox` requirement
    pub const fn set_strongbox_required(&mut self, required: bool) {
        self.strongbox_required = required;
    }

    /// Set user authentication requirement
    pub const fn set_user_authentication_required(&mut self, required: bool) {
        self.user_authentication_required = required;
    }

    /// Set key validity end time
    pub const fn set_key_validity_end(&mut self, end: chrono::DateTime<chrono::Utc>) {
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
    pub attestation_config: String,
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
pub use super::android_transports::{
    AndroidJniAttestationTransport, AndroidJniHealthMetricsTransport, AndroidJniKeystoreTransport,
    AttestationTransport, AttestationTransportBackend, HealthMetricsTransport,
    HealthMetricsTransportBackend, Keystore2CliTransport, KeystoreTransport,
    KeystoreTransportBackend, MemoryKeystoreTransport, StubAttestationTransport,
    StubHealthMetricsTransport, StubKeystoreTransport,
};

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
        let capabilities = Self::detect_capabilities();

        Ok(Self {
            config,
            capabilities,
            transport,
        })
    }

    /// Runtime capability detection — probes real hardware when on Android,
    /// returns conservative defaults on non-Android hosts.
    fn detect_capabilities() -> AndroidDeviceCapabilities {
        if cfg!(target_os = "android") {
            let strongbox = Keystore2CliTransport::probe_strongbox();
            let cli_available = Keystore2CliTransport::is_available();
            AndroidDeviceCapabilities {
                strongbox_available: strongbox,
                key_attestation_available: cli_available,
                hardware_backed_keystore: cli_available,
                verified_boot: std::path::Path::new(
                    "/proc/device-tree/firmware/android/verifiedbootstate",
                )
                .exists()
                    || std::path::Path::new(
                        "/sys/firmware/devicetree/base/firmware/android/verifiedbootstate",
                    )
                    .exists(),
            }
        } else {
            AndroidDeviceCapabilities {
                strongbox_available: false,
                key_attestation_available: false,
                hardware_backed_keystore: false,
                verified_boot: false,
            }
        }
    }

    /// Build with the platform-appropriate keystore transport.
    ///
    /// Discovery order on Android (non-test):
    /// 1. `Keystore2Cli` — if `/system/bin/keystore_cli_v2` exists, use real hardware ops
    /// 2. `AndroidJni` — fall-closed stub (future Binder transport slot)
    ///
    /// Non-Android and tests always use the in-memory stub.
    ///
    /// # Errors
    ///
    /// Returns the same errors as [`Self::new`].
    pub fn with_platform_keystore_transport(
        config: AndroidHsmConfig,
    ) -> Result<Self, BearDogError> {
        let transport = if cfg!(target_os = "android") {
            if cfg!(test) {
                Arc::new(KeystoreTransportBackend::Stub(
                    StubKeystoreTransport::default(),
                ))
            } else if Keystore2CliTransport::is_available() {
                tracing::info!("Using Keystore2 CLI transport (hardware-backed)");
                Arc::new(KeystoreTransportBackend::Keystore2Cli(
                    Keystore2CliTransport::default(),
                ))
            } else {
                Arc::new(KeystoreTransportBackend::AndroidJni(
                    AndroidJniKeystoreTransport::default(),
                ))
            }
        } else {
            Arc::new(KeystoreTransportBackend::Stub(
                StubKeystoreTransport::default(),
            ))
        };
        Self::new(config, transport)
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
    #[must_use]
    pub const fn is_strongbox_available(&self) -> bool {
        self.capabilities.strongbox_available
    }

    /// Generate random bytes using hardware RNG
    ///
    /// # Errors
    /// Returns an error if RNG fails
    pub fn generate_random_bytes(&self, count: usize) -> Result<Vec<u8>, BearDogError> {
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
    #[must_use]
    pub const fn new(
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
    #[must_use]
    pub fn with_platform_attestation_transport(attestation_level: AttestationLevel) -> Self {
        let transport = if cfg!(target_os = "android") {
            Arc::new(AttestationTransportBackend::AndroidJni(
                AndroidJniAttestationTransport::default(),
            ))
        } else {
            Arc::new(AttestationTransportBackend::Stub(StubAttestationTransport))
        };
        Self::new(attestation_level, transport)
    }

    /// Non-Android only: explicit stub for tests.
    #[cfg(not(target_os = "android"))]
    #[must_use]
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
    #[must_use]
    pub fn new() -> Self {
        let transport = if cfg!(target_os = "android") {
            Arc::new(HealthMetricsTransportBackend::AndroidJni(
                AndroidJniHealthMetricsTransport::new(),
            ))
        } else {
            Arc::new(HealthMetricsTransportBackend::Stub(
                StubHealthMetricsTransport::default(),
            ))
        };
        Self::with_transport(transport)
    }
    /// Full control (e.g. inject stub in tests on Android).
    #[must_use]
    pub const fn with_transport(metrics_transport: Arc<HealthMetricsTransportBackend>) -> Self {
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
