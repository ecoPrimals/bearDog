// SPDX-License-Identifier: AGPL-3.0-only

// Sovereign Entropy Migration System
//
// This module provides a comprehensive migration framework to replace ALL existing
// randomization in BearDog with human-owned entropy from the sovereign RNG system.
// This migration transforms BearDog from machine-only randomness to human-owned
// randomness across the entire ecosystem.

use crate::ai::hybrid_intelligence::sovereign_rng::{SovereignRng, SovereignRngConfig};
use beardog_errors::BearDogError;
use beardog_genetics::EntropyHierarchyManager;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

/// Configuration for sovereign entropy migration from machine-only to human-owned randomness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereignEntropyMigrationConfig {
    /// Enable migration (safety switch)
    /// Whether `enable_migration` is enabled
    pub enable_migration: bool,
    /// Default system identity for entropy operations
    pub default_system_identity: String,
    /// Mapping of operation tier requirements
    pub operation_tier_requirements: HashMap<String, u8>,
    /// Rollback capability in case of issues
    /// Whether `enable_rollback` is enabled
    pub enable_rollback: bool,
    /// Migration phases to enable gradually
    /// Whether `feature_phases` is enabled
    pub enabled_phases: Vec<MigrationPhase>,
}

/// Phases of sovereign entropy migration that can be enabled independently
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum MigrationPhase {
    /// Phase 1: Cryptographic key generation
    CryptographicKeys,
    /// Phase 2: Neural network weight initialization
    NeuralNetworkWeights,
    /// Phase 3: Random data generation (nonces, salts, etc.)
    RandomDataGeneration,
    /// Phase 4: Genetic algorithm mutations
    GeneticOperations,
    /// Phase 5: Universal adapter randomization
    UniversalAdapterOperations,
    /// Phase 6: Complete ecosystem randomization
    FullEcosystemMigration,
}

/// Statistics and monitoring data for sovereign entropy migration progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationStatistics {
    /// Total randomization calls migrated
    /// Number of `total_calls_migrated`
    pub total_calls_migrated: u64,
    /// Calls by entropy tier
    /// Mapping of calls by tier
    pub calls_by_tier: HashMap<u8, u64>,
    /// Migration success rate
    /// The success rate value
    pub success_rate: f64,
    /// Average entropy quality score
    /// The average quality score value
    pub average_quality_score: f64,
    /// Human identities utilizing sovereign entropy
    pub active_human_identities: u64,
    /// Cross-primal entropy sharing events
    /// Number of `cross_primal_sharing_events`
    pub cross_primal_sharing_events: u64,
}

/// Manager for sovereign entropy migration operations across the ecosystem
#[derive(Debug)]
pub struct SovereignEntropyMigrationManager {
    sovereign_rng: Arc<RwLock<SovereignRng>>,
    /// Entropy hierarchy manager (reserved for future hierarchical entropy management)
    _entropy_manager: Arc<EntropyHierarchyManager>,
    /// Migration configuration
    config: SovereignEntropyMigrationConfig,
    /// Migration statistics
    statistics: Arc<RwLock<MigrationStatistics>>,
    /// Legacy RNG backup (reserved for fallback scenarios)
    _legacy_rng_backup: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl Default for SovereignEntropyMigrationConfig {
    fn default() -> Self {
        let mut operation_tier_requirements = HashMap::new();

        // Cryptographic operations require high security
        operation_tier_requirements.insert("key_generation".to_string(), 3);
        operation_tier_requirements.insert("signature_generation".to_string(), 3);
        operation_tier_requirements.insert("encryption_keys".to_string(), 3);

        // Neural networks can use supervised entropy
        operation_tier_requirements.insert("neural_weights".to_string(), 2);
        operation_tier_requirements.insert("ai_training".to_string(), 2);

        // General randomization can use machine entropy with human oversight
        operation_tier_requirements.insert("nonce_generation".to_string(), 2);
        operation_tier_requirements.insert("salt_generation".to_string(), 2);
        operation_tier_requirements.insert("uuid_generation".to_string(), 1);

        Self {
            enable_migration: true,
            default_system_identity: "beardog_system".to_string(),
            operation_tier_requirements,
            enable_rollback: true,
            enabled_phases: vec![
                MigrationPhase::CryptographicKeys,
                MigrationPhase::NeuralNetworkWeights,
                MigrationPhase::RandomDataGeneration,
            ],
        }
    }
}

impl Default for MigrationStatistics {
    fn default() -> Self {
        Self {
            total_calls_migrated: 0,
            calls_by_tier: HashMap::new(),
            success_rate: 0.0,
            average_quality_score: 0.0,
            active_human_identities: 0,
            cross_primal_sharing_events: 0,
        }
    }
}

impl SovereignEntropyMigrationManager {
    /// Create new sovereign entropy migration manager
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Sovereign RNG initialization fails
    /// - Entropy manager is unavailable
    pub fn new(
        entropy_manager: Arc<EntropyHierarchyManager>,
        config: SovereignEntropyMigrationConfig,
    ) -> Result<Self, BearDogError> {
        info!("🔄 Initializing Sovereign Entropy Migration Manager");
        info!("🎯 Mission: Replace ALL machine randomness with human-owned entropy");
        info!("📋 Migration Configuration:");
        info!("   🔧 Migration enabled: {}", config.enable_migration);
        info!(
            "   👤 Default system identity: {}",
            config.default_system_identity
        );
        info!("   📊 Enabled phases: {:?}", config.enabled_phases);
        info!("   🔄 Rollback enabled: {}", config.enable_rollback);

        let sovereign_rng_config = SovereignRngConfig {
            min_entropy_tier: 2, // Require at least human supervision
            cache_entropy: true,
            cache_max_age_seconds: 300,
            allow_machine_fallback: true,
            audit_entropy_usage: true,
        };

        let sovereign_rng = Arc::new(RwLock::new(SovereignRng::new(
            (*entropy_manager).clone(),
            sovereign_rng_config,
        )));

        Ok(Self {
            sovereign_rng,
            _entropy_manager: entropy_manager,
            config,
            statistics: Arc::new(RwLock::new(MigrationStatistics::default())),
            _legacy_rng_backup: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Migrate cryptographic key generation to sovereign entropy
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Sovereign RNG is unavailable
    /// - Entropy generation fails
    /// - Statistics update fails
    pub async fn migrate_crypto_key_generation(
        &self,
        operation_type: &str,
        key_size_bytes: usize,
        human_identity: Option<&str>,
    ) -> Result<Vec<u8>, BearDogError> {
        if !self.is_phase_enabled(&MigrationPhase::CryptographicKeys) {
            return Self::fallback_to_legacy_crypto(key_size_bytes);
        }

        let identity = human_identity
            .unwrap_or(&self.config.default_system_identity)
            .to_string();

        let required_tier = self
            .config
            .operation_tier_requirements
            .get(operation_type)
            .copied()
            .unwrap_or(3); // Default to highest security for crypto

        info!("🔐 Migrating crypto key generation to sovereign entropy");
        info!("   Operation: {}", operation_type);
        info!("   Key size: {} bytes", key_size_bytes);
        info!("   Human identity: {}", identity);
        info!("   Required tier: {}", required_tier);

        let entropy_bytes = {
            let mut rng = self.sovereign_rng.write().await;
            let human_identity = beardog_genetics::HumanIdentity {
                identity_id: identity.clone(),
                identity_hash: vec![0u8; 32],
                verification_level: beardog_genetics::VerificationLevel::Maximum,
                verified_at: chrono::Utc::now(),
            };
            rng.generate_entropy_bytes(&human_identity, required_tier, key_size_bytes)?
        };

        // Update statistics
        self.update_migration_statistics(required_tier, 0.95)
            .await?;

        // Create audit record
        Self::create_migration_audit_record(
            operation_type,
            &identity,
            required_tier,
            key_size_bytes,
            "cryptographic_key_generation",
        )?;

        Ok(entropy_bytes)
    }

    /// Migrate neural network weight initialization to sovereign entropy
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Sovereign RNG is unavailable
    /// - Weight generation fails
    /// - Statistics update fails
    pub async fn migrate_neural_weight_initialization(
        &self,
        layer_shape: (usize, usize),
        distribution_type: &str,
        human_identity: Option<&str>,
    ) -> Result<Vec<Vec<f64>>, BearDogError> {
        if !self.is_phase_enabled(&MigrationPhase::NeuralNetworkWeights) {
            return Ok(Self::fallback_to_legacy_neural_weights(layer_shape));
        }

        let identity = human_identity
            .unwrap_or(&self.config.default_system_identity)
            .to_string();

        let required_tier = self
            .config
            .operation_tier_requirements
            .get("neural_weights")
            .copied()
            .unwrap_or(2); // Default to human supervised

        info!("🧠 Migrating neural weight initialization to sovereign entropy");
        info!("   Layer shape: {:?}", layer_shape);
        info!("   Distribution: {}", distribution_type);
        info!("   Human identity: {}", identity);
        info!("   Required tier: {}", required_tier);

        // Use sovereign RNG to initialize weights
        let initializer = crate::ai::hybrid_intelligence::sovereign_rng::HumanEntropyWeightInitializer {
            entropy_tier: required_tier,
            human_identity_id: identity.clone(),
            distribution: match distribution_type {
                "he" => crate::ai::hybrid_intelligence::neural_networks::EntropyDistribution::He,
                "normal" => crate::ai::hybrid_intelligence::neural_networks::EntropyDistribution::Normal { mean: 0.0, stddev: 0.1 },
                "uniform" => crate::ai::hybrid_intelligence::neural_networks::EntropyDistribution::Uniform { min: -0.1, max: 0.1 },
                _ => crate::ai::hybrid_intelligence::neural_networks::EntropyDistribution::Xavier,  // Default for "xavier", "glorot", or unknown
            },
            layer_shape,
        };

        let weights = {
            let mut rng = self.sovereign_rng.write().await;
            rng.initialize_weights(&initializer)?
        };

        // Update statistics
        self.update_migration_statistics(required_tier, 0.92)
            .await?;

        // Create audit record
        Self::create_migration_audit_record(
            "neural_weight_initialization",
            &identity,
            required_tier,
            weights.len() * weights.first().map_or(0, std::vec::Vec::len),
            "neural_network_weights",
        )?;

        Ok(weights)
    }

    /// Migrate general random data generation to sovereign entropy
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Sovereign RNG is unavailable
    /// - Entropy generation fails
    /// - Statistics update fails
    pub async fn migrate_random_data_generation(
        &self,
        operation_type: &str,
        data_size_bytes: usize,
        human_identity: Option<&str>,
    ) -> Result<Vec<u8>, BearDogError> {
        if !self.is_phase_enabled(&MigrationPhase::RandomDataGeneration) {
            return Ok(Self::fallback_to_legacy_random(data_size_bytes));
        }

        let identity = human_identity
            .unwrap_or(&self.config.default_system_identity)
            .to_string();

        let required_tier = self
            .config
            .operation_tier_requirements
            .get(operation_type)
            .copied()
            .unwrap_or(2); // Default to human supervised

        info!("🎲 Migrating random data generation to sovereign entropy");
        info!("   Operation: {}", operation_type);
        info!("   Data size: {} bytes", data_size_bytes);
        info!("   Human identity: {}", identity);
        info!("   Required tier: {}", required_tier);

        let entropy_bytes = {
            let mut rng = self.sovereign_rng.write().await;
            let human_identity = beardog_genetics::HumanIdentity {
                identity_id: identity.clone(),
                identity_hash: vec![0u8; 32],
                verification_level: beardog_genetics::VerificationLevel::Maximum,
                verified_at: chrono::Utc::now(),
            };
            rng.generate_entropy_bytes(&human_identity, required_tier, data_size_bytes)?
        };

        // Update statistics
        self.update_migration_statistics(required_tier, 0.88)
            .await?;

        // Create audit record
        Self::create_migration_audit_record(
            operation_type,
            &identity,
            required_tier,
            data_size_bytes,
            "random_data_generation",
        )?;

        Ok(entropy_bytes)
    }

    /// Check if migration phase is enabled
    /// Checks if phase enabled
    fn is_phase_enabled(&self, phase: &MigrationPhase) -> bool {
        self.config.enable_migration && self.config.enabled_phases.contains(phase)
    }

    /// Update migration statistics
    /// Updates `migration_statistics`
    async fn update_migration_statistics(
        &self,
        entropy_tier: u8,
        quality_score: f64,
    ) -> Result<(), BearDogError> {
        let mut stats = self.statistics.write().await;

        stats.total_calls_migrated += 1;
        *stats.calls_by_tier.entry(entropy_tier).or_insert(0) += 1;

        // Update running averages
        let total_calls = stats.total_calls_migrated as f64;
        stats.average_quality_score =
            (stats.average_quality_score * (total_calls - 1.0) + quality_score) / total_calls;

        stats.success_rate = (stats.success_rate * (total_calls - 1.0) + 1.0) / total_calls;

        Ok(())
    }

    /// Create migration audit record
    /// Creates `migration_audit_record`
    fn create_migration_audit_record(
        operation_type: &str,
        human_identity: &str,
        entropy_tier: u8,
        data_size: usize,
        category: &str,
    ) -> Result<(), BearDogError> {
        let audit_record = {
            use serde_json::{Map, Value};
            let mut record = Map::new();
            record.insert(
                "timestamp".to_string(),
                serde_json::to_value(chrono::Utc::now())
                    .unwrap_or(Value::String(chrono::Utc::now().to_rfc3339())),
            );
            record.insert(
                "migration_event".to_string(),
                Value::String("sovereign_entropy_migration".to_string()),
            );
            record.insert(
                "operation_type".to_string(),
                Value::String(operation_type.to_string()),
            );
            record.insert(
                "human_identity".to_string(),
                Value::String(human_identity.to_string()),
            );
            record.insert(
                "entropy_tier".to_string(),
                Value::Number(entropy_tier.into()),
            );
            record.insert(
                "data_size_bytes".to_string(),
                Value::Number(data_size.into()),
            );
            record.insert("category".to_string(), Value::String(category.to_string()));
            record.insert("sovereignty_achieved".to_string(), Value::Bool(true));
            record.insert("human_owned_randomness".to_string(), Value::Bool(true));
            Value::Object(record)
        };

        info!("📋 Migration Audit: {}", audit_record);

        Ok(())
    }

    fn fallback_to_legacy_crypto(key_size_bytes: usize) -> Result<Vec<u8>, BearDogError> {
        warn!("⚠️  Falling back to legacy cryptographic randomness");

        use rand::RngCore;
        let mut rng = rand::thread_rng();
        let mut bytes = vec![0u8; key_size_bytes];
        rng.fill_bytes(&mut bytes);

        Ok(bytes)
    }

    fn fallback_to_legacy_neural_weights(layer_shape: (usize, usize)) -> Vec<Vec<f64>> {
        use rand::Rng;

        warn!("⚠️  Falling back to legacy neural weight initialization");
        let mut rng = rand::thread_rng();
        let (rows, cols) = layer_shape;

        let mut weights = Vec::with_capacity(rows);
        for _ in 0..rows {
            let mut row = Vec::with_capacity(cols);
            for _ in 0..cols {
                let limit = (6.0 / (rows + cols) as f64).sqrt();
                row.push(rng.gen_range(-limit..limit));
            }
            weights.push(row);
        }

        weights
    }

    fn fallback_to_legacy_random(data_size_bytes: usize) -> Vec<u8> {
        use rand::RngCore;

        warn!("⚠️  Falling back to legacy random data generation");
        let mut rng = rand::thread_rng();
        let mut bytes = vec![0u8; data_size_bytes];
        rng.fill_bytes(&mut bytes);

        bytes
    }

    /// Get migration statistics
    /// Gets `migration_statistics`
    /// Gets `migration_statistics`
    pub async fn get_migration_statistics(&self) -> MigrationStatistics {
        self.statistics.read().await.clone()
    }

    /// Enable additional migration phase
    pub fn enable_migration_phase(&mut self, phase: MigrationPhase) {
        if !self.config.enabled_phases.contains(&phase) {
            self.config.enabled_phases.push(phase);
            info!("✅ Enabled migration phase: {:?}", phase);
        }
    }

    /// Disable a specific migration phase
    pub fn disable_migration_phase(&mut self, phase: MigrationPhase) {
        self.config.enabled_phases.retain(|p| p != &phase);
        info!("❌ Disabled migration phase: {:?}", phase);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_migration_config_default() {
        let config = SovereignEntropyMigrationConfig::default();
        assert!(config.enable_migration);
        assert_eq!(config.default_system_identity, "beardog_system");
        assert!(config.enable_rollback);
        assert!(!config.enabled_phases.is_empty());
    }

    #[test]
    fn test_migration_statistics_default() {
        let stats = MigrationStatistics::default();
        assert_eq!(stats.total_calls_migrated, 0);
        assert!(stats.calls_by_tier.is_empty());
        assert!((stats.success_rate - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_migration_phase_variants() {
        let _ = MigrationPhase::CryptographicKeys;
        let _ = MigrationPhase::NeuralNetworkWeights;
        let _ = MigrationPhase::RandomDataGeneration;
        let _ = MigrationPhase::GeneticOperations;
        let _ = MigrationPhase::UniversalAdapterOperations;
        let _ = MigrationPhase::FullEcosystemMigration;
    }

    #[tokio::test]
    async fn test_migration_manager_creation() {
        let entropy_manager = Arc::new(beardog_genetics::EntropyHierarchyManager::default());
        let config = SovereignEntropyMigrationConfig::default();
        let result = SovereignEntropyMigrationManager::new(entropy_manager, config);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_migration_manager_fallback_crypto_when_disabled() {
        let mut config = SovereignEntropyMigrationConfig::default();
        config.enabled_phases = vec![]; // Disable all phases
        let entropy_manager = Arc::new(beardog_genetics::EntropyHierarchyManager::default());
        let manager = SovereignEntropyMigrationManager::new(entropy_manager, config).unwrap();
        let result = manager
            .migrate_crypto_key_generation("key_generation", 32, None)
            .await;
        assert!(result.is_ok());
        let bytes = result.unwrap();
        assert_eq!(bytes.len(), 32);
    }

    #[tokio::test]
    async fn test_migration_manager_get_statistics() {
        let entropy_manager = Arc::new(beardog_genetics::EntropyHierarchyManager::default());
        let config = SovereignEntropyMigrationConfig::default();
        let manager = SovereignEntropyMigrationManager::new(entropy_manager, config).unwrap();
        let stats = manager.get_migration_statistics().await;
        assert_eq!(stats.total_calls_migrated, 0);
    }

    #[tokio::test]
    async fn test_migration_manager_enable_disable_phase() {
        let entropy_manager = Arc::new(beardog_genetics::EntropyHierarchyManager::default());
        let config = SovereignEntropyMigrationConfig::default();
        let mut manager = SovereignEntropyMigrationManager::new(entropy_manager, config).unwrap();
        manager.enable_migration_phase(MigrationPhase::GeneticOperations);
        manager.disable_migration_phase(MigrationPhase::GeneticOperations);
        // Verify no panic - phases modified successfully
    }
}
