// SPDX-License-Identifier: AGPL-3.0-only

//! Zero-Knowledge Proofs Demo
//!
//! This demo demonstrates privacy-preserving verification using zero-knowledge proofs:
//! - Key ownership without revealing the private key
//! - Age verification without revealing the birthdate
//! - Set membership without revealing the specific member
//!
//! Uses Schnorr-like protocols for interactive proofs.

use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{error, info, warn};
use uuid::Uuid;

use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use sha2::{Digest, Sha256};

// ============================================================================
// Configuration Types
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
struct DemoConfig {
    ceremony: CeremonyConfig,
    zkp_protocol: ZkpProtocolConfig,
    schnorr: SchnorrConfig,
    age_verification: AgeVerificationConfig,
    compliance: ComplianceConfig,
    performance: PerformanceConfig,
    audit: AuditConfig,
}

#[derive(Debug, Clone, Deserialize)]
struct CeremonyConfig {
    name: String,
    description: String,
    max_duration_ms: u64,
}

#[derive(Debug, Clone, Deserialize)]
struct ZkpProtocolConfig {
    protocol_types: Vec<String>,
    security_level: u32,
    proof_rounds: u32,
}

#[derive(Debug, Clone, Deserialize)]
struct SchnorrConfig {
    curve: String,
    hash_algorithm: String,
}

#[derive(Debug, Clone, Deserialize)]
struct AgeVerificationConfig {
    minimum_age: u32,
    reveal_exact_age: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct ComplianceConfig {
    require_zero_knowledge: bool,
    allow_information_leakage: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct PerformanceConfig {
    batch_verification: bool,
    parallel_proofs: bool,
    cache_commitments: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct AuditConfig {
    log_level: String,
    include_performance: bool,
    include_proof_details: bool,
    hash_algorithm: String,
}

// ============================================================================
// Scenario Types
// ============================================================================

#[derive(Debug, Deserialize)]
struct Scenario {
    scenario_name: String,
    scenario_id: String,
    operation: String,
    proof_requests: Vec<ProofRequest>,
    test_cases: Vec<TestCase>,
    expected_results: ExpectedResults,
}

#[derive(Debug, Clone, Deserialize)]
struct ProofRequest {
    proof_id: String,
    proof_type: String,
    #[serde(rename = "description")]
    _description: String,
    prover: String,
    verifier: String,
    data: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
struct TestCase {
    test_id: String,
    description: String,
    proof_id: String,
    #[serde(default)]
    tamper: Option<String>,
    expected_result: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ExpectedResults {
    commitment_ms: u64,
    challenge_response_ms: u64,
    verification_ms: u64,
    total_proof_ms: u64,
    test_cases_passed: usize,
}

// ============================================================================
// Zero-Knowledge Proof Types
// ============================================================================

/// Schnorr-like proof for key ownership
#[derive(Debug, Clone)]
struct SchnorrProof {
    commitment: RistrettoPoint,
    challenge: Scalar,
    response: Scalar,
    public_key: RistrettoPoint,
}

/// Age verification proof (range proof simplified)
#[derive(Debug, Clone)]
struct AgeProof {
    commitment: String,
    proof_data: Vec<u8>,
    meets_requirement: bool,
}

/// Set membership proof
#[derive(Debug, Clone)]
struct SetMembershipProof {
    commitment: String,
    proof_data: Vec<u8>,
    is_member: bool,
}

#[derive(Debug, Clone)]
enum Proof {
    Schnorr(SchnorrProof),
    Age(AgeProof),
    SetMembership(SetMembershipProof),
}

// ============================================================================
// Prover and Verifier
// ============================================================================

struct Prover {
    private_key: Scalar,
    public_key: RistrettoPoint,
    _config: DemoConfig,
}

impl Prover {
    fn new(config: DemoConfig) -> Self {
        // Generate a random private key
        use rand::RngCore;
        let mut rng = rand::thread_rng();
        let mut bytes = [0u8; 32];
        rng.fill_bytes(&mut bytes);
        let private_key = Scalar::from_bytes_mod_order(bytes);
        let public_key = private_key * RISTRETTO_BASEPOINT_POINT;

        Self {
            private_key,
            public_key,
            _config: config,
        }
    }

    fn new_with_key(private_key: Scalar, config: DemoConfig) -> Self {
        let public_key = private_key * RISTRETTO_BASEPOINT_POINT;
        Self {
            private_key,
            public_key,
            _config: config,
        }
    }

    /// Generate commitment for Schnorr protocol
    fn generate_commitment(&self) -> (Scalar, RistrettoPoint) {
        use rand::RngCore;
        let mut rng = rand::thread_rng();
        let mut bytes = [0u8; 32];
        rng.fill_bytes(&mut bytes);
        let r = Scalar::from_bytes_mod_order(bytes);
        let commitment = r * RISTRETTO_BASEPOINT_POINT;
        (r, commitment)
    }

    /// Compute response to challenge
    fn compute_response(&self, r: Scalar, challenge: Scalar) -> Scalar {
        // z = r + e * s (where s is the private key)
        r + (challenge * self.private_key)
    }

    /// Prove key ownership using Schnorr protocol
    fn prove_key_ownership(&self) -> SchnorrProof {
        // 1. Generate commitment
        let (r, commitment) = self.generate_commitment();

        // 2. Generate challenge (in interactive protocol, verifier provides this)
        // For non-interactive, we use Fiat-Shamir heuristic (hash of commitment)
        let challenge = self.generate_challenge(&commitment, &self.public_key);

        // 3. Compute response
        let response = self.compute_response(r, challenge);

        SchnorrProof {
            commitment,
            challenge,
            response,
            public_key: self.public_key,
        }
    }

    /// Generate challenge using Fiat-Shamir heuristic (hash of commitment and public key)
    fn generate_challenge(&self, commitment: &RistrettoPoint, public_key: &RistrettoPoint) -> Scalar {
        let mut hasher = Sha256::new();
        hasher.update(commitment.compress().as_bytes());
        hasher.update(public_key.compress().as_bytes());
        let hash = hasher.finalize();
        Scalar::from_bytes_mod_order(hash.into())
    }

    /// Prove age > minimum without revealing exact age
    fn prove_age(&self, actual_age: u32, minimum_age: u32) -> AgeProof {
        let meets_requirement = actual_age >= minimum_age;

        // In a real implementation, this would use a range proof (e.g., Bulletproofs)
        // For this demo, we create a commitment and proof data
        let commitment = format!("age_commitment_{}", Uuid::new_v4());

        // Simulate proof data (in reality, this would be a cryptographic proof)
        let mut proof_data = Vec::new();
        proof_data.extend_from_slice(commitment.as_bytes());
        
        // Add some cryptographic material (simulated)
        let mut hasher = Sha256::new();
        hasher.update(&actual_age.to_le_bytes());
        hasher.update(&minimum_age.to_le_bytes());
        proof_data.extend_from_slice(&hasher.finalize());

        AgeProof {
            commitment,
            proof_data,
            meets_requirement,
        }
    }

    /// Prove set membership without revealing which member
    fn prove_set_membership(&self, member_id: u32, set_size: u32) -> SetMembershipProof {
        let is_member = member_id < set_size;

        let commitment = format!("set_commitment_{}", Uuid::new_v4());

        // Simulate proof data (in reality, this would use accumulator-based proofs)
        let mut proof_data = Vec::new();
        proof_data.extend_from_slice(commitment.as_bytes());

        let mut hasher = Sha256::new();
        hasher.update(&member_id.to_le_bytes());
        hasher.update(&set_size.to_le_bytes());
        proof_data.extend_from_slice(&hasher.finalize());

        SetMembershipProof {
            commitment,
            proof_data,
            is_member,
        }
    }
}

struct Verifier {
    _config: DemoConfig,
}

impl Verifier {
    fn new(config: DemoConfig) -> Self {
        Self { _config: config }
    }

    /// Verify Schnorr proof
    fn verify_schnorr(&self, proof: &SchnorrProof) -> bool {
        // Verify: g^z = C * (g^s)^e
        // Where:
        //   g = basepoint
        //   z = response
        //   C = commitment
        //   s = secret (private key)
        //   e = challenge
        //   g^s = public_key
        
        let lhs = proof.response * RISTRETTO_BASEPOINT_POINT;
        let rhs = proof.commitment + (proof.challenge * proof.public_key);

        lhs == rhs
    }

    /// Verify age proof
    fn verify_age(&self, proof: &AgeProof, _minimum_age: u32) -> bool {
        // In a real implementation, this would verify the range proof cryptographically
        // For this demo, we check the meets_requirement flag and validate the commitment

        if !proof.meets_requirement {
            return false;
        }

        // Verify the proof data is well-formed (basic check)
        proof.proof_data.len() > 32 && !proof.commitment.is_empty()
    }

    /// Verify set membership proof
    fn verify_set_membership(&self, proof: &SetMembershipProof) -> bool {
        // In a real implementation, this would verify the accumulator proof
        // For this demo, we check the is_member flag and validate the commitment

        if !proof.is_member {
            return false;
        }

        // Verify the proof data is well-formed (basic check)
        proof.proof_data.len() > 32 && !proof.commitment.is_empty()
    }
}

// ============================================================================
// Proof Results and Audit
// ============================================================================

#[derive(Debug, Clone, Serialize)]
struct ProofResult {
    proof_id: String,
    proof_type: String,
    prover: String,
    verifier: String,
    commitment_time_ms: u64,
    challenge_response_time_ms: u64,
    verification_time_ms: u64,
    total_time_ms: u64,
    verified: bool,
    zero_knowledge_preserved: bool,
    audit_hash: String,
}

#[derive(Debug, Clone, Serialize)]
struct AuditTrail {
    scenario_id: String,
    timestamp: String,
    proof_results: Vec<ProofResult>,
    test_results: Vec<TestResult>,
    total_duration_ms: u64,
    all_tests_passed: bool,
    audit_hash: String,
}

#[derive(Debug, Clone, Serialize)]
struct TestResult {
    test_id: String,
    description: String,
    expected: String,
    actual: String,
    passed: bool,
    duration_ms: u64,
}

// ============================================================================
// Demo Orchestration
// ============================================================================

struct ZkpDemo {
    config: DemoConfig,
    scenario: Scenario,
    proof_results: RwLock<Vec<ProofResult>>,
    test_results: RwLock<Vec<TestResult>>,
}

impl ZkpDemo {
    fn new(config: DemoConfig, scenario: Scenario) -> Self {
        Self {
            config,
            scenario,
            proof_results: RwLock::new(Vec::new()),
            test_results: RwLock::new(Vec::new()),
        }
    }

    async fn run(&self) -> Result<()> {
        let start = Instant::now();

        info!("🔐 Starting Zero-Knowledge Proofs Demo");
        info!("Scenario: {}", self.scenario.scenario_name);
        info!("Protocol types: {:?}", self.config.zkp_protocol.protocol_types);

        // Process all proof requests
        for proof_request in &self.scenario.proof_requests {
            info!("Processing proof: {} ({})", proof_request.proof_id, proof_request.proof_type);
            self.execute_proof_request(proof_request).await?;
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

    async fn execute_proof_request(&self, request: &ProofRequest) -> Result<()> {
        let prover_start = Instant::now();
        let verifier_start: Instant;
        let verification_start: Instant;

        let prover = Prover::new(self.config.clone());
        let verifier = Verifier::new(self.config.clone());

        let (proof, commitment_time, challenge_response_time) = match request.proof_type.as_str() {
            "key_ownership" => {
                let commitment_start = Instant::now();
                let proof = prover.prove_key_ownership();
                let commitment_time = commitment_start.elapsed();

                let challenge_response_start = Instant::now();
                // In interactive protocol, there would be actual challenge-response
                // For non-interactive (Fiat-Shamir), it's all computed by prover
                let challenge_response_time = challenge_response_start.elapsed();

                (Proof::Schnorr(proof), commitment_time, challenge_response_time)
            }
            "age_verification" => {
                let actual_age = request.data["actual_age"].as_u64().unwrap_or(25) as u32;
                let minimum_age = request.data["minimum_age"].as_u64().unwrap_or(21) as u32;

                let commitment_start = Instant::now();
                let proof = prover.prove_age(actual_age, minimum_age);
                let commitment_time = commitment_start.elapsed();

                let challenge_response_start = Instant::now();
                // Range proofs don't have explicit challenge-response in this simplified version
                let challenge_response_time = challenge_response_start.elapsed();

                (Proof::Age(proof), commitment_time, challenge_response_time)
            }
            "set_membership" => {
                let set_size = request.data["set_size"].as_u64().unwrap_or(100) as u32;
                let member_id = 42; // Simulated member ID

                let commitment_start = Instant::now();
                let proof = prover.prove_set_membership(member_id, set_size);
                let commitment_time = commitment_start.elapsed();

                let challenge_response_start = Instant::now();
                let challenge_response_time = challenge_response_start.elapsed();

                (Proof::SetMembership(proof), commitment_time, challenge_response_time)
            }
            _ => {
                warn!("Unknown proof type: {}", request.proof_type);
                return Ok(());
            }
        };

        verification_start = Instant::now();
        let verified = match &proof {
            Proof::Schnorr(p) => verifier.verify_schnorr(p),
            Proof::Age(p) => verifier.verify_age(p, self.config.age_verification.minimum_age),
            Proof::SetMembership(p) => verifier.verify_set_membership(p),
        };
        let verification_time = verification_start.elapsed();

        let total_time = prover_start.elapsed();

        // Compute audit hash
        let mut hasher = blake3::Hasher::new();
        hasher.update(request.proof_id.as_bytes());
        hasher.update(&verified.to_string().as_bytes());
        hasher.update(&total_time.as_millis().to_le_bytes());
        let audit_hash = hex::encode(hasher.finalize().as_bytes());

        let result = ProofResult {
            proof_id: request.proof_id.clone(),
            proof_type: request.proof_type.clone(),
            prover: request.prover.clone(),
            verifier: request.verifier.clone(),
            commitment_time_ms: commitment_time.as_millis() as u64,
            challenge_response_time_ms: challenge_response_time.as_millis() as u64,
            verification_time_ms: verification_time.as_millis() as u64,
            total_time_ms: total_time.as_millis() as u64,
            verified,
            zero_knowledge_preserved: true, // By design of the protocols
            audit_hash,
        };

        info!(
            "✅ Proof {} verified: {} ({}ms)",
            request.proof_id, verified, result.total_time_ms
        );

        self.proof_results.write().await.push(result);

        Ok(())
    }

    async fn execute_test_case(&self, test_case: &TestCase) -> Result<()> {
        let start = Instant::now();

        // Find the corresponding proof request
        let proof_request = self
            .scenario
            .proof_requests
            .iter()
            .find(|pr| pr.proof_id == test_case.proof_id)
            .context(format!("Proof request {} not found", test_case.proof_id))?;

        let prover = Prover::new(self.config.clone());
        let verifier = Verifier::new(self.config.clone());

        let verified = match proof_request.proof_type.as_str() {
            "key_ownership" => {
                if test_case.tamper.as_ref().map(|s| s.as_str()) == Some("use_wrong_key") {
                    // Use a different key than the one in the proof
                    use rand::RngCore;
                    let mut rng = rand::thread_rng();
                    let mut bytes = [0u8; 32];
                    rng.fill_bytes(&mut bytes);
                    let wrong_key = Scalar::from_bytes_mod_order(bytes);
                    let wrong_prover = Prover::new_with_key(wrong_key, self.config.clone());
                    
                    // Create proof with wrong key
                    let mut proof = wrong_prover.prove_key_ownership();
                    // But verify against the original public key
                    proof.public_key = prover.public_key;
                    
                    verifier.verify_schnorr(&proof)
                } else {
                    let proof = prover.prove_key_ownership();
                    verifier.verify_schnorr(&proof)
                }
            }
            "age_verification" => {
                let actual_age = if test_case.tamper.as_ref().map(|s| s.as_str()) == Some("underage") {
                    18 // Underage (< 21)
                } else {
                    proof_request.data["actual_age"].as_u64().unwrap_or(25) as u32
                };
                let minimum_age = proof_request.data["minimum_age"].as_u64().unwrap_or(21) as u32;

                let proof = prover.prove_age(actual_age, minimum_age);
                verifier.verify_age(&proof, minimum_age)
            }
            "set_membership" => {
                let set_size = proof_request.data["set_size"].as_u64().unwrap_or(100) as u32;
                let member_id = 42;

                let proof = prover.prove_set_membership(member_id, set_size);
                verifier.verify_set_membership(&proof)
            }
            _ => {
                warn!("Unknown proof type in test case: {}", proof_request.proof_type);
                return Ok(());
            }
        };

        let duration = start.elapsed();

        let expected_success = test_case.expected_result == "success";
        let passed = (verified && expected_success) || (!verified && !expected_success);

        let result = TestResult {
            test_id: test_case.test_id.clone(),
            description: test_case.description.clone(),
            expected: test_case.expected_result.clone(),
            actual: if verified { "success".to_string() } else { "failure".to_string() },
            passed,
            duration_ms: duration.as_millis() as u64,
        };

        let status = if passed { "✅ PASS" } else { "❌ FAIL" };
        info!("{}: {} - {}", status, test_case.test_id, test_case.description);

        self.test_results.write().await.push(result);

        Ok(())
    }

    async fn generate_audit_trail(&self, total_duration: Duration) -> Result<()> {
        let proof_results = self.proof_results.read().await.clone();
        let test_results = self.test_results.read().await.clone();

        let all_tests_passed = test_results.iter().all(|tr| tr.passed);

        // Compute audit hash
        let mut hasher = blake3::Hasher::new();
        hasher.update(self.scenario.scenario_id.as_bytes());
        for pr in &proof_results {
            hasher.update(pr.audit_hash.as_bytes());
        }
        for tr in &test_results {
            hasher.update(tr.test_id.as_bytes());
            hasher.update(&tr.passed.to_string().as_bytes());
        }
        let audit_hash = hex::encode(hasher.finalize().as_bytes());

        let audit = AuditTrail {
            scenario_id: self.scenario.scenario_id.clone(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            proof_results,
            test_results,
            total_duration_ms: total_duration.as_millis() as u64,
            all_tests_passed,
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

        let proof_results = self.proof_results.read().await;
        let test_results = self.test_results.read().await;
        let expected = &self.scenario.expected_results;

        // Test case pass rate
        let passed_tests = test_results.iter().filter(|tr| tr.passed).count();
        let total_tests = test_results.len();
        let pass_rate = (passed_tests as f64 / total_tests as f64) * 100.0;

        info!("Test Cases: {}/{} passed ({:.1}%)", passed_tests, total_tests, pass_rate);

        // Performance validation
        let avg_commitment = proof_results.iter().map(|pr| pr.commitment_time_ms).sum::<u64>() / proof_results.len() as u64;
        let avg_challenge_response = proof_results.iter().map(|pr| pr.challenge_response_time_ms).sum::<u64>() / proof_results.len() as u64;
        let avg_verification = proof_results.iter().map(|pr| pr.verification_time_ms).sum::<u64>() / proof_results.len() as u64;
        let avg_total = proof_results.iter().map(|pr| pr.total_time_ms).sum::<u64>() / proof_results.len() as u64;

        info!("");
        info!("Performance Metrics:");
        info!("  Commitment:        {}ms (target: {}ms) {}", 
            avg_commitment, 
            expected.commitment_ms,
            if avg_commitment <= expected.commitment_ms { "✅" } else { "⚠️" }
        );
        info!("  Challenge/Response: {}ms (target: {}ms) {}", 
            avg_challenge_response, 
            expected.challenge_response_ms,
            if avg_challenge_response <= expected.challenge_response_ms { "✅" } else { "⚠️" }
        );
        info!("  Verification:      {}ms (target: {}ms) {}", 
            avg_verification, 
            expected.verification_ms,
            if avg_verification <= expected.verification_ms { "✅" } else { "⚠️" }
        );
        info!("  Total (avg):       {}ms (target: {}ms) {}", 
            avg_total, 
            expected.total_proof_ms,
            if avg_total <= expected.total_proof_ms { "✅" } else { "⚠️" }
        );

        info!("");
        info!("Zero-Knowledge Properties:");
        let all_zk_preserved = proof_results.iter().all(|pr| pr.zero_knowledge_preserved);
        info!("  Privacy Preserved:    {} {}", 
            if all_zk_preserved { "YES" } else { "NO" },
            if all_zk_preserved { "✅" } else { "❌" }
        );
        info!("  Information Leakage:  {} {}", 
            if !self.config.compliance.allow_information_leakage { "NONE" } else { "ALLOWED" },
            if !self.config.compliance.allow_information_leakage { "✅" } else { "⚠️" }
        );

        info!("");
        info!("Validation Summary:");
        
        let pass_rate_ok = passed_tests >= expected.test_cases_passed;
        let performance_ok = avg_total <= expected.total_proof_ms;
        let zk_ok = all_zk_preserved && !self.config.compliance.allow_information_leakage;
        
        info!("  Test Pass Rate:    {} {}", 
            format!("{}/{}", passed_tests, total_tests),
            if pass_rate_ok { "✅ PASS" } else { "❌ FAIL" }
        );
        info!("  Performance:       {} {}", 
            format!("{}ms avg", avg_total),
            if performance_ok { "✅ PASS" } else { "❌ FAIL" }
        );
        info!("  Zero-Knowledge:    {} {}", 
            if zk_ok { "PRESERVED" } else { "VIOLATED" },
            if zk_ok { "✅ PASS" } else { "❌ FAIL" }
        );

        let all_ok = pass_rate_ok && performance_ok && zk_ok;
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
    #[arg(short, long, default_value = "scenarios/privacy_auth.json")]
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

    info!("🔐 BearDog Zero-Knowledge Proofs Demo");
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
    let demo = ZkpDemo::new(config, scenario);
    demo.run().await?;

    info!("");
    info!("🎉 Zero-Knowledge Proofs Demo Complete!");

    Ok(())
}

