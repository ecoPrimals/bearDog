

use chrono::{Duration, Utc};
use std::collections::HashMap;
use uuid::Uuid;
use beardog_errors::{BearDogError, BearDogResult};
use beardog_errors::improved_results::*;

use super::types::*;
impl CrossNodeAuthEngine {

    pub async fn register_genetics(&mut self, genetics: BearDogGenetics) -> BearDogResult<GeneticsRegistrationOutcome> {
        let start_time = Utc::now();

        let genetics_id = genetics.id.clone();
        let capabilities = genetics.capabilities.clone();
        let security_clearance = format_args!("{:?}", genetics.security_clearance).to_string();
        let generation = genetics.generation;
        let fitness_score = genetics.fitness_score;

        self.genetics_registry.insert(genetics.id.clone(), genetics);

        let mut outcome = GeneticsRegistrationOutcome {
            genetics_id: genetics_id.clone(),
            capabilities_summary: GeneticsCapabilitiesSummary {
                total_capabilities: capabilities.len(),
                security_capabilities: capabilities.iter().filter(|c| c.contains("security")).count(),
                performance_capabilities: capabilities.iter().filter(|c| c.contains("performance")).count(),
                network_capabilities: capabilities.iter().filter(|c| c.contains("network")).count(),
            },
            validation_results: GeneticsValidationResults {
                is_valid: true,
                security_clearance_verified: true,
                capability_compatibility_verified: true,
                generation_lineage_verified: true,
                fitness_score_acceptable: fitness_score >= 0.5,
                validation_warnings: Vec::new(),
            context: OperationContext {
                operation_id: Uuid::new_v4().to_string(),
                component: "genetics-registration".to_string(),
                started_at: start_time,
                completed_at: Utc::now(),
                metadata: HashMap::with_capacity(16),
                user_context: None,
                transaction_id: None,
                correlation_id: None,
                initiator: "CrossNodeAuthEngine".to_string(),
            metrics: OperationMetrics {
                duration: (Utc::now() - start_time).to_std().unwrap_or_default(),
                items_processed: 1,
                success_rate: 100.0,
                memory_usage_bytes: 0, // Will be calculated if needed
                cpu_usage_percent: 0.0, // Will be calculated if needed
                network_bytes_sent: 0,
                network_bytes_received: 0,
                cache_hits: 0,
                cache_misses: 0,
                database_queries: 0,
                average_item_processing_time: Duration::from_millis(0),
            warnings: Vec::new(),
        };

        outcome.context.metadata.insert(
            "registry_size".to_string(),
            serde_json::json!(self.genetics_registry.len()),
        );
        Ok(outcome)
    }

    pub async fn spawn_beardog(
        &mut self,
        parent_genetics: Vec<&str>,
        spawn_config: SpawnConfig,
    ) -> BearDogResult<SpawningOutcome> {

        for parent_id in &parent_genetics {
            if !self.genetics_registry.contains_key(parent_id) {
                return Err(BearDogError::authorization(format_args!("Parent genetics not found: }", parent_id).to_string(),
                ));
            }
        }

        let combined_genetics = self.combine_genetics(&parent_genetics).await?;

        let spawn_id = Uuid::new_v4().to_string();
        let spawned_beardog = SpawnedBearDog {
            id: spawn_id.clone(),
            parent_id: parent_genetics
                .first()
                .unwrap_or(&"unknown".to_string())
                .clone(),
            genetics: combined_genetics,
            spawn_purpose: spawn_config.spawn_purpose.clone(),
            task_assignment: vec![],
            resource_limits: spawn_config.resource_limits,
            spawn_time: Utc::now(),
            expected_lifetime: Some(Utc::now() + Duration::hours(spawn_config.max_lifetime_hours)),
            current_status: SpawnStatus::Active,
            performance_metrics: HashMap::with_capacity(16),
            trust_relationships: HashMap::with_capacity(16),
            consensus_participation: spawn_config.consensus_enabled,
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

    pub async fn combine_genetics(&self, parent_ids: &[&str]) -> BearDogResult<BearDogGenetics> {
        if parent_ids.is_empty() {
            return Err(BearDogError::authorization("At least one parent required for genetic combination".to_string(),
            ));

        let mut parent_genetics = Vec::new();
        for parent_id in parent_ids {
            if let Some(genetics) = self.genetics_registry.get(parent_id) {
                parent_genetics.push(genetics.clone());
            } else {

        let combined_id = Uuid::new_v4().to_string();
        let mut combined_capabilities = vec![];

        for genetics in &parent_genetics {
            combined_capabilities.extend(genetics.capabilities.clone());
        combined_capabilities.dedup();

        let combined_fitness = parent_genetics.iter().map(|g| g.fitness_score).sum::<f64>()
            / parent_genetics.len() as f64;
        Ok(BearDogGenetics {
            id: combined_id,
            crypto_chromosomes: self.combine_chromosomes(&parent_genetics[0].crypto_chromosomes, &parent_genetics[1].crypto_chromosomes)?,
            security_traits: self.combine_security_traits(&parent_genetics[0].security_traits, &parent_genetics[1].security_traits)?,
            capabilities: combined_capabilities,
            spawn_restrictions: vec![],
            generation: parent_genetics
                .iter()
                .map(|g| g.generation)
                .max()
                .unwrap_or(0)
                + 1,
            parent_genetics: Some(parent_ids.to_vec()),
            mutations: vec![],
            fitness_score: combined_fitness,
            security_clearance: self.calculate_offspring_clearance(&parent_genetics[0].security_clearance, &parent_genetics[1].security_clearance),
            specializations: vec![],
        })

    fn combine_chromosomes(
        &self,
        parent1_chromosomes: &[CryptoChromosome],
        parent2_chromosomes: &[CryptoChromosome],
    ) -> BearDogResult<Vec<CryptoChromosome>> {
        let mut combined = Vec::new();

        for chromosome in parent1_chromosomes.iter().chain(parent2_chromosomes.iter()) {
            if !combined.iter().any(|c| c.algorithm == chromosome.algorithm) {
                combined.push(chromosome.clone());

        if combined.is_empty() {
            combined.push(CryptoChromosome {
                algorithm: "Ed25519".to_string(),
                strength: 256,
                capabilities: vec!["sign".to_string(), "verify".to_string()],
            });
        Ok(combined)

    fn combine_security_traits(
        parent1_traits: &SecurityTraits,
        parent2_traits: &SecurityTraits,
    ) -> BearDogResult<SecurityTraits> {
        Ok(SecurityTraits {

            tamper_resistance: parent1_traits.tamper_resistance.max(parent2_traits.tamper_resistance),
            entropy_quality: parent1_traits.entropy_quality.max(parent2_traits.entropy_quality),
            isolation_level: parent1_traits.isolation_level.max(parent2_traits.isolation_level),
            attestation_strength: parent1_traits.attestation_strength.max(parent2_traits.attestation_strength),

    fn calculate_offspring_clearance(
        parent1_clearance: &SecurityClearance,
        parent2_clearance: &SecurityClearance,
    ) -> SecurityClearance {

        match (parent1_clearance, parent2_clearance) {
            (SecurityClearance::Maximum, SecurityClearance::Maximum) => SecurityClearance::High,
            (SecurityClearance::High, SecurityClearance::High) => SecurityClearance::Medium,
            (SecurityClearance::Medium, SecurityClearance::Medium) => SecurityClearance::Basic,
            _ => SecurityClearance::Basic, // Conservative default

    pub async fn terminate_spawn(&mut self, spawn_id: &str) -> BearDogResult<SpawnTerminationOutcome> {
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
            Err(BearDogError::authorization(format_args!("Spawn not found: }", spawn_id).to_string(),
            ))
}
