

use chrono::{Duration, Utc};
use std::collections::HashMap;
use uuid::Uuid;
use beardog_errors::BearDogError;
use beardog_errors::improved_results::*;

use super::types::*;
impl CrossNodeAuthEngine {

/// Register Genetics operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn register_genetics(&mut self, genetics: BearDogGenetics) -> Result<GeneticsRegistrationOutcome, BearDogError> {
        let start_time = Utc::now();

        let genetics_id = &genetics.id;
        let capabilities = &genetics.capabilities;
        let security_clearance = format!("{:?}", genetics.security_clearance);
        let generation = genetics.generation;
        let fitness_score = genetics.fitness_score;

        self.genetics_registry.insert(genetics.id.clone(), genetics);

        let mut outcome = GeneticsRegistrationOutcome {
            genetics_id: genetics_id.clone(),
            capabilities_summary: GeneticsCapabilitiesSummary {
                total_capabilities: capabilities.len(),
                security_capabilities: capabilities.iter().filter(|c| c.contains("security")).count(),
                performance_capabilities: capabilities.iter().filter(|c| c.contains("performance")).count(),
                network_capabilities: capabilities.iter(GeneticsValidationResults {
                is_valid: true,
                security_clearance_verified: true,
                capability_compatibility_verified: true,
                generation_lineage_verified: true,
                fitness_score_acceptable: fitness_score >= 0.5,
                validation_warnings: Vec::new(),
            },
            context: OperationContext {
                operation_id: Uuid::new_v4().to_string(),
                component: "genetics-registration".to_string(),
                completed_at: Utc::now(),
                metadata: HashMap::with_capacity(None,
                transaction_id: None,
                correlation_id: None,
                initiator: "CrossNodeAuthEngine".to_string(),
            },
            metrics: OperationMetrics {
                duration: (Utc::now(1,
                success_rate: 100.0,
                memory_usage_bytes: 0, // Will be calculated if needed
                cpu_usage_percent: 0.0, // Will be calculated if needed
                network_bytes_sent: 0,
                network_bytes_received: 0,
                cache_hits: 0,
                cache_misses: 0,
                database_queries: 0,
                average_item_processing_time: Duration::from_millis(0),
            },
            warnings: Vec::new(),
        };

        outcome.context.metadata.insert(
            "registry_size".to_string(),
            serde_json::json!(self.genetics_registry.len(Vec<&str>,
        spawn_config: SpawnConfig,
    ) -> Result<SpawningOutcome, BearDogError> {

        for parent_id in &parent_genetics {
            if !self.genetics_registry.contains_key(parent_id) {
                return Err(BearDogError::authorization(}", parent_id)));
            }
        }

        let combined_genetics = self.combine_genetics(&parent_genetics)?;

        let spawn_id = Uuid::new_v4().to_string();
        let spawned_beardog = SpawnedBearDog {
            id: spawn_id.clone(),
            parent_id: parent_genetics
                .first(combined_genetics,
            spawn_purpose: &spawn_config.spawn_purpose,
            task_assignment: vec![],
            resource_limits: spawn_config.resource_limits,
            spawn_time: Utc::now(),
            expected_lifetime: Some(Utc::now() + Duration::hours(SpawnStatus::Active,
            performance_metrics: HashMap::with_capacity(16),
            trust_relationships: HashMap::with_capacity(spawn_config.consensus_enabled,
            ecosystem_connections: vec![],

        self.spawned_beardogs
            .insert(spawn_id.clone(), spawned_beardog.clone());

        let mut outcome = create_spawning_outcome(
            spawn_id,
            parent_genetics.clone(),
            spawn_config.spawn_purpose,
            spawned_beardog.expected_lifetime,

        outcome.context.started_at = start_time;
        outcome.context.completed_at = Utc::now();
        outcome.metrics.duration = (Utc::now() - start_time).to_std().unwrap_or_default();
        outcome.metrics.items_processed = 1;
        outcome.metrics.success_rate = 100.0;

            "parent_count".to_string(),
            serde_json::json!(parent_genetics.len()),
            "total_spawns".to_string(),
            serde_json::json!(self.spawned_beardogs.len()),
            "consensus_enabled".to_string(),
            serde_json::json!(spawn_config.consensus_enabled),

/// Combine Genetics operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn combine_genetics(&self, parent_ids: &[&str]) -> Result<BearDogGenetics, BearDogError> {
        if parent_ids.is_empty() {
            return Err(BearDogError::authorization("At least one parent required for genetic combination"));

        let mut parent_genetics = Vec::new();
        for parent_id in parent_ids {
            if let Some(genetics) = self.genetics_registry.get(parent_id) {
                parent_genetics.push(&genetics);
            } else {

        let combined_id = Uuid::new_v4().to_string();
        let mut combined_capabilities = vec![];

        for genetics in &parent_genetics {
            combined_capabilities.extend(&genetics.capabilities);
        combined_capabilities.dedup();

        let combined_fitness = parent_genetics.iter().map(|g| g.fitness_score).sum::<f64>()
            / parent_genetics.len(combined_id,
            crypto_chromosomes: self.combine_chromosomes(self.combine_security_traits(&parent_genetics[0].security_traits, &parent_genetics[1].security_traits)?,
            capabilities: combined_capabilities,
            spawn_restrictions: vec![],
            generation: parent_genetics
                .iter()
                .map(|g| g.generation)
                .max()
                .unwrap_or(0)
                + 1,
            parent_genetics: Some(vec![],
            fitness_score: combined_fitness,
            security_clearance: self.calculate_offspring_clearance(vec![],
        })


    fn combine_chromosomes(&[CryptoChromosome],
        parent2_chromosomes: &[CryptoChromosome],
    ) -> Result<Vec<CryptoChromosome>, BearDogError> {
        let mut combined = Vec::new();

        for chromosome in parent1_chromosomes.iter().chain(parent2_chromosomes.iter()) {
            if !combined.iter().any(|c| c.algorithm == chromosome.algorithm) {
                combined.push(&chromosome);

        if combined.is_empty() {
            combined.push(CryptoChromosome {
                algorithm: "Ed25519".to_string(),
                capabilities: vec!["sign".to_string(&SecurityTraits,
        parent2_traits: &SecurityTraits,
    ) -> Result<SecurityTraits, BearDogError> {
        Ok(SecurityTraits {

            tamper_resistance: parent1_traits.tamper_resistance.max(parent2_traits.tamper_resistance),
            entropy_quality: parent1_traits.entropy_quality.max(parent2_traits.entropy_quality),
            isolation_level: parent1_traits.isolation_level.max(parent2_traits.isolation_level),
            attestation_strength: parent1_traits.attestation_strength.max(&SecurityClearance,
        parent2_clearance: &SecurityClearance,
    ) -> SecurityClearance {

        match (parent1_clearance, parent2_clearance) {
            (SecurityClearance::Maximum, SecurityClearance::Maximum) => SecurityClearance::High,
            (SecurityClearance::High, SecurityClearance::High) => SecurityClearance::Medium,
            (SecurityClearance::Medium, SecurityClearance::Medium) => SecurityClearance::Basic,
            _ => SecurityClearance::Basic, // Conservative default

/// Terminate Spawn operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn terminate_spawn(&mut self, spawn_id: &str) -> Result<SpawnTerminationOutcome, BearDogError> {
        if let Some(spawn) = self.spawned_beardogs.get_mut(spawn_id) {

            let total_runtime = (Utc::now() - spawn.spawn_time).to_std().unwrap_or_default();

            spawn.current_status = SpawnStatus::Terminated;

            let mut outcome = create_termination_outcome(
                spawn_id.to_string(),
                "Manual termination requested".to_string(),
                total_runtime,
            );

            outcome.context.started_at = start_time;
            outcome.context.completed_at = Utc::now();
            outcome.metrics.duration = (Utc::now() - start_time).to_std().unwrap_or_default();
            outcome.metrics.items_processed = 1;
            outcome.metrics.success_rate = 100.0;

            outcome.context.metadata.insert(
                "spawn_purpose".to_string(),
                serde_json::json!(spawn.spawn_purpose),
                "spawn_generation".to_string(),
                serde_json::json!(spawn.genetics.generation),
                "remaining_spawns".to_string(),
                serde_json::json!(self.spawned_beardogs.len() - 1),
            Ok(outcome)
        } else {
            Err(BearDogError::authorization(}", spawn_id)))
}
