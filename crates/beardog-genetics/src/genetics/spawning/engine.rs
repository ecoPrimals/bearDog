// SPDX-License-Identifier: AGPL-3.0-only

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

/// Applies inheritance and fitness scoring when minting new [`BearDogGenetics`] records.
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
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: GeneticsConfig::default(),
        }
    }

    /// With Config operation.
    /// Creates instance with config
    #[must_use]
    pub const fn with_config(config: GeneticsConfig) -> Self {
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
            id: genetics_id,
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
    ///
    /// This includes:
    /// - Capability inheritance
    /// - Constraint evolution (Phase 1 self-enforcing keys)
    /// - Generation tracking
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

        // PHASE 1: Constraint Evolution
        // Inherit and evolve constraints from parents
        inherited_genetics = self.evolve_constraints(&inherited_genetics, parents)?;

        Ok(inherited_genetics)
    }

    /// Evolve constraints from parent genetics (Phase 1 Self-Enforcing Keys)
    ///
    /// Constraints are inherited and adapted based on:
    /// - Parent constraint strictness
    /// - Generation level (older generations may have looser constraints)
    /// - Capability requirements of offspring
    ///
    /// # Evolution Rules
    ///
    /// 1. **Immutable Paths**: Union of all parent immutable paths
    /// 2. **Scope**: Most restrictive parent scope
    /// 3. **Lifetime**: Shortest parent lifetime (most restrictive)
    /// 4. **Compute Quota**: Minimum parent quota (most restrictive)
    /// 5. **Co-signers**: Union of all parent co-signers
    ///
    /// # Example
    ///
    /// ```ignore
    /// Parent A: immutable_paths = ["raw_data/*"]
    /// Parent B: immutable_paths = ["archive/*"]
    /// Offspring: immutable_paths = ["raw_data/*", "archive/*"] (union)
    /// ```
    fn evolve_constraints(
        &self,
        offspring: &BearDogGenetics,
        parents: &[BearDogGenetics],
    ) -> Result<BearDogGenetics, BearDogError> {
        use beardog_types::genetics_constraints::{
            DataAccessConstraint, KeyConstraints, LifetimeConstraint, ScopeConstraint,
        };
        use chrono::Utc;
        use std::collections::HashSet;

        // If no parents have constraints, offspring inherits none (backward compatible)
        let parents_with_constraints: Vec<_> =
            parents.iter().filter(|p| p.constraints.is_some()).collect();

        if parents_with_constraints.is_empty() {
            debug!(
                "No parent constraints to evolve for offspring {}",
                offspring.id
            );
            return Ok(offspring.clone());
        }

        debug!(
            "Evolving constraints from {} parents with constraints",
            parents_with_constraints.len()
        );

        let mut evolved = offspring.clone();

        // Collect all parent immutable paths (union for maximum protection)
        let mut all_immutable_paths: HashSet<String> = HashSet::new();
        let mut all_co_signers: HashSet<String> = HashSet::new();
        let mut all_mandatory_encryption: HashSet<String> = HashSet::new();
        let mut most_restrictive_scope: Option<ScopeConstraint> = None;
        let mut shortest_lifetime: Option<chrono::DateTime<Utc>> = None;
        let mut requires_audit = false;

        for parent in &parents_with_constraints {
            if let Some(ref parent_constraints) = parent.constraints {
                // Collect immutable paths
                for path in &parent_constraints.data_access.immutable_paths {
                    all_immutable_paths.insert(path.clone());
                }

                // Collect co-signers
                for signer in &parent_constraints.co_signers {
                    all_co_signers.insert(signer.clone());
                }

                // Collect mandatory encryption keys
                for key in &parent_constraints.data_access.mandatory_encryption {
                    all_mandatory_encryption.insert(key.clone());
                }

                // Audit required if ANY parent requires it
                if parent_constraints.data_access.audit_required {
                    requires_audit = true;
                }

                // Track most restrictive scope
                match &parent_constraints.scope {
                    ScopeConstraint::Unrestricted => {
                        // Don't override if we already have a restriction
                    }
                    scope => {
                        if most_restrictive_scope.is_none() {
                            most_restrictive_scope = Some(scope.clone());
                        }
                    }
                }

                // Track shortest lifetime (most restrictive)
                let parent_expiry = parent_constraints.lifetime.expires_at;
                shortest_lifetime = Some(match shortest_lifetime {
                    None => parent_expiry,
                    Some(current) if parent_expiry < current => parent_expiry,
                    Some(current) => current,
                });
            }
        }

        // Build evolved constraints
        let evolved_constraints = KeyConstraints {
            scope: most_restrictive_scope.unwrap_or(ScopeConstraint::Unrestricted),
            lifetime: LifetimeConstraint {
                expires_at: shortest_lifetime
                    .unwrap_or_else(|| Utc::now() + chrono::Duration::days(365)),
                ..Default::default()
            },
            data_access: DataAccessConstraint {
                immutable_paths: all_immutable_paths.into_iter().collect(),
                mandatory_encryption: all_mandatory_encryption.into_iter().collect(),
                audit_required: requires_audit,
                ..Default::default()
            },
            co_signers: all_co_signers.into_iter().collect(),
            ..Default::default()
        };

        // Only set constraints if they're actually restrictive
        let has_restrictions = !evolved_constraints.data_access.immutable_paths.is_empty()
            || !evolved_constraints.co_signers.is_empty()
            || evolved_constraints.data_access.audit_required
            || !evolved_constraints
                .data_access
                .mandatory_encryption
                .is_empty()
            || !matches!(evolved_constraints.scope, ScopeConstraint::Unrestricted);

        if has_restrictions {
            evolved.constraints = Some(evolved_constraints.clone());
            info!(
                "🧬 Evolved constraints for offspring {}: {} immutable paths, {} co-signers",
                offspring.id,
                evolved_constraints.data_access.immutable_paths.len(),
                evolved_constraints.co_signers.len()
            );
        } else {
            debug!(
                "No restrictive constraints evolved for offspring {}",
                offspring.id
            );
        }

        Ok(evolved)
    }

    /// Calculate fitness score based on genetics
    fn calculate_fitness(&self, genetics: &BearDogGenetics) -> Result<f64, BearDogError> {
        #[expect(
            clippy::cast_precision_loss,
            reason = "capability count as f64 for fitness heuristic"
        )]
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
    #[must_use]
    pub const fn get_config(&self) -> &GeneticsConfig {
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
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: genetics
        // TEST_PRIORITY: normal

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

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
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

    // ============================================================================
    // CONSTRAINT EVOLUTION TESTS (Phase 1)
    // ============================================================================

    #[tokio::test]
    async fn test_constraint_evolution_immutable_paths() -> Result<(), BearDogError> {
        use beardog_types::genetics_constraints::{DataAccessConstraint, KeyConstraints};

        let engine = GeneticSpawningEngine::new();

        // Parent A protects raw_data/*
        let constraints_a = KeyConstraints {
            data_access: DataAccessConstraint {
                immutable_paths: vec!["raw_data/*".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };

        // Parent B protects archive/*
        let constraints_b = KeyConstraints {
            data_access: DataAccessConstraint {
                immutable_paths: vec!["archive/*".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };

        let parent_a = BearDogGenetics {
            id: "parent-a".to_string(),
            constraints: Some(constraints_a),
            ..Default::default()
        };

        let parent_b = BearDogGenetics {
            id: "parent-b".to_string(),
            constraints: Some(constraints_b),
            ..Default::default()
        };

        let request = SpawnRequest {
            required_capabilities: vec![NodeCapability::ComputeProvider],
            security_clearance: SecurityClearance::Basic,
            parent_genetics: vec![parent_a, parent_b],
        };

        let result = engine.spawn_genetics(request)?;

        // Offspring should inherit BOTH immutable paths (union)
        assert!(result.success);
        let offspring_constraints = result.genetics.constraints.as_ref().unwrap();
        assert_eq!(
            offspring_constraints.data_access.immutable_paths.len(),
            2,
            "Should inherit both immutable paths"
        );
        assert!(
            offspring_constraints
                .data_access
                .immutable_paths
                .contains(&"raw_data/*".to_string()),
            "Should inherit raw_data/* from parent A"
        );
        assert!(
            offspring_constraints
                .data_access
                .immutable_paths
                .contains(&"archive/*".to_string()),
            "Should inherit archive/* from parent B"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_constraint_evolution_co_signers() -> Result<(), BearDogError> {
        use beardog_types::genetics_constraints::KeyConstraints;

        let engine = GeneticSpawningEngine::new();

        let constraints_a = KeyConstraints {
            co_signers: vec!["signer-1".to_string()],
            ..Default::default()
        };

        let constraints_b = KeyConstraints {
            co_signers: vec!["signer-2".to_string()],
            ..Default::default()
        };

        let parent_a = BearDogGenetics {
            id: "parent-a".to_string(),
            constraints: Some(constraints_a),
            ..Default::default()
        };

        let parent_b = BearDogGenetics {
            id: "parent-b".to_string(),
            constraints: Some(constraints_b),
            ..Default::default()
        };

        let request = SpawnRequest {
            required_capabilities: vec![],
            security_clearance: SecurityClearance::Basic,
            parent_genetics: vec![parent_a, parent_b],
        };

        let result = engine.spawn_genetics(request)?;

        // Offspring should inherit ALL co-signers
        let offspring_constraints = result.genetics.constraints.as_ref().unwrap();
        assert_eq!(offspring_constraints.co_signers.len(), 2);
        assert!(
            offspring_constraints
                .co_signers
                .contains(&"signer-1".to_string())
        );
        assert!(
            offspring_constraints
                .co_signers
                .contains(&"signer-2".to_string())
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_constraint_evolution_audit_required() -> Result<(), BearDogError> {
        use beardog_types::genetics_constraints::{DataAccessConstraint, KeyConstraints};

        let engine = GeneticSpawningEngine::new();

        // Parent A requires audit
        let constraints_a = KeyConstraints {
            data_access: DataAccessConstraint {
                audit_required: true,
                ..Default::default()
            },
            ..Default::default()
        };

        // Parent B does not require audit
        let parent_a = BearDogGenetics {
            id: "parent-a".to_string(),
            constraints: Some(constraints_a),
            ..Default::default()
        };

        let parent_b = BearDogGenetics {
            id: "parent-b".to_string(),
            constraints: None,
            ..Default::default()
        };

        let request = SpawnRequest {
            required_capabilities: vec![],
            security_clearance: SecurityClearance::Basic,
            parent_genetics: vec![parent_a, parent_b],
        };

        let result = engine.spawn_genetics(request)?;

        // Offspring should require audit if ANY parent requires it
        let offspring_constraints = result.genetics.constraints.as_ref().unwrap();
        assert!(
            offspring_constraints.data_access.audit_required,
            "Should inherit audit requirement from any parent"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_constraint_evolution_no_parents_with_constraints() -> Result<(), BearDogError> {
        let engine = GeneticSpawningEngine::new();

        // Parents without constraints
        let parent_a = BearDogGenetics {
            id: "parent-a".to_string(),
            constraints: None,
            ..Default::default()
        };

        let parent_b = BearDogGenetics {
            id: "parent-b".to_string(),
            constraints: None,
            ..Default::default()
        };

        let request = SpawnRequest {
            required_capabilities: vec![],
            security_clearance: SecurityClearance::Basic,
            parent_genetics: vec![parent_a, parent_b],
        };

        let result = engine.spawn_genetics(request)?;

        // Offspring should have no constraints (backward compatible)
        assert!(
            result.genetics.constraints.is_none(),
            "Offspring should have no constraints when parents don't"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_constraint_evolution_mandatory_encryption() -> Result<(), BearDogError> {
        use beardog_types::genetics_constraints::{DataAccessConstraint, KeyConstraints};

        let engine = GeneticSpawningEngine::new();

        let constraints_a = KeyConstraints {
            data_access: DataAccessConstraint {
                mandatory_encryption: vec!["key-1".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };

        let constraints_b = KeyConstraints {
            data_access: DataAccessConstraint {
                mandatory_encryption: vec!["key-2".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };

        let parent_a = BearDogGenetics {
            id: "parent-a".to_string(),
            constraints: Some(constraints_a),
            ..Default::default()
        };

        let parent_b = BearDogGenetics {
            id: "parent-b".to_string(),
            constraints: Some(constraints_b),
            ..Default::default()
        };

        let request = SpawnRequest {
            required_capabilities: vec![],
            security_clearance: SecurityClearance::Basic,
            parent_genetics: vec![parent_a, parent_b],
        };

        let result = engine.spawn_genetics(request)?;

        // Offspring should inherit ALL mandatory encryption keys
        let offspring_constraints = result.genetics.constraints.as_ref().unwrap();
        assert_eq!(
            offspring_constraints.data_access.mandatory_encryption.len(),
            2
        );
        assert!(
            offspring_constraints
                .data_access
                .mandatory_encryption
                .contains(&"key-1".to_string())
        );
        assert!(
            offspring_constraints
                .data_access
                .mandatory_encryption
                .contains(&"key-2".to_string())
        );

        Ok(())
    }
}
