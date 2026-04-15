// SPDX-License-Identifier: AGPL-3.0-or-later

//! Android Keystore / attestation / health transport traits and stubs.
//!
//! These are the **JNI boundary ports** — the ONLY items that need real Android hardware.
//! Production code injects real JNI transports; tests and non-Android hosts use the deterministic
//! stubs defined here.

use async_trait::async_trait;
use beardog_errors::BearDogError;
use parking_lot::Mutex;
use sha2::{Digest, Sha256};

use super::android::AndroidKeyParams;
use super::key::KeyType as HsmKeyType;
use super::status;
use super::tier::AttestationLevel;

// --- Keystore transport (JNI boundary) -----------------------------------------------------------

/// Thin port for Android Keystore JNI communication.
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

/// Stub transport with deterministic behavior for exercising [`super::android::AndroidKeystore`]
/// logic off-device.
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

// --- Attestation transport -----------------------------------------------------------------------

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

// --- Health metrics transport --------------------------------------------------------------------

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

/// On Android, collects process-level metrics available without custom JNI.
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
    #[allow(clippy::cast_precision_loss)]
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
