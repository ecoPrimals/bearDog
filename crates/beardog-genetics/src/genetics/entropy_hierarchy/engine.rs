//! Entropy Hierarchy Manager
//!
//! This module provides the core EntropyHierarchyManager that orchestrates
//! all entropy hierarchy operations including seed management, validation,
//! and cryptographic operations.

use super::monitoring::{EntropyHealthStatus, EntropyMonitor, PerformanceMetrics};
use super::sources::EntropyMixingEngine;
use super::types::*;
use super::validation::EntropyValidator;
use crate::genetics::human_entropy::MultiModalHumanEntropyCollector;
// // use beardog_tunnel::tunnel::hsm::{HsmManager, SecurityLevel};
use beardog_errors::{BearDogError, BearDogResult};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

/// Central manager for the entropy hierarchy system
pub struct EntropyHierarchyManager {
    /// Configuration for entropy hierarchy
    pub config: EntropyHierarchyConfig,

    /// Active entropy seeds
    pub active_seeds: HashMap<Uuid, EntropySeed>,

    /// Entropy mixing engine
    pub mixing_engine: EntropyMixingEngine,

    /// HSM manager for cryptographic operations
    // pub hsm_manager: Arc<HsmManager>,

    /// Human entropy collector
    pub human_entropy_collector: Arc<MultiModalHumanEntropyCollector>,

    /// Entropy validator for quality assessment and proof generation
    pub validator: EntropyValidator,

    /// Monitor for statistics and health tracking
    pub monitor: EntropyMonitor,
}

impl EntropyHierarchyManager {
    /// Create a new entropy hierarchy manager
    pub fn new(
        config: EntropyHierarchyConfig,
        // hsm_manager: Arc<HsmManager>,
        human_entropy_collector: Arc<MultiModalHumanEntropyCollector>,
    ) -> Self {
        let mixing_engine = EntropyMixingEngine::new(config.clone());
        let validator = EntropyValidator::new(/* hsm_manager.clone(), */ config.clone());
        let monitor = EntropyMonitor::new(config.clone());

        Self {
            config,
            active_seeds: HashMap::new(),
            mixing_engine,
            // hsm_manager,
            human_entropy_collector,
            validator,
            monitor,
        }
    }

    /// Create a new human entropy seed
    pub async fn create_human_seed(
        &mut self,
        entropy_class: EntropyClass,
        lifetime_policy: SeedLifetimePolicy,
        owner_identity: HumanIdentity,
        seed_bytes: Vec<u8>,
    ) -> BearDogResult<Uuid> {
        // Validate entropy quality
        self.validator.validate_entropy_quality(&entropy_class)?;

        // Create the seed
        let seed = EntropySeed::new_human_entropy(
            entropy_class,
            lifetime_policy,
            owner_identity,
            seed_bytes,
            self,
        )
        .await?;

        let seed_id = seed.id;
        self.active_seeds.insert(seed_id, seed);

        Ok(seed_id)
    }

    /// Create a new event-based seed for social contexts
    pub async fn create_event_seed(
        &mut self,
        event_context: SocialContext,
        sharing_policy: SharingPolicy,
        owner_identity: HumanIdentity,
        seed_bytes: Vec<u8>,
    ) -> BearDogResult<Uuid> {
        if !self.config.enable_event_seeds {
            return Err(BearDogError::Internal {
                message: "Event seeds are disabled in configuration".to_string(),
            });
        }

        // Create the event seed
        let seed = EntropySeed::new_event_seed(
            event_context,
            sharing_policy,
            owner_identity,
            seed_bytes,
            self,
        )
        .await?;

        let seed_id = seed.id;
        self.active_seeds.insert(seed_id, seed);

        Ok(seed_id)
    }

    /// Get a reference to a seed
    pub fn get_seed(&self, seed_id: &Uuid) -> Option<&EntropySeed> {
        self.active_seeds.get(seed_id)
    }

    /// Get a mutable reference to a seed
    pub fn get_seed_mut(&mut self, seed_id: &Uuid) -> Option<&mut EntropySeed> {
        self.active_seeds.get_mut(seed_id)
    }

    /// Use a seed for a cryptographic operation
    pub fn use_seed(&mut self, seed_id: &Uuid, operation: &str) -> BearDogResult<Vec<u8>> {
        let seed =
            self.active_seeds
                .get_mut(seed_id)
                .ok_or_else(|| BearDogError::InvalidInput {
                    message: format!("Seed with ID {seed_id} not found"),
                })?;

        seed.use_for_operation(operation)
    }

    /// Generate a signature using HSM (placeholder - real implementation would use actual HSM methods)
    pub async fn generate_signature(&self, data: &[u8], key_id: &str) -> BearDogResult<Vec<u8>> {
        // For now, use a simple hash as placeholder since HsmManager doesn't have generate_signature
        use sha3::{Digest, Sha3_256};
        let mut hasher = Sha3_256::new();
        hasher.update(data);
        hasher.update(key_id.as_bytes());
        Ok(hasher.finalize().to_vec())
    }

    /// Generate ownership proof for human identity
    pub async fn generate_ownership_proof(
        &self,
        owner_identity: &HumanIdentity,
        entropy_data: &[u8],
    ) -> BearDogResult<OwnershipProof> {
        self.validator
            .generate_ownership_proof(owner_identity, entropy_data)
            .await
    }

    /// Generate irreproducibility proof for entropy
    pub async fn generate_irreproducibility_proof(
        &self,
        entropy_data: &[u8],
        entropy_class: &EntropyClass,
    ) -> BearDogResult<IrreproducibilityProof> {
        self.validator
            .generate_irreproducibility_proof(entropy_data, entropy_class)
            .await
    }

    /// Mix multiple entropy sources
    pub fn mix_entropy_sources(&self, sources: Vec<EntropyClass>) -> BearDogResult<EntropyClass> {
        self.mixing_engine.mix_entropy_sources(sources)
    }

    /// Validate entropy quality
    pub fn validate_entropy_quality(&self, entropy_class: &EntropyClass) -> BearDogResult<f64> {
        self.validator.validate_entropy_quality(entropy_class)
    }

    /// Transfer ownership of a seed
    pub fn transfer_seed_ownership(
        &mut self,
        seed_id: &Uuid,
        new_owner: HumanIdentity,
    ) -> BearDogResult<()> {
        if !self.config.enable_ownership_transfer {
            return Err(BearDogError::Internal {
                message: "Ownership transfer is disabled in configuration".to_string(),
            });
        }

        let seed =
            self.active_seeds
                .get_mut(seed_id)
                .ok_or_else(|| BearDogError::InvalidInput {
                    message: format!("Seed with ID {seed_id} not found"),
                })?;

        seed.transfer_ownership(new_owner)
    }

    /// Expire ownership of a seed
    pub fn expire_seed_ownership(&mut self, seed_id: &Uuid) -> BearDogResult<()> {
        let seed =
            self.active_seeds
                .get_mut(seed_id)
                .ok_or_else(|| BearDogError::InvalidInput {
                    message: format!("Seed with ID {seed_id} not found"),
                })?;

        seed.expire_ownership()
    }

    /// Destroy a seed securely
    pub fn destroy_seed(&mut self, seed_id: &Uuid) -> BearDogResult<()> {
        if let Some(mut seed) = self.active_seeds.remove(seed_id) {
            seed.destroy();
            Ok(())
        } else {
            Err(BearDogError::InvalidInput {
                message: format!("Seed with ID {seed_id} not found"),
            })
        }
    }

    /// Clean up expired seeds
    pub fn cleanup_expired_seeds(&mut self) {
        self.monitor.cleanup_expired_seeds(&mut self.active_seeds);
    }

    /// Get statistics about the entropy hierarchy
    pub fn get_statistics(&self) -> EntropyHierarchyStats {
        self.monitor.get_statistics(&self.active_seeds)
    }

    /// Get detailed analytics
    pub fn get_detailed_analytics(&self) -> super::monitoring::EntropyAnalytics {
        self.monitor.get_detailed_analytics(&self.active_seeds)
    }

    /// Get health status of the system
    pub fn get_health_status(&self) -> EntropyHealthStatus {
        self.monitor.get_health_status(&self.active_seeds)
    }

    /// Get performance metrics
    pub fn get_performance_metrics(&self) -> PerformanceMetrics {
        self.monitor.get_performance_metrics(&self.active_seeds)
    }

    /// List all active seed IDs
    pub fn list_active_seeds(&self) -> Vec<Uuid> {
        self.active_seeds.keys().cloned().collect()
    }

    /// List seeds by entropy class
    pub fn list_seeds_by_entropy_class(
        &self,
        entropy_class_filter: fn(&EntropyClass) -> bool,
    ) -> Vec<Uuid> {
        self.active_seeds
            .iter()
            .filter(|(_, seed)| entropy_class_filter(&seed.entropy_class))
            .map(|(id, _)| *id)
            .collect()
    }

    /// List seeds by ownership type
    pub fn list_seeds_by_ownership(
        &self,
        ownership_filter: fn(&SeedOwnership) -> bool,
    ) -> Vec<Uuid> {
        self.active_seeds
            .iter()
            .filter(|(_, seed)| ownership_filter(&seed.ownership))
            .map(|(id, _)| *id)
            .collect()
    }

    /// Get seeds owned by specific human identity
    pub fn get_seeds_owned_by(&self, owner_id: &str) -> Vec<Uuid> {
        self.active_seeds
            .iter()
            .filter(|(_, seed)| {
                if let Some(owner) = seed.get_current_owner() {
                    owner.identity_id == owner_id
                } else {
                    false
                }
            })
            .map(|(id, _)| *id)
            .collect()
    }

    /// Get seeds with specific social context
    pub fn get_event_seeds(&self, event_type_filter: Option<&str>) -> Vec<Uuid> {
        self.active_seeds
            .iter()
            .filter(|(_, seed)| {
                if let Some(social_context) = &seed.social_context {
                    if let Some(filter) = event_type_filter {
                        social_context.event_type.to_string().contains(filter)
                    } else {
                        true
                    }
                } else {
                    false
                }
            })
            .map(|(id, _)| *id)
            .collect()
    }

    /// Verify ownership proof for a seed
    pub async fn verify_seed_ownership_proof(
        &self,
        seed_id: &Uuid,
        proof: &OwnershipProof,
        owner_identity: &HumanIdentity,
    ) -> BearDogResult<bool> {
        let seed = self
            .active_seeds
            .get(seed_id)
            .ok_or_else(|| BearDogError::InvalidInput {
                message: format!("Seed with ID {seed_id} not found"),
            })?;

        self.validator
            .verify_ownership_proof(proof, owner_identity, seed.seed_bytes.as_bytes())
            .await
    }

    /// Verify irreproducibility proof for a seed
    pub async fn verify_seed_irreproducibility_proof(
        &self,
        seed_id: &Uuid,
        proof: &IrreproducibilityProof,
    ) -> BearDogResult<bool> {
        let seed = self
            .active_seeds
            .get(seed_id)
            .ok_or_else(|| BearDogError::InvalidInput {
                message: format!("Seed with ID {seed_id} not found"),
            })?;

        self.validator
            .verify_irreproducibility_proof(proof, seed.seed_bytes.as_bytes(), &seed.entropy_class)
            .await
    }

    /// Get entropy mixing recommendations
    pub fn get_mixing_recommendations(&self, sources: &[EntropyClass]) -> Vec<String> {
        self.mixing_engine.get_mixing_recommendations(sources)
    }

    /// Validate entropy combination
    pub fn validate_entropy_combination(&self, sources: &[EntropyClass]) -> BearDogResult<()> {
        self.mixing_engine.validate_entropy_combination(sources)
    }

    /// Get entropy quality assessment
    pub fn assess_entropy_quality(
        &self,
        entropy_class: &EntropyClass,
    ) -> BearDogResult<EntropyQualityAssessment> {
        let quality_score = self.validator.validate_entropy_quality(entropy_class)?;
        let security_level = self.validator.check_security_requirements(entropy_class)?;
        let weighted_score = self
            .mixing_engine
            .calculate_weighted_entropy_score(entropy_class);

        Ok(EntropyQualityAssessment {
            quality_score,
            // security_level,
            weighted_score,
            entropy_tier: match entropy_class {
                EntropyClass::HumanLivedExperience { .. } => 3,
                EntropyClass::HumanSupervisedMachine { .. } => 2,
                EntropyClass::StoreBoughtMachine { .. } => 1,
            },
            recommendations: self.get_quality_recommendations(entropy_class),
        })
    }

    /// Get recommendations for improving entropy quality
    fn get_quality_recommendations(&self, entropy_class: &EntropyClass) -> Vec<String> {
        let mut recommendations = Vec::new();

        match entropy_class {
            EntropyClass::HumanLivedExperience { source_type, .. } => match source_type {
                HumanEntropySource::MultiModalHuman { .. } => {
                    recommendations.push(
                        "Excellent choice: Multi-modal human entropy provides maximum security"
                            .to_string(),
                    );
                }
                _ => {
                    recommendations.push("Consider combining with other human entropy sources for multi-modal entropy".to_string());
                }
            },
            EntropyClass::HumanSupervisedMachine { .. } => {
                recommendations.push(
                    "Consider upgrading to pure human entropy for maximum security".to_string(),
                );
                recommendations
                    .push("Ensure human validator has appropriate verification level".to_string());
            }
            EntropyClass::StoreBoughtMachine {
                reproducibility_index,
                ..
            } => {
                if *reproducibility_index > 0.7 {
                    recommendations.push(
                        "High reproducibility detected - consider human supervision".to_string(),
                    );
                }
                recommendations.push(
                    "Machine entropy should be used only when human entropy is not available"
                        .to_string(),
                );
            }
        }

        recommendations
    }

    /// Update configuration
    pub fn update_config(&mut self, new_config: EntropyHierarchyConfig) {
        self.config = new_config.clone();
        self.mixing_engine = EntropyMixingEngine::new(new_config.clone());
        self.validator = EntropyValidator::new(/* self.hsm_manager.clone(), */ new_config.clone());
        self.monitor = EntropyMonitor::new(new_config);
    }

    /// Get current configuration
    pub fn get_config(&self) -> &EntropyHierarchyConfig {
        &self.config
    }
}

/// Assessment of entropy quality
#[derive(Debug, Clone)]
pub struct EntropyQualityAssessment {
    /// Overall quality score of the entropy
    pub quality_score: f64,
    /// Security level based on entropy quality
    // pub security_level: SecurityLevel,
    /// Weighted score considering various factors
    pub weighted_score: f64,
    /// Tier classification of the entropy
    pub entropy_tier: u8,
    /// Recommendations for improving entropy quality
    pub recommendations: Vec<String>,
}
