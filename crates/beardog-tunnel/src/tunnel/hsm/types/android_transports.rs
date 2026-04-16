// SPDX-License-Identifier: AGPL-3.0-or-later

//! Android Keystore / attestation / health transport traits and stubs.
//!
//! These are the **JNI boundary ports** — the ONLY items that need real Android hardware.
//! Production code injects real JNI transports; tests and non-Android hosts use the deterministic
//! stubs defined here.

use std::future::{Future, ready};

use beardog_errors::BearDogError;
use parking_lot::Mutex;
use sha2::{Digest, Sha256};

use super::android::AndroidKeyParams;
use super::key::KeyType as HsmKeyType;
use super::status;
use super::tier::AttestationLevel;

// --- Keystore transport (JNI boundary) -----------------------------------------------------------

/// Thin port for Android Keystore JNI communication.
pub trait KeystoreTransport: Send + Sync {
    /// JNI: generate a new key; returned bytes are implementation-defined (e.g. handle or pubkey).
    fn jni_generate_key(
        &self,
        alias: &str,
        params: &AndroidKeyParams,
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send;
    /// JNI: sign `data` with the key named `alias`.
    fn jni_sign(
        &self,
        alias: &str,
        data: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send;
    /// JNI: verify `signature` over `data` for `alias`.
    fn jni_verify(
        &self,
        alias: &str,
        data: &[u8],
        signature: &[u8],
    ) -> impl Future<Output = Result<bool, BearDogError>> + Send;
    /// JNI: encrypt `plaintext` with `alias`.
    fn jni_encrypt(
        &self,
        alias: &str,
        plaintext: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send;
    /// JNI: decrypt `ciphertext` with `alias`.
    fn jni_decrypt(
        &self,
        alias: &str,
        ciphertext: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send;
    /// JNI: list all key aliases in the keystore.
    fn jni_list_aliases(&self) -> impl Future<Output = Result<Vec<String>, BearDogError>> + Send;
    /// JNI: delete the key named `alias`.
    fn jni_delete_key(&self, alias: &str) -> impl Future<Output = Result<(), BearDogError>> + Send;
    /// JNI: import raw key material under `alias`.
    fn jni_import_key(
        &self,
        alias: &str,
        key_data: &[u8],
        key_type: HsmKeyType,
    ) -> impl Future<Output = Result<(), BearDogError>> + Send;
}

/// Enum dispatch for [`KeystoreTransport`].
#[derive(Debug)]
pub enum KeystoreTransportBackend {
    /// Non-Android hosts: deterministic in-memory port (no JNI).
    #[cfg(not(target_os = "android"))]
    Stub(MemoryKeystoreTransport),
    /// Android: Keystore JNI adapter (in-memory until Keymaster JNI is wired).
    #[cfg(target_os = "android")]
    AndroidJni(MemoryKeystoreTransport),
}

impl KeystoreTransport for KeystoreTransportBackend {
    fn jni_generate_key(
        &self,
        alias: &str,
        params: &AndroidKeyParams,
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        match self {
            #[cfg(not(target_os = "android"))]
            Self::Stub(t) => t.jni_generate_key(alias, params),
            #[cfg(target_os = "android")]
            Self::AndroidJni(t) => t.jni_generate_key(alias, params),
        }
    }

    fn jni_sign(
        &self,
        alias: &str,
        data: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        match self {
            #[cfg(not(target_os = "android"))]
            Self::Stub(t) => t.jni_sign(alias, data),
            #[cfg(target_os = "android")]
            Self::AndroidJni(t) => t.jni_sign(alias, data),
        }
    }

    fn jni_verify(
        &self,
        alias: &str,
        data: &[u8],
        signature: &[u8],
    ) -> impl Future<Output = Result<bool, BearDogError>> + Send {
        match self {
            #[cfg(not(target_os = "android"))]
            Self::Stub(t) => t.jni_verify(alias, data, signature),
            #[cfg(target_os = "android")]
            Self::AndroidJni(t) => t.jni_verify(alias, data, signature),
        }
    }

    fn jni_encrypt(
        &self,
        alias: &str,
        plaintext: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        match self {
            #[cfg(not(target_os = "android"))]
            Self::Stub(t) => t.jni_encrypt(alias, plaintext),
            #[cfg(target_os = "android")]
            Self::AndroidJni(t) => t.jni_encrypt(alias, plaintext),
        }
    }

    fn jni_decrypt(
        &self,
        alias: &str,
        ciphertext: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        match self {
            #[cfg(not(target_os = "android"))]
            Self::Stub(t) => t.jni_decrypt(alias, ciphertext),
            #[cfg(target_os = "android")]
            Self::AndroidJni(t) => t.jni_decrypt(alias, ciphertext),
        }
    }

    fn jni_list_aliases(&self) -> impl Future<Output = Result<Vec<String>, BearDogError>> + Send {
        match self {
            #[cfg(not(target_os = "android"))]
            Self::Stub(t) => t.jni_list_aliases(),
            #[cfg(target_os = "android")]
            Self::AndroidJni(t) => t.jni_list_aliases(),
        }
    }

    fn jni_delete_key(&self, alias: &str) -> impl Future<Output = Result<(), BearDogError>> + Send {
        match self {
            #[cfg(not(target_os = "android"))]
            Self::Stub(t) => t.jni_delete_key(alias),
            #[cfg(target_os = "android")]
            Self::AndroidJni(t) => t.jni_delete_key(alias),
        }
    }

    fn jni_import_key(
        &self,
        alias: &str,
        key_data: &[u8],
        key_type: HsmKeyType,
    ) -> impl Future<Output = Result<(), BearDogError>> + Send {
        match self {
            #[cfg(not(target_os = "android"))]
            Self::Stub(t) => t.jni_import_key(alias, key_data, key_type),
            #[cfg(target_os = "android")]
            Self::AndroidJni(t) => t.jni_import_key(alias, key_data, key_type),
        }
    }
}

/// Shared in-memory keystore behavior for host CI/tests ([`KeystoreTransportBackend::Stub`]) and for
/// the Android JNI-shaped backend until Keymaster calls are implemented.
#[derive(Debug, Default)]
pub struct MemoryKeystoreTransport {
    keys: Mutex<std::collections::HashMap<String, Vec<u8>>>,
}

/// Host-only name for [`MemoryKeystoreTransport`] (not compiled on Android targets).
#[cfg(not(target_os = "android"))]
pub type StubKeystoreTransport = MemoryKeystoreTransport;

/// Android Keystore JNI transport handle (same backing as host stub until JNI is wired).
#[cfg(target_os = "android")]
pub type AndroidJniKeystoreTransport = MemoryKeystoreTransport;

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

async fn stub_jni_verify(
    this: &MemoryKeystoreTransport,
    alias: &str,
    data: &[u8],
    signature: &[u8],
) -> Result<bool, BearDogError> {
    let expected = KeystoreTransport::jni_sign(this, alias, data).await?;
    Ok(expected == signature)
}

impl KeystoreTransport for MemoryKeystoreTransport {
    fn jni_generate_key(
        &self,
        alias: &str,
        _params: &AndroidKeyParams,
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        let handle = stub_digest(alias.as_bytes());
        self.keys.lock().insert(alias.to_string(), handle.clone());
        ready(Ok(handle))
    }

    fn jni_sign(
        &self,
        alias: &str,
        data: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        if !self.keys.lock().contains_key(alias) {
            return ready(Err(BearDogError::not_found(format!(
                "stub keystore: no key for alias {alias}"
            ))));
        }
        let mut input = Vec::with_capacity(alias.len() + data.len());
        input.extend_from_slice(alias.as_bytes());
        input.extend_from_slice(data);
        ready(Ok(stub_digest(&input)))
    }

    fn jni_verify(
        &self,
        alias: &str,
        data: &[u8],
        signature: &[u8],
    ) -> impl Future<Output = Result<bool, BearDogError>> + Send {
        stub_jni_verify(self, alias, data, signature)
    }

    fn jni_encrypt(
        &self,
        alias: &str,
        plaintext: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        ready(Ok(xor_with_alias(alias, plaintext)))
    }

    fn jni_decrypt(
        &self,
        alias: &str,
        ciphertext: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        ready(Ok(xor_with_alias(alias, ciphertext)))
    }

    fn jni_list_aliases(&self) -> impl Future<Output = Result<Vec<String>, BearDogError>> + Send {
        ready(Ok(self.keys.lock().keys().cloned().collect()))
    }

    fn jni_delete_key(&self, alias: &str) -> impl Future<Output = Result<(), BearDogError>> + Send {
        self.keys.lock().remove(alias);
        ready(Ok(()))
    }

    fn jni_import_key(
        &self,
        alias: &str,
        key_data: &[u8],
        _key_type: HsmKeyType,
    ) -> impl Future<Output = Result<(), BearDogError>> + Send {
        self.keys
            .lock()
            .insert(alias.to_string(), key_data.to_vec());
        ready(Ok(()))
    }
}

// --- Attestation transport -----------------------------------------------------------------------

/// Port for Android Key Attestation JNI (hardware-backed attestation).
pub trait AttestationTransport: Send + Sync {
    /// JNI: initialize attestation for the given security level.
    fn jni_initialize(
        &self,
        level: AttestationLevel,
    ) -> impl Future<Output = Result<(), BearDogError>> + Send;
}

/// Enum dispatch for [`AttestationTransport`].
#[derive(Debug)]
pub enum AttestationTransportBackend {
    /// Non-Android hosts: no-op stand-in (no JNI).
    #[cfg(not(target_os = "android"))]
    Stub(StubAttestationTransport),
    /// Android: attestation JNI port (no-op until Key Attestation JNI is wired).
    #[cfg(target_os = "android")]
    AndroidJni(AndroidJniAttestationTransport),
}

impl AttestationTransport for AttestationTransportBackend {
    fn jni_initialize(
        &self,
        level: AttestationLevel,
    ) -> impl Future<Output = Result<(), BearDogError>> + Send {
        match self {
            #[cfg(not(target_os = "android"))]
            Self::Stub(t) => t.jni_initialize(level),
            #[cfg(target_os = "android")]
            Self::AndroidJni(t) => t.jni_initialize(level),
        }
    }
}

/// Stub attestation transport for non-Android tests (no JNI).
#[cfg(not(target_os = "android"))]
#[derive(Debug, Clone, Copy, Default)]
pub struct StubAttestationTransport;

#[cfg(not(target_os = "android"))]
impl AttestationTransport for StubAttestationTransport {
    fn jni_initialize(
        &self,
        _level: AttestationLevel,
    ) -> impl Future<Output = Result<(), BearDogError>> + Send {
        ready(Ok(()))
    }
}

/// Android Key Attestation JNI adapter (placeholder until hardware attestation JNI is implemented).
#[cfg(target_os = "android")]
#[derive(Debug, Clone, Copy, Default)]
pub struct AndroidJniAttestationTransport;

#[cfg(target_os = "android")]
impl AttestationTransport for AndroidJniAttestationTransport {
    fn jni_initialize(
        &self,
        _level: AttestationLevel,
    ) -> impl Future<Output = Result<(), BearDogError>> + Send {
        ready(Ok(()))
    }
}

// --- Health metrics transport --------------------------------------------------------------------

/// Port for collecting HSM health / performance metrics (JNI on device, stub in tests).
pub trait HealthMetricsTransport: Send + Sync {
    /// Collect current HSM/process performance metrics (JNI on Android, stub in tests).
    fn collect_performance_metrics(
        &self,
    ) -> impl Future<Output = Result<status::PerformanceMetrics, BearDogError>> + Send;
}

/// Enum dispatch for [`HealthMetricsTransport`].
#[derive(Debug)]
pub enum HealthMetricsTransportBackend {
    /// Non-Android test stub (no JNI).
    Stub(StubHealthMetricsTransport),
    #[cfg(target_os = "android")]
    AndroidJni(AndroidJniHealthMetricsTransport),
}

async fn health_metrics_backend_collect(
    backend: &HealthMetricsTransportBackend,
) -> Result<status::PerformanceMetrics, BearDogError> {
    match backend {
        HealthMetricsTransportBackend::Stub(t) => {
            HealthMetricsTransport::collect_performance_metrics(t).await
        }
        #[cfg(target_os = "android")]
        HealthMetricsTransportBackend::AndroidJni(t) => {
            HealthMetricsTransport::collect_performance_metrics(t).await
        }
    }
}

impl HealthMetricsTransport for HealthMetricsTransportBackend {
    fn collect_performance_metrics(
        &self,
    ) -> impl Future<Output = Result<status::PerformanceMetrics, BearDogError>> + Send {
        health_metrics_backend_collect(self)
    }
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

impl HealthMetricsTransport for StubHealthMetricsTransport {
    fn collect_performance_metrics(
        &self,
    ) -> impl Future<Output = Result<status::PerformanceMetrics, BearDogError>> + Send {
        ready(Ok(self.metrics.clone()))
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
async fn android_jni_collect_performance_metrics(
    this: &AndroidJniHealthMetricsTransport,
) -> Result<status::PerformanceMetrics, BearDogError> {
    let mut m = status::PerformanceMetrics::default();
    m.uptime_seconds = this.start.elapsed().as_secs();
    m.memory_usage_mb = android_resident_memory_mb().unwrap_or(0.0);
    m.latency_ms = m.average_latency_ms;
    Ok(m)
}

#[cfg(target_os = "android")]
impl HealthMetricsTransport for AndroidJniHealthMetricsTransport {
    fn collect_performance_metrics(
        &self,
    ) -> impl Future<Output = Result<status::PerformanceMetrics, BearDogError>> + Send {
        android_jni_collect_performance_metrics(self)
    }
}
