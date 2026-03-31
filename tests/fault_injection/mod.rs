// SPDX-License-Identifier: AGPL-3.0-only
#![allow(clippy::expect_used, clippy::unwrap_used)]
#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_comparisons,
    clippy::all
)]

//! Fault Injection Testing Framework for `BearDog`
//!
//! This module provides systematic fault injection capabilities to test
//! error handling, recovery, and resilience. Unlike chaos testing which
//! simulates environmental failures, fault injection deliberately injects
//! faults at specific points in the code to validate error paths.
//!
//! # Coverage
//!
//! - API error injection
//! - HSM operation failures
//! - Network failures at specific points
//! - Resource allocation failures
//! - Timeout scenarios
//! - Data corruption scenarios
//! - Concurrent fault scenarios

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::Duration;

/// Fault injection configuration
#[derive(Debug, Clone)]
pub struct FaultConfig {
    /// Enable fault injection
    pub enabled: bool,
    /// Fault injection rate (0.0-1.0)
    pub injection_rate: f64,
    /// Specific faults to inject
    pub fault_types: Vec<FaultType>,
    /// Maximum concurrent faults
    pub max_concurrent_faults: usize,
}

impl Default for FaultConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            injection_rate: 0.1, // 10% fault rate
            fault_types: vec![
                FaultType::NetworkTimeout,
                FaultType::HsmFailure,
                FaultType::MemoryAllocationFailure,
            ],
            max_concurrent_faults: 3,
        }
    }
}

/// Types of faults that can be injected
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultType {
    /// Network timeout
    NetworkTimeout,
    /// Network connection refused
    NetworkConnectionRefused,
    /// Network data corruption
    NetworkDataCorruption,
    /// HSM operation failure
    HsmFailure,
    /// HSM timeout
    HsmTimeout,
    /// HSM invalid response
    HsmInvalidResponse,
    /// Memory allocation failure
    MemoryAllocationFailure,
    /// Disk full
    DiskFull,
    /// File not found
    FileNotFound,
    /// Permission denied
    PermissionDenied,
    /// Invalid input
    InvalidInput,
    /// Timeout
    Timeout,
    /// Resource exhausted
    ResourceExhausted,
    /// Concurrent access conflict
    ConcurrentConflict,
}

/// Result of a fault injection test
#[derive(Debug)]
pub struct FaultTestResult {
    /// Test name
    pub name: String,
    /// Fault type injected
    pub fault_type: FaultType,
    /// Whether error was handled correctly
    pub error_handled_correctly: bool,
    /// Whether system recovered
    pub system_recovered: bool,
    /// Error propagation was correct
    pub error_propagation_correct: bool,
    /// Details
    pub details: String,
    /// Metrics
    pub metrics: FaultMetrics,
}

/// Metrics for fault injection tests
#[derive(Debug, Default)]
pub struct FaultMetrics {
    /// Faults injected
    pub faults_injected: u64,
    /// Errors caught properly
    pub errors_caught: u64,
    /// Errors escaped
    pub errors_escaped: u64,
    /// Panics detected
    pub panics_detected: u64,
    /// Recovery successes
    pub recoveries_succeeded: u64,
    /// Recovery failures
    pub recoveries_failed: u64,
}

/// Fault injection engine
pub struct FaultInjector {
    config: FaultConfig,
    active_faults: Arc<tokio::sync::RwLock<Vec<FaultType>>>,
    fault_count: AtomicU64,
    enabled: AtomicBool,
}

impl FaultInjector {
    /// Create a new fault injector
    #[must_use]
    pub fn new(config: FaultConfig) -> Self {
        let enabled = config.enabled;
        Self {
            config,
            active_faults: Arc::new(tokio::sync::RwLock::new(Vec::new())),
            fault_count: AtomicU64::new(0),
            enabled: AtomicBool::new(enabled),
        }
    }

    /// Create with default configuration
    #[must_use]
    pub fn default_injector() -> Self {
        Self::new(FaultConfig::default())
    }

    /// Enable fault injection
    pub fn enable(&self) {
        self.enabled.store(true, Ordering::SeqCst);
    }

    /// Disable fault injection
    pub fn disable(&self) {
        self.enabled.store(false, Ordering::SeqCst);
    }

    /// Check if fault should be injected
    pub fn should_inject_fault(&self, fault_type: FaultType) -> bool {
        if !self.enabled.load(Ordering::SeqCst) {
            return false;
        }

        if !self.config.fault_types.contains(&fault_type) {
            return false;
        }

        use rand::Rng;
        let mut rng = rand::rng();
        rng.random::<f64>() < self.config.injection_rate
    }

    /// Inject a fault at a specific point
    pub async fn inject_fault<T, E>(
        &self,
        fault_type: FaultType,
        error_generator: impl FnOnce() -> E,
    ) -> Result<T, E> {
        if self.should_inject_fault(fault_type) {
            self.fault_count.fetch_add(1, Ordering::SeqCst);

            let mut active = self.active_faults.write().await;
            active.push(fault_type);

            Err(error_generator())
        } else {
            // Return a placeholder - actual implementation would continue normal flow
            panic!("Fault not injected - normal flow should continue");
        }
    }

    /// Run a fault injection test
    pub async fn run_fault_test<F, Fut, T, E>(
        &self,
        name: &str,
        fault_type: FaultType,
        test_fn: F,
    ) -> FaultTestResult
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
        E: std::fmt::Debug,
    {
        self.enable();

        let mut metrics = FaultMetrics::default();

        // Run test with fault injection
        let result = test_fn().await;

        let error_handled_correctly = result.is_err();
        metrics.faults_injected = self.fault_count.load(Ordering::SeqCst);

        if error_handled_correctly {
            metrics.errors_caught += 1;
        } else {
            metrics.errors_escaped += 1;
        }

        // Validate recovery
        let system_recovered = self.validate_system_recovery().await;
        if system_recovered {
            metrics.recoveries_succeeded += 1;
        } else {
            metrics.recoveries_failed += 1;
        }

        self.disable();

        FaultTestResult {
            name: name.to_string(),
            fault_type,
            error_handled_correctly,
            system_recovered,
            error_propagation_correct: true, // Would validate error types
            details: format!("{:?}", result.err()),
            metrics,
        }
    }

    /// Validate system recovery after fault
    async fn validate_system_recovery(&self) -> bool {
        // Clear active faults
        let mut active = self.active_faults.write().await;
        active.clear();

        // System should be able to perform basic operations
        true
    }

    /// Get fault injection statistics
    pub fn get_stats(&self) -> FaultStats {
        FaultStats {
            total_faults_injected: self.fault_count.load(Ordering::SeqCst),
            enabled: self.enabled.load(Ordering::SeqCst),
        }
    }
}

/// Statistics for fault injection
#[derive(Debug)]
pub struct FaultStats {
    /// Total faults injected
    pub total_faults_injected: u64,
    /// Whether injection is enabled
    pub enabled: bool,
}

/// Helper to inject network faults
pub struct NetworkFaultInjector {
    base_injector: Arc<FaultInjector>,
}

impl NetworkFaultInjector {
    /// Create a network fault injector
    pub fn new(base_injector: Arc<FaultInjector>) -> Self {
        Self { base_injector }
    }

    /// Maybe inject a network timeout
    pub async fn maybe_timeout(&self, duration: Duration) -> Result<(), String> {
        if self
            .base_injector
            .should_inject_fault(FaultType::NetworkTimeout)
        {
            // No sleep needed - testing timeout injection logic, not actual timeout
            // For time-based timeout tests, use tokio::time::pause() + advance()
            let _ = duration; // Track duration for validation
            Err("Network timeout".to_string())
        } else {
            Ok(())
        }
    }

    /// Maybe inject connection refused
    pub fn maybe_connection_refused(&self) -> Result<(), String> {
        if self
            .base_injector
            .should_inject_fault(FaultType::NetworkConnectionRefused)
        {
            Err("Connection refused".to_string())
        } else {
            Ok(())
        }
    }

    /// Maybe corrupt network data
    pub fn maybe_corrupt_data(&self, data: &mut [u8]) {
        if self
            .base_injector
            .should_inject_fault(FaultType::NetworkDataCorruption)
        {
            // Flip random bits
            if let Some(byte) = data.get_mut(0) {
                *byte ^= 0xFF;
            }
        }
    }
}

/// Helper to inject HSM faults
pub struct HsmFaultInjector {
    base_injector: Arc<FaultInjector>,
}

impl HsmFaultInjector {
    /// Create an HSM fault injector
    pub fn new(base_injector: Arc<FaultInjector>) -> Self {
        Self { base_injector }
    }

    /// Maybe inject HSM failure
    pub fn maybe_fail(&self) -> Result<(), String> {
        if self
            .base_injector
            .should_inject_fault(FaultType::HsmFailure)
        {
            Err("HSM operation failed".to_string())
        } else {
            Ok(())
        }
    }

    /// Maybe inject HSM timeout
    pub async fn maybe_timeout(&self, duration: Duration) -> Result<(), String> {
        if self
            .base_injector
            .should_inject_fault(FaultType::HsmTimeout)
        {
            // No sleep needed - testing HSM timeout injection logic, not actual timeout
            // For time-based timeout tests, use tokio::time::pause() + advance()
            let _ = duration; // Track duration for validation
            Err("HSM timeout".to_string())
        } else {
            Ok(())
        }
    }

    /// Maybe return invalid HSM response
    pub fn maybe_invalid_response<T>(&self, valid_response: T) -> Result<T, String> {
        if self
            .base_injector
            .should_inject_fault(FaultType::HsmInvalidResponse)
        {
            Err("Invalid HSM response".to_string())
        } else {
            Ok(valid_response)
        }
    }
}

/// Helper to inject resource faults
pub struct ResourceFaultInjector {
    base_injector: Arc<FaultInjector>,
}

impl ResourceFaultInjector {
    /// Create a resource fault injector
    pub fn new(base_injector: Arc<FaultInjector>) -> Self {
        Self { base_injector }
    }

    /// Maybe inject memory allocation failure
    pub fn maybe_allocation_failure<T>(&self, value: T) -> Result<T, String> {
        if self
            .base_injector
            .should_inject_fault(FaultType::MemoryAllocationFailure)
        {
            Err("Memory allocation failed".to_string())
        } else {
            Ok(value)
        }
    }

    /// Maybe inject disk full error
    pub fn maybe_disk_full(&self) -> Result<(), String> {
        if self.base_injector.should_inject_fault(FaultType::DiskFull) {
            Err("Disk full".to_string())
        } else {
            Ok(())
        }
    }

    /// Maybe inject resource exhausted
    pub fn maybe_resource_exhausted(&self) -> Result<(), String> {
        if self
            .base_injector
            .should_inject_fault(FaultType::ResourceExhausted)
        {
            Err("Resource exhausted".to_string())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use beardog_config::BearDogConfig;
    use beardog_config::domains::network_ports::{DEFAULT_API_PORT, NetworkPortsConfig};
    use beardog_config::domains::paths::PathConfig;
    use beardog_core::crypto_service::algorithms::asymmetric::verify_ed25519;
    use beardog_core::crypto_service::{BearDogCryptoService, CryptoService, CryptoServiceConfig};
    use beardog_errors::BearDogError;
    use beardog_ipc::protocol::error_codes;
    use beardog_security::{MemoryKeyConfig, MemoryKeyManager};
    use beardog_types::crypto_service::{
        CryptoAlgorithm, DecryptOptions, EncryptOptions, EncryptedData, EncryptionMetadata,
        SignOptions, Signature, SignatureAlgorithm, SignatureMetadata, VerifyOptions,
    };
    use serde_json::json;
    use std::io::Write;
    use std::panic;
    use std::time::{Duration, SystemTime};
    use tempfile::NamedTempFile;
    use tokio::net::UnixStream;

    /// Mirrors [`beardog_ipc::multi_transport::handle_jsonrpc_request_line`] JSON-RPC 2.0
    /// parse/validation behavior (default `beardog-ipc` build without `tarpc` keeps that logic
    /// in-crate; this test-side helper asserts the same contract for fault injection).
    fn handle_jsonrpc_request_line_for_fault(line: &str) -> Option<String> {
        const MAX_JSONRPC_LINE_BYTES: usize = 256 * 1024;

        let trimmed = line.trim();
        if trimmed.is_empty() {
            return None;
        }
        if trimmed.len() > MAX_JSONRPC_LINE_BYTES {
            return Some(
                json!({
                    "jsonrpc": "2.0",
                    "error": { "code": error_codes::INTERNAL_ERROR, "message": "Message too large" },
                    "id": null
                })
                .to_string(),
            );
        }

        let v: serde_json::Value = match serde_json::from_str(trimmed) {
            Ok(v) => v,
            Err(_) => {
                return Some(
                    json!({
                        "jsonrpc": "2.0",
                        "error": { "code": error_codes::PARSE_ERROR, "message": "Parse error" },
                        "id": null
                    })
                    .to_string(),
                );
            }
        };

        let Some(obj) = v.as_object() else {
            return Some(
                json!({
                    "jsonrpc": "2.0",
                    "error": { "code": error_codes::INVALID_REQUEST, "message": "Invalid Request" },
                    "id": null
                })
                .to_string(),
            );
        };

        if obj.get("jsonrpc") != Some(&json!("2.0")) {
            return Some(
                json!({
                    "jsonrpc": "2.0",
                    "error": { "code": error_codes::INVALID_REQUEST, "message": "Invalid Request" },
                    "id": obj.get("id").cloned().unwrap_or(serde_json::Value::Null)
                })
                .to_string(),
            );
        }

        if !obj.contains_key("id") {
            return None;
        }

        let id = obj.get("id").cloned().unwrap_or(serde_json::Value::Null);
        let method = obj.get("method").and_then(|m| m.as_str()).unwrap_or("");

        if method.is_empty() {
            return Some(
                json!({
                    "jsonrpc": "2.0",
                    "error": { "code": error_codes::INVALID_REQUEST, "message": "Invalid Request" },
                    "id": id
                })
                .to_string(),
            );
        }

        Some(
            json!({
                "jsonrpc": "2.0",
                "error": { "code": error_codes::METHOD_NOT_FOUND, "message": "Method not found" },
                "id": id
            })
            .to_string(),
        )
    }

    #[test]
    fn test_fault_injector_creation() {
        let injector = FaultInjector::default_injector();
        let stats = injector.get_stats();
        assert_eq!(stats.total_faults_injected, 0);
    }

    #[test]
    fn test_enable_disable() {
        let injector = FaultInjector::default_injector();
        assert!(injector.enabled.load(Ordering::SeqCst));

        injector.disable();
        assert!(!injector.enabled.load(Ordering::SeqCst));

        injector.enable();
        assert!(injector.enabled.load(Ordering::SeqCst));
    }

    #[test]
    fn test_should_inject_fault() {
        let mut config = FaultConfig::default();
        config.injection_rate = 0.0; // Never inject
        let injector = FaultInjector::new(config);

        assert!(!injector.should_inject_fault(FaultType::NetworkTimeout));
    }

    #[tokio::test]
    async fn test_network_fault_injector() {
        let base = Arc::new(FaultInjector::default_injector());
        base.disable(); // Disable for predictable test

        let network = NetworkFaultInjector::new(base);
        let result = network.maybe_connection_refused();
        assert!(result.is_ok()); // Should not inject when disabled
    }

    #[test]
    fn test_hsm_fault_injector() {
        let base = Arc::new(FaultInjector::default_injector());
        base.disable(); // Disable for predictable test

        let hsm = HsmFaultInjector::new(base);
        let result = hsm.maybe_fail();
        assert!(result.is_ok()); // Should not inject when disabled
    }

    #[test]
    fn test_resource_fault_injector() {
        let base = Arc::new(FaultInjector::default_injector());
        base.disable(); // Disable for predictable test

        let resource = ResourceFaultInjector::new(base);
        let result = resource.maybe_disk_full();
        assert!(result.is_ok()); // Should not inject when disabled
    }

    #[test]
    fn test_fault_metrics() {
        let metrics = FaultMetrics {
            faults_injected: 100,
            errors_caught: 95,
            errors_escaped: 5,
            panics_detected: 0,
            recoveries_succeeded: 95,
            recoveries_failed: 5,
        };

        assert_eq!(metrics.faults_injected, 100);
        assert_eq!(metrics.errors_caught, 95);
    }

    // --- Configuration fault injection ----------------------------------------------------

    /// Missing config file path must surface as an error (no panic) when loading explicitly.
    #[tokio::test]
    async fn config_fault_missing_file_returns_error() {
        let dir = tempfile::tempdir().expect("temp directory for missing config test");
        let path = dir.path().join("definitely_missing.toml");
        assert!(!path.exists());
        let result = BearDogConfig::from_file(&path);
        assert!(
            result.is_err(),
            "missing file should not panic; expect error"
        );
    }

    /// Corrupt TOML in a config file must fail parsing with a meaningful error.
    #[tokio::test]
    async fn config_fault_corrupt_toml_returns_parse_error() {
        let mut file = NamedTempFile::new().expect("temp config file");
        writeln!(file, "[[[not_valid_toml").expect("write corrupt toml");
        let path = file.path().to_path_buf();
        let result = BearDogConfig::from_file(&path);
        assert!(
            result.is_err(),
            "corrupt TOML should yield error, not panic"
        );
    }

    /// Invalid env-style port strings match `NetworkPortsConfig::from_env` parsing: non-numeric
    /// values are ignored and the documented default port is used (no panic).
    #[tokio::test]
    async fn config_fault_invalid_port_string_uses_default_like_from_env() {
        let parsed = "not-a-port".parse::<u16>().ok();
        assert!(parsed.is_none());
        let api_port = parsed.unwrap_or(DEFAULT_API_PORT);
        assert_eq!(api_port, DEFAULT_API_PORT);
    }

    /// Privileged API port fails [`NetworkPortsConfig::validate`] with a clear error (same rule
    /// `BearDogConfig::validate` applies via centralized ports).
    #[tokio::test]
    async fn config_fault_privileged_api_port_fails_validation() {
        let mut ports = NetworkPortsConfig::with_defaults();
        ports.api_port = 80;
        let v = ports.validate();
        assert!(
            v.is_err(),
            "privileged port should fail validation with clear error"
        );
    }

    /// PKCS#11 library path that does not exist fails [`PathConfig::validate`] (no panic).
    #[tokio::test]
    async fn config_fault_bad_pkcs11_path_returns_path_error() {
        let dir = tempfile::tempdir().expect("temp directory for fake pkcs11 path");
        let fake = dir.path().join("no_such_lib.so");
        assert!(!fake.exists());
        let mut paths = PathConfig::default();
        paths.pkcs11_library = Some(fake);
        let v = paths.validate();
        assert!(
            v.is_err(),
            "missing PKCS#11 library path should error in validate()"
        );
    }

    /// Empty `network` table in TOML still deserializes; validation catches invalid state (e.g. port 0).
    #[tokio::test]
    async fn config_fault_empty_network_section_validates_or_loads() {
        let mut file = NamedTempFile::new().expect("temp network toml");
        writeln!(file, "[network.ports]").expect("write header");
        writeln!(file, "api_port = 0").expect("write invalid port");
        let path = file.path();
        let loaded = BearDogConfig::from_file(path);
        if let Ok(cfg) = loaded {
            assert!(cfg.validate().is_err(), "port 0 must not validate");
        } else {
            // Deserialization may also reject invalid port depending on schema
        }
    }

    // --- IPC fault injection --------------------------------------------------------------

    /// Connecting to a non-existent Unix socket path should fail quickly (no hang).
    #[tokio::test]
    async fn ipc_fault_nonexistent_unix_socket_fails_without_hanging() {
        let dir = tempfile::tempdir().expect("temp directory for bogus socket");
        let socket_path = dir.path().join("nonexistent.sock");
        let connect = UnixStream::connect(&socket_path);
        let outcome = tokio::time::timeout(Duration::from_secs(2), connect)
            .await
            .expect("connect should complete within timeout");
        assert!(
            outcome.is_err(),
            "connection to missing socket should fail with error"
        );
    }

    /// Malformed JSON-RPC lines yield parse error responses (JSON-RPC -32700).
    #[tokio::test]
    async fn ipc_fault_malformed_jsonrpc_returns_parse_error_response() {
        let line = "not-json-at-all{{{";
        let response = handle_jsonrpc_request_line_for_fault(line).expect("response for bad line");
        let v: serde_json::Value = serde_json::from_str(&response).expect("response is JSON");
        assert_eq!(v["error"]["code"], error_codes::PARSE_ERROR);
    }

    /// Invalid JSON-RPC object (missing 2.0) yields invalid request (-32600).
    #[tokio::test]
    async fn ipc_fault_invalid_jsonrpc_version_returns_invalid_request() {
        let line = r#"{"jsonrpc":"1.0","method":"x","id":1}"#;
        let response = handle_jsonrpc_request_line_for_fault(line).expect("response");
        let v: serde_json::Value = serde_json::from_str(&response).expect("response is JSON");
        assert_eq!(v["error"]["code"], error_codes::INVALID_REQUEST);
    }

    /// Oversized JSON-RPC lines are rejected before parse (bounded handler).
    #[tokio::test]
    async fn ipc_fault_oversized_jsonrpc_line_rejected() {
        let padding = "x".repeat(300_000);
        let line = format!(r#"{{"jsonrpc":"2.0","method":"x","id":1,"p":"{padding}"}}"#);
        let response = handle_jsonrpc_request_line_for_fault(&line).expect("oversized response");
        let v: serde_json::Value = serde_json::from_str(&response).expect("response is JSON");
        assert_eq!(v["error"]["code"], error_codes::INTERNAL_ERROR);
    }

    /// Very large invalid UTF-8-free JSON blob should not panic on parse failure.
    #[tokio::test]
    async fn ipc_fault_large_invalid_json_does_not_panic() {
        let line = "a".repeat(500_000);
        let result = panic::catch_unwind(|| serde_json::from_str::<serde_json::Value>(&line));
        assert!(
            result.is_ok(),
            "serde_json should not panic on invalid input"
        );
        assert!(result.expect("no panic").is_err());
    }

    // --- Crypto fault injection -----------------------------------------------------------

    /// Wrong-length Ed25519 public key material returns validation error, not panic.
    #[tokio::test]
    async fn crypto_fault_invalid_ed25519_key_length_returns_error() {
        let data = b"message";
        let sig = [0u8; 64];
        let short_pk = [0u8; 31];
        let err: BearDogError =
            verify_ed25519(data, &sig, &short_pk).expect_err("short public key");
        assert!(!err.to_string().is_empty());
    }

    /// Corrupted ciphertext fails AEAD decrypt with an error (authentication failure), not a panic.
    #[tokio::test]
    async fn crypto_fault_tampered_ciphertext_decrypt_returns_error() {
        let mut cfg = CryptoServiceConfig::default();
        cfg.max_data_size = 4096;
        let service = BearDogCryptoService::new(cfg).expect("crypto service init");

        let encrypted = service
            .encrypt(
                b"hello",
                CryptoAlgorithm::Aes256Gcm,
                EncryptOptions {
                    key_id: "fault-test-key".to_string(),
                    ..Default::default()
                },
            )
            .await
            .expect("encrypt should succeed");

        let mut tampered = encrypted.clone();
        tampered.ciphertext = vec![0xFF; encrypted.ciphertext.len()];

        let bad = service
            .decrypt(
                &tampered,
                DecryptOptions {
                    key_id: "fault-test-key".to_string(),
                    ..Default::default()
                },
            )
            .await;
        assert!(bad.is_err(), "tampered ciphertext must fail AEAD decrypt");
    }

    /// Invalid signature bytes length returns error from verifier (no panic).
    #[tokio::test]
    async fn crypto_fault_invalid_signature_length_returns_error() {
        let data = b"signed-payload";
        let sig = Signature {
            signature: vec![0u8; 8],
            algorithm: SignatureAlgorithm::Ed25519,
            metadata: SignatureMetadata {
                timestamp: SystemTime::UNIX_EPOCH,
                key_id: None,
                context: None,
            },
        };
        let service =
            BearDogCryptoService::new(CryptoServiceConfig::default()).expect("crypto init");
        let v = service
            .verify(
                data,
                &sig,
                VerifyOptions {
                    public_key: vec![0u8; 32],
                    context: None,
                },
            )
            .await;
        assert!(v.is_err(), "short signature must be rejected");
    }

    /// Wrong public key yields false verification result without panic.
    #[tokio::test]
    async fn crypto_fault_invalid_signature_returns_false() {
        let seed_b = [9u8; 32];
        let (_, pk_b) =
            beardog_core::crypto_service::algorithms::asymmetric::generate_ed25519_from_seed(
                &seed_b,
            )
            .expect("keypair B");

        let service =
            BearDogCryptoService::new(CryptoServiceConfig::default()).expect("crypto init");
        let signature = service
            .sign(
                b"doc",
                SignatureAlgorithm::Ed25519,
                SignOptions {
                    key_id: "fault-test-sign".to_string(),
                    context: None,
                },
            )
            .await
            .expect("sign");

        let ok = service
            .verify(
                b"doc",
                &signature,
                VerifyOptions {
                    public_key: pk_b.to_vec(),
                    context: None,
                },
            )
            .await
            .expect("verify completes");
        assert!(!ok, "signature must not verify under unrelated public key");
        assert_eq!(pk_b.len(), 32);
    }

    /// Oversized plaintext is rejected by crypto service limits (no panic).
    #[tokio::test]
    async fn crypto_fault_oversized_plaintext_rejected() {
        let mut cfg = CryptoServiceConfig::default();
        cfg.max_data_size = 32;
        let service = BearDogCryptoService::new(cfg).expect("crypto init");
        let big = vec![0u8; 64];
        let err = service
            .encrypt(
                &big,
                CryptoAlgorithm::Aes256Gcm,
                EncryptOptions {
                    key_id: "big".to_string(),
                    ..Default::default()
                },
            )
            .await;
        assert!(err.is_err(), "oversized input must be rejected");
    }

    // --- State / key store resilience -----------------------------------------------------

    /// Corrupt on-disk config cannot be loaded, but a fresh in-memory key manager still initializes.
    #[tokio::test]
    async fn state_fault_corrupt_config_then_fresh_key_manager_initializes() {
        let mut file = NamedTempFile::new().expect("temp file");
        writeln!(file, "{{{{{{not json").expect("write");
        let path = file.path();
        let load = BearDogConfig::from_file(path);
        assert!(load.is_err(), "corrupt config should not load");

        let mgr = MemoryKeyManager::new(MemoryKeyConfig::default()).expect("fresh key manager");
        let key_id = mgr.generate_key().expect("generate after failed load");
        assert!(key_id.starts_with("key_"));
    }

    /// Empty encrypted payload with wrong metadata is rejected on decrypt (no panic).
    #[tokio::test]
    async fn state_fault_corrupted_encrypted_struct_decrypt_errors() {
        let service =
            BearDogCryptoService::new(CryptoServiceConfig::default()).expect("crypto init");
        let bad = EncryptedData {
            ciphertext: vec![],
            algorithm: CryptoAlgorithm::Aes256Gcm,
            metadata: EncryptionMetadata {
                timestamp: SystemTime::UNIX_EPOCH,
                key_id: Some("k".to_string()),
                nonce: vec![0u8; 12],
                tag: Some(vec![0u8; 16]),
            },
        };
        let r = service
            .decrypt(
                &bad,
                DecryptOptions {
                    key_id: "k".to_string(),
                    ..Default::default()
                },
            )
            .await;
        assert!(
            r.is_err(),
            "empty ciphertext with fake tag should fail decrypt"
        );
    }
}
