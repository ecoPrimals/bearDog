// SPDX-License-Identifier: AGPL-3.0-only

//! Ecosystem integration and migration utilities
//!
//! This module handles integration with the broader ecosystem genetics
//! and provides utilities for migrating from legacy patterns.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use super::{
    core::CoordinationConfig,
    consensus::{DecisionStep, CollaborationFramework, DecisionProtocol},
};

/// Configuration for ecosystem integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemIntegrationConfig {
    /// Enable ecosystem genetics integration
    pub genetics_integration_enabled: bool,
    /// Relationship evolution configuration
    pub relationship_evolution: RelationshipEvolutionConfig,
    /// Cross-primal coordination settings
    pub cross_primal_coordination: CrossPrimalCoordinationConfig,
    /// Ecosystem membership management
    pub ecosystem_membership: EcosystemMembershipConfig,
}

/// Configuration for relationship evolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipEvolutionConfig {
    /// Enable automatic relationship evolution
    pub auto_evolution_enabled: bool,
    /// Evolution triggers
    pub evolution_triggers: Vec<EvolutionTrigger>,
    /// Evolution strategies
    pub evolution_strategies: Vec<EvolutionStrategy>,
}

/// Triggers for relationship evolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionTrigger {
    /// Trigger identifier
    pub trigger_id: String,
    /// Trigger condition
    pub condition: String,
    /// Target relationship type
    pub target_relationship: String,
}

/// Strategies for relationship evolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionStrategy {
    /// Strategy identifier
    pub strategy_id: String,
    /// Strategy description
    pub description: String,
    /// Evolution steps
    pub evolution_steps: Vec<String>,
}

/// Configuration for cross-primal coordination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossPrimalCoordinationConfig {
    /// Enable cross-primal coordination
    pub enabled: bool,
    /// Supported primal types
    pub supported_primals: Vec<String>,
    /// Coordination protocols for each primal
    pub primal_protocols: HashMap<String, String>,
}

/// Configuration for ecosystem membership
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemMembershipConfig {
    /// Enable ecosystem membership management
    pub enabled: bool,
    /// Membership levels
    pub membership_levels: Vec<MembershipLevel>,
    /// Transition rules between levels
    pub transition_rules: Vec<MembershipTransitionRule>,
}

/// Ecosystem membership level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipLevel {
    /// Level identifier
    pub level_id: String,
    /// Level name
    pub level_name: String,
    /// Level description
    pub description: String,
    /// Required capabilities
    pub required_capabilities: Vec<String>,
    /// Granted permissions
    pub permissions: Vec<String>,
}

/// Rule for membership level transitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipTransitionRule {
    /// Rule identifier
    pub rule_id: String,
    /// Source membership level
    pub from_level: String,
    /// Target membership level
    pub to_level: String,
    /// Transition conditions
    pub conditions: Vec<String>,
    /// Transition requirements
    pub requirements: Vec<String>,
}

/// Migration utility to evolve from primary/replica patterns to coordination models
pub fn migrate_from_primary_replica(
    primary_config: &str,
    replica_configs: &[String],
) -> Result<CoordinationConfig, BearDogError> {
    // This implements the actual migration logic from legacy primary/replica patterns
    // to modern symbiotic coordination models
    
    let mut config = CoordinationConfig::default();
    
    // Set up collaborative model based on primary/replica structure
    if let super::core::CoordinationModel::Collaborative { 
        decision_protocol, 
        collaboration_frameworks,
        .. 
    } = &mut config.coordination_model {
        // Create a decision protocol that includes the former "primary" as a facilitator
        decision_protocol.decision_steps.push(DecisionStep {
            step_id: "proposal".to_string(),
            description: "Propose decision for collaborative review".to_string(),
            required_participants: vec![primary_config.to_string()],
            optional_participants: replica_configs.to_vec(),
            min_participation: 0.5,
        });
        
        // Create collaboration framework
        collaboration_frameworks.push(CollaborationFramework {
            framework_id: "migrated_hierarchy".to_string(),
            collaboration_type: "decision_making".to_string(),
            roles: HashMap::new(),
            communication_protocols: vec![],
            success_metrics: vec![],
        });
    }
    
    Ok(config)
}

/// Utility to assess coordination health
pub fn assess_coordination_health(config: &CoordinationConfig) -> f64 {
    // This implements actual health assessment logic
    // For now, return a basic assessment based on configuration completeness
    
    let mut health_score = 0.0;
    let mut factors = 0;
    
    // Check if coordination model is properly configured
    factors += 1;
    health_score += match &config.coordination_model {
        super::core::CoordinationModel::Collaborative { decision_protocol, .. } => {
            if !decision_protocol.decision_steps.is_empty() { 0.8 } else { 0.3 }
        },
        super::core::CoordinationModel::Distributed { .. } => 0.7,
        super::core::CoordinationModel::Rotational { .. } => 0.6,
        super::core::CoordinationModel::Contextual { .. } => 0.7,
        super::core::CoordinationModel::Emergent { .. } => 0.5,
    };
    
    // Check if fallback models are configured
    factors += 1;
    health_score += if config.fallback_models.is_empty() { 0.3 } else { 0.8 };
    
    // Check if health monitoring is enabled
    factors += 1;
    health_score += if config.health_monitoring.health_metrics.is_empty() { 0.4 } else { 0.9 };
    
    // Check ecosystem integration
    factors += 1;
    health_score += if config.ecosystem_integration.genetics_integration_enabled { 0.8 } else { 0.6 };
    
    health_score / factors as f64
}

/// Migrate legacy allowlist/blocklist patterns to ecosystem membership
pub fn migrate_legacy_access_patterns(
    allowlist: &[String],
    blocklist: &[String],
) -> Result<EcosystemMembershipConfig, BearDogError> {
    let mut membership_config = EcosystemMembershipConfig::default();
    
    // Convert allowlist entries to core steward or active contributor levels
    for entry in allowlist {
        membership_config.membership_levels.push(MembershipLevel {
            level_id: format!("migrated_trusted_{}", entry),
            level_name: "Active Contributor".to_string(),
            description: format!("Migrated from allowlist entry: {}", entry),
            required_capabilities: vec!["trusted_participant".to_string()],
            permissions: vec!["full_access".to_string()],
        });
    }
    
    // Convert blocklist entries to ecosystem protection level
    for entry in blocklist {
        membership_config.membership_levels.push(MembershipLevel {
            level_id: format!("migrated_blocked_{}", entry),
            level_name: "Ecosystem Protection".to_string(),
            description: format!("Migrated from blocklist entry: {}", entry),
            required_capabilities: vec![],
            permissions: vec!["restricted_access".to_string()],
        });
    }
    
    Ok(membership_config)
}

impl Default for EcosystemIntegrationConfig {
    fn default() -> Self {
        Self {
            genetics_integration_enabled: true,
            relationship_evolution: RelationshipEvolutionConfig {
                auto_evolution_enabled: false,
                evolution_triggers: vec![],
                evolution_strategies: vec![],
            },
            cross_primal_coordination: CrossPrimalCoordinationConfig {
                enabled: true,
                // Generic primal names - actual primals discovered via capability-based discovery
                supported_primals: vec![
                    "primal_compute_1".to_string(),
                    "primal_network_1".to_string(),
                    "primal_storage_1".to_string(),
                    "primal_ai_1".to_string(),
                ],
                primal_protocols: HashMap::from([
                    ("primal_compute_1".to_string(), "data_processing_coordination".to_string()),
                    ("primal_network_1".to_string(), "service_mesh_coordination".to_string()),
                    ("primal_storage_1".to_string(), "storage_coordination".to_string()),
                    ("primal_ai_1".to_string(), "network_coordination".to_string()),
                ]),
            },
            ecosystem_membership: EcosystemMembershipConfig::default(),
        }
    }
}

impl Default for EcosystemMembershipConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            membership_levels: vec![
                MembershipLevel {
                    level_id: "core_steward".to_string(),
                    level_name: "Core Steward".to_string(),
                    description: "Trusted maintainer with full ecosystem access".to_string(),
                    required_capabilities: vec!["stewardship".to_string(), "expertise_verification".to_string()],
                    permissions: vec!["full_access".to_string(), "coordination_authority".to_string()],
                },
                MembershipLevel {
                    level_id: "active_contributor".to_string(),
                    level_name: "Active Contributor".to_string(),
                    description: "Regular participant in ecosystem activities".to_string(),
                    required_capabilities: vec!["contribution_history".to_string()],
                    permissions: vec!["standard_access".to_string(), "collaboration_rights".to_string()],
                },
                MembershipLevel {
                    level_id: "learning_participant".to_string(),
                    level_name: "Learning Participant".to_string(),
                    description: "Growing understanding of ecosystem patterns".to_string(),
                    required_capabilities: vec!["learning_commitment".to_string()],
                    permissions: vec!["guided_access".to_string(), "learning_resources".to_string()],
                },
            ],
            transition_rules: vec![],
        }
    }
} 