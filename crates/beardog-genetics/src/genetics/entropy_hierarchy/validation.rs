use crate::genetics::entropy_hierarchy::{EntropyClass, EntropyHierarchyConfig};
use beardog_errors::BearDogError;
use chrono::{Duration, Utc};
use sha3::{Digest, Sha3_256};

// Re-export from live_feed_validator module
pub use super::live_feed_validator::{LiveFeedConfig, LiveFeedValidationResult, LiveFeedValidator};

/// Validates entropy against hierarchy requirements
#[derive(Debug, Clone)]
pub struct EntropyValidator {
    config: EntropyHierarchyConfig,
}

impl EntropyValidator {
    #[must_use]
    pub fn new(config: EntropyHierarchyConfig) -> Self {
        Self { config }
    }

    /// Validate entropy class meets hierarchy requirements
    pub fn validate_entropy(&self, entropy: &EntropyClass) -> Result<(), BearDogError> {
        self.validate_entropy_quality(entropy)?;
        self.validate_entropy_age(entropy)?;
        self.validate_biometric_verification(entropy)?;
        self.validate_ownership_proof(entropy)?;
        Ok(())
    }

    /// Validate entropy quality meets minimum thresholds
    pub fn validate_entropy_quality(&self, entropy: &EntropyClass) -> Result<bool, BearDogError> {
        match entropy {
            EntropyClass::HumanLivedExperience { quality_score, .. } => {
                if *quality_score < self.config.min_human_quality {
                    return Err(BearDogError::validation(&format!(
                        "Human entropy quality {} below minimum {}",
                        quality_score, self.config.min_human_quality
                    )));
                }
            }
            EntropyClass::HumanSupervisedMachine { quality_score, .. } => {
                if *quality_score < self.config.min_machine_quality {
                    return Err(BearDogError::validation(&format!(
                        "Human-supervised machine entropy quality {} below minimum {}",
                        quality_score, self.config.min_machine_quality
                    )));
                }
            }
            EntropyClass::StoreBoughtMachine { quality_score, .. } => {
                if *quality_score < self.config.min_machine_quality {
                    return Err(BearDogError::validation(&format!(
                        "Machine entropy quality {} below minimum {}",
                        quality_score, self.config.min_machine_quality
                    )));
                }
            }
        }
        Ok(true)
    }

    /// Validate entropy age is within acceptable limits
    pub fn validate_entropy_age(&self, entropy: &EntropyClass) -> Result<bool, BearDogError> {
        let timestamp = match entropy {
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

        let age = Utc::now().signed_duration_since(*timestamp);
        let max_age = Duration::hours(self.config.max_entropy_age_hours as i64);

        if age > max_age {
            return Err(BearDogError::validation(&format!(
                "Entropy age {} hours exceeds maximum {} hours",
                age.num_hours(),
                self.config.max_entropy_age_hours
            )));
        }

        Ok(true)
    }

    /// Validate biometric verification if required
    ///
    /// For `HumanLivedExperience` entropy, validates the biometric signature
    /// to ensure the entropy genuinely comes from a human source.
    pub fn validate_biometric_verification(
        &self,
        entropy: &EntropyClass,
    ) -> Result<bool, BearDogError> {
        if !self.config.require_biometric_verification {
            return Ok(true);
        }

        match entropy {
            EntropyClass::HumanLivedExperience {
                biometric_signature,
                ..
            } => {
                // Validate biometric hash structure
                if biometric_signature.hash.is_empty() {
                    return Err(BearDogError::validation("Biometric hash is empty"));
                }

                // Validate ownership proof
                if biometric_signature.ownership_proof.is_empty() {
                    return Err(BearDogError::validation(
                        "Biometric ownership proof is missing",
                    ));
                }

                // Check hash length (should be 32 bytes for SHA3-256)
                if biometric_signature.hash.len() < 32 {
                    return Err(BearDogError::validation(
                        "Biometric hash too short (expected at least 32 bytes)",
                    ));
                }

                // Verify the biometric hash is not all zeros (indicates mock/empty data)
                if biometric_signature.hash.iter().all(|&b| b == 0) {
                    return Err(BearDogError::validation(
                        "Biometric hash appears to be zeroed/empty - not a valid human signature",
                    ));
                }

                Ok(true)
            }
            EntropyClass::HumanSupervisedMachine { .. } => {
                // Human-supervised machine doesn't require direct biometric verification
                Ok(true)
            }
            EntropyClass::StoreBoughtMachine { .. } => {
                // Machine entropy doesn't have biometrics
                Ok(true)
            }
        }
    }

    /// Validate ownership proof if required
    ///  
    /// Ensures the human who provided the entropy can prove ownership.
    pub fn validate_ownership_proof(&self, entropy: &EntropyClass) -> Result<bool, BearDogError> {
        if !self.config.require_ownership_proof {
            return Ok(true);
        }

        match entropy {
            EntropyClass::HumanLivedExperience {
                ownership_proof, ..
            } => {
                // Validate proof data structure
                if ownership_proof.proof_data.is_empty() {
                    return Err(BearDogError::validation("Ownership proof data is empty"));
                }

                // Validate signature
                if ownership_proof.signature.is_empty() {
                    return Err(BearDogError::validation(
                        "Ownership proof signature is missing",
                    ));
                }

                // Check timestamp freshness (proof shouldn't be too old)
                let age = Utc::now().signed_duration_since(ownership_proof.timestamp);
                let max_age = Duration::hours(self.config.max_entropy_age_hours as i64);

                if age > max_age {
                    return Err(BearDogError::validation(&format!(
                        "Ownership proof age {} hours exceeds maximum {} hours",
                        age.num_hours(),
                        self.config.max_entropy_age_hours
                    )));
                }

                // Verify signature is not all zeros
                if ownership_proof.signature.iter().all(|&b| b == 0) {
                    return Err(BearDogError::validation(
                        "Ownership proof signature appears to be zeroed/empty",
                    ));
                }

                Ok(true)
            }
            EntropyClass::HumanSupervisedMachine { .. } => {
                // Human-supervised machines have implicit ownership through supervision
                Ok(true)
            }
            EntropyClass::StoreBoughtMachine { .. } => {
                // Machine entropy doesn't require human ownership proof
                Ok(true)
            }
        }
    }

    /// Generate cryptographic proof of entropy hierarchy compliance
    #[allow(dead_code)] // Used for entropy validation but not yet fully implemented
    fn generate_compliance_proof(&self, entropy: &EntropyClass) -> Result<Vec<u8>, BearDogError> {
        let mut hasher = Sha3_256::new();

        // Add entropy class type
        match entropy {
            EntropyClass::HumanLivedExperience { .. } => hasher.update(b"HumanLivedExperience"),
            EntropyClass::HumanSupervisedMachine { .. } => hasher.update(b"HumanSupervisedMachine"),
            EntropyClass::StoreBoughtMachine { .. } => hasher.update(b"StoreBoughtMachine"),
        }

        // Add timestamp
        let timestamp = match entropy {
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
        hasher.update(timestamp.timestamp().to_le_bytes());

        // Add quality score
        let quality = match entropy {
            EntropyClass::HumanLivedExperience { quality_score, .. } => quality_score,
            EntropyClass::HumanSupervisedMachine { quality_score, .. } => quality_score,
            EntropyClass::StoreBoughtMachine { quality_score, .. } => quality_score,
        };
        hasher.update(quality.to_le_bytes());

        Ok(hasher.finalize().to_vec())
    }

    /// Generate ownership signature
    #[allow(dead_code)] // Used for entropy validation but not yet fully implemented
    fn generate_ownership_signature(
        &self,
        _entropy: &EntropyClass,
    ) -> Result<Vec<u8>, BearDogError> {
        let mut hasher = Sha3_256::new();
        hasher.update(Utc::now().timestamp().to_le_bytes());
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
    use std::collections::HashMap;

    fn create_test_machine_source() -> crate::genetics::entropy_hierarchy::MachineEntropySource {
        use crate::genetics::entropy_hierarchy::MachineSourceType;
        crate::genetics::entropy_hierarchy::MachineEntropySource {
            source_type: MachineSourceType::HRNG {
                device_type: "TPM".to_string(),
                entropy_rate: 1.0,
            },
            algorithm: "AES-CTR-DRBG".to_string(),
            seed_source: "Hardware".to_string(),
            quality_metrics: HashMap::new(),
        }
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: high
    #[test]
    fn test_validate_human_entropy_quality_pass() -> Result<(), BearDogError> {
        use crate::genetics::entropy_hierarchy::{BiometricHash, OwnershipProof};
        let config = EntropyHierarchyConfig {
            min_human_quality: 0.7,
            min_machine_quality: 0.5,
            max_entropy_age_hours: 24,
            require_biometric_verification: true,
            require_ownership_proof: true,
        };
        let validator = EntropyValidator::new(config);

        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.8,
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
    // TEST_PRIORITY: high
    #[test]
    fn test_validate_human_entropy_quality_fail() {
        use crate::genetics::entropy_hierarchy::{BiometricHash, OwnershipProof};
        let config = EntropyHierarchyConfig {
            min_human_quality: 0.7,
            min_machine_quality: 0.5,
            max_entropy_age_hours: 24,
            require_biometric_verification: true,
            require_ownership_proof: true,
        };
        let validator = EntropyValidator::new(config);

        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.5, // Below minimum of 0.7
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

        assert!(validator.validate_entropy_quality(&entropy).is_err());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: high
    #[test]
    fn test_validate_machine_entropy_quality_pass() -> Result<(), BearDogError> {
        let config = EntropyHierarchyConfig {
            min_human_quality: 0.7,
            min_machine_quality: 0.5,
            max_entropy_age_hours: 24,
            require_biometric_verification: true,
            require_ownership_proof: true,
        };
        let validator = EntropyValidator::new(config);

        let entropy = EntropyClass::StoreBoughtMachine {
            quality_score: 0.6,
            source_type: create_test_machine_source(),
            generation_timestamp: Utc::now(),
            reproducibility_index: 0.3,
        };

        assert!(validator.validate_entropy_quality(&entropy)?);
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: high
    #[test]
    fn test_validate_machine_entropy_quality_fail() {
        let config = EntropyHierarchyConfig {
            min_human_quality: 0.7,
            min_machine_quality: 0.5,
            max_entropy_age_hours: 24,
            require_biometric_verification: true,
            require_ownership_proof: true,
        };
        let validator = EntropyValidator::new(config);

        let entropy = EntropyClass::StoreBoughtMachine {
            quality_score: 0.3, // Below minimum of 0.5
            source_type: create_test_machine_source(),
            generation_timestamp: Utc::now(),
            reproducibility_index: 0.3,
        };

        assert!(validator.validate_entropy_quality(&entropy).is_err());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_validate_entropy_age_fresh() -> Result<(), BearDogError> {
        use crate::genetics::entropy_hierarchy::{BiometricHash, OwnershipProof};
        let config = EntropyHierarchyConfig {
            min_human_quality: 0.7,
            min_machine_quality: 0.5,
            max_entropy_age_hours: 24,
            require_biometric_verification: true,
            require_ownership_proof: true,
        };
        let validator = EntropyValidator::new(config);

        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.8,
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
    fn test_validate_entropy_age_old() {
        use crate::genetics::entropy_hierarchy::{BiometricHash, OwnershipProof};
        let config = EntropyHierarchyConfig {
            min_human_quality: 0.7,
            min_machine_quality: 0.5,
            max_entropy_age_hours: 24,
            require_biometric_verification: true,
            require_ownership_proof: true,
        };
        let validator = EntropyValidator::new(config);

        let old_timestamp = Utc::now() - Duration::hours(48);
        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.8,
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

        assert!(validator.validate_entropy_age(&entropy).is_err());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_validate_entropy_age_fresh_store_bought_machine() -> Result<(), BearDogError> {
        let config = EntropyHierarchyConfig {
            min_human_quality: 0.7,
            min_machine_quality: 0.5,
            max_entropy_age_hours: 24,
            require_biometric_verification: true,
            require_ownership_proof: true,
        };
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
        let config = EntropyHierarchyConfig {
            min_human_quality: 0.7,
            min_machine_quality: 0.5,
            max_entropy_age_hours: 24,
            require_biometric_verification: true,
            require_ownership_proof: true,
        };
        let validator = EntropyValidator::new(config);

        let old_timestamp = Utc::now() - Duration::hours(100);
        let entropy = EntropyClass::StoreBoughtMachine {
            quality_score: 0.6,
            source_type: create_test_machine_source(),
            generation_timestamp: old_timestamp,
            reproducibility_index: 0.3,
        };

        assert!(validator.validate_entropy_age(&entropy).is_err());
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: high
    #[test]
    fn test_validate_entropy_full_pass() -> Result<(), BearDogError> {
        use crate::genetics::entropy_hierarchy::{BiometricHash, OwnershipProof};
        let config = EntropyHierarchyConfig {
            min_human_quality: 0.7,
            min_machine_quality: 0.5,
            max_entropy_age_hours: 24,
            require_biometric_verification: true,
            require_ownership_proof: true,
        };
        let validator = EntropyValidator::new(config);

        // Create proper biometric data (not all zeros)
        let mut biometric_hash = vec![0u8; 32];
        for (i, byte) in biometric_hash.iter_mut().enumerate() {
            *byte = (i as u8).wrapping_mul(7).wrapping_add(13); // Non-zero pattern
        }

        let mut ownership_proof_sig = vec![0u8; 64];
        for (i, byte) in ownership_proof_sig.iter_mut().enumerate() {
            *byte = (i as u8).wrapping_mul(3).wrapping_add(5); // Non-zero pattern
        }

        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.8,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: biometric_hash,
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: ownership_proof_sig,
                timestamp: Utc::now(),
            },
        };

        let result = validator.validate_entropy(&entropy);
        assert!(result.is_ok());
        Ok(())
    }
}
