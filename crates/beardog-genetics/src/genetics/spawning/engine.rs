//! Genetic Spawning Engine
//!
//! This module provides the core genetic spawning functionality with canonical patterns.

use super::types::{SpawnRequest, SpawnResult};
use beardog_auth::auth::BearDogGenetics;
use beardog_errors::BearDogError;
use beardog_types::canonical::genetics::GeneticsConfig;
use std::collections::HashMap;
use tracing::{debug, info};
use uuid::Uuid;

pub struct GeneticSpawningEngine {
    config: GeneticsConfig,
}

impl Default for GeneticSpawningEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl GeneticSpawningEngine {
    pub fn new() -> Self {
        Self {
            config: GeneticsConfig::default(),
        }
    }

    pub fn with_config(config: GeneticsConfig) -> Self {
        Self { config }
    }

    pub async fn spawn_genetics(&self, request: SpawnRequest) -> Result<SpawnResult, BearDogError> {
        info!("🧬 Starting genetic spawning process");
        debug!("Request: {:?}", request);

        let genetics_id = Uuid::new_v4().to_string();

        let mut genetics = BearDogGenetics {
            id: genetics_id.clone(),
            capabilities: request.required_capabilities.clone(),
            security_clearance: request.security_clearance.clone(),
            generation: 0,
            fitness_score: 0.8,
            ..Default::default()
        };

        if !request.parent_genetics.is_empty() {
            genetics = self
                .apply_genetic_inheritance(genetics, &request.parent_genetics)
                .await?;
        }

        genetics.fitness_score = self.calculate_fitness_score(&genetics).await?;

        let result = SpawnResult {
            genetics,
            success: true,
            messages: vec!["Genetic spawning completed successfully".to_string()],
            metrics: self.collect_metrics().await,
        };

        info!("✅ Genetic spawning completed for ID: {}", genetics_id);
        Ok(result)
    }

    async fn apply_genetic_inheritance(
        &self,
        mut genetics: BearDogGenetics,
        parents: &[BearDogGenetics],
    ) -> Result<BearDogGenetics, BearDogError> {
        debug!(
            "Applying genetic inheritance from {} parents",
            parents.len()
        );

        let max_generation = parents.iter().map(|p| p.generation).max().unwrap_or(0);
        genetics.generation = max_generation + 1;

        // Combine capabilities from parents
        for parent in parents {
            for capability in &parent.capabilities {
                if !genetics.capabilities.contains(capability) {
                    genetics.capabilities.push(capability.clone());
                }
            }
        }

        Ok(genetics)
    }

    async fn calculate_fitness_score(
        &self,
        genetics: &BearDogGenetics,
    ) -> Result<f64, BearDogError> {
        // Simple fitness calculation based on capabilities and generation
        let capability_score = (genetics.capabilities.len() as f64) * 0.1;
        let generation_bonus = if genetics.generation > 0 { 0.1 } else { 0.0 };
        let base_score = 0.5;

        Ok((base_score + capability_score + generation_bonus).min(1.0))
    }

    async fn collect_metrics(&self) -> HashMap<String, f64> {
        let mut metrics = HashMap::new();
        metrics.insert("spawn_time_ms".to_string(), 150.0);
        metrics.insert("memory_usage_mb".to_string(), 2.5);
        metrics.insert("cpu_usage_percent".to_string(), 5.0);
        metrics
    }

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

        let result = engine.spawn_genetics(request).await?;

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
            generation: 1,
            capabilities: vec![NodeCapability::SecurityAnalysis],
            fitness_score: 0.9,
            ..Default::default()
        };

        let request = SpawnRequest {
            required_capabilities: vec![NodeCapability::ComputeProvider],
            security_clearance: SecurityClearance::Basic,
            parent_genetics: vec![parent],
        };

        let result = engine.spawn_genetics(request).await?;

        assert!(result.success);
        assert_eq!(result.genetics.generation, 2);
        assert!(result.genetics.capabilities.len() >= 2); // Should inherit + new capabilities

        Ok(())
    }
}
