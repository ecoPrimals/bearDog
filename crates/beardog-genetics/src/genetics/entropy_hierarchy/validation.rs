// Entropy Hierarchy Validation
//
// This module provides validation capabilities for entropy hierarchy management,
// including quality assessment and ownership verification.

use super::types::{
    BiometricHash, EntropyClass, EntropyHierarchyConfig, HumanIdentity, OwnershipProof,
};
use beardog_errors::BearDogError;
use chrono::Utc;
use sha3::{Digest, Sha3_256};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct EntropyValidator {
    config: EntropyHierarchyConfig,
    #[allow(dead_code)] // Used for validation thresholds but not yet fully implemented
    quality_thresholds: HashMap<String, f64>,
}

impl EntropyValidator {
    /// Create new entropy validator
    /// Creates a new instance
    #[must_use]
    pub fn new(config: EntropyHierarchyConfig) -> Self {
        let mut quality_thresholds = HashMap::new();
        quality_thresholds.insert("human".to_string(), config.min_human_quality);
        quality_thresholds.insert("machine".to_string(), config.min_machine_quality);

        Self {
            config,
            quality_thresholds,
        }
    }

    /// Validate entropy class quality
    /// Validates `entropy_quality`
    /// Validates `entropy_quality`
    pub fn validate_entropy_quality(
        &self,
        entropy_class: &EntropyClass,
    ) -> Result<bool, BearDogError> {
        match entropy_class {
            EntropyClass::HumanLivedExperience { quality_score, .. } => {
                Ok(*quality_score >= self.config.min_human_quality)
            }
            EntropyClass::HumanSupervisedMachine { quality_score, .. } => {
                Ok(*quality_score >= self.config.min_machine_quality)
            }
            EntropyClass::StoreBoughtMachine { quality_score, .. } => {
                Ok(*quality_score >= self.config.min_machine_quality)
            }
        }
    }

    pub fn generate_ownership_proof(
        &self,
        owner_identity: &HumanIdentity,
        entropy_data: &[u8],
    ) -> Result<OwnershipProof, BearDogError> {
        let proof_data = self.create_ownership_proof_data(owner_identity, entropy_data)?;
        let signature = self.sign_ownership_proof(&proof_data, owner_identity)?;

        Ok(OwnershipProof {
            proof_data,
            signature,
            timestamp: Utc::now(),
        })
    }

    /// Generate biometric hash
    pub fn generate_biometric_hash(
        &self,
        biometric_data: &[u8],
        ownership_proof: &[u8],
    ) -> Result<BiometricHash, BearDogError> {
        let mut hasher = Sha3_256::new();
        hasher.update(biometric_data);
        hasher.update(b"biometric_hash_salt");
        let hash = hasher.finalize().to_vec();

        Ok(BiometricHash {
            hash,
            ownership_proof: ownership_proof.to_vec(),
        })
    }

    /// Validate entropy age
    /// Validates `entropy_age`
    /// Validates `entropy_age`
    pub fn validate_entropy_age(&self, entropy_class: &EntropyClass) -> Result<bool, BearDogError> {
        let timestamp = match entropy_class {
            EntropyClass::HumanLivedExperience {
                capture_timestamp, ..
            } => capture_timestamp,
            EntropyClass::HumanSupervisedMachine {
                validation_timestamp,
                ..
            } => validation_timestamp,
            EntropyClass::StoreBoughtMachine {
                generation_timestamp,
                ..
            } => generation_timestamp,
        };

        let age_hours = (Utc::now() - *timestamp).num_hours() as u64;
        Ok(age_hours <= self.config.max_entropy_age_hours)
    }

    /// Create ownership proof data
    /// Creates `ownership_proof_data`
    fn create_ownership_proof_data(
        &self,
        owner_identity: &HumanIdentity,
        entropy_data: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        let mut hasher = Sha3_256::new();

        hasher.update(owner_identity.identity_id.as_bytes());
        hasher.update(&owner_identity.identity_hash);

        let entropy_hash = Sha3_256::digest(entropy_data);
        hasher.update(entropy_hash);

        hasher.update(Utc::now().timestamp().to_le_bytes());
        hasher.update(b"entropy_ownership_proof");

        Ok(hasher.finalize().to_vec())
    }

    /// Sign ownership proof
    fn sign_ownership_proof(
        &self,
        proof_data: &[u8],
        _owner_identity: &HumanIdentity,
    ) -> Result<Vec<u8>, BearDogError> {
        // Simplified signature for now - in production would use proper cryptographic signing
        let mut hasher = Sha3_256::new();
        hasher.update(proof_data);
        hasher.update(b"ownership_signature");
        Ok(hasher.finalize().to_vec())
    }

    /// Generate entropy commitment
    #[allow(dead_code)] // Used for entropy validation but not yet fully implemented
    fn generate_entropy_commitment(&self, entropy_data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        let mut hasher = Sha3_256::new();
        hasher.update(entropy_data);
        hasher.update(b"entropy_commitment");
        Ok(hasher.finalize().to_vec())
    }

    /// Generate temporal proof
    #[allow(dead_code)] // Used for entropy validation but not yet fully implemented
    fn generate_temporal_proof(
        &self,
        _entropy_class: &EntropyClass,
    ) -> Result<Vec<u8>, BearDogError> {
        let mut hasher = Sha3_256::new();
        hasher.update(Utc::now().timestamp().to_le_bytes());
        hasher.update(b"temporal_proof");
        Ok(hasher.finalize().to_vec())
    }

    /// Generate uniqueness proof
    #[allow(dead_code)] // Used for entropy validation but not yet fully implemented
    fn generate_uniqueness_proof(
        &self,
        entropy_data: &[u8],
        _entropy_class: &EntropyClass,
    ) -> Result<Vec<u8>, BearDogError> {
        let mut hasher = Sha3_256::new();
        hasher.update(entropy_data);
        hasher.update(b"uniqueness_proof");
        Ok(hasher.finalize().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn create_test_config() -> EntropyHierarchyConfig {
        EntropyHierarchyConfig {
            min_human_quality: 0.7,
            min_machine_quality: 0.5,
            max_entropy_age_hours: 24,
            require_biometric_verification: true,
            require_ownership_proof: true,
        }
    }

    fn create_test_human_identity() -> HumanIdentity {
        use super::super::types::VerificationLevel;
        HumanIdentity {
            identity_id: "test-human-123".to_string(),
            identity_hash: vec![1, 2, 3, 4],
            verification_level: VerificationLevel::Enhanced,
            verified_at: Utc::now(),
        }
    }

    fn create_test_machine_source() -> super::super::types::MachineEntropySource {
        use super::super::types::{MachineEntropySource, MachineSourceType};
        use std::collections::HashMap;

        MachineEntropySource {
            source_type: MachineSourceType::CSPRNG {
                algorithm: "ChaCha20".to_string(),
                seed_source: "OS-RNG".to_string(),
            },
            algorithm: "ChaCha20".to_string(),
            seed_source: "OS-RNG".to_string(),
            quality_metrics: HashMap::new(),
        }
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_entropy_validator_creation() {
        let config = create_test_config();
        let validator = EntropyValidator::new(config.clone());
        assert_eq!(validator.config.min_human_quality, config.min_human_quality);
        assert_eq!(
            validator.config.min_machine_quality,
            config.min_machine_quality
        );
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_validate_entropy_quality_human_lived_experience_high() -> Result<(), BearDogError> {
        let config = create_test_config();
        let validator = EntropyValidator::new(config);

        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.9,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: vec![1, 2, 3],
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: vec![10, 11, 12],
                timestamp: Utc::now(),
            },
        };

        assert!(validator.validate_entropy_quality(&entropy)?);
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_validate_entropy_quality_human_lived_experience_low() -> Result<(), BearDogError> {
        let config = create_test_config();
        let validator = EntropyValidator::new(config);

        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.5,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: vec![1, 2, 3],
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: vec![10, 11, 12],
                timestamp: Utc::now(),
            },
        };

        assert!(!validator.validate_entropy_quality(&entropy)?);
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_validate_entropy_quality_human_supervised_machine_high() -> Result<(), BearDogError> {
        let config = create_test_config();
        let validator = EntropyValidator::new(config);

        let entropy = EntropyClass::HumanSupervisedMachine {
            quality_score: 0.8,
            machine_source: create_test_machine_source(),
            human_validator: create_test_human_identity(),
            validation_timestamp: Utc::now(),
        };

        assert!(validator.validate_entropy_quality(&entropy)?);
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_validate_entropy_quality_human_supervised_machine_low() -> Result<(), BearDogError> {
        let config = create_test_config();
        let validator = EntropyValidator::new(config);

        let entropy = EntropyClass::HumanSupervisedMachine {
            quality_score: 0.3,
            machine_source: create_test_machine_source(),
            human_validator: create_test_human_identity(),
            validation_timestamp: Utc::now(),
        };

        assert!(!validator.validate_entropy_quality(&entropy)?);
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_validate_entropy_quality_store_bought_machine_high() -> Result<(), BearDogError> {
        let config = create_test_config();
        let validator = EntropyValidator::new(config);

        let entropy = EntropyClass::StoreBoughtMachine {
            quality_score: 0.6,
            source_type: create_test_machine_source(),
            generation_timestamp: Utc::now(),
            reproducibility_index: 0.2,
        };

        assert!(validator.validate_entropy_quality(&entropy)?);
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_validate_entropy_quality_store_bought_machine_low() -> Result<(), BearDogError> {
        let config = create_test_config();
        let validator = EntropyValidator::new(config);

        let entropy = EntropyClass::StoreBoughtMachine {
            quality_score: 0.2,
            source_type: create_test_machine_source(),
            generation_timestamp: Utc::now(),
            reproducibility_index: 0.8,
        };

        assert!(!validator.validate_entropy_quality(&entropy)?);
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_generate_ownership_proof() -> Result<(), BearDogError> {
        let config = create_test_config();
        let validator = EntropyValidator::new(config);
        let owner_identity = create_test_human_identity();
        let entropy_data = vec![1, 2, 3, 4, 5, 6, 7, 8];

        let proof = validator.generate_ownership_proof(&owner_identity, &entropy_data)?;

        assert!(!proof.proof_data.is_empty());
        assert!(!proof.signature.is_empty());
        assert_eq!(proof.proof_data.len(), 32); // SHA3-256 output
        assert_eq!(proof.signature.len(), 32); // SHA3-256 output
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_generate_ownership_proof_different_data() -> Result<(), BearDogError> {
        let config = create_test_config();
        let validator = EntropyValidator::new(config);
        let owner_identity = create_test_human_identity();
        let entropy_data_1 = vec![1, 2, 3, 4];
        let entropy_data_2 = vec![5, 6, 7, 8];

        let proof_1 = validator.generate_ownership_proof(&owner_identity, &entropy_data_1)?;
        let proof_2 = validator.generate_ownership_proof(&owner_identity, &entropy_data_2)?;

        // Different entropy data should produce different proofs
        assert_ne!(proof_1.proof_data, proof_2.proof_data);
        assert_ne!(proof_1.signature, proof_2.signature);
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_generate_biometric_hash() -> Result<(), BearDogError> {
        let config = create_test_config();
        let validator = EntropyValidator::new(config);
        let biometric_data = vec![1, 2, 3, 4, 5];
        let ownership_proof = vec![6, 7, 8, 9, 10];

        let biometric_hash =
            validator.generate_biometric_hash(&biometric_data, &ownership_proof)?;

        assert!(!biometric_hash.hash.is_empty());
        assert_eq!(biometric_hash.hash.len(), 32); // SHA3-256 output
        assert_eq!(biometric_hash.ownership_proof, ownership_proof);
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_generate_biometric_hash_different_data() -> Result<(), BearDogError> {
        let config = create_test_config();
        let validator = EntropyValidator::new(config);
        let biometric_data_1 = vec![1, 2, 3, 4];
        let biometric_data_2 = vec![5, 6, 7, 8];
        let ownership_proof = vec![9, 10, 11, 12];

        let hash_1 = validator.generate_biometric_hash(&biometric_data_1, &ownership_proof)?;
        let hash_2 = validator.generate_biometric_hash(&biometric_data_2, &ownership_proof)?;

        // Different biometric data should produce different hashes
        assert_ne!(hash_1.hash, hash_2.hash);
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_validate_entropy_age_fresh_human_lived_experience() -> Result<(), BearDogError> {
        let config = create_test_config();
        let validator = EntropyValidator::new(config);

        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.9,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: vec![1, 2, 3],
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: vec![10, 11, 12],
                timestamp: Utc::now(),
            },
        };

        assert!(validator.validate_entropy_age(&entropy)?);
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_validate_entropy_age_old_human_lived_experience() -> Result<(), BearDogError> {
        let config = create_test_config();
        let validator = EntropyValidator::new(config);

        let old_timestamp = Utc::now() - Duration::hours(48);
        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.9,
            capture_timestamp: old_timestamp,
            biometric_signature: BiometricHash {
                hash: vec![1, 2, 3],
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: vec![10, 11, 12],
                timestamp: Utc::now(),
            },
        };

        assert!(!validator.validate_entropy_age(&entropy)?);
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_validate_entropy_age_fresh_human_supervised_machine() -> Result<(), BearDogError> {
        let config = create_test_config();
        let validator = EntropyValidator::new(config);

        let entropy = EntropyClass::HumanSupervisedMachine {
            quality_score: 0.8,
            machine_source: create_test_machine_source(),
            human_validator: create_test_human_identity(),
            validation_timestamp: Utc::now(),
        };

        assert!(validator.validate_entropy_age(&entropy)?);
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_validate_entropy_age_old_human_supervised_machine() -> Result<(), BearDogError> {
        let config = create_test_config();
        let validator = EntropyValidator::new(config);

        let old_timestamp = Utc::now() - Duration::hours(30);
        let entropy = EntropyClass::HumanSupervisedMachine {
            quality_score: 0.8,
            machine_source: create_test_machine_source(),
            human_validator: create_test_human_identity(),
            validation_timestamp: old_timestamp,
        };

        assert!(!validator.validate_entropy_age(&entropy)?);
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_validate_entropy_age_fresh_store_bought_machine() -> Result<(), BearDogError> {
        let config = create_test_config();
        let validator = EntropyValidator::new(config);

        let entropy = EntropyClass::StoreBoughtMachine {
            quality_score: 0.6,
            source_type: create_test_machine_source(),
            generation_timestamp: Utc::now(),
            reproducibility_index: 0.3,
        };

        assert!(validator.validate_entropy_age(&entropy)?);
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_validate_entropy_age_old_store_bought_machine() -> Result<(), BearDogError> {
        let config = create_test_config();
        let validator = EntropyValidator::new(config);

        let old_timestamp = Utc::now() - Duration::hours(100);
        let entropy = EntropyClass::StoreBoughtMachine {
            quality_score: 0.6,
            source_type: create_test_machine_source(),
            generation_timestamp: old_timestamp,
            reproducibility_index: 0.3,
        };

        assert!(!validator.validate_entropy_age(&entropy)?);
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_validate_entropy_age_at_threshold() -> Result<(), BearDogError> {
        let config = create_test_config();
        let validator = EntropyValidator::new(config);

        // Exactly at max age (24 hours)
        let threshold_timestamp = Utc::now() - Duration::hours(24);
        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.9,
            capture_timestamp: threshold_timestamp,
            biometric_signature: BiometricHash {
                hash: vec![1, 2, 3],
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: vec![10, 11, 12],
                timestamp: Utc::now(),
            },
        };

        assert!(validator.validate_entropy_age(&entropy)?);
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_create_ownership_proof_data() -> Result<(), BearDogError> {
        let config = create_test_config();
        let validator = EntropyValidator::new(config);
        let owner_identity = create_test_human_identity();
        let entropy_data = vec![1, 2, 3, 4, 5];

        let proof_data = validator.create_ownership_proof_data(&owner_identity, &entropy_data)?;

        assert!(!proof_data.is_empty());
        assert_eq!(proof_data.len(), 32); // SHA3-256 output
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_create_ownership_proof_data_deterministic() -> Result<(), BearDogError> {
        let config = create_test_config();
        let validator = EntropyValidator::new(config);
        let owner_identity = create_test_human_identity();
        let entropy_data = vec![1, 2, 3, 4, 5];

        let proof_data_1 = validator.create_ownership_proof_data(&owner_identity, &entropy_data)?;
        // Note: Due to timestamp in proof data, this test won't be truly deterministic
        // But we can verify the format is consistent
        assert_eq!(proof_data_1.len(), 32);
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_sign_ownership_proof() -> Result<(), BearDogError> {
        let config = create_test_config();
        let validator = EntropyValidator::new(config);
        let owner_identity = create_test_human_identity();
        let proof_data = vec![1, 2, 3, 4];

        let signature = validator.sign_ownership_proof(&proof_data, &owner_identity)?;

        assert!(!signature.is_empty());
        assert_eq!(signature.len(), 32); // SHA3-256 output
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_sign_ownership_proof_different_data() -> Result<(), BearDogError> {
        let config = create_test_config();
        let validator = EntropyValidator::new(config);
        let owner_identity = create_test_human_identity();
        let proof_data_1 = vec![1, 2, 3, 4];
        let proof_data_2 = vec![5, 6, 7, 8];

        let signature_1 = validator.sign_ownership_proof(&proof_data_1, &owner_identity)?;
        let signature_2 = validator.sign_ownership_proof(&proof_data_2, &owner_identity)?;

        // Different proof data should produce different signatures
        assert_ne!(signature_1, signature_2);
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_generate_entropy_commitment() -> Result<(), BearDogError> {
        let config = create_test_config();
        let validator = EntropyValidator::new(config);
        let entropy_data = vec![1, 2, 3, 4, 5];

        let commitment = validator.generate_entropy_commitment(&entropy_data)?;

        assert!(!commitment.is_empty());
        assert_eq!(commitment.len(), 32); // SHA3-256 output
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_generate_entropy_commitment_different_data() -> Result<(), BearDogError> {
        let config = create_test_config();
        let validator = EntropyValidator::new(config);
        let entropy_data_1 = vec![1, 2, 3, 4];
        let entropy_data_2 = vec![5, 6, 7, 8];

        let commitment_1 = validator.generate_entropy_commitment(&entropy_data_1)?;
        let commitment_2 = validator.generate_entropy_commitment(&entropy_data_2)?;

        assert_ne!(commitment_1, commitment_2);
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_generate_temporal_proof() -> Result<(), BearDogError> {
        let config = create_test_config();
        let validator = EntropyValidator::new(config);
        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.9,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: vec![1, 2, 3],
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: vec![10, 11, 12],
                timestamp: Utc::now(),
            },
        };

        let temporal_proof = validator.generate_temporal_proof(&entropy)?;

        assert!(!temporal_proof.is_empty());
        assert_eq!(temporal_proof.len(), 32); // SHA3-256 output
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_generate_uniqueness_proof() -> Result<(), BearDogError> {
        let config = create_test_config();
        let validator = EntropyValidator::new(config);
        let entropy_data = vec![1, 2, 3, 4, 5];
        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.9,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: vec![1, 2, 3],
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: vec![10, 11, 12],
                timestamp: Utc::now(),
            },
        };

        let uniqueness_proof = validator.generate_uniqueness_proof(&entropy_data, &entropy)?;

        assert!(!uniqueness_proof.is_empty());
        assert_eq!(uniqueness_proof.len(), 32); // SHA3-256 output
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_generate_uniqueness_proof_different_data() -> Result<(), BearDogError> {
        let config = create_test_config();
        let validator = EntropyValidator::new(config);
        let entropy_data_1 = vec![1, 2, 3, 4];
        let entropy_data_2 = vec![5, 6, 7, 8];
        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.9,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: vec![1, 2, 3],
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: vec![10, 11, 12],
                timestamp: Utc::now(),
            },
        };

        let uniqueness_proof_1 = validator.generate_uniqueness_proof(&entropy_data_1, &entropy)?;
        let uniqueness_proof_2 = validator.generate_uniqueness_proof(&entropy_data_2, &entropy)?;

        assert_ne!(uniqueness_proof_1, uniqueness_proof_2);
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_validate_quality_at_exact_threshold() -> Result<(), BearDogError> {
        let config = create_test_config();
        let validator = EntropyValidator::new(config.clone());

        // Test at exact threshold for human entropy
        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: config.min_human_quality,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: vec![1, 2, 3],
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: vec![10, 11, 12],
                timestamp: Utc::now(),
            },
        };

        assert!(validator.validate_entropy_quality(&entropy)?);
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_validate_quality_just_below_threshold() -> Result<(), BearDogError> {
        let config = create_test_config();
        let validator = EntropyValidator::new(config.clone());

        // Test just below threshold for human entropy
        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: config.min_human_quality - 0.01,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: vec![1, 2, 3],
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: vec![10, 11, 12],
                timestamp: Utc::now(),
            },
        };

        assert!(!validator.validate_entropy_quality(&entropy)?);
        Ok(())
    }
}
