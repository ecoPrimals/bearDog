// SPDX-License-Identifier: AGPL-3.0-only

//! Genetics management implementation

use super::types::*;
use beardog_errors::BearDogError;
use uuid::Uuid;

impl CrossNodeAuthEngine {
    /// Register genetics
    pub fn register_genetics(&mut self, genetics: BearDogGenetics) -> Result<(), BearDogError> {
        self.genetics_registry.insert(genetics.id.clone(), genetics);
        Ok(())
    }

    /// Combine genetics from parent nodes
    pub fn combine_genetics(&self, parent_ids: &[&str]) -> Result<BearDogGenetics, BearDogError> {
        if parent_ids.is_empty() {
            return Err(BearDogError::business(
                "At least one parent required for genetic combination".to_string(),
            ));
        }

        // Stub implementation - would do actual genetic combination
        let combined_id = Uuid::new_v4().to_string();

        Ok(BearDogGenetics {
            id: combined_id,
            crypto_chromosomes: vec![],
            security_traits: SecurityTraits::default(),
            capabilities: vec![],
            spawn_restrictions: vec![],
            generation: 1,
            parent_genetics: Some(parent_ids.iter().map(|s| (*s).to_string()).collect()),
            mutations: vec![],
            fitness_score: 0.8,
            security_clearance: SecurityClearance::Medium,
            specializations: vec![],
            constraints: None, // No constraints for combined genetics (can be added separately)
            constraint_signature: None,
            public_key: None,
        })
    }

    /// Terminate a spawn
    pub fn terminate_spawn(&mut self, spawn_id: &str) -> Result<(), BearDogError> {
        if let Some(spawn) = self.spawned_beardogs.get_mut(spawn_id) {
            spawn.current_status = SpawnStatus::Terminated;
            Ok(())
        } else {
            Err(BearDogError::not_found(format!(
                "Spawn not found: {spawn_id}"
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::collections::HashMap;

    #[test]
    fn test_register_genetics_success() {
        let mut engine = CrossNodeAuthEngine::default();

        let genetics = BearDogGenetics {
            id: "gen-1".to_string(),
            crypto_chromosomes: vec![],
            security_traits: SecurityTraits::default(),
            capabilities: vec![],
            spawn_restrictions: vec![],
            generation: 1,
            parent_genetics: None,
            mutations: vec![],
            fitness_score: 0.9,
            security_clearance: SecurityClearance::High,
            specializations: vec![],
            public_key: Some(vec![]),
            constraints: None,
            constraint_signature: None,
        };

        let result = engine.register_genetics(genetics.clone());
        assert!(result.is_ok(), "Registration should succeed");
        assert!(engine.genetics_registry.contains_key("gen-1"));
    }

    #[test]
    fn test_register_genetics_overwrites_existing() {
        let mut engine = CrossNodeAuthEngine::default();

        let genetics1 = BearDogGenetics {
            id: "gen-1".to_string(),
            fitness_score: 0.5,
            ..Default::default()
        };

        let genetics2 = BearDogGenetics {
            id: "gen-1".to_string(),
            fitness_score: 0.9,
            ..Default::default()
        };

        engine.register_genetics(genetics1).unwrap();
        engine.register_genetics(genetics2).unwrap();

        let registered = engine.genetics_registry.get("gen-1").unwrap();
        assert_eq!(registered.fitness_score, 0.9, "Should have updated value");
    }

    #[test]
    fn test_combine_genetics_single_parent() {
        let engine = CrossNodeAuthEngine::default();

        let result = engine.combine_genetics(&["parent-1"]);
        assert!(result.is_ok(), "Single parent combination should succeed");

        let combined = result.unwrap();
        assert_eq!(combined.generation, 1);
        assert_eq!(combined.fitness_score, 0.8);
        assert_eq!(combined.security_clearance, SecurityClearance::Medium);
        assert!(combined.parent_genetics.is_some());
        assert_eq!(combined.parent_genetics.unwrap(), vec!["parent-1"]);
    }

    #[test]
    fn test_combine_genetics_multiple_parents() {
        let engine = CrossNodeAuthEngine::default();

        let result = engine.combine_genetics(&["parent-1", "parent-2", "parent-3"]);
        assert!(result.is_ok(), "Multiple parent combination should succeed");

        let combined = result.unwrap();
        assert!(combined.parent_genetics.is_some());
        let parents = combined.parent_genetics.unwrap();
        assert_eq!(parents.len(), 3);
        assert!(parents.contains(&"parent-1".to_string()));
        assert!(parents.contains(&"parent-2".to_string()));
        assert!(parents.contains(&"parent-3".to_string()));
    }

    #[test]
    fn test_combine_genetics_empty_parents_fails() {
        let engine = CrossNodeAuthEngine::default();

        let result = engine.combine_genetics(&[]);
        assert!(result.is_err(), "Empty parents should fail");
    }

    #[test]
    fn test_combine_genetics_generates_unique_id() {
        let engine = CrossNodeAuthEngine::default();

        let result1 = engine.combine_genetics(&["parent-1"]).unwrap();
        let result2 = engine.combine_genetics(&["parent-1"]).unwrap();

        assert_ne!(
            result1.id, result2.id,
            "Each combination should generate unique ID"
        );
    }

    #[test]
    fn test_terminate_spawn_success() {
        let mut engine = CrossNodeAuthEngine::default();
        let spawn_id = "spawn-1".to_string();

        let spawn = SpawnedBearDog {
            spawn_id: spawn_id.clone(),
            parent_id: "parent-1".to_string(),
            genetics: BearDogGenetics::default(),
            spawn_purpose: SpawnPurpose::TaskExecution,
            task_assignment: vec![],
            resource_limits: ResourceLimits::default(),
            spawn_time: Utc::now(),
            expected_lifetime: None,
            current_status: SpawnStatus::Active,
            performance_metrics: HashMap::new(),
            trust_relationships: HashMap::new(),
            consensus_participation: false,
            ecosystem_connections: vec![],
        };
        engine.spawned_beardogs.insert(spawn_id.clone(), spawn);

        let result = engine.terminate_spawn(&spawn_id);
        assert!(result.is_ok(), "Termination should succeed");

        let terminated = engine.spawned_beardogs.get(&spawn_id).unwrap();
        assert!(matches!(terminated.current_status, SpawnStatus::Terminated));
    }

    #[test]
    fn test_terminate_spawn_not_found() {
        let mut engine = CrossNodeAuthEngine::default();

        let result = engine.terminate_spawn("nonexistent");
        assert!(result.is_err(), "Terminating nonexistent spawn should fail");
    }
}
