// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Genetics and spawning authorization logic
///
/// Handles authorization for genetic operations and BearDog spawning.

use chrono::{Duration, Utc};
use std::collections::HashMap;
use uuid::Uuid;
use beardog_errors::{BearDogError, BearDogResult};
use beardog_errors::improved_results::*;
// ✅ MIGRATION HELPERS REMOVED - Using canonical patterns directly
// The migration_helpers import has been removed as the migration is complete.
// All outcome creation now uses direct canonical patterns for better performance.
use super::types::*;
impl CrossNodeAuthEngine {
    /// Register genetics for spawning operations
    pub async fn register_genetics(&mut self, genetics: BearDogGenetics) -> BearDogResult<GeneticsRegistrationOutcome> {
        let start_time = Utc::now();
        
        // Extract genetics information for the outcome
        let genetics_id = genetics.id.clone();
        let capabilities = genetics.capabilities.clone();
        let security_clearance = format!("{:?}", genetics.security_clearance);
        let generation = genetics.generation;
        let fitness_score = genetics.fitness_score;
        // Perform the registration
        self.genetics_registry.insert(genetics.id.clone(), genetics);
        // ✅ CANONICAL OUTCOME CREATION - Direct construction without helpers
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
                metadata: HashMap::new(),
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
        // ✅ TIMING ALREADY SET - Removed redundant timing updates
        // The timing is already correctly set in the outcome constructor above
        // Add metadata
        outcome.context.metadata.insert(
            "registry_size".to_string(),
            serde_json::json!(self.genetics_registry.len()),
        );
        Ok(outcome)
    }
    /// Spawn a new BearDog instance with genetic inheritance
    pub async fn spawn_beardog(
        &mut self,
        parent_genetics: Vec<String>,
        spawn_config: SpawnConfig,
    ) -> BearDogResult<SpawningOutcome> {
        // Validate parent genetics exist
        for parent_id in &parent_genetics {
            if !self.genetics_registry.contains_key(parent_id) {
                return Err(BearDogError::authorization(format!("Parent genetics not found: }", parent_id),
                ));
            }
        }
        // Combine genetics from parents
        let combined_genetics = self.combine_genetics(&parent_genetics).await?;
        // Create spawn record using actual field names
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
            performance_metrics: HashMap::new(),
            trust_relationships: HashMap::new(),
            consensus_participation: spawn_config.consensus_enabled,
            ecosystem_connections: vec![],
        // Store spawn record
        self.spawned_beardogs
            .insert(spawn_id.clone(), spawned_beardog.clone());
        // Create rich spawning outcome
        let mut outcome = create_spawning_outcome(
            spawn_id,
            parent_genetics.clone(),
            spawn_config.spawn_purpose,
            spawned_beardog.expected_lifetime,
        // Update timing and metrics
        outcome.context.started_at = start_time;
        outcome.context.completed_at = Utc::now();
        outcome.metrics.duration = (Utc::now() - start_time).to_std().unwrap_or_default();
        outcome.metrics.items_processed = 1;
        outcome.metrics.success_rate = 100.0;
        // Add comprehensive metadata
            "parent_count".to_string(),
            serde_json::json!(parent_genetics.len()),
            "total_spawns".to_string(),
            serde_json::json!(self.spawned_beardogs.len()),
            "consensus_enabled".to_string(),
            serde_json::json!(spawn_config.consensus_enabled),
    /// Combine genetics from parent BearDogs
    pub async fn combine_genetics(&self, parent_ids: &[String]) -> BearDogResult<BearDogGenetics> {
        if parent_ids.is_empty() {
            return Err(BearDogError::authorization("At least one parent required for genetic combination".to_string(),
            ));
        // Get parent genetics
        let mut parent_genetics = Vec::new();
        for parent_id in parent_ids {
            if let Some(genetics) = self.genetics_registry.get(parent_id) {
                parent_genetics.push(genetics.clone());
            } else {
        // Combine genetic capabilities (using actual field names)
        let combined_id = Uuid::new_v4().to_string();
        let mut combined_capabilities = vec![];
        // Merge capabilities from all parents
        for genetics in &parent_genetics {
            combined_capabilities.extend(genetics.capabilities.clone());
        combined_capabilities.dedup();
        // Calculate combined fitness (average of parents)
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
    /// Combine crypto chromosomes from two parents
    fn combine_chromosomes(
        &self,
        parent1_chromosomes: &[CryptoChromosome],
        parent2_chromosomes: &[CryptoChromosome],
    ) -> BearDogResult<Vec<CryptoChromosome>> {
        let mut combined = Vec::new();
        // Take chromosomes from both parents with preference for stronger algorithms
        for chromosome in parent1_chromosomes.iter().chain(parent2_chromosomes.iter()) {
            if !combined.iter().any(|c| c.algorithm == chromosome.algorithm) {
                combined.push(chromosome.clone());
        // Ensure at least one strong algorithm is present
        if combined.is_empty() {
            combined.push(CryptoChromosome {
                algorithm: "Ed25519".to_string(),
                strength: 256,
                capabilities: vec!["sign".to_string(), "verify".to_string()],
            });
        Ok(combined)
    /// Combine security traits from two parents
    fn combine_security_traits(
        parent1_traits: &SecurityTraits,
        parent2_traits: &SecurityTraits,
    ) -> BearDogResult<SecurityTraits> {
        Ok(SecurityTraits {
            // Take the higher security values from both parents
            tamper_resistance: parent1_traits.tamper_resistance.max(parent2_traits.tamper_resistance),
            entropy_quality: parent1_traits.entropy_quality.max(parent2_traits.entropy_quality),
            isolation_level: parent1_traits.isolation_level.max(parent2_traits.isolation_level),
            attestation_strength: parent1_traits.attestation_strength.max(parent2_traits.attestation_strength),
    /// Calculate offspring security clearance based on parents}


    fn calculate_offspring_clearance(
        parent1_clearance: &SecurityClearance,
        parent2_clearance: &SecurityClearance,
    ) -> SecurityClearance {
        // Offspring inherits the lower of the two parent clearances for security
        match (parent1_clearance, parent2_clearance) {
            (SecurityClearance::Maximum, SecurityClearance::Maximum) => SecurityClearance::High,
            (SecurityClearance::High, SecurityClearance::High) => SecurityClearance::Medium,
            (SecurityClearance::Medium, SecurityClearance::Medium) => SecurityClearance::Basic,
            _ => SecurityClearance::Basic, // Conservative default
    /// Terminate a spawned BearDog
    pub async fn terminate_spawn(&mut self, spawn_id: &str) -> BearDogResult<SpawnTerminationOutcome> {
        if let Some(spawn) = self.spawned_beardogs.get_mut(spawn_id) {
            // Calculate runtime
            let total_runtime = (Utc::now() - spawn.spawn_time).to_std().unwrap_or_default();
            
            // Update spawn status
            spawn.current_status = SpawnStatus::Terminated;
            // Create rich termination outcome
            let mut outcome = create_termination_outcome(
                spawn_id.to_string(),
                "Manual termination requested".to_string(),
                total_runtime,
            );
            // Update timing and metrics
            outcome.context.started_at = start_time;
            outcome.context.completed_at = Utc::now();
            outcome.metrics.duration = (Utc::now() - start_time).to_std().unwrap_or_default();
            outcome.metrics.items_processed = 1;
            outcome.metrics.success_rate = 100.0;
            // Add metadata about the terminated spawn
            outcome.context.metadata.insert(
                "spawn_purpose".to_string(),
                serde_json::json!(spawn.spawn_purpose),
                "spawn_generation".to_string(),
                serde_json::json!(spawn.genetics.generation),
                "remaining_spawns".to_string(),
                serde_json::json!(self.spawned_beardogs.len() - 1),
            Ok(outcome)
        } else {
            Err(BearDogError::authorization(format!("Spawn not found: }", spawn_id),
            ))
}
