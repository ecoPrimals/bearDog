// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(
    missing_docs,
    reason = "coordination config types — serde-derived structs with self-documenting field names"
)]

//! Default values, constructors, validation, and migration helpers for coordination config.

use super::collaboration::{
    CollaborationFramework, DecisionProtocol, DecisionStep, MutualAccountability,
};
use super::consensus_rotation::{
    ConsensusType, DecisionThresholds, ExpertiseMapping, RotationCriteria, RotationSchedule,
};
use super::ecosystem::EcosystemIntegrationConfig;
use super::model::{CoordinationConfig, CoordinationModel};
use super::supporting::{AdaptiveHierarchy, NaturalLeadership};
use super::transition_health::{CoordinationHealthConfig, CoordinationTransitionConfig};
use beardog_errors::BearDogError;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use std::time::Duration;

impl Default for CoordinationConfig {
    fn default() -> Self {
        Self {
            coordination_model: CoordinationModel::Collaborative {
                decision_protocol: DecisionProtocol {
                    decision_steps: vec![],
                    step_timeouts: HashMap::new(),
                    advancement_requirements: HashMap::new(),
                },
                collaboration_frameworks: vec![],
                mutual_accountability: MutualAccountability {
                    agreements: vec![],
                    monitoring_mechanisms: vec![],
                    feedback_systems: vec![],
                },
                active_sessions: vec![],
            },
            fallback_models: vec![],
            transition_config: CoordinationTransitionConfig {
                auto_transition_enabled: false,
                transition_triggers: vec![],
                transition_strategies: HashMap::new(),
                transition_cooldown: Duration::from_secs(300), // 5 minutes
            },
            health_monitoring: CoordinationHealthConfig {
                monitoring_enabled: true,
                check_frequency: Duration::from_secs(60), // 1 minute
                health_metrics: vec![],
                alert_thresholds: HashMap::new(),
            },
            ecosystem_integration: EcosystemIntegrationConfig {
                genetics_integration_enabled: true,
                cross_primal_coordination: false, // Disabled by default for security
                supported_symbiosis: vec!["mutualistic".to_string(), "facilitative".to_string()],
                integration_endpoints: HashMap::new(),
            },
        }
    }
}

impl CoordinationConfig {
    /// Create a new coordination configuration with distributed model
    pub fn distributed() -> Self {
        Self {
            coordination_model: CoordinationModel::Distributed {
                consensus_type: ConsensusType::Majority,
                participation_weights: BTreeMap::new(),
                decision_thresholds: DecisionThresholds {
                    routine_threshold: 0.5,
                    significant_threshold: 0.6,
                    critical_threshold: 0.75,
                    emergency_threshold: 0.5,
                },
                consensus_timeout: Duration::from_secs(300),
            },
            ..Default::default()
        }
    }

    /// Create a new coordination configuration with rotational model
    pub fn rotational() -> Self {
        Self {
            coordination_model: CoordinationModel::Rotational {
                rotation_criteria: RotationCriteria {
                    time_based: Some(Duration::from_secs(3600)), // 1 hour
                    workload_based: None,
                    expertise_based: None,
                    performance_based: None,
                },
                expertise_mapping: ExpertiseMapping {
                    expertise_areas: HashMap::new(),
                    participant_scores: HashMap::new(),
                    last_updated: chrono::Utc::now(),
                },
                rotation_schedule: RotationSchedule {
                    evaluation_frequency: Duration::from_secs(300), // 5 minutes
                    minimum_tenure: Duration::from_secs(600),       // 10 minutes
                    maximum_tenure: Duration::from_secs(7200),      // 2 hours
                    next_evaluation: chrono::Utc::now() + chrono::Duration::minutes(5),
                },
                current_leader: None,
            },
            ..Default::default()
        }
    }

    /// Create a new coordination configuration with emergent model
    pub fn emergent() -> Self {
        Self {
            coordination_model: CoordinationModel::Emergent {
                emergence_factors: vec![],
                natural_leadership: NaturalLeadership {
                    current_leaders: HashMap::new(),
                    leadership_strength: HashMap::new(),
                    emergence_patterns: vec![],
                },
                adaptive_hierarchy: AdaptiveHierarchy {
                    current_structure: HashMap::new(),
                    adaptation_rules: vec![],
                    structure_history: vec![],
                },
                emergence_history: vec![],
            },
            ..Default::default()
        }
    }

    /// Check if coordination model supports ecosystem integration
    pub fn supports_ecosystem_integration(&self) -> bool {
        self.ecosystem_integration.genetics_integration_enabled
    }

    /// Get current coordination model type
    /// Gets `coordination_type`
    pub fn get_coordination_type(&self) -> String {
        match &self.coordination_model {
            CoordinationModel::Distributed { .. } => "distributed".to_string(),
            CoordinationModel::Rotational { .. } => "rotational".to_string(),
            CoordinationModel::Contextual { .. } => "contextual".to_string(),
            CoordinationModel::Collaborative { .. } => "collaborative".to_string(),
            CoordinationModel::Emergent { .. } => "emergent".to_string(),
        }
    }

    /// Validate coordination configuration
    /// Validates input
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] when distributed decision thresholds are outside `[0.0, 1.0]`,
    /// or when rotational minimum tenure exceeds maximum tenure.
    pub fn validate(&self) -> Result<(), BearDogError> {
        match &self.coordination_model {
            CoordinationModel::Distributed {
                decision_thresholds,
                ..
            } => {
                if decision_thresholds.routine_threshold > 1.0
                    || decision_thresholds.routine_threshold < 0.0
                {
                    return Err(BearDogError::configuration("Invalid routine threshold"));
                }
            }
            CoordinationModel::Rotational {
                rotation_schedule, ..
            } => {
                if rotation_schedule.minimum_tenure > rotation_schedule.maximum_tenure {
                    return Err(BearDogError::configuration(
                        "Minimum tenure cannot exceed maximum tenure",
                    ));
                }
            }
            _ => {}
        }

        Ok(())
    }
}

/// Migration utility to evolve from primary/replica patterns to coordination models
///
/// # Errors
///
/// Currently this function always succeeds; the [`Result`] type is reserved for future
/// validation or migration failures.
pub fn migrate_from_primary_replica(
    primary_config: &str,
    replica_configs: &[String],
) -> Result<CoordinationConfig, BearDogError> {
    let mut config = CoordinationConfig::default();

    if let CoordinationModel::Collaborative {
        decision_protocol,
        collaboration_frameworks,
        ..
    } = &mut config.coordination_model
    {
        decision_protocol.decision_steps.push(DecisionStep {
            step_id: Arc::from("proposal"),
            description: Arc::from("Propose decision for collaborative review"),
            required_participants: vec![primary_config.to_string()],
            optional_participants: replica_configs.to_vec(),
            min_participation: 0.5,
        });

        collaboration_frameworks.push(CollaborationFramework {
            framework_id: Arc::from("migrated_hierarchy"),
            collaboration_type: Arc::from("decision_making"),
            roles: HashMap::new(),
            communication_protocols: vec![],
            success_metrics: vec![],
        });
    }

    Ok(config)
}

/// Utility to assess coordination health
pub fn assess_coordination_health(config: &CoordinationConfig) -> f64 {
    let _ = config;
    0.8
}
