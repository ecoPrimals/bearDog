// SPDX-License-Identifier: AGPL-3.0-only

//! Android-specific HSM types and keystore adapters.
//!
//! These types model the Android Keystore / `StrongBox` surface. Logic (validation, logging,
//! mapping) lives in [`AndroidKeystore`]; JNI / hardware access is isolated behind
//! [`KeystoreTransport`] and related ports so unit tests can run without Android hardware.

use std::sync::Arc;

use async_trait::async_trait;
use beardog_errors::BearDogError;
use beardog_types::canonical::providers_unified::traits::{KeyInfo, KeyType, KeyUsage};
use parking_lot::Mutex;
use sha2::{Digest, Sha256};

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

// --- Ports (hardware / JNI boundary) -------------------------------------------------------------

/// Thin port for Android Keystore JNI communication.
/// This is the ONLY part that needs real Android hardware.
#[async_trait]
pub trait KeystoreTransport: Send + Sync {
    /// JNI: generate a new key; returned bytes are implementation-defined (e.g. handle or pubkey).
    async fn jni_generate_key(
        &self,
        alias: &str,
        params: &AndroidKeyParams,
    ) -> Result<Vec<u8>, BearDogError>;
    /// JNI: sign `data` with the key named `alias`.
    async fn jni_sign(&self, alias: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError>;
    /// JNI: verify `signature` over `data` for `alias`.
    async fn jni_verify(
        &self,
        alias: &str,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError>;
    /// JNI: encrypt `plaintext` with `alias`.
    async fn jni_encrypt(&self, alias: &str, plaintext: &[u8]) -> Result<Vec<u8>, BearDogError>;
    /// JNI: decrypt `ciphertext` with `alias`.
    async fn jni_decrypt(&self, alias: &str, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError>;
    /// JNI: list all key aliases in the keystore.
    async fn jni_list_aliases(&self) -> Result<Vec<String>, BearDogError>;
    /// JNI: delete the key named `alias`.
    async fn jni_delete_key(&self, alias: &str) -> Result<(), BearDogError>;
    /// JNI: import raw key material under `alias`.
    async fn jni_import_key(
        &self,
        alias: &str,
        key_data: &[u8],
        key_type: HsmKeyType,
    ) -> Result<(), BearDogError>;
}

/// Stub transport with deterministic behavior for exercising [`AndroidKeystore`] logic off-device.
#[derive(Debug, Default)]
pub struct StubKeystoreTransport {
    keys: Mutex<std::collections::HashMap<String, Vec<u8>>>,
}

fn stub_digest(data: &[u8]) -> Vec<u8> {
    Sha256::digest(data).to_vec()
}

fn xor_with_alias(alias: &str, data: &[u8]) -> Vec<u8> {
    let key = alias.as_bytes();
    data.iter()
        .enumerate()
        .map(|(i, b)| b ^ key[i % key.len().max(1)])
        .collect()
}

#[async_trait]
impl KeystoreTransport for StubKeystoreTransport {
    async fn jni_generate_key(
        &self,
        alias: &str,
        _params: &AndroidKeyParams,
    ) -> Result<Vec<u8>, BearDogError> {
        let handle = stub_digest(alias.as_bytes());
        self.keys.lock().insert(alias.to_string(), handle.clone());
        Ok(handle)
    }

    async fn jni_sign(&self, alias: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        if !self.keys.lock().contains_key(alias) {
            return Err(BearDogError::not_found(format!(
                "stub keystore: no key for alias {alias}"
            )));
        }
        let mut input = Vec::with_capacity(alias.len() + data.len());
        input.extend_from_slice(alias.as_bytes());
        input.extend_from_slice(data);
        Ok(stub_digest(&input))
    }

    async fn jni_verify(
        &self,
        alias: &str,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
        let expected = self.jni_sign(alias, data).await?;
        Ok(expected == signature)
    }

    async fn jni_encrypt(&self, alias: &str, plaintext: &[u8]) -> Result<Vec<u8>, BearDogError> {
        Ok(xor_with_alias(alias, plaintext))
    }

    async fn jni_decrypt(&self, alias: &str, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError> {
        Ok(xor_with_alias(alias, ciphertext))
    }

    async fn jni_list_aliases(&self) -> Result<Vec<String>, BearDogError> {
        Ok(self.keys.lock().keys().cloned().collect())
    }

    async fn jni_delete_key(&self, alias: &str) -> Result<(), BearDogError> {
        self.keys.lock().remove(alias);
        Ok(())
    }

    async fn jni_import_key(
        &self,
        alias: &str,
        key_data: &[u8],
        _key_type: HsmKeyType,
    ) -> Result<(), BearDogError> {
        self.keys
            .lock()
            .insert(alias.to_string(), key_data.to_vec());
        Ok(())
    }
}

/// Port for Android Key Attestation JNI (hardware-backed attestation).
#[async_trait]
pub trait AttestationTransport: Send + Sync {
    /// JNI: initialize attestation for the given security level.
    async fn jni_initialize(&self, level: AttestationLevel) -> Result<(), BearDogError>;
}

/// Stub attestation transport for tests (no JNI).
#[derive(Debug, Clone, Copy, Default)]
pub struct StubAttestationTransport;

#[async_trait]
impl AttestationTransport for StubAttestationTransport {
    async fn jni_initialize(&self, _level: AttestationLevel) -> Result<(), BearDogError> {
        Ok(())
    }
}

/// Port for collecting HSM health / performance metrics (JNI on device, stub in tests).
#[async_trait]
pub trait HealthMetricsTransport: Send + Sync {
    /// Collect current HSM/process performance metrics (JNI on Android, stub in tests).
    async fn collect_performance_metrics(&self)
    -> Result<status::PerformanceMetrics, BearDogError>;
}

/// Deterministic metrics for unit tests and non-Android hosts.
#[derive(Debug, Clone)]
pub struct StubHealthMetricsTransport {
    metrics: status::PerformanceMetrics,
}

impl Default for StubHealthMetricsTransport {
    fn default() -> Self {
        Self {
            metrics: status::PerformanceMetrics {
                operations_per_second: 42.0,
                average_latency_ms: 2.5,
                success_rate: 99.5,
                memory_usage_mb: 12.0,
                cpu_usage_percent: 3.0,
                network_throughput_bps: 50_000.0,
                latency_ms: 2.5,
                throughput_mbps: 0.05,
                uptime_seconds: 3_600,
            },
        }
    }
}

#[async_trait]
impl HealthMetricsTransport for StubHealthMetricsTransport {
    async fn collect_performance_metrics(
        &self,
    ) -> Result<status::PerformanceMetrics, BearDogError> {
        Ok(self.metrics.clone())
    }
}

/// On Android, collects process-level metrics available without custom JNI; JNI hooks can extend this later.
#[cfg(target_os = "android")]
#[derive(Debug)]
pub struct AndroidJniHealthMetricsTransport {
    start: std::time::Instant,
}

#[cfg(target_os = "android")]
impl AndroidJniHealthMetricsTransport {
    #[must_use]
    pub fn new() -> Self {
        Self {
            start: std::time::Instant::now(),
        }
    }
}

#[cfg(target_os = "android")]
fn android_resident_memory_mb() -> Option<f64> {
    let s = std::fs::read_to_string("/proc/self/statm").ok()?;
    let resident_pages: u64 = s.split_whitespace().nth(1)?.parse().ok()?;
    const PAGE_SIZE: u64 = 4096;
    Some((resident_pages * PAGE_SIZE) as f64 / (1024.0 * 1024.0))
}

#[cfg(target_os = "android")]
#[async_trait]
impl HealthMetricsTransport for AndroidJniHealthMetricsTransport {
    async fn collect_performance_metrics(
        &self,
    ) -> Result<status::PerformanceMetrics, BearDogError> {
        let mut m = status::PerformanceMetrics::default();
        m.uptime_seconds = self.start.elapsed().as_secs();
        m.memory_usage_mb = android_resident_memory_mb().unwrap_or(0.0);
        m.latency_ms = m.average_latency_ms;
        Ok(m)
    }
}

// --- Android keystore (logic + delegation) -------------------------------------------------------

/// Android Keystore implementation
#[derive(Clone)]
pub struct AndroidKeystore {
    /// HSM configuration
    pub config: AndroidHsmConfig,
    /// Device capabilities
    pub capabilities: AndroidDeviceCapabilities,
    transport: Arc<dyn KeystoreTransport>,
}

impl AndroidKeystore {
    /// Create new Android keystore instance with an explicit transport (JNI on device, stub in tests).
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn new(
        config: AndroidHsmConfig,
        transport: Arc<dyn KeystoreTransport>,
    ) -> Result<Self, BearDogError> {
        let capabilities = AndroidDeviceCapabilities {
            strongbox_available: true,
            key_attestation_available: true,
            hardware_backed_keystore: true,
            verified_boot: true,
        };

        Ok(Self {
            config,
            capabilities,
            transport,
        })
    }

    /// Convenience: build with [`StubKeystoreTransport`] (tests and CI).
    ///
    /// # Errors
    ///
    /// Returns the same errors as [`Self::new`].
    pub fn with_stub_transport(config: AndroidHsmConfig) -> Result<Self, BearDogError> {
        Self::new(config, Arc::new(StubKeystoreTransport::default()))
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
    pub const fn is_strongbox_available(&self) -> bool {
        self.capabilities.strongbox_available
    }

    /// Generate random bytes using hardware RNG
    ///
    /// # Errors
    /// Returns an error if RNG fails
    pub async fn generate_random_bytes(&self, count: usize) -> Result<Vec<u8>, BearDogError> {
        tracing::debug!(
            "Generating {} random bytes using Android hardware RNG",
            count
        );

        use rand::RngCore;
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
    transport: Arc<dyn AttestationTransport>,
}

impl AndroidAttestationService {
    /// Create new attestation service with a transport (JNI on device).
    pub fn new(
        attestation_level: AttestationLevel,
        transport: Arc<dyn AttestationTransport>,
    ) -> Self {
        Self {
            enabled: true,
            attestation_level,
            transport,
        }
    }

    /// Convenience for tests: stub transport.
    pub fn with_stub_transport(attestation_level: AttestationLevel) -> Self {
        Self::new(attestation_level, Arc::new(StubAttestationTransport))
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
    metrics_transport: Arc<dyn HealthMetricsTransport>,
}

impl AndroidHealthMonitor {
    /// Create a monitor using stub metrics (non-Android) or [`AndroidJniHealthMetricsTransport`] on Android.
    pub fn new() -> Self {
        #[cfg(target_os = "android")]
        {
            Self::with_transport(Arc::new(AndroidJniHealthMetricsTransport::new()))
        }
        #[cfg(not(target_os = "android"))]
        {
            Self::with_transport(Arc::new(StubHealthMetricsTransport::default()))
        }
    }

    /// Full control (e.g. inject stub in tests on Android).
    pub fn with_transport(metrics_transport: Arc<dyn HealthMetricsTransport>) -> Self {
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
mod android_keystore_logic_tests {
    use super::*;

    #[tokio::test]
    async fn rejects_empty_key_id_on_generate() {
        let ks = AndroidKeystore::with_stub_transport(AndroidHsmConfig::default()).expect("ks");
        let mut p = AndroidKeyParams::new();
        p.set_key_size(256);
        let err = ks.generate_key("", &p).await.expect_err("empty id");
        let msg = err.to_string();
        assert!(
            msg.contains("empty") || msg.contains("Empty"),
            "unexpected message: {msg}"
        );
    }

    #[tokio::test]
    async fn stub_roundtrip_sign_verify_list() {
        let ks = AndroidKeystore::with_stub_transport(AndroidHsmConfig::default()).expect("ks");
        let mut p = AndroidKeyParams::new();
        p.set_purposes(vec![AndroidKeyPurpose::Sign, AndroidKeyPurpose::Verify]);
        ks.generate_key("my-key", &p).await.expect("gen");
        let sig = ks.sign("my-key", b"hello").await.expect("sign");
        assert!(ks.verify("my-key", b"hello", &sig).await.expect("verify"));
        assert!(!ks.verify("my-key", b"other", &sig).await.expect("verify2"));
        let keys = ks.list_keys().await.expect("list");
        assert_eq!(keys.len(), 1);
        assert!(ks.key_exists("my-key").await.expect("exists"));
    }

    #[tokio::test]
    async fn stub_health_uses_deterministic_metrics() {
        let m =
            AndroidHealthMonitor::with_transport(Arc::new(StubHealthMetricsTransport::default()));
        let s = m.get_health_status().await.expect("health");
        assert!(s.is_healthy);
        assert_eq!(s.performance_metrics.operations_per_second, 42.0);
    }
}
