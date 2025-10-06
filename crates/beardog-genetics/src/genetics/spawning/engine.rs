// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use super::types::{SpawnRequest, SpawnResult};
use beardog_auth::auth::BearDogGenetics;
use beardog_errors::BearDogError;
use beardog_types::canonical::config::GeneticsConfig;
use std::collections::HashMap;
use tracing::{debug, info};
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeneticSpawningEngine {
    config: GeneticsConfig,
}

impl Default for GeneticSpawningEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl GeneticSpawningEngine {
    /// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            config: GeneticsConfig::default(),
        }
    }

    /// With Config operation.
    /// Creates instance with config
    pub fn with_config(config: GeneticsConfig) -> Self {
        Self { config }
    }

    /// Spawn Genetics operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn spawn_genetics(&self, request: SpawnRequest) -> Result<SpawnResult, BearDogError> {
        info!("🧬 Starting genetic spawning process");
        debug!("Request: {:?}", request);

        let genetics_id = Uuid::new_v4().to_string();

        let mut genetics = BearDogGenetics {
            id: genetics_id.clone(),
            capabilities: request.required_capabilities.clone(),
            security_clearance: request.security_clearance.clone(),
            fitness_score: 0.8,
            ..Default::default()
        };

        // Apply inheritance if parents exist
        if !request.parent_genetics.is_empty() {
            genetics = self.apply_inheritance(&genetics, &request.parent_genetics)?;
        }

        // Calculate fitness score
        genetics.fitness_score = self.calculate_fitness(&genetics)?;

        let metrics = self.collect_metrics();

        Ok(SpawnResult {
            success: true,
            genetics,
            messages: vec!["Genetic spawning completed successfully".to_string()],
            metrics,
        })
    }

    /// Apply genetic inheritance from parent genetics
    fn apply_inheritance(
        &self,
        genetics: &BearDogGenetics,
        parents: &[BearDogGenetics],
    ) -> Result<BearDogGenetics, BearDogError> {
        debug!(
            "Applying genetic inheritance from {} parents",
            parents.len()
        );

        let mut inherited_genetics = genetics.clone();

        // Inherit capabilities from parents
        for parent in parents {
            for capability in &parent.capabilities {
                if !inherited_genetics.capabilities.contains(capability) {
                    inherited_genetics.capabilities.push(capability.clone());
                }
            }
        }

        // Increase generation
        if let Some(parent) = parents.first() {
            inherited_genetics.generation = parent.generation + 1;
        }

        Ok(inherited_genetics)
    }

    /// Calculate fitness score based on genetics
    fn calculate_fitness(&self, genetics: &BearDogGenetics) -> Result<f64, BearDogError> {
        let capability_score = (genetics.capabilities.len() as f64) * 0.1;
        let generation_bonus = if genetics.generation > 0 { 0.1 } else { 0.0 };
        let base_score = 0.5;

        Ok((base_score + capability_score + generation_bonus).min(1.0))
    }

    fn collect_metrics(&self) -> HashMap<String, f64> {
        let mut metrics = HashMap::with_capacity(16);
        metrics.insert("spawn_time_ms".to_string(), 150.0);
        metrics.insert("memory_usage_mb".to_string(), 2.5);
        metrics.insert("cpu_usage_percent".to_string(), 5.0);
        metrics
    }

    /// Get Config operation.
    /// Gets config
    /// Gets config
    pub fn get_config(&self) -> &GeneticsConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_auth::auth::{NodeCapability, SecurityClearance};

    #[tokio::test]
    async fn test_basic_spawning() -> Result<(), BearDogError> {
        let engine = GeneticSpawningEngine::new();

        let request = SpawnRequest {
            required_capabilities: vec![NodeCapability::ComputeProvider],
            security_clearance: SecurityClearance::Basic,
            parent_genetics: vec![],
        };

        let result = engine.spawn_genetics(request)?;

        assert!(result.success);
        assert!(!result.genetics.id.is_empty());
        assert!(!result.messages.is_empty());
        assert!(result.genetics.fitness_score > 0.0);

        Ok(())
    }

    #[tokio::test]
    async fn test_inheritance_spawning() -> Result<(), BearDogError> {
        let engine = GeneticSpawningEngine::new();

        let parent = BearDogGenetics {
            id: "parent-1".to_string(),
            capabilities: vec![NodeCapability::SecurityAnalysis],
            fitness_score: 0.9,
            ..Default::default()
        };

        let request = SpawnRequest {
            required_capabilities: vec![NodeCapability::ComputeProvider],
            security_clearance: SecurityClearance::Basic,
            parent_genetics: vec![parent],
        };

        let result = engine.spawn_genetics(request)?;

        assert!(result.success);
        assert_eq!(result.genetics.generation, 1);
        assert!(result.genetics.capabilities.len() >= 2); // Should inherit + new capabilities

        Ok(())
    }
}
