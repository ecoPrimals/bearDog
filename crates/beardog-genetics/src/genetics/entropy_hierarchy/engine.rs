

use super::monitoring::{EntropyHealthStatus, EntropyMonitor, PerformanceMetrics};
use super::sources::EntropyMixingEngine;
use super::types::*;
use super::validation::EntropyValidator;
use crate::genetics::human_entropy::MultiModalHumanEntropyCollector;

use beardog_errors::{BearDogError, BearDogResult};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub struct EntropyHierarchyManager {

    pub config: EntropyHierarchyConfig,

    pub active_seeds: HashMap<Uuid, EntropySeed>,

    pub mixing_engine: EntropyMixingEngine,

    pub human_entropy_collector: Arc<MultiModalHumanEntropyCollector>,

    pub validator: EntropyValidator,

    pub monitor: EntropyMonitor,
}
impl EntropyHierarchyManager {

    pub fn new(
        config: EntropyHierarchyConfig,

        human_entropy_collector: Arc<MultiModalHumanEntropyCollector>,
    ) -> Self {
        let mixing_engine = EntropyMixingEngine::new(config.clone());
        let validator = EntropyValidator::new(EntropyHierarchyConfig::default());
        let monitor = EntropyMonitor::new(config.clone());
        Self {
            config,
            active_seeds: HashMap::with_capacity(16),
            mixing_engine,

            human_entropy_collector,
            validator,
            monitor,
        }
    }

    pub async fn create_human_seed(
        &mut self,
        entropy_class: EntropyClass,
        lifetime_policy: SeedLifetimePolicy,
        owner_identity: HumanIdentity,
        seed_bytes: Vec<u8>,
    ) -> BearDogResult<Uuid> {

        self.validator.validate_entropy_quality(&entropy_class)?;

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

    pub async fn create_event_seed(
        event_context: SocialContext,
        sharing_policy: SharingPolicy,
        if !self.config.enable_event_seeds {
            return Err(BearDogError::internal("Event seeds are disabled in configuration".to_string(),
            ));

        let seed = EntropySeed::new_event_seed(
            event_context,
            sharing_policy,

    pub fn get_seed(&self, seed_id: &Uuid) -> Option<&EntropySeed> {
        self.active_seeds.get(seed_id)

    pub fn get_seed_mut(&mut self, seed_id: &Uuid) -> Option<&mut EntropySeed> {
        self.active_seeds.get_mut(seed_id)

    pub fn use_seed(&mut self, seed_id: &Uuid, operation: &str) -> BearDogResult<Vec<u8>> {
        let seed =
            self.active_seeds
                .get_mut(seed_id)
                .ok_or_else(|| BearDogError::invalid_input(format!("Seed with ID {seed_id} not found")))?;
        seed.use_for_operation(operation)

    pub async fn generate_signature(&self, data: &[u8], key_id: &str) -> BearDogResult<Vec<u8>> {

        use sha3::{Digest, Sha3_256};
        let mut hasher = Sha3_256::new();
        hasher.update(data);
        hasher.update(key_id.as_bytes());
        Ok(hasher.finalize().to_vec())

    pub async fn generate_ownership_proof(
        &self,
        owner_identity: &HumanIdentity,
        entropy_data: &[u8],
    ) -> BearDogResult<OwnershipProof> {
        self.validator
            .generate_ownership_proof(owner_identity, entropy_data)
            .await

    pub async fn generate_irreproducibility_proof(
        entropy_class: &EntropyClass,
    ) -> BearDogResult<IrreproducibilityProof> {
            .generate_irreproducibility_proof(entropy_data, entropy_class)

    pub fn mix_entropy_sources(&self, sources: Vec<EntropyClass>) -> BearDogResult<EntropyClass> {
        self.mixing_engine.mix_entropy_sources(sources)

    pub fn validate_entropy_quality(&self, entropy_class: &EntropyClass) -> BearDogResult<f64> {
        self.validator.validate_entropy_quality(entropy_class)

    pub fn transfer_seed_ownership(
        seed_id: &Uuid,
        new_owner: HumanIdentity,
    ) -> BearDogResult<()> {
        if !self.config.enable_ownership_transfer {
                message: "Ownership transfer is disabled in configuration".to_string(),
        seed.transfer_ownership(new_owner)

    pub fn expire_seed_ownership(&mut self, seed_id: &Uuid) -> BearDogResult<()> {
        seed.expire_ownership()

    pub fn destroy_seed(&mut self, seed_id: &Uuid) -> BearDogResult<()> {
        if let Some(mut seed) = self.active_seeds.remove(seed_id) {
            seed.destroy();
            Ok(())
        } else {
            Err(BearDogError::invalid_input(format!("Seed with ID {seed_id} not found")))

    pub fn cleanup_expired_seeds(&mut self) {
        self.monitor.cleanup_expired_seeds(&mut self.active_seeds);

    pub fn get_statistics(&self) -> EntropyHierarchyStats {
        self.monitor.get_statistics(&self.active_seeds)

    pub fn get_detailed_analytics(&self) -> super::monitoring::EntropyAnalytics {
        self.monitor.get_detailed_analytics(&self.active_seeds)

    pub fn get_health_status(&self) -> EntropyHealthStatus {
        self.monitor.get_health_status(&self.active_seeds)

    pub fn get_performance_metrics(&self) -> PerformanceMetrics {
        self.monitor.get_performance_metrics(&self.active_seeds)

    pub fn list_active_seeds(&self) -> Vec<Uuid> {
        self.active_seeds.keys().cloned().collect()

    pub fn list_seeds_by_entropy_class(
        entropy_class_filter: fn(&EntropyClass) -> bool,
    ) -> Vec<Uuid> {
        self.active_seeds
            .iter()
            .filter(|(_, seed)| entropy_class_filter(&seed.entropy_class))
            .map(|(id, _)| *id)
            .collect()

    pub fn list_seeds_by_ownership(
        ownership_filter: fn(&SeedOwnership) -> bool,
            .filter(|(_, seed)| ownership_filter(&seed.ownership))

    pub fn get_seeds_owned_by(&self, owner_id: &str) -> Vec<Uuid> {
            .filter(|(_, seed)| {
                if let Some(owner) = seed.get_current_owner() {
                    owner.identity_id == owner_id
                } else {
                    false
                }
            })

    pub fn get_event_seeds(&self, event_type_filter: Option<&str>) -> Vec<Uuid> {
                if let Some(social_context) = &seed.social_context {
                    if let Some(filter) = event_type_filter {
                        social_context.event_type.to_string().contains(filter)
                    } else {
                        true
                    }

    pub async fn verify_seed_ownership_proof(
        proof: &OwnershipProof,
    ) -> BearDogResult<bool> {
        let seed = self
            .active_seeds
            .get(seed_id)
            .ok_or_else(|| BearDogError::invalid_input(format!("Seed with ID {seed_id} not found")))?;
            .verify_ownership_proof(proof, owner_identity, seed.seed_bytes.as_bytes())

    pub async fn verify_seed_irreproducibility_proof(
        proof: &IrreproducibilityProof,
            .verify_irreproducibility_proof(proof, seed.seed_bytes.as_bytes(), &seed.entropy_class)

    pub fn get_mixing_recommendations(&self, sources: &[EntropyClass]) -> Vec<String> {
        self.mixing_engine.get_mixing_recommendations(sources)

    pub fn validate_entropy_combination(&self, sources: &[EntropyClass]) -> BearDogResult<()> {
        self.mixing_engine.validate_entropy_combination(sources)

    pub fn assess_entropy_quality(
    ) -> BearDogResult<EntropyQualityAssessment> {
        let quality_score = self.validator.validate_entropy_quality(entropy_class)?;
        let _security_level = self.validator.check_security_requirements(entropy_class)?;
        let weighted_score = self
            .mixing_engine
            .calculate_weighted_entropy_score(entropy_class);
        Ok(EntropyQualityAssessment {
            quality_score,

            weighted_score,
            entropy_tier: match entropy_class {
                EntropyClass::HumanLivedExperience { .. } => 3,
                EntropyClass::HumanSupervisedMachine { .. } => 2,
                EntropyClass::StoreBoughtMachine { .. } => 1,
            },
            recommendations: self.get_quality_recommendations(entropy_class),
        })

    fn get_quality_recommendations(&self, entropy_class: &EntropyClass) -> Vec<String> {
        let mut recommendations = Vec::new();
        match entropy_class {
            EntropyClass::HumanLivedExperience { source_type, .. } => match source_type {
                HumanEntropySource::MultiModalHuman { .. } => {
                    recommendations.push(
                        "Excellent choice: Multi-modal human entropy provides maximum security"
                            .to_string(),
                    );
                _ => {
                    recommendations.push("Consider combining with other human entropy sources for multi-modal entropy".to_string());
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
                        "High reproducibility detected - consider human supervision".to_string(),
                    "Machine entropy should be used only when human entropy is not available"
                        .to_string(),
        recommendations

    pub fn update_config(&mut self, new_config: EntropyHierarchyConfig) {
        self.config = new_config.clone();
        self.mixing_engine = EntropyMixingEngine::new(new_config.clone());
        self.validator = EntropyValidator::new(new_config.clone());
        self.monitor = EntropyMonitor::new(new_config);

    pub fn get_config(&self) -> &EntropyHierarchyConfig {
        &self.config

#[derive(Debug, Clone)]
pub struct EntropyQualityAssessment {

    pub quality_score: f64,

    pub weighted_score: f64,

    pub entropy_tier: u8,

    pub recommendations: Vec<String>,
