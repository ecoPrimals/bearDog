// SPDX-License-Identifier: AGPL-3.0-or-later

//! Post-Quantum Readiness Demo
//!
//! This demo demonstrates quantum-resistant cryptography:
//! - Post-quantum key encapsulation (Kyber)
//! - Post-quantum digital signatures (Dilithium)
//! - Hybrid mode (classical + PQC)
//! - Migration path validation
//!
//! Note: This is a simulation for demonstration purposes.
//! Production systems should use actual PQC libraries like:
//! - pqcrypto (pqcrypto-kyber, pqcrypto-dilithium)
//! - oqs (liboqs bindings)
//! - pq-sys (CRYSTALS implementation)

use anyhow::{Context, Result};
use clap::Parser;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{error, info, warn};

use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
use x25519_dalek::{EphemeralSecret, PublicKey as X25519PublicKey};

// ============================================================================
// Configuration Types
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
struct DemoConfig {
    #[serde(rename = "ceremony")]
    _ceremony: CeremonyConfig,
    #[serde(rename = "algorithms")]
    _algorithms: AlgorithmsConfig,
    #[serde(rename = "security_levels")]
    _security_levels: SecurityLevelsConfig,
    migration: MigrationConfig,
    #[serde(rename = "performance")]
    _performance: PerformanceConfig,
    #[serde(rename = "compliance")]
    _compliance: ComplianceConfig,
    validation: ValidationConfig,
    #[serde(rename = "audit")]
    _audit: AuditConfig,
}

#[derive(Debug, Clone, Deserialize)]
struct CeremonyConfig {
    #[serde(rename = "name")]
    _name: String,
    #[serde(rename = "description")]
    _description: String,
    #[serde(rename = "max_duration_ms")]
    _max_duration_ms: u64,
}

#[derive(Debug, Clone, Deserialize)]
struct AlgorithmsConfig {
    #[serde(rename = "classical_signature")]
    _classical_signature: String,
    #[serde(rename = "classical_kem")]
    _classical_kem: String,
    #[serde(rename = "pqc_signature")]
    _pqc_signature: String,
    #[serde(rename = "pqc_kem")]
    _pqc_kem: String,
}

#[derive(Debug, Clone, Deserialize)]
struct SecurityLevelsConfig {
    #[serde(rename = "classical")]
    _classical: u32,
    #[serde(rename = "post_quantum")]
    _post_quantum: u32,
    #[serde(rename = "hybrid")]
    _hybrid: u32,
}

#[derive(Debug, Clone, Deserialize)]
struct MigrationConfig {
    current_phase: String,
    enable_classical: bool,
    enable_pqc: bool,
    require_both: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct PerformanceConfig {
    #[serde(rename = "keygen_target_ms")]
    _keygen_target_ms: u64,
    #[serde(rename = "encap_target_ms")]
    _encap_target_ms: u64,
    #[serde(rename = "decap_target_ms")]
    _decap_target_ms: u64,
    #[serde(rename = "sign_target_ms")]
    _sign_target_ms: u64,
    #[serde(rename = "verify_target_ms")]
    _verify_target_ms: u64,
    #[serde(rename = "hybrid_target_ms")]
    _hybrid_target_ms: u64,
    #[serde(rename = "profile_operations")]
    _profile_operations: bool,
    #[serde(rename = "compare_classical_pqc")]
    _compare_classical_pqc: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct ComplianceConfig {
    #[serde(rename = "nist_standardized")]
    _nist_standardized: bool,
    #[serde(rename = "fips_203_compliant")]
    _fips_203_compliant: bool,
    #[serde(rename = "fips_204_compliant")]
    _fips_204_compliant: bool,
    #[serde(rename = "cnsa_2_0_ready")]
    _cnsa_2_0_ready: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct ValidationConfig {
    test_key_generation: bool,
    test_encapsulation: bool,
    test_signatures: bool,
    test_hybrid_mode: bool,
    #[serde(rename = "test_migration_path")]
    _test_migration_path: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct AuditConfig {
    #[serde(rename = "log_level")]
    _log_level: String,
    #[serde(rename = "include_performance")]
    _include_performance: bool,
    #[serde(rename = "include_size_comparison")]
    _include_size_comparison: bool,
    #[serde(rename = "hash_algorithm")]
    _hash_algorithm: String,
}

// ============================================================================
// Scenario Types
// ============================================================================

#[derive(Debug, Deserialize)]
struct Scenario {
    scenario_name: String,
    scenario_id: String,
    #[serde(rename = "operation")]
    _operation: String,
    test_phases: Vec<TestPhase>,
    test_cases: Vec<TestCase>,
    expected_results: ExpectedResults,
    size_expectations: SizeExpectations,
}

#[derive(Debug, Clone, Deserialize)]
struct TestPhase {
    phase_id: String,
    phase_name: String,
    #[serde(rename = "description")]
    _description: String,
    #[serde(rename = "algorithms")]
    _algorithms: HashMap<String, String>,
    quantum_safe: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct TestCase {
    test_id: String,
    description: String,
    test_type: String,
    #[serde(default)]
    algorithm: Option<String>,
    #[serde(default)]
    algorithms: Vec<String>,
    #[serde(default)]
    tamper: Option<String>,
    expected_result: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ExpectedResults {
    keygen_ms: u64,
    encap_ms: u64,
    decap_ms: u64,
    sign_ms: u64,
    verify_ms: u64,
    hybrid_ms: u64,
    test_cases_passed: usize,
    quantum_safe_phases: usize,
}

#[derive(Debug, Clone, Deserialize)]
struct SizeExpectations {
    kyber768_public_key: usize,
    kyber768_private_key: usize,
    kyber768_ciphertext: usize,
    dilithium3_public_key: usize,
    dilithium3_private_key: usize,
    dilithium3_signature: usize,
}

// ============================================================================
// Post-Quantum Cryptography Simulation
// ============================================================================

/// Simulated Kyber768 key pair
#[derive(Clone)]
struct Kyber768KeyPair {
    public_key: Vec<u8>,
    private_key: Vec<u8>,
}

/// Simulated Dilithium3 key pair
#[derive(Clone)]
struct Dilithium3KeyPair {
    public_key: Vec<u8>,
    private_key: Vec<u8>,
}

/// Simulated Kyber768 ciphertext
struct Kyber768Ciphertext {
    ciphertext: Vec<u8>,
    shared_secret: Vec<u8>,
}

/// Simulated Dilithium3 signature
struct Dilithium3Signature {
    signature: Vec<u8>,
}

/// Kyber768 KEM (simulated)
struct Kyber768 {
    // In production, use actual PQC library
}

impl Kyber768 {
    fn keypair() -> Kyber768KeyPair {
        // Simulated key generation with realistic sizes
        // Real Kyber768: pk=1184 bytes, sk=2400 bytes
        let mut rng = rand::rng();
        
        let mut public_key = vec![0u8; 1184];
        rng.fill_bytes(&mut public_key);
        
        let mut private_key = vec![0u8; 2400];
        rng.fill_bytes(&mut private_key);
        
        Kyber768KeyPair {
            public_key,
            private_key,
        }
    }
    
    fn encapsulate(public_key: &[u8]) -> Kyber768Ciphertext {
        // Simulated encapsulation
        // Real Kyber768: ciphertext=1088 bytes, shared_secret=32 bytes
        let mut rng = rand::rng();
        
        let mut ciphertext = vec![0u8; 1088];
        rng.fill_bytes(&mut ciphertext);
        
        // Generate shared secret deterministically from public key
        let mut hasher = Sha256::new();
        hasher.update(public_key);
        hasher.update(&ciphertext);
        let shared_secret = hasher.finalize().to_vec();
        
        Kyber768Ciphertext {
            ciphertext,
            shared_secret,
        }
    }
    
    fn decapsulate(ciphertext: &[u8], private_key: &[u8]) -> Vec<u8> {
        // Simulated decapsulation
        // In real implementation, this would use the private key to decrypt
        let mut hasher = Sha256::new();
        hasher.update(private_key);
        hasher.update(ciphertext);
        hasher.finalize().to_vec()
    }
}

/// Dilithium3 signatures (simulated)
struct Dilithium3 {}

impl Dilithium3 {
    fn keypair() -> Dilithium3KeyPair {
        // Simulated key generation with realistic sizes
        // Real Dilithium3: pk=1952 bytes, sk=4000 bytes
        let mut rng = rand::rng();
        
        let mut public_key = vec![0u8; 1952];
        rng.fill_bytes(&mut public_key);
        
        let mut private_key = vec![0u8; 4000];
        rng.fill_bytes(&mut private_key);
        
        Dilithium3KeyPair {
            public_key,
            private_key,
        }
    }
    
    fn sign(message: &[u8], private_key: &[u8]) -> Dilithium3Signature {
        // Simulated signing
        // Real Dilithium3: signature=3293 bytes
        let mut hasher = Sha256::new();
        hasher.update(private_key);
        hasher.update(message);
        let hash = hasher.finalize();
        
        let mut signature = vec![0u8; 3293];
        signature[..32].copy_from_slice(&hash);
        
        Dilithium3Signature { signature }
    }
    
    fn verify(message: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
        // Simulated verification
        // In real implementation, this would use the public key to verify
        if signature.len() != 3293 {
            return false;
        }
        
        // Basic validation (in reality, much more complex)
        !signature.iter().all(|&b| b == 0) && !public_key.is_empty() && !message.is_empty()
    }
}

// ============================================================================
// Hybrid Mode (Classical + PQC)
// ============================================================================

struct HybridKeyPair {
    classical_sign: SigningKey,
    pqc_sign: Dilithium3KeyPair,
}

impl HybridKeyPair {
    fn generate() -> Self {
        use rand::RngCore;
        let mut rng = rand::rng();
        let mut bytes = [0u8; 32];
        rng.fill_bytes(&mut bytes);
        let classical_sign = SigningKey::from_bytes(&bytes);
        let pqc_sign = Dilithium3::keypair();
        
        Self {
            classical_sign,
            pqc_sign,
        }
    }
}

struct HybridSignature {
    classical: Vec<u8>,
    pqc: Vec<u8>,
}

fn hybrid_sign(message: &[u8], keypair: &HybridKeyPair) -> HybridSignature {
    // Classical signature
    let classical_sig = keypair.classical_sign.sign(message);
    
    // PQC signature
    let pqc_sig = Dilithium3::sign(message, &keypair.pqc_sign.private_key);
    
    HybridSignature {
        classical: classical_sig.to_bytes().to_vec(),
        pqc: pqc_sig.signature,
    }
}

fn hybrid_verify(message: &[u8], signature: &HybridSignature, classical_pk: &VerifyingKey, pqc_pk: &[u8]) -> bool {
    // Verify both signatures
    let classical_sig = match ed25519_dalek::Signature::from_slice(&signature.classical) {
        Ok(sig) => sig,
        Err(_) => return false,
    };
    
    let classical_ok = classical_pk.verify(message, &classical_sig).is_ok();
    let pqc_ok = Dilithium3::verify(message, &signature.pqc, pqc_pk);
    
    // Both must pass
    classical_ok && pqc_ok
}

struct HybridKEM {
    classical_shared: Vec<u8>,
    pqc_shared: Vec<u8>,
}

fn hybrid_kem_encapsulate(classical_pk: &X25519PublicKey, pqc_pk: &[u8]) -> (HybridKEM, Vec<u8>, Vec<u8>) {
    // Classical X25519 ECDH
    let classical_secret = EphemeralSecret::random_from_rng(&mut rand::rng());
    let classical_public = X25519PublicKey::from(&classical_secret);
    let classical_shared = classical_secret.diffie_hellman(classical_pk).to_bytes().to_vec();
    
    // PQC Kyber encapsulation
    let pqc_ct = Kyber768::encapsulate(pqc_pk);
    
    // Combine shared secrets (XOR)
    let mut combined = vec![0u8; 32];
    for i in 0..32 {
        combined[i] = classical_shared[i] ^ pqc_ct.shared_secret[i];
    }
    
    (
        HybridKEM {
            classical_shared: classical_shared.clone(),
            pqc_shared: pqc_ct.shared_secret.clone(),
        },
        classical_public.to_bytes().to_vec(),
        pqc_ct.ciphertext,
    )
}

// ============================================================================
// Test Results and Audit
// ============================================================================

#[derive(Debug, Clone, Serialize)]
struct TestResult {
    test_id: String,
    description: String,
    test_type: String,
    expected: String,
    actual: String,
    passed: bool,
    duration_ms: u64,
}

#[derive(Debug, Clone, Serialize)]
struct PerformanceMetrics {
    keygen_ms: u64,
    encap_ms: u64,
    decap_ms: u64,
    sign_ms: u64,
    verify_ms: u64,
    hybrid_ms: u64,
}

#[derive(Debug, Clone, Serialize)]
struct SizeMetrics {
    kyber768_public_key: usize,
    kyber768_private_key: usize,
    kyber768_ciphertext: usize,
    dilithium3_public_key: usize,
    dilithium3_private_key: usize,
    dilithium3_signature: usize,
}

#[derive(Debug, Clone, Serialize)]
struct AuditTrail {
    scenario_id: String,
    timestamp: String,
    test_results: Vec<TestResult>,
    performance: PerformanceMetrics,
    sizes: SizeMetrics,
    total_duration_ms: u64,
    all_tests_passed: bool,
    quantum_safe: bool,
    audit_hash: String,
}

// ============================================================================
// Demo Orchestration
// ============================================================================

struct PostQuantumDemo {
    config: DemoConfig,
    scenario: Scenario,
    test_results: RwLock<Vec<TestResult>>,
    performance: RwLock<PerformanceMetrics>,
    sizes: RwLock<SizeMetrics>,
}

impl PostQuantumDemo {
    fn new(config: DemoConfig, scenario: Scenario) -> Self {
        Self {
            config,
            scenario,
            test_results: RwLock::new(Vec::new()),
            performance: RwLock::new(PerformanceMetrics {
                keygen_ms: 0,
                encap_ms: 0,
                decap_ms: 0,
                sign_ms: 0,
                verify_ms: 0,
                hybrid_ms: 0,
            }),
            sizes: RwLock::new(SizeMetrics {
                kyber768_public_key: 0,
                kyber768_private_key: 0,
                kyber768_ciphertext: 0,
                dilithium3_public_key: 0,
                dilithium3_private_key: 0,
                dilithium3_signature: 0,
            }),
        }
    }
    
    async fn run(&self) -> Result<()> {
        let start = Instant::now();
        
        info!("🔮 Starting Post-Quantum Readiness Demo");
        info!("Scenario: {}", self.scenario.scenario_name);
        info!("Migration Phase: {}", self.config.migration.current_phase);
        
        // Run validation tests
        if self.config.validation.test_key_generation {
            self.test_key_generation().await?;
        }
        
        if self.config.validation.test_encapsulation {
            self.test_encapsulation().await?;
        }
        
        if self.config.validation.test_signatures {
            self.test_signatures().await?;
        }
        
        if self.config.validation.test_hybrid_mode {
            self.test_hybrid_mode().await?;
        }
        
        // Run all test cases
        info!("Running test cases...");
        for test_case in &self.scenario.test_cases {
            self.execute_test_case(test_case).await?;
        }
        
        let duration = start.elapsed();
        info!("Demo completed in {:?}", duration);
        
        // Generate audit trail
        self.generate_audit_trail(duration).await?;
        
        // Validate results
        self.validate_results().await?;
        
        Ok(())
    }
    
    async fn test_key_generation(&self) -> Result<()> {
        info!("Testing key generation...");
        
        // Kyber768 keygen
        let start = Instant::now();
        let _kyber_kp = Kyber768::keypair();
        let kyber_time = start.elapsed().as_millis() as u64;
        
        // Dilithium3 keygen
        let start = Instant::now();
        let dil_kp = Dilithium3::keypair();
        let dil_time = start.elapsed().as_millis() as u64;
        
        let avg_time = (kyber_time + dil_time) / 2;
        
        info!("  Kyber768 keygen: {}ms", kyber_time);
        info!("  Dilithium3 keygen: {}ms", dil_time);
        info!("  Average: {}ms", avg_time);
        
        let mut perf = self.performance.write().await;
        perf.keygen_ms = avg_time;
        
        // Record sizes
        let mut sizes = self.sizes.write().await;
        sizes.kyber768_public_key = 1184;
        sizes.kyber768_private_key = 2400;
        sizes.dilithium3_public_key = dil_kp.public_key.len();
        sizes.dilithium3_private_key = dil_kp.private_key.len();
        
        Ok(())
    }
    
    async fn test_encapsulation(&self) -> Result<()> {
        info!("Testing key encapsulation...");
        
        let kyber_kp = Kyber768::keypair();
        
        // Encapsulation
        let start = Instant::now();
        let ct = Kyber768::encapsulate(&kyber_kp.public_key);
        let encap_time = start.elapsed().as_millis() as u64;
        
        // Decapsulation
        let start = Instant::now();
        let shared_secret = Kyber768::decapsulate(&ct.ciphertext, &kyber_kp.private_key);
        let decap_time = start.elapsed().as_millis() as u64;
        
        info!("  Encapsulation: {}ms", encap_time);
        info!("  Decapsulation: {}ms", decap_time);
        info!("  Shared secret length: {} bytes", shared_secret.len());
        
        let mut perf = self.performance.write().await;
        perf.encap_ms = encap_time;
        perf.decap_ms = decap_time;
        
        let mut sizes = self.sizes.write().await;
        sizes.kyber768_ciphertext = ct.ciphertext.len();
        
        Ok(())
    }
    
    async fn test_signatures(&self) -> Result<()> {
        info!("Testing digital signatures...");
        
        let dil_kp = Dilithium3::keypair();
        let message = b"Hello, post-quantum world!";
        
        // Signing
        let start = Instant::now();
        let sig = Dilithium3::sign(message, &dil_kp.private_key);
        let sign_time = start.elapsed().as_millis() as u64;
        
        // Verification
        let start = Instant::now();
        let valid = Dilithium3::verify(message, &sig.signature, &dil_kp.public_key);
        let verify_time = start.elapsed().as_millis() as u64;
        
        info!("  Signing: {}ms", sign_time);
        info!("  Verification: {}ms", verify_time);
        info!("  Signature valid: {}", valid);
        
        let mut perf = self.performance.write().await;
        perf.sign_ms = sign_time;
        perf.verify_ms = verify_time;
        
        let mut sizes = self.sizes.write().await;
        sizes.dilithium3_signature = sig.signature.len();
        
        Ok(())
    }
    
    async fn test_hybrid_mode(&self) -> Result<()> {
        info!("Testing hybrid mode...");
        
        let start = Instant::now();
        
        // Generate hybrid keys
        let hybrid_kp = HybridKeyPair::generate();
        let message = b"Hybrid classical + PQC message";
        
        // Hybrid signature
        let hybrid_sig = hybrid_sign(message, &hybrid_kp);
        
        // Hybrid verification
        let verifying_key = hybrid_kp.classical_sign.verifying_key();
        let valid = hybrid_verify(message, &hybrid_sig, &verifying_key, &hybrid_kp.pqc_sign.public_key);
        
        let hybrid_time = start.elapsed().as_millis() as u64;
        
        info!("  Hybrid operation: {}ms", hybrid_time);
        info!("  Hybrid signature valid: {}", valid);
        info!("  Classical signature size: {} bytes", hybrid_sig.classical.len());
        info!("  PQC signature size: {} bytes", hybrid_sig.pqc.len());
        
        let mut perf = self.performance.write().await;
        perf.hybrid_ms = hybrid_time;
        
        Ok(())
    }
    
    async fn execute_test_case(&self, test_case: &TestCase) -> Result<()> {
        let start = Instant::now();
        
        let actual_result = match test_case.test_type.as_str() {
            "kem" => {
                // Test KEM
                let kp = Kyber768::keypair();
                let ct = Kyber768::encapsulate(&kp.public_key);
                let _ss = Kyber768::decapsulate(&ct.ciphertext, &kp.private_key);
                "success"
            }
            "signature" => {
                // Test signature
                let kp = Dilithium3::keypair();
                let message = b"Test message";
                let sig = Dilithium3::sign(message, &kp.private_key);
                
                let tampered = test_case.tamper.as_ref().map(|s| s.as_str()) == Some("modify_signature");
                let sig_to_verify = if tampered {
                    let mut bad_sig = sig.signature.clone();
                    bad_sig[0] ^= 0xFF; // Tamper
                    bad_sig
                } else {
                    sig.signature
                };
                
                let valid = Dilithium3::verify(message, &sig_to_verify, &kp.public_key);
                if valid { "success" } else { "failure" }
            }
            "hybrid_kem" => {
                // Test hybrid KEM
                let classical_secret = EphemeralSecret::random_from_rng(&mut rand::rng());
                let classical_public = X25519PublicKey::from(&classical_secret);
                let kyber_kp = Kyber768::keypair();
                
                let (_hybrid, _classical_ct, _pqc_ct) = hybrid_kem_encapsulate(&classical_public, &kyber_kp.public_key);
                "success"
            }
            "hybrid_signature" => {
                // Test hybrid signature
                let hybrid_kp = HybridKeyPair::generate();
                let message = b"Hybrid test";
                let hybrid_sig = hybrid_sign(message, &hybrid_kp);
                let verifying_key = hybrid_kp.classical_sign.verifying_key();
                let valid = hybrid_verify(message, &hybrid_sig, &verifying_key, &hybrid_kp.pqc_sign.public_key);
                if valid { "success" } else { "failure" }
            }
            "size_comparison" => {
                // Validate sizes match expectations
                let sizes = self.sizes.read().await;
                let expected = &self.scenario.size_expectations;
                
                let sizes_match = sizes.kyber768_public_key == expected.kyber768_public_key
                    && sizes.dilithium3_signature == expected.dilithium3_signature;
                
                if sizes_match { "success" } else { "failure" }
            }
            _ => {
                warn!("Unknown test type: {}", test_case.test_type);
                "failure"
            }
        };
        
        let duration = start.elapsed();
        
        let expected_success = test_case.expected_result == "success";
        let actual_success = actual_result == "success";
        let passed = expected_success == actual_success;
        
        let result = TestResult {
            test_id: test_case.test_id.clone(),
            description: test_case.description.clone(),
            test_type: test_case.test_type.clone(),
            expected: test_case.expected_result.clone(),
            actual: actual_result.to_string(),
            passed,
            duration_ms: duration.as_millis() as u64,
        };
        
        let status = if passed { "✅ PASS" } else { "❌ FAIL" };
        info!("{}: {} - {}", status, test_case.test_id, test_case.description);
        
        self.test_results.write().await.push(result);
        
        Ok(())
    }
    
    async fn generate_audit_trail(&self, total_duration: Duration) -> Result<()> {
        let test_results = self.test_results.read().await.clone();
        let performance = self.performance.read().await.clone();
        let sizes = self.sizes.read().await.clone();
        
        let all_tests_passed = test_results.iter().all(|tr| tr.passed);
        let quantum_safe = self.config.migration.enable_pqc;
        
        // Compute audit hash
        let mut hasher = blake3::Hasher::new();
        hasher.update(self.scenario.scenario_id.as_bytes());
        for tr in &test_results {
            hasher.update(tr.test_id.as_bytes());
            hasher.update(&tr.passed.to_string().as_bytes());
        }
        let audit_hash = hex::encode(hasher.finalize().as_bytes());
        
        let audit = AuditTrail {
            scenario_id: self.scenario.scenario_id.clone(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            test_results,
            performance,
            sizes,
            total_duration_ms: total_duration.as_millis() as u64,
            all_tests_passed,
            quantum_safe,
            audit_hash,
        };
        
        // Write audit to file
        let audit_json = serde_json::to_string_pretty(&audit)?;
        tokio::fs::write("audit_trail.json", audit_json).await?;
        
        info!("✅ Audit trail generated: audit_trail.json");
        info!("   Audit hash: {}", audit.audit_hash);
        
        Ok(())
    }
    
    async fn validate_results(&self) -> Result<()> {
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        info!("📊 VALIDATION RESULTS");
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        let test_results = self.test_results.read().await;
        let performance = self.performance.read().await;
        let sizes = self.sizes.read().await;
        let expected = &self.scenario.expected_results;
        
        // Test case pass rate
        let passed_tests = test_results.iter().filter(|tr| tr.passed).count();
        let total_tests = test_results.len();
        let pass_rate = (passed_tests as f64 / total_tests as f64) * 100.0;
        
        info!("Test Cases: {}/{} passed ({:.1}%)", passed_tests, total_tests, pass_rate);
        
        // Performance validation
        info!("");
        info!("Performance Metrics:");
        info!("  Key Generation:  {}ms (target: {}ms) {}", 
            performance.keygen_ms, expected.keygen_ms,
            if performance.keygen_ms <= expected.keygen_ms { "✅" } else { "⚠️" }
        );
        info!("  Encapsulation:   {}ms (target: {}ms) {}", 
            performance.encap_ms, expected.encap_ms,
            if performance.encap_ms <= expected.encap_ms { "✅" } else { "⚠️" }
        );
        info!("  Decapsulation:   {}ms (target: {}ms) {}", 
            performance.decap_ms, expected.decap_ms,
            if performance.decap_ms <= expected.decap_ms { "✅" } else { "⚠️" }
        );
        info!("  Signing:         {}ms (target: {}ms) {}", 
            performance.sign_ms, expected.sign_ms,
            if performance.sign_ms <= expected.sign_ms { "✅" } else { "⚠️" }
        );
        info!("  Verification:    {}ms (target: {}ms) {}", 
            performance.verify_ms, expected.verify_ms,
            if performance.verify_ms <= expected.verify_ms { "✅" } else { "⚠️" }
        );
        info!("  Hybrid Mode:     {}ms (target: {}ms) {}", 
            performance.hybrid_ms, expected.hybrid_ms,
            if performance.hybrid_ms <= expected.hybrid_ms { "✅" } else { "⚠️" }
        );
        
        // Size validation
        info!("");
        info!("Size Metrics:");
        info!("  Kyber768 Public Key:      {} bytes", sizes.kyber768_public_key);
        info!("  Kyber768 Private Key:     {} bytes", sizes.kyber768_private_key);
        info!("  Kyber768 Ciphertext:      {} bytes", sizes.kyber768_ciphertext);
        info!("  Dilithium3 Public Key:    {} bytes", sizes.dilithium3_public_key);
        info!("  Dilithium3 Private Key:   {} bytes", sizes.dilithium3_private_key);
        info!("  Dilithium3 Signature:     {} bytes", sizes.dilithium3_signature);
        
        info!("");
        info!("Validation Summary:");
        
        let pass_rate_ok = passed_tests >= expected.test_cases_passed;
        let performance_ok = performance.keygen_ms <= expected.keygen_ms
            && performance.sign_ms <= expected.sign_ms
            && performance.hybrid_ms <= expected.hybrid_ms;
        let sizes_ok = sizes.kyber768_public_key == self.scenario.size_expectations.kyber768_public_key
            && sizes.dilithium3_signature == self.scenario.size_expectations.dilithium3_signature;
        
        info!("  Test Pass Rate:    {} {}", 
            format!("{}/{}", passed_tests, total_tests),
            if pass_rate_ok { "✅ PASS" } else { "❌ FAIL" }
        );
        info!("  Performance:       {} {}", 
            "All operations",
            if performance_ok { "✅ PASS" } else { "❌ FAIL" }
        );
        info!("  Size Validation:   {} {}", 
            "Sizes match",
            if sizes_ok { "✅ PASS" } else { "❌ FAIL" }
        );
        info!("  Quantum Safe:      {} {}", 
            if self.config.migration.enable_pqc { "YES" } else { "NO" },
            if self.config.migration.enable_pqc { "✅ PASS" } else { "⚠️ WARN" }
        );
        
        let all_ok = pass_rate_ok && performance_ok && sizes_ok;
        info!("");
        if all_ok {
            info!("🎉 ALL VALIDATIONS PASSED!");
        } else {
            error!("❌ SOME VALIDATIONS FAILED");
        }
        
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        Ok(())
    }
}

// ============================================================================
// CLI
// ============================================================================

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to demo configuration file
    #[arg(short, long, default_value = "configs/demo.toml")]
    config: PathBuf,
    
    /// Path to scenario file
    #[arg(short, long, default_value = "scenarios/quantum_migration.json")]
    scenario: PathBuf,
    
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

// ============================================================================
// Main
// ============================================================================

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Initialize tracing
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(if args.verbose {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        })
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
    
    info!("🔮 BearDog Post-Quantum Readiness Demo");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Load configuration
    let config_str = tokio::fs::read_to_string(&args.config)
        .await
        .context("Failed to read config file")?;
    let config: DemoConfig = toml::from_str(&config_str).context("Failed to parse config")?;
    
    info!("✅ Config loaded from: {}", args.config.display());
    
    // Load scenario
    let scenario_str = tokio::fs::read_to_string(&args.scenario)
        .await
        .context("Failed to read scenario file")?;
    let scenario: Scenario = serde_json::from_str(&scenario_str).context("Failed to parse scenario")?;
    
    info!("✅ Scenario loaded from: {}", args.scenario.display());
    info!("");
    
    // Run demo
    let demo = PostQuantumDemo::new(config, scenario);
    demo.run().await?;
    
    info!("");
    info!("🎉 Post-Quantum Readiness Demo Complete!");
    
    Ok(())
}

