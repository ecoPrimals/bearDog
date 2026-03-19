// SPDX-License-Identifier: AGPL-3.0-only

// Entropy Sources Management
//
// This module provides entropy source management and mixing capabilities
// for the entropy hierarchy system.

use super::types::{
    EntropyClass, EntropyHierarchyConfig, EntropySeed, FusionAlgorithm, HumanEntropySource,
    HumanEntropyType, MixingStrategy, SeedMetadata,
};
use beardog_errors::BearDogError;
// Note: rand utilities available for future entropy mixing enhancements
use sha3::{Digest, Sha3_256};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct EntropyMixingEngine {
    config: EntropyHierarchyConfig,
}

impl EntropyMixingEngine {
    /// Create new entropy mixing engine
    /// Creates a new instance
    #[must_use]
    pub fn new(config: &EntropyHierarchyConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /// Mix entropy sources using specified algorithm
    pub fn mix_entropy_sources(
        &self,
        sources: &[Vec<u8>],
        algorithm: &FusionAlgorithm,
    ) -> Result<Vec<u8>, BearDogError> {
        if sources.is_empty() {
            return Err(BearDogError::invalid_input("No entropy sources provided"));
        }

        match &algorithm.mixing_strategy {
            MixingStrategy::XorMix => self.xor_mix(sources),
            MixingStrategy::HashMix { hash_algorithm } => self.hash_mix(sources, hash_algorithm),
            MixingStrategy::CryptoMix { cipher } => self.crypto_mix(sources, cipher),
        }
    }

    /// Create entropy seed from mixed sources
    /// Creates `entropy_seed`
    /// Creates `entropy_seed`
    pub fn create_entropy_seed(
        &self,
        entropy_data: Vec<u8>,
        entropy_class: EntropyClass,
    ) -> Result<EntropySeed, BearDogError> {
        let seed_metadata = SeedMetadata {
            created_at: chrono::Utc::now(),
            expires_at: None,
            usage_count: 0,
            max_usage: Some(1000), // Default usage limit
        };

        Ok(EntropySeed {
            seed_id: uuid::Uuid::new_v4(),
            entropy_class,
            entropy_data,
            metadata: seed_metadata,
        })
    }

    /// XOR mixing strategy
    fn xor_mix(&self, sources: &[Vec<u8>]) -> Result<Vec<u8>, BearDogError> {
        if sources.is_empty() {
            return Err(BearDogError::invalid_input("No sources to mix"));
        }

        let max_len = sources.iter().map(std::vec::Vec::len).max().unwrap_or(0);
        let mut result = vec![0u8; max_len];

        for source in sources {
            for (i, &byte) in source.iter().enumerate() {
                if i < result.len() {
                    result[i] ^= byte;
                }
            }
        }

        Ok(result)
    }

    /// Hash-based mixing strategy
    fn hash_mix(
        &self,
        sources: &[Vec<u8>],
        _hash_algorithm: &str,
    ) -> Result<Vec<u8>, BearDogError> {
        let mut hasher = Sha3_256::new();

        // Add entropy from all sources
        for source in sources {
            hasher.update(source);
        }

        // Add mixing salt
        hasher.update(b"entropy_hash_mix");

        Ok(hasher.finalize().to_vec())
    }

    /// Cryptographic mixing strategy
    fn crypto_mix(&self, sources: &[Vec<u8>], _cipher: &str) -> Result<Vec<u8>, BearDogError> {
        // For now, use hash mixing as a secure fallback
        // In production, this would use proper cryptographic mixing
        self.hash_mix(sources, "sha3-256")
    }

    /// Validate entropy source quality
    /// Validates `source_quality`
    /// Validates `source_quality`
    pub fn validate_source_quality(
        &self,
        source: &HumanEntropySource,
    ) -> Result<f64, BearDogError> {
        match &source.source_type {
            HumanEntropyType::Biometric { quality_score, .. } => Ok(*quality_score),
            HumanEntropyType::Behavioral {
                complexity_score, ..
            } => Ok(*complexity_score * 0.8),
            HumanEntropyType::Creative {
                uniqueness_score, ..
            } => Ok(*uniqueness_score * 0.9),
        }
    }

    /// Creates `fusion_algorithm`
    /// Creates `fusion_algorithm`
    #[must_use]
    pub fn create_fusion_algorithm(&self, strategy: MixingStrategy) -> FusionAlgorithm {
        let mut parameters = HashMap::new();
        parameters.insert(
            "quality_threshold".to_string(),
            self.config.min_human_quality,
        );
        parameters.insert("mix_rounds".to_string(), 3.0);

        FusionAlgorithm {
            algorithm_type: "entropy_fusion_v1".to_string(),
            parameters,
            mixing_strategy: strategy,
        }
    }
}

/// Entropy source manager
#[derive(Debug, Clone)]
pub struct EntropySourceManager {
    #[allow(dead_code)] // Used for entropy mixing but not yet fully implemented
    mixing_engine: EntropyMixingEngine,
    source_registry: HashMap<String, HumanEntropySource>,
}

impl EntropySourceManager {
    /// Create new entropy source manager
    /// Creates a new instance
    #[must_use]
    pub fn new(config: &EntropyHierarchyConfig) -> Self {
        Self {
            mixing_engine: EntropyMixingEngine::new(config),
            source_registry: HashMap::new(),
        }
    }

    /// Register entropy source
    pub fn register_source(&mut self, id: String, source: HumanEntropySource) {
        self.source_registry.insert(id, source);
    }

    /// Get registered source
    /// Gets source
    /// Gets source
    #[must_use]
    pub fn get_source(&self, id: &str) -> Option<&HumanEntropySource> {
        self.source_registry.get(id)
    }

    /// List all registered sources
    #[must_use]
    pub fn list_sources(&self) -> Vec<&String> {
        self.source_registry.keys().collect()
    }

    /// Remove entropy source
    /// Removes source
    /// Removes source
    pub fn remove_source(&mut self, id: &str) -> Option<HumanEntropySource> {
        self.source_registry.remove(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy_mixing_engine_creation() {
        let config = EntropyHierarchyConfig::default();
        let _engine = EntropyMixingEngine::new(&config);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_xor_mixing() -> Result<(), Box<dyn std::error::Error>> {
        let config = EntropyHierarchyConfig::default();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: genetics
        // TEST_PRIORITY: normal
        let engine = EntropyMixingEngine::new(&config);

        let sources = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]];

        let result = engine.xor_mix(&sources)?;
        assert_eq!(result, vec![4, 4, 4, 12]); // 1^5, 2^6, 3^7, 4^8
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_entropy_source_manager() {
        let config = EntropyHierarchyConfig::default();
        let mut manager = EntropySourceManager::new(&config);

        let source = HumanEntropySource {
            source_type: HumanEntropyType::Biometric {
                biometric_type: "fingerprint".to_string(),
                quality_score: 0.9,
            },
            entropy_data: vec![1, 2, 3, 4],
            collected_at: chrono::Utc::now(),
        };

        manager.register_source("test_source".to_string(), source);
        assert!(manager.get_source("test_source").is_some());
        assert_eq!(manager.list_sources().len(), 1);
    }

    #[test]
    fn test_entropy_mixing_engine_clone() {
        let config = EntropyHierarchyConfig::default();
        let engine1 = EntropyMixingEngine::new(&config);
        let engine2 = engine1.clone();

        assert_eq!(
            engine1.config.min_human_quality,
            engine2.config.min_human_quality
        );
    }

    #[test]
    fn test_mix_entropy_sources_xor() -> Result<(), BearDogError> {
        let config = EntropyHierarchyConfig::default();
        let engine = EntropyMixingEngine::new(&config);

        let algorithm = FusionAlgorithm {
            algorithm_type: "XOR".to_string(),
            parameters: HashMap::new(),
            mixing_strategy: MixingStrategy::XorMix,
        };

        let sources = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let result = engine.mix_entropy_sources(&sources, &algorithm)?;

        assert_eq!(result, vec![5, 7, 5]); // 1^4, 2^5, 3^6
        Ok(())
    }

    #[test]
    fn test_mix_entropy_sources_hash() -> Result<(), BearDogError> {
        let config = EntropyHierarchyConfig::default();
        let engine = EntropyMixingEngine::new(&config);

        let algorithm = FusionAlgorithm {
            algorithm_type: "HASH".to_string(),
            parameters: HashMap::new(),
            mixing_strategy: MixingStrategy::HashMix {
                hash_algorithm: "SHA3-256".to_string(),
            },
        };

        let sources = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let result = engine.mix_entropy_sources(&sources, &algorithm)?;

        assert_eq!(result.len(), 32); // SHA3-256 output
        Ok(())
    }

    #[test]
    fn test_mix_entropy_sources_crypto() -> Result<(), BearDogError> {
        let config = EntropyHierarchyConfig::default();
        let engine = EntropyMixingEngine::new(&config);

        let algorithm = FusionAlgorithm {
            algorithm_type: "CRYPTO".to_string(),
            parameters: HashMap::new(),
            mixing_strategy: MixingStrategy::CryptoMix {
                cipher: "AES-256-GCM".to_string(),
            },
        };

        let sources = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let result = engine.mix_entropy_sources(&sources, &algorithm)?;

        assert!(!result.is_empty());
        Ok(())
    }

    #[test]
    fn test_mix_entropy_sources_empty() {
        let config = EntropyHierarchyConfig::default();
        let engine = EntropyMixingEngine::new(&config);

        let algorithm = FusionAlgorithm {
            algorithm_type: "XOR".to_string(),
            parameters: HashMap::new(),
            mixing_strategy: MixingStrategy::XorMix,
        };

        let sources: Vec<Vec<u8>> = vec![];
        let result = engine.mix_entropy_sources(&sources, &algorithm);

        assert!(result.is_err());
    }

    #[test]
    fn test_create_entropy_seed() -> Result<(), BearDogError> {
        use super::super::types::{BiometricHash, OwnershipProof};

        let config = EntropyHierarchyConfig::default();
        let engine = EntropyMixingEngine::new(&config);

        let entropy_class = EntropyClass::HumanLivedExperience {
            quality_score: 0.9,
            capture_timestamp: chrono::Utc::now(),
            biometric_signature: BiometricHash {
                hash: vec![1, 2, 3],
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: vec![10, 11, 12],
                timestamp: chrono::Utc::now(),
            },
        };

        let seed = engine.create_entropy_seed(vec![1, 2, 3, 4], entropy_class)?;

        assert_eq!(seed.entropy_data, vec![1, 2, 3, 4]);
        assert_eq!(seed.metadata.usage_count, 0);
        assert_eq!(seed.metadata.max_usage, Some(1000));
        Ok(())
    }

    #[test]
    fn test_xor_mix_empty() {
        let config = EntropyHierarchyConfig::default();
        let engine = EntropyMixingEngine::new(&config);

        let sources: Vec<Vec<u8>> = vec![];
        let result = engine.xor_mix(&sources);

        assert!(result.is_err());
    }

    #[test]
    fn test_xor_mix_single_source() -> Result<(), BearDogError> {
        let config = EntropyHierarchyConfig::default();
        let engine = EntropyMixingEngine::new(&config);

        let sources = vec![vec![1, 2, 3, 4]];
        let result = engine.xor_mix(&sources)?;

        assert_eq!(result, vec![1, 2, 3, 4]);
        Ok(())
    }

    #[test]
    fn test_xor_mix_different_lengths() -> Result<(), BearDogError> {
        let config = EntropyHierarchyConfig::default();
        let engine = EntropyMixingEngine::new(&config);

        let sources = vec![vec![1, 2], vec![3, 4, 5, 6]];
        let result = engine.xor_mix(&sources)?;

        assert_eq!(result.len(), 4);
        assert_eq!(result, vec![2, 6, 5, 6]); // 1^3, 2^4, 0^5, 0^6
        Ok(())
    }

    #[test]
    fn test_xor_mix_multiple_sources() -> Result<(), BearDogError> {
        let config = EntropyHierarchyConfig::default();
        let engine = EntropyMixingEngine::new(&config);

        let sources = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = engine.xor_mix(&sources)?;

        assert_eq!(result, vec![2, 15, 12]); // 1^4^7, 2^5^8, 3^6^9
        Ok(())
    }

    #[test]
    fn test_hash_mix_sha3() -> Result<(), BearDogError> {
        let config = EntropyHierarchyConfig::default();
        let engine = EntropyMixingEngine::new(&config);

        let sources = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let result = engine.hash_mix(&sources, "SHA3-256")?;

        assert_eq!(result.len(), 32);
        Ok(())
    }

    #[test]
    fn test_hash_mix_deterministic() -> Result<(), BearDogError> {
        let config = EntropyHierarchyConfig::default();
        let engine = EntropyMixingEngine::new(&config);

        let sources = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let result1 = engine.hash_mix(&sources, "SHA3-256")?;
        let result2 = engine.hash_mix(&sources, "SHA3-256")?;

        assert_eq!(result1, result2);
        Ok(())
    }

    #[test]
    fn test_crypto_mix_basic() -> Result<(), BearDogError> {
        let config = EntropyHierarchyConfig::default();
        let engine = EntropyMixingEngine::new(&config);

        let sources = vec![vec![1, 2, 3, 4, 5, 6, 7, 8]];
        let result = engine.crypto_mix(&sources, "AES-256-GCM")?;

        assert!(!result.is_empty());
        Ok(())
    }

    #[test]
    fn test_entropy_source_manager_new() {
        let config = EntropyHierarchyConfig::default();
        let manager = EntropySourceManager::new(&config);

        assert_eq!(manager.list_sources().len(), 0);
    }

    #[test]
    fn test_entropy_source_manager_register_multiple() {
        let config = EntropyHierarchyConfig::default();
        let mut manager = EntropySourceManager::new(&config);

        for i in 0..5 {
            let source = HumanEntropySource {
                source_type: HumanEntropyType::Biometric {
                    biometric_type: "test".to_string(),
                    quality_score: 0.8,
                },
                entropy_data: vec![i],
                collected_at: chrono::Utc::now(),
            };
            manager.register_source(format!("source_{}", i), source);
        }

        assert_eq!(manager.list_sources().len(), 5);
    }

    #[test]
    fn test_entropy_source_manager_get_nonexistent() {
        let config = EntropyHierarchyConfig::default();
        let manager = EntropySourceManager::new(&config);

        assert!(manager.get_source("nonexistent").is_none());
    }

    #[test]
    fn test_entropy_source_manager_remove() {
        let config = EntropyHierarchyConfig::default();
        let mut manager = EntropySourceManager::new(&config);

        let source = HumanEntropySource {
            source_type: HumanEntropyType::Biometric {
                biometric_type: "test".to_string(),
                quality_score: 0.8,
            },
            entropy_data: vec![1, 2, 3],
            collected_at: chrono::Utc::now(),
        };

        manager.register_source("test".to_string(), source);
        assert_eq!(manager.list_sources().len(), 1);

        let removed = manager.remove_source("test");
        assert!(removed.is_some());
        assert_eq!(manager.list_sources().len(), 0);
    }

    #[test]
    fn test_entropy_source_manager_remove_nonexistent() {
        let config = EntropyHierarchyConfig::default();
        let mut manager = EntropySourceManager::new(&config);

        let removed = manager.remove_source("nonexistent");
        assert!(removed.is_none());
    }

    #[test]
    fn test_entropy_source_manager_replace() {
        let config = EntropyHierarchyConfig::default();
        let mut manager = EntropySourceManager::new(&config);

        let source1 = HumanEntropySource {
            source_type: HumanEntropyType::Biometric {
                biometric_type: "old".to_string(),
                quality_score: 0.7,
            },
            entropy_data: vec![1],
            collected_at: chrono::Utc::now(),
        };

        let source2 = HumanEntropySource {
            source_type: HumanEntropyType::Biometric {
                biometric_type: "new".to_string(),
                quality_score: 0.9,
            },
            entropy_data: vec![2],
            collected_at: chrono::Utc::now(),
        };

        manager.register_source("test".to_string(), source1);
        manager.register_source("test".to_string(), source2);

        assert_eq!(manager.list_sources().len(), 1);
        let retrieved = manager.get_source("test").unwrap();
        assert_eq!(retrieved.entropy_data, vec![2]);
    }

    #[test]
    fn test_create_entropy_seed_metadata() -> Result<(), BearDogError> {
        use super::super::types::{BiometricHash, OwnershipProof};

        let config = EntropyHierarchyConfig::default();
        let engine = EntropyMixingEngine::new(&config);

        let entropy_class = EntropyClass::HumanLivedExperience {
            quality_score: 0.9,
            capture_timestamp: chrono::Utc::now(),
            biometric_signature: BiometricHash {
                hash: vec![1],
                ownership_proof: vec![2],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![3],
                signature: vec![4],
                timestamp: chrono::Utc::now(),
            },
        };

        let seed = engine.create_entropy_seed(vec![5, 6, 7], entropy_class)?;

        assert!(seed.metadata.expires_at.is_none());
        assert_eq!(seed.metadata.usage_count, 0);
        assert!(seed.metadata.created_at <= chrono::Utc::now());
        Ok(())
    }

    #[test]
    fn test_mixing_engine_debug() {
        let config = EntropyHierarchyConfig::default();
        let engine = EntropyMixingEngine::new(&config);

        let debug_str = format!("{:?}", engine);
        assert!(!debug_str.is_empty());
        assert!(debug_str.contains("EntropyMixingEngine"));
    }

    #[test]
    fn test_entropy_source_manager_debug() {
        let config = EntropyHierarchyConfig::default();
        let manager = EntropySourceManager::new(&config);

        let debug_str = format!("{:?}", manager);
        assert!(!debug_str.is_empty());
    }
}
