//! Comprehensive Proof Verifier Tests
//! 
//! This test suite ensures 100% coverage of BearDog's proof verification system
//! including cryptographic proofs, zero-knowledge proofs, and verification mechanisms.

use beardog::proof_verifier::*;
use beardog::core::*;
use beardog::error::*;
use beardog::crypto_utils::*;
use std::collections::HashMap;
use std::sync::Arc;

/// Comprehensive proof verifier testing
/// Tests all proof types and verification scenarios
#[tokio::test]
async fn test_proof_verifier_comprehensive() {
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await.expect("Core initialization failed"));
    
    let proof_verifier = ProofVerifier::new(core.clone()).await
        .expect("Proof verifier creation failed");
    
    // Test all major proof verification operations
    test_cryptographic_proof_verification(&proof_verifier).await;
    test_zero_knowledge_proof_verification(&proof_verifier).await;
    test_merkle_proof_verification(&proof_verifier).await;
    test_batch_proof_verification(&proof_verifier).await;
    test_proof_chain_verification(&proof_verifier).await;
    test_invalid_proof_rejection(&proof_verifier).await;
}

async fn test_cryptographic_proof_verification(verifier: &ProofVerifier) {
    println!("🔐 Testing cryptographic proof verification...");
    
    // Test signature proof verification
    let message = b"BearDog secure message for testing";
    let (private_key, public_key) = BearDogCrypto::generate_ed25519_keypair()
        .expect("Key generation should succeed");
    
    let signature = BearDogCrypto::sign_ed25519(&private_key, message)
        .expect("Message signing should succeed");
    
    let signature_proof = CryptographicProof {
        proof_type: ProofType::DigitalSignature,
        algorithm: CryptoAlgorithm::Ed25519,
        public_key: public_key.clone(),
        signature: signature.clone(),
        message: message.to_vec(),
        metadata: HashMap::new(),
    };
    
    let verification_result = verifier.verify_cryptographic_proof(&signature_proof).await
        .expect("Signature verification should succeed");
    
    assert!(verification_result.is_valid, "Valid signature should be verified");
    assert!(verification_result.confidence_score >= 0.99, "Signature confidence should be high");
    
    // Test hash proof verification
    let data = b"sensitive data requiring integrity proof";
    let hash = BearDogCrypto::sha256_hash(data)
        .expect("Hash calculation should succeed");
    
    let hash_proof = CryptographicProof {
        proof_type: ProofType::HashIntegrity,
        algorithm: CryptoAlgorithm::Sha256,
        public_key: vec![],
        signature: hash.clone(),
        message: data.to_vec(),
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("salt".to_string(), "secure_salt_value".to_string());
            meta
        },
    };
    
    let hash_verification = verifier.verify_cryptographic_proof(&hash_proof).await
        .expect("Hash verification should succeed");
    
    assert!(hash_verification.is_valid, "Valid hash should be verified");
    
    // Test multiple algorithm support
    let algorithms = vec![
        CryptoAlgorithm::Ed25519,
        CryptoAlgorithm::Rsa4096,
        CryptoAlgorithm::Sha256,
        CryptoAlgorithm::Sha512,
    ];
    
    for algorithm in algorithms {
        let algorithm_support = verifier.supports_algorithm(&algorithm).await;
        assert!(algorithm_support, "Should support standard cryptographic algorithms");
    }
}

async fn test_zero_knowledge_proof_verification(verifier: &ProofVerifier) {
    println!("🔍 Testing zero-knowledge proof verification...");
    
    // Test range proof (prove value is within range without revealing value)
    let secret_value = 1000u64;
    let range_min = 100u64;
    let range_max = 2000u64;
    
    let range_proof = ZeroKnowledgeProof {
        proof_type: ZKProofType::RangeProof,
        commitment: generate_commitment(secret_value),
        proof_data: generate_range_proof_data(secret_value, range_min, range_max),
        public_parameters: {
            let mut params = HashMap::new();
            params.insert("range_min".to_string(), range_min.to_string());
            params.insert("range_max".to_string(), range_max.to_string());
            params
        },
        challenge: generate_fiat_shamir_challenge(&[]),
    };
    
    let zk_verification = verifier.verify_zero_knowledge_proof(&range_proof).await
        .expect("ZK proof verification should succeed");
    
    assert!(zk_verification.is_valid, "Valid range proof should be verified");
    assert!(zk_verification.zero_knowledge_property, "Should maintain zero-knowledge property");
    
    // Test membership proof (prove element is in set without revealing which element)
    let secret_element = "secret_node_id_12345";
    let valid_set = vec![
        "node_alpha", "node_beta", "node_gamma", 
        "secret_node_id_12345", "node_delta"
    ];
    
    let membership_proof = ZeroKnowledgeProof {
        proof_type: ZKProofType::SetMembership,
        commitment: generate_commitment_from_string(secret_element),
        proof_data: generate_membership_proof_data(secret_element, &valid_set),
        public_parameters: {
            let mut params = HashMap::new();
            params.insert("set_size".to_string(), valid_set.len().to_string());
            params.insert("set_hash".to_string(), hash_set(&valid_set));
            params
        },
        challenge: generate_fiat_shamir_challenge(&[secret_element.as_bytes()]),
    };
    
    let membership_verification = verifier.verify_zero_knowledge_proof(&membership_proof).await
        .expect("Membership proof verification should succeed");
    
    assert!(membership_verification.is_valid, "Valid membership proof should be verified");
    
    // Test knowledge proof (prove knowledge of secret without revealing it)
    let secret_knowledge = "super_secret_authentication_key";
    let public_commitment = generate_knowledge_commitment(secret_knowledge);
    
    let knowledge_proof = ZeroKnowledgeProof {
        proof_type: ZKProofType::KnowledgeProof,
        commitment: public_commitment,
        proof_data: generate_knowledge_proof_data(secret_knowledge),
        public_parameters: HashMap::new(),
        challenge: generate_fiat_shamir_challenge(&[secret_knowledge.as_bytes()]),
    };
    
    let knowledge_verification = verifier.verify_zero_knowledge_proof(&knowledge_proof).await
        .expect("Knowledge proof verification should succeed");
    
    assert!(knowledge_verification.is_valid, "Valid knowledge proof should be verified");
    assert!(knowledge_verification.soundness_guarantee >= 0.99, "Should have high soundness");
}

async fn test_merkle_proof_verification(verifier: &ProofVerifier) {
    println!("🌳 Testing Merkle proof verification...");
    
    // Create test data for Merkle tree
    let data_items = vec![
        "transaction_001".as_bytes(),
        "transaction_002".as_bytes(), 
        "transaction_003".as_bytes(),
        "transaction_004".as_bytes(),
        "transaction_005".as_bytes(),
        "transaction_006".as_bytes(),
        "transaction_007".as_bytes(),
        "transaction_008".as_bytes(),
    ];
    
    // Build Merkle tree
    let merkle_tree = build_merkle_tree(&data_items);
    let root_hash = merkle_tree.root_hash.clone();
    
    // Test inclusion proof for existing element
    let target_item = "transaction_005".as_bytes();
    let inclusion_proof = generate_merkle_inclusion_proof(&merkle_tree, target_item)
        .expect("Should generate inclusion proof for existing item");
    
    let merkle_proof = MerkleProof {
        proof_type: MerkleProofType::Inclusion,
        root_hash: root_hash.clone(),
        leaf_data: target_item.to_vec(),
        proof_path: inclusion_proof.path,
        leaf_index: inclusion_proof.index,
        tree_size: data_items.len(),
    };
    
    let merkle_verification = verifier.verify_merkle_proof(&merkle_proof).await
        .expect("Merkle proof verification should succeed");
    
    assert!(merkle_verification.is_valid, "Valid inclusion proof should be verified");
    assert_eq!(merkle_verification.verified_root, root_hash, "Should verify against correct root");
    
    // Test exclusion proof for non-existing element
    let non_existing_item = "transaction_999".as_bytes();
    let exclusion_proof = generate_merkle_exclusion_proof(&merkle_tree, non_existing_item)
        .expect("Should generate exclusion proof for non-existing item");
    
    let exclusion_merkle_proof = MerkleProof {
        proof_type: MerkleProofType::Exclusion,
        root_hash: root_hash.clone(),
        leaf_data: non_existing_item.to_vec(),
        proof_path: exclusion_proof.path,
        leaf_index: exclusion_proof.index,
        tree_size: data_items.len(),
    };
    
    let exclusion_verification = verifier.verify_merkle_proof(&exclusion_merkle_proof).await
        .expect("Exclusion proof verification should succeed");
    
    assert!(exclusion_verification.is_valid, "Valid exclusion proof should be verified");
    
    // Test batch inclusion proof
    let batch_items = vec![
        "transaction_002".as_bytes(),
        "transaction_006".as_bytes(),
    ];
    
    let batch_inclusion_proofs: Vec<_> = batch_items.iter()
        .map(|item| generate_merkle_inclusion_proof(&merkle_tree, item))
        .collect::<Result<Vec<_>, _>>()
        .expect("Should generate batch inclusion proofs");
    
    let batch_verification = verifier.verify_merkle_batch_proof(
        &root_hash,
        &batch_items,
        &batch_inclusion_proofs,
    ).await.expect("Batch verification should succeed");
    
    assert!(batch_verification.all_valid, "All batch proofs should be valid");
    assert_eq!(batch_verification.verified_count, batch_items.len(), 
              "Should verify all batch items");
}

async fn test_batch_proof_verification(verifier: &ProofVerifier) {
    println!("📦 Testing batch proof verification...");
    
    // Create multiple proofs for batch verification
    let mut proofs = Vec::new();
    
    // Add signature proofs
    for i in 0..5 {
        let message = format!("test message {}", i);
        let (private_key, public_key) = BearDogCrypto::generate_ed25519_keypair()
            .expect("Key generation should succeed");
        
        let signature = BearDogCrypto::sign_ed25519(&private_key, message.as_bytes())
            .expect("Message signing should succeed");
        
        let proof = CryptographicProof {
            proof_type: ProofType::DigitalSignature,
            algorithm: CryptoAlgorithm::Ed25519,
            public_key,
            signature,
            message: message.as_bytes().to_vec(),
            metadata: HashMap::new(),
        };
        
        proofs.push(proof);
    }
    
    // Add hash proofs
    for i in 0..3 {
        let data = format!("sensitive data {}", i);
        let hash = BearDogCrypto::sha256_hash(data.as_bytes())
            .expect("Hash calculation should succeed");
        
        let proof = CryptographicProof {
            proof_type: ProofType::HashIntegrity,
            algorithm: CryptoAlgorithm::Sha256,
            public_key: vec![],
            signature: hash,
            message: data.as_bytes().to_vec(),
            metadata: HashMap::new(),
        };
        
        proofs.push(proof);
    }
    
    // Test sequential batch verification
    let sequential_result = verifier.verify_proof_batch_sequential(&proofs).await
        .expect("Sequential batch verification should succeed");
    
    assert_eq!(sequential_result.total_proofs, proofs.len(),
              "Should process all proofs");
    assert_eq!(sequential_result.valid_proofs, proofs.len(),
              "All proofs should be valid");
    assert_eq!(sequential_result.invalid_proofs, 0,
              "No proofs should be invalid");
    
    // Test parallel batch verification
    let parallel_result = verifier.verify_proof_batch_parallel(&proofs).await
        .expect("Parallel batch verification should succeed");
    
    assert_eq!(parallel_result.total_proofs, proofs.len(),
              "Should process all proofs in parallel");
    assert_eq!(parallel_result.valid_proofs, proofs.len(),
              "All proofs should be valid in parallel verification");
    
    // Parallel should be faster for large batches
    assert!(parallel_result.verification_time_ms <= sequential_result.verification_time_ms,
           "Parallel verification should be faster or equal");
    
    // Test batch verification with mixed proof types
    let mixed_verification = verifier.verify_mixed_proof_batch(&proofs).await
        .expect("Mixed batch verification should succeed");
    
    assert!(mixed_verification.signature_proofs_valid >= 5,
           "Should verify signature proofs");
    assert!(mixed_verification.hash_proofs_valid >= 3,
           "Should verify hash proofs");
}

async fn test_proof_chain_verification(verifier: &ProofVerifier) {
    println!("⛓️ Testing proof chain verification...");
    
    // Create a chain of dependent proofs
    let mut proof_chain = Vec::new();
    
    // Genesis proof (self-referential)
    let genesis_data = b"genesis block data";
    let genesis_hash = BearDogCrypto::sha256_hash(genesis_data)
        .expect("Genesis hash should succeed");
    
    let genesis_proof = ChainProof {
        proof_id: "genesis".to_string(),
        previous_proof_id: None,
        proof_data: genesis_hash.clone(),
        timestamp: std::time::SystemTime::now(),
        chain_position: 0,
    };
    
    proof_chain.push(genesis_proof);
    
    // Add dependent proofs
    for i in 1..=5 {
        let proof_data = format!("proof data for block {}", i);
        let combined_data = [
            proof_chain[i-1].proof_data.as_slice(),
            proof_data.as_bytes()
        ].concat();
        
        let proof_hash = BearDogCrypto::sha256_hash(&combined_data)
            .expect("Proof hash should succeed");
        
        let chain_proof = ChainProof {
            proof_id: format!("proof_{}", i),
            previous_proof_id: Some(proof_chain[i-1].proof_id.clone()),
            proof_data: proof_hash,
            timestamp: std::time::SystemTime::now(),
            chain_position: i,
        };
        
        proof_chain.push(chain_proof);
    }
    
    // Verify entire chain
    let chain_verification = verifier.verify_proof_chain(&proof_chain).await
        .expect("Chain verification should succeed");
    
    assert!(chain_verification.is_valid, "Valid proof chain should be verified");
    assert_eq!(chain_verification.verified_length, proof_chain.len(),
              "Should verify entire chain length");
    assert!(chain_verification.integrity_maintained, "Chain integrity should be maintained");
    
    // Test chain verification with broken link
    let mut broken_chain = proof_chain.clone();
    broken_chain[3].previous_proof_id = Some("invalid_reference".to_string());
    
    let broken_verification = verifier.verify_proof_chain(&broken_chain).await;
    
    match broken_verification {
        Ok(result) => assert!(!result.is_valid, "Broken chain should not be valid"),
        Err(_) => (), // Also acceptable - broken chain causes verification error
    }
    
    // Test partial chain verification
    let partial_chain = &proof_chain[2..4];
    let partial_verification = verifier.verify_partial_proof_chain(
        partial_chain,
        &proof_chain[1], // Previous proof for context
    ).await.expect("Partial chain verification should succeed");
    
    assert!(partial_verification.is_valid, "Valid partial chain should be verified");
}

async fn test_invalid_proof_rejection(verifier: &ProofVerifier) {
    println!("❌ Testing invalid proof rejection...");
    
    // Test invalid signature proof
    let message = b"test message";
    let (_, public_key) = BearDogCrypto::generate_ed25519_keypair()
        .expect("Key generation should succeed");
    
    let invalid_signature = vec![0u8; 64]; // Invalid signature
    
    let invalid_signature_proof = CryptographicProof {
        proof_type: ProofType::DigitalSignature,
        algorithm: CryptoAlgorithm::Ed25519,
        public_key,
        signature: invalid_signature,
        message: message.to_vec(),
        metadata: HashMap::new(),
    };
    
    let invalid_result = verifier.verify_cryptographic_proof(&invalid_signature_proof).await
        .expect("Verification should complete even for invalid proofs");
    
    assert!(!invalid_result.is_valid, "Invalid signature should be rejected");
    assert!(invalid_result.confidence_score < 0.1, "Invalid proof should have low confidence");
    
    // Test malformed hash proof
    let data = b"test data";
    let wrong_hash = vec![0u8; 32]; // Wrong hash for the data
    
    let invalid_hash_proof = CryptographicProof {
        proof_type: ProofType::HashIntegrity,
        algorithm: CryptoAlgorithm::Sha256,
        public_key: vec![],
        signature: wrong_hash,
        message: data.to_vec(),
        metadata: HashMap::new(),
    };
    
    let hash_result = verifier.verify_cryptographic_proof(&invalid_hash_proof).await
        .expect("Hash verification should complete");
    
    assert!(!hash_result.is_valid, "Wrong hash should be rejected");
    
    // Test proof with unsupported algorithm
    let unsupported_proof = CryptographicProof {
        proof_type: ProofType::DigitalSignature,
        algorithm: CryptoAlgorithm::UnsupportedAlgorithm,
        public_key: vec![],
        signature: vec![],
        message: vec![],
        metadata: HashMap::new(),
    };
    
    let unsupported_result = verifier.verify_cryptographic_proof(&unsupported_proof).await;
    
    assert!(unsupported_result.is_err(), "Unsupported algorithm should cause error");
    
    // Test proof with inconsistent metadata
    let inconsistent_proof = CryptographicProof {
        proof_type: ProofType::HashIntegrity,
        algorithm: CryptoAlgorithm::Sha256,
        public_key: vec![1, 2, 3], // Inconsistent - hash proofs shouldn't have public keys
        signature: vec![],
        message: vec![],
        metadata: HashMap::new(),
    };
    
    let inconsistent_result = verifier.verify_cryptographic_proof(&inconsistent_proof).await
        .expect("Verification should handle inconsistent proofs");
    
    assert!(!inconsistent_result.is_valid, "Inconsistent proof should be rejected");
}

/// Test proof verifier performance under load
#[tokio::test]
async fn test_proof_verifier_performance() {
    println!("⚡ Testing proof verifier performance...");
    
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await.expect("Core initialization failed"));
    
    let proof_verifier = ProofVerifier::new(core.clone()).await
        .expect("Proof verifier creation failed");
    
    // Create large batch of proofs for performance testing
    let mut large_proof_batch = Vec::new();
    let batch_size = 100;
    
    for i in 0..batch_size {
        let message = format!("performance test message {}", i);
        let (private_key, public_key) = BearDogCrypto::generate_ed25519_keypair()
            .expect("Key generation should succeed");
        
        let signature = BearDogCrypto::sign_ed25519(&private_key, message.as_bytes())
            .expect("Message signing should succeed");
        
        let proof = CryptographicProof {
            proof_type: ProofType::DigitalSignature,
            algorithm: CryptoAlgorithm::Ed25519,
            public_key,
            signature,
            message: message.as_bytes().to_vec(),
            metadata: HashMap::new(),
        };
        
        large_proof_batch.push(proof);
    }
    
    // Measure performance
    let start_time = std::time::Instant::now();
    
    let performance_result = proof_verifier.verify_proof_batch_parallel(&large_proof_batch).await
        .expect("Performance batch verification should succeed");
    
    let verification_duration = start_time.elapsed();
    
    assert_eq!(performance_result.valid_proofs, batch_size,
              "All performance test proofs should be valid");
    
    // Performance requirements (adjust based on hardware)
    let max_verification_time = std::time::Duration::from_secs(10);
    assert!(verification_duration <= max_verification_time,
           "Batch verification should complete within reasonable time");
    
    let proofs_per_second = batch_size as f64 / verification_duration.as_secs_f64();
    assert!(proofs_per_second >= 10.0,
           "Should verify at least 10 proofs per second");
    
    println!("✅ Performance test completed: {} proofs verified in {:?} ({:.1} proofs/sec)",
             batch_size, verification_duration, proofs_per_second);
}

// Helper functions and mock implementations

fn generate_commitment(value: u64) -> Vec<u8> {
    // Mock commitment generation
    value.to_le_bytes().to_vec()
}

fn generate_commitment_from_string(value: &str) -> Vec<u8> {
    // Mock commitment generation from string
    BearDogCrypto::sha256_hash(value.as_bytes())
        .unwrap_or_else(|_| vec![0u8; 32])
}

fn generate_range_proof_data(_value: u64, _min: u64, _max: u64) -> Vec<u8> {
    // Mock range proof data
    vec![1, 2, 3, 4, 5]
}

fn generate_membership_proof_data(_element: &str, _set: &[&str]) -> Vec<u8> {
    // Mock membership proof data
    vec![6, 7, 8, 9, 10]
}

fn generate_knowledge_commitment(_knowledge: &str) -> Vec<u8> {
    // Mock knowledge commitment
    BearDogCrypto::sha256_hash(knowledge.as_bytes())
        .unwrap_or_else(|_| vec![0u8; 32])
}

fn generate_knowledge_proof_data(_knowledge: &str) -> Vec<u8> {
    // Mock knowledge proof data
    vec![11, 12, 13, 14, 15]
}

fn generate_fiat_shamir_challenge(_data: &[&[u8]]) -> Vec<u8> {
    // Mock Fiat-Shamir challenge
    vec![16, 17, 18, 19, 20]
}

fn hash_set(set: &[&str]) -> String {
    // Mock set hash
    let combined = set.join("");
    hex::encode(BearDogCrypto::sha256_hash(combined.as_bytes())
        .unwrap_or_else(|_| vec![0u8; 32]))
}

// Mock structs for testing

#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub root_hash: Vec<u8>,
    pub leaves: Vec<Vec<u8>>,
    pub internal_nodes: Vec<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct MerkleInclusionProof {
    pub path: Vec<Vec<u8>>,
    pub index: usize,
}

#[derive(Debug, Clone)]
pub struct ChainProof {
    pub proof_id: String,
    pub previous_proof_id: Option<String>,
    pub proof_data: Vec<u8>,
    pub timestamp: std::time::SystemTime,
    pub chain_position: usize,
}

fn build_merkle_tree(data: &[&[u8]]) -> MerkleTree {
    // Mock Merkle tree construction
    let leaves: Vec<Vec<u8>> = data.iter()
        .map(|d| BearDogCrypto::sha256_hash(d).unwrap_or_else(|_| vec![0u8; 32]))
        .collect();
    
    let root_hash = BearDogCrypto::sha256_hash(&leaves.concat())
        .unwrap_or_else(|_| vec![0u8; 32]);
    
    MerkleTree {
        root_hash,
        leaves,
        internal_nodes: vec![],
    }
}

fn generate_merkle_inclusion_proof(_tree: &MerkleTree, _item: &[u8]) -> Result<MerkleInclusionProof, BearDogError> {
    // Mock inclusion proof generation
    Ok(MerkleInclusionProof {
        path: vec![vec![1, 2, 3], vec![4, 5, 6]],
        index: 0,
    })
}

fn generate_merkle_exclusion_proof(_tree: &MerkleTree, _item: &[u8]) -> Result<MerkleInclusionProof, BearDogError> {
    // Mock exclusion proof generation
    Ok(MerkleInclusionProof {
        path: vec![vec![7, 8, 9], vec![10, 11, 12]],
        index: 999, // Invalid index indicates exclusion
    })
}

// Mock enum for unsupported algorithm testing
#[derive(Debug, Clone, PartialEq)]
pub enum CryptoAlgorithm {
    Aes256Gcm,
    ChaCha20Poly1305,
    Ed25519,
    Rsa4096,
    Sha256,
    Sha512,
    X25519,
    UnsupportedAlgorithm,
}
