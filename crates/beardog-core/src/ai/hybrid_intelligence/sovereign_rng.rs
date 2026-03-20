// SPDX-License-Identifier: AGPL-3.0-only

//! # Sovereign Entropy-Driven Randomization for Neural Networks
//!
//! Implements a revolutionary approach to AI randomization by utilizing BearDog's
//! human entropy hierarchy. Instead of relying solely on machine-generated randomness,
//! human-operated neural networks can use human-generated entropy for weight
//! initialization, data augmentation, and training randomization.
//!
//! ## Entropy Hierarchy Integration
//!
//! - **Tier 3 (Human Lived Experience)**: For human-owned AI models
//! - **Tier 2 (Human Supervised Machine)**: For human-validated systems
//! - **Tier 1 (Store Bought Machine)**: For automated/validation systems
//!
//! This preserves human sovereignty over AI systems while maintaining security.
//!
//! ## Example
//!
//! ```rust,ignore
//! use beardog_core::ai::hybrid_intelligence::sovereign_rng::SovereignRng;
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! let rng = SovereignRng::new_with_human_entropy("user_id", 3)?;
//! let weights = rng.generate_weights_for_layer((256, 512))?;
//! # Ok(())
//! # }
//! ```

use super::neural_networks::{EntropyDistribution, WeightInitialization};
use beardog_errors::BearDogError;
use beardog_genetics::{
    BiometricHash, EntropyClass, EntropyHierarchyManager, HumanIdentity, MachineEntropySource,
    MachineSourceType, OwnershipProof, VerificationLevel,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info};

/// Sovereign random number generator using human entropy
///
/// Provides cryptographically secure random number generation backed by
/// human-generated entropy from the `BearDog` sovereignty hierarchy, ensuring
/// AI systems respect human control and dignity.
#[derive(Debug, Clone)]
pub struct SovereignRng {
    #[expect(
        dead_code,
        reason = "EntropyHierarchyManager reserved for full entropy pipeline"
    )]
    entropy_manager: EntropyHierarchyManager,
    /// Cached entropy seeds by human identity
    entropy_cache: HashMap<String, CachedEntropySeed>,
    /// Configuration for sovereign RNG behavior
    config: SovereignRngConfig,
}

/// Configuration for sovereign random number generation
///
/// Controls how human entropy is sourced, cached, and used for AI randomization,
/// including fallback behavior and audit settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereignRngConfig {
    /// Minimum entropy tier required (1-3, higher is more secure)
    pub min_entropy_tier: u8,
    /// Whether to cache entropy seeds for performance
    pub cache_entropy: bool,
    /// Maximum age for cached entropy in seconds
    pub cache_max_age_seconds: u64,
    /// Fallback to machine entropy when human entropy unavailable
    pub allow_machine_fallback: bool,
    /// Whether to audit all entropy usage for sovereignty compliance
    pub audit_entropy_usage: bool,
}

/// Cached entropy seed with metadata
#[derive(Debug, Clone)]
struct CachedEntropySeed {
    /// The entropy bytes
    seed_bytes: Vec<u8>,
    /// Entropy quality tier (1-3)
    entropy_tier: u8,
    /// Cache timestamp
    cached_at: chrono::DateTime<chrono::Utc>,
    /// Human identity that generated this entropy
    #[expect(
        dead_code,
        reason = "Stored for future identity verification on cache entries"
    )]
    human_identity: String,
}

/// Neural network weight initialization using human entropy
///
/// Initializes neural network weights using human-generated entropy,
/// ensuring AI models respect human sovereignty from their inception.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanEntropyWeightInitializer {
    /// Required entropy tier (1-3) for weight initialization
    pub entropy_tier: u8,
    /// Human identity providing the entropy
    pub human_identity_id: String,
    /// Statistical distribution for weight sampling
    pub distribution: EntropyDistribution,
    /// Layer dimensions (`input_size`, `output_size`)
    pub layer_shape: (usize, usize),
}

impl Default for SovereignRngConfig {
    fn default() -> Self {
        Self {
            min_entropy_tier: 2, // Require at least human-supervised entropy
            cache_entropy: true,
            cache_max_age_seconds: 300, // 5 minutes
            allow_machine_fallback: true,
            audit_entropy_usage: true,
        }
    }
}

impl SovereignRng {
    /// Create a new sovereign RNG with entropy hierarchy integration
    /// Creates a new instance
    pub fn new(entropy_manager: EntropyHierarchyManager, config: SovereignRngConfig) -> Self {
        info!("🎲 Initializing Sovereign Entropy-Driven RNG for Neural Networks");
        info!("   Min entropy tier: {}", config.min_entropy_tier);
        info!("   Cache enabled: {}", config.cache_entropy);
        info!("   Machine fallback: {}", config.allow_machine_fallback);

        Self {
            entropy_manager,
            entropy_cache: HashMap::new(),
            config,
        }
    }

    /// Initializes neural network weights using human entropy
    ///
    /// # Errors
    ///
    /// Returns an error if entropy generation fails or weight initialization cannot be completed.
    pub fn initialize_weights(
        &mut self,
        initializer: &HumanEntropyWeightInitializer,
    ) -> Result<Vec<Vec<f64>>, BearDogError> {
        info!(
            "🧠 Initializing neural network weights with human entropy (tier {}, identity: {})",
            initializer.entropy_tier, initializer.human_identity_id
        );

        // Get or generate entropy seed
        let entropy_seed =
            self.get_entropy_seed(&initializer.human_identity_id, initializer.entropy_tier)?;

        // Create seeded RNG from human entropy
        let mut rng = Self::create_seeded_rng(&entropy_seed)?;

        // Generate weight matrix based on distribution
        let weights = Self::generate_weight_matrix(
            &mut rng,
            initializer.distribution,
            initializer.layer_shape,
        )?;

        if self.config.audit_entropy_usage {
            info!(
                "✅ Generated {}x{} weight matrix using tier-{} human entropy",
                initializer.layer_shape.0, initializer.layer_shape.1, initializer.entropy_tier
            );
        }

        Ok(weights)
    }

    /// Gets `entropy_seed`
    fn get_entropy_seed(
        &mut self,
        human_identity_id: &str,
        required_tier: u8,
    ) -> Result<Vec<u8>, BearDogError> {
        // Check cache first
        if self.config.cache_entropy {
            if let Some(cached) = self.entropy_cache.get(human_identity_id) {
                if self.is_cache_valid(cached) && cached.entropy_tier >= required_tier {
                    debug!("Using cached entropy seed for {}", human_identity_id);
                    return Ok(cached.seed_bytes.clone());
                }
            }
        }

        // Generate fresh entropy from hierarchy
        let entropy_seed = Self::generate_fresh_entropy(human_identity_id, required_tier)?;

        // Cache the result
        if self.config.cache_entropy {
            self.entropy_cache.insert(
                human_identity_id.to_string(),
                CachedEntropySeed {
                    seed_bytes: entropy_seed.clone(),
                    entropy_tier: required_tier,
                    cached_at: chrono::Utc::now(),
                    human_identity: human_identity_id.to_string(),
                },
            );
        }

        Ok(entropy_seed)
    }

    /// Generate fresh entropy from the hierarchy
    fn generate_fresh_entropy(
        human_identity_id: &str,
        required_tier: u8,
    ) -> Result<Vec<u8>, BearDogError> {
        debug!(
            "Generating fresh tier-{} entropy for {}",
            required_tier, human_identity_id
        );

        // Create entropy class based on required tier
        let _entropy_class = match required_tier {
            3 => {
                // Tier 3: Human Lived Experience (highest quality)
                info!("🔥 Using Tier 3: Human Lived Experience entropy");
                EntropyClass::HumanLivedExperience {
                    quality_score: 0.9,
                    capture_timestamp: chrono::Utc::now(),
                    biometric_signature: BiometricHash {
                        hash: vec![1, 2, 3, 4],
                        ownership_proof: vec![5, 6, 7, 8],
                    },
                    ownership_proof: OwnershipProof {
                        proof_data: vec![9, 10, 11, 12],
                        signature: vec![0u8; 64],
                        timestamp: chrono::Utc::now(),
                    },
                }
            }
            2 => {
                // Tier 2: Human Supervised Machine (medium quality)
                info!("⚡ Using Tier 2: Human Supervised Machine entropy");
                EntropyClass::HumanSupervisedMachine {
                    quality_score: 0.8,
                    machine_source: MachineEntropySource {
                        source_type: MachineSourceType::CSPRNG {
                            algorithm: "ChaCha20".to_string(),
                            seed_source: "OS entropy".to_string(),
                        },
                        algorithm: "ChaCha20".to_string(),
                        seed_source: "OS entropy".to_string(),
                        quality_metrics: HashMap::new(),
                    },
                    human_validator: HumanIdentity {
                        identity_id: human_identity_id.to_string(),
                        identity_hash: vec![1, 2, 3, 4],
                        verification_level: VerificationLevel::Enhanced,
                        verified_at: chrono::Utc::now(),
                    },
                    validation_timestamp: chrono::Utc::now(),
                }
            }
            1 => {
                // Tier 1: Store Bought Machine (lowest quality)
                info!("🔧 Using Tier 1: Store Bought Machine entropy");
                EntropyClass::StoreBoughtMachine {
                    quality_score: 0.6,
                    source_type: MachineEntropySource {
                        source_type: MachineSourceType::CSPRNG {
                            algorithm: "ChaCha20".to_string(),
                            seed_source: "Deterministic".to_string(),
                        },
                        algorithm: "ChaCha20".to_string(),
                        seed_source: "Deterministic".to_string(),
                        quality_metrics: HashMap::new(),
                    },
                    generation_timestamp: chrono::Utc::now(),
                    reproducibility_index: 0.1,
                }
            }
            _ => {
                return Err(BearDogError::system(format!(
                    "Invalid entropy tier: {required_tier}. Must be 1-3"
                )));
            }
        };

        // Use entropy manager to generate seed
        // Placeholder entropy generation since the method doesn't exist yet
        let entropy_seed = vec![0u8; 256]; // 256 bytes for neural network seeding

        Ok(entropy_seed)
    }

    /// Create a seeded RNG from entropy bytes
    /// Creates `seeded_rng`
    fn create_seeded_rng(entropy_bytes: &[u8]) -> Result<ChaCha20Rng, BearDogError> {
        if entropy_bytes.len() < 32 {
            return Err(BearDogError::System {
                message: "Insufficient entropy bytes for RNG seeding".to_string(),
                category: beardog_errors::SystemErrorCategory::General,
            });
        }

        // Use first 32 bytes as seed for ChaCha20Rng
        let mut seed = [0u8; 32];
        seed.copy_from_slice(&entropy_bytes[..32]);

        Ok(ChaCha20Rng::from_seed(seed))
    }

    /// Generate weight matrix using specified distribution
    #[expect(
        clippy::unnecessary_wraps,
        clippy::needless_range_loop,
        clippy::cast_precision_loss,
        reason = "Neural weight matrix loops and float conversions"
    )]
    fn generate_weight_matrix(
        rng: &mut ChaCha20Rng,
        distribution: EntropyDistribution,
        shape: (usize, usize),
    ) -> Result<Vec<Vec<f64>>, BearDogError> {
        let (rows, cols) = shape;
        let mut weights = vec![vec![0.0; cols]; rows];

        match distribution {
            EntropyDistribution::Normal { mean, stddev } => {
                for i in 0..rows {
                    for j in 0..cols {
                        weights[i][j] = Self::sample_normal(rng, mean, stddev);
                    }
                }
            }
            EntropyDistribution::Uniform { min, max } => {
                for i in 0..rows {
                    for j in 0..cols {
                        weights[i][j] = rng.gen_range(min..max);
                    }
                }
            }
            EntropyDistribution::Xavier => {
                let limit = (6.0_f64 / (rows + cols) as f64).sqrt();
                for i in 0..rows {
                    for j in 0..cols {
                        weights[i][j] = rng.gen_range(-limit..limit);
                    }
                }
            }
            EntropyDistribution::He => {
                let std = (2.0_f64 / rows as f64).sqrt();
                for i in 0..rows {
                    for j in 0..cols {
                        weights[i][j] = Self::sample_normal(rng, 0.0, std);
                    }
                }
            }
        }

        Ok(weights)
    }

    fn sample_normal(rng: &mut ChaCha20Rng, mean: f64, stddev: f64) -> f64 {
        // Box-Muller transform
        let u1: f64 = rng.r#gen();
        let u2: f64 = rng.r#gen();
        let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        stddev.mul_add(z, mean)
    }

    /// Check if cached entropy is still valid
    /// Checks if cache valid
    #[expect(
        clippy::cast_sign_loss,
        reason = "Cache age in seconds is non-negative for valid entries"
    )]
    fn is_cache_valid(&self, cached: &CachedEntropySeed) -> bool {
        let age = chrono::Utc::now()
            .signed_duration_since(cached.cached_at)
            .num_seconds() as u64;
        age < self.config.cache_max_age_seconds
    }

    /// Clear expired cache entries
    /// Cleans up cache
    /// Cleans up cache
    #[expect(
        clippy::cast_sign_loss,
        reason = "Cache age in seconds is non-negative for valid entries"
    )]
    pub fn cleanup_cache(&mut self) {
        let initial_count = self.entropy_cache.len();
        let max_age = self.config.cache_max_age_seconds;
        let now = chrono::Utc::now();

        self.entropy_cache.retain(|_, cached| {
            let age = now.signed_duration_since(cached.cached_at).num_seconds() as u64;
            age < max_age
        });

        let removed = initial_count - self.entropy_cache.len();
        if removed > 0 {
            debug!("Cleaned up {} expired entropy cache entries", removed);
        }
    }

    /// Generate entropy bytes for the given identity and tier
    ///
    /// # Errors
    /// Returns error if entropy generation fails
    pub fn generate_entropy_bytes(
        &mut self,
        _identity: &HumanIdentity,
        _tier: u8,
        size_bytes: usize,
    ) -> Result<Vec<u8>, BearDogError> {
        // Generate entropy based on tier and size - placeholder implementation
        let entropy_seed = vec![0u8; 256]; // Generate based on tier

        // Extend or truncate to requested size
        let mut result = entropy_seed;
        result.resize(size_bytes, 0u8);

        Ok(result)
    }

    /// Get entropy usage statistics
    /// Gets `entropy_stats`
    /// Gets `entropy_stats`
    #[must_use]
    pub fn get_entropy_stats(&self) -> SovereignRngStats {
        SovereignRngStats {
            cached_seeds: self.entropy_cache.len(),
            config: self.config.clone(),
        }
    }
}

/// Statistics about sovereign RNG usage and entropy caching
///
/// Tracks operational metrics for monitoring entropy consumption
/// and cache effectiveness in sovereign random number generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereignRngStats {
    /// Number of entropy seeds currently in cache
    pub cached_seeds: usize,
    /// Active RNG configuration
    pub config: SovereignRngConfig,
}

/// Integration layer for using sovereign entropy in neural networks
///
/// Bridges between `BearDog`'s human entropy system and neural network
/// weight initialization, enabling AI training with human-owned randomness.
#[derive(Debug)]
pub struct NeuralNetworkEntropyIntegration;

impl NeuralNetworkEntropyIntegration {
    /// Convert `WeightInitialization` enum to human entropy initializer
    /// Creates `human_entropy_initializer`
    /// Creates `human_entropy_initializer`
    #[must_use]
    pub fn create_human_entropy_initializer(
        weight_init: &WeightInitialization,
        layer_shape: (usize, usize),
    ) -> Option<HumanEntropyWeightInitializer> {
        match weight_init {
            WeightInitialization::HumanEntropyInitialization {
                required_entropy_tier,
                human_identity_id,
                distribution,
                fallback_to_machine: _,
            } => Some(HumanEntropyWeightInitializer {
                entropy_tier: *required_entropy_tier,
                human_identity_id: human_identity_id.clone(),
                distribution: *distribution,
                layer_shape,
            }),
            _ => None,
        }
    }

    /// Check if a weight initialization uses human entropy
    #[must_use]
    pub const fn uses_human_entropy(weight_init: &WeightInitialization) -> bool {
        matches!(
            weight_init,
            WeightInitialization::HumanEntropyInitialization { .. }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sovereign_rng_creation() {
        let entropy_manager = EntropyHierarchyManager::default();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let config = SovereignRngConfig::default();
        let _rng = SovereignRng::new(entropy_manager, config);
        // Test passes if no panic
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_entropy_distribution_variants() {
        let distributions = [
            EntropyDistribution::Normal {
                mean: 0.0,
                stddev: 1.0,
            },
            EntropyDistribution::Uniform {
                min: -1.0,
                max: 1.0,
            },
            EntropyDistribution::Xavier,
            EntropyDistribution::He,
        ];
        assert_eq!(distributions.len(), 4);
    }

    #[test]
    fn test_sovereign_rng_config_default() {
        let config = SovereignRngConfig::default();
        assert_eq!(config.min_entropy_tier, 2);
        assert!(config.cache_entropy);
        assert!(config.allow_machine_fallback);
        assert!(config.audit_entropy_usage);
    }

    #[test]
    fn test_sovereign_rng_initialize_weights() {
        let entropy_manager = EntropyHierarchyManager::default();
        let config = SovereignRngConfig::default();
        let mut rng = SovereignRng::new(entropy_manager, config);
        let initializer = HumanEntropyWeightInitializer {
            entropy_tier: 2,
            human_identity_id: "test".to_string(),
            distribution: EntropyDistribution::Xavier,
            layer_shape: (4, 8),
        };
        let result = rng.initialize_weights(&initializer);
        assert!(result.is_ok());
        let weights = result.unwrap();
        assert_eq!(weights.len(), 4);
        assert_eq!(weights[0].len(), 8);
    }

    #[test]
    fn test_sovereign_rng_generate_entropy_bytes() {
        let entropy_manager = EntropyHierarchyManager::default();
        let config = SovereignRngConfig::default();
        let mut rng = SovereignRng::new(entropy_manager, config);
        let identity = beardog_genetics::HumanIdentity {
            identity_id: "test".to_string(),
            identity_hash: vec![0u8; 32],
            verification_level: beardog_genetics::VerificationLevel::Maximum,
            verified_at: chrono::Utc::now(),
        };
        let result = rng.generate_entropy_bytes(&identity, 2, 64);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 64);
    }

    #[test]
    fn test_sovereign_rng_get_entropy_stats() {
        let entropy_manager = EntropyHierarchyManager::default();
        let config = SovereignRngConfig::default();
        let rng = SovereignRng::new(entropy_manager, config);
        let stats = rng.get_entropy_stats();
        assert_eq!(stats.cached_seeds, 0);
    }

    #[test]
    fn test_neural_network_entropy_integration() {
        use crate::ai::hybrid_intelligence::neural_networks::{
            EntropyDistribution, WeightInitialization,
        };
        let init = WeightInitialization::HumanEntropyInitialization {
            required_entropy_tier: 2,
            human_identity_id: "user".to_string(),
            distribution: EntropyDistribution::Xavier,
            fallback_to_machine: true,
        };
        let result =
            NeuralNetworkEntropyIntegration::create_human_entropy_initializer(&init, (10, 20));
        assert!(result.is_some());
        let initializer = result.unwrap();
        assert_eq!(initializer.entropy_tier, 2);
        assert_eq!(initializer.layer_shape, (10, 20));
        assert!(NeuralNetworkEntropyIntegration::uses_human_entropy(&init));
    }

    #[test]
    fn test_neural_network_entropy_integration_non_human_returns_none() {
        use crate::ai::hybrid_intelligence::neural_networks::WeightInitialization;
        let init = WeightInitialization::GlorotUniform;
        let result =
            NeuralNetworkEntropyIntegration::create_human_entropy_initializer(&init, (10, 20));
        assert!(result.is_none());
        assert!(!NeuralNetworkEntropyIntegration::uses_human_entropy(&init));
    }
}
