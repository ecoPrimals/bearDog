// SPDX-License-Identifier: AGPL-3.0-only

//! Genetics modules for ecosystem evolution
//!
//! This module contains the specialized genetics structures that handle different
//! aspects of ecosystem evolution: relationships, membership, coordination, trust,
//! intelligence, adaptive relationships, and contextual decision-making.

use super::support::*;
use crate::ecosystem_evolution::support::HealingProtocol;

// ============================================================================
// Genetics Structs
// ============================================================================

#[derive(Debug, Clone, Default)]
pub struct RelationshipEvolutionGenetics {
    /// Collection of evolution strategies
    pub evolution_strategies: Vec<String>,
    /// Collection of fitness functions
    pub fitness_functions: Vec<String>,
    /// Collection of mutation operators
    pub mutation_operators: Vec<String>,
    /// Collection of crossover methods
    pub crossover_methods: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct EcosystemMembershipGenetics {
    /// The membership evolution value
    pub membership_evolution: MembershipEvolution,
    /// The trust computation value
    pub trust_computation: TrustComputation,
    /// Collection of adaptation algorithms
    pub adaptation_algorithms: Vec<String>,
    /// The membership transitions value
    pub membership_transitions: MembershipTransitions,
}

#[derive(Debug, Clone, Default)]
pub struct SymbioticCoordinationGenetics {
    /// The coordination evolution value
    pub coordination_evolution: CoordinationEvolution,
    /// The leadership emergence value
    pub leadership_emergence: LeadershipEmergence,
    /// Collection of consensus mechanisms
    pub consensus_mechanisms: Vec<String>,
    /// The conflict resolution value
    pub conflict_resolution: ConflictResolution,
}

#[derive(Debug, Clone, Default)]
pub struct TrustEvolutionGenetics {
    /// Collection of trust building algorithms
    pub trust_building_algorithms: Vec<String>,
    /// Collection of healing protocols
    pub healing_protocols: Vec<HealingProtocol>,
    /// The trust measurement value
    pub trust_measurement: TrustMeasurement,
    /// Collection of evolution pathways
    pub evolution_pathways: Vec<String>,
}

/// Emergent ecosystem intelligence
#[derive(Debug, Clone, Default)]
pub struct EcosystemIntelligenceGenetics {
    /// The collective decision making value
    pub collective_decision_making: CollectiveDecisionMaking,
    /// The ecosystem memory value
    pub ecosystem_memory: EcosystemMemory,
    /// The predictive capabilities value
    pub predictive_capabilities: PredictiveCapabilities,
    /// The emergent behavior detection value
    pub emergent_behavior_detection: EmergentBehaviorDetection,
}

/// Adaptive relationship management
#[derive(Debug, Clone, Default)]
pub struct AdaptiveRelationshipGenetics {
    /// The relationship optimization value
    pub relationship_optimization: RelationshipOptimization,
    /// The context awareness value
    pub context_awareness: ContextAwareness,
    /// Collection of adaptation triggers
    pub adaptation_triggers: Vec<String>,
    /// The relationship repair value
    pub relationship_repair: RelationshipRepair,
}

/// Contextual decision making genetics
#[derive(Debug, Clone, Default)]
pub struct ContextualDecisionGenetics {
    /// The context evaluation value
    pub context_evaluation: ContextEvaluationEngine,
    /// Collection of decision trees
    pub decision_trees: Vec<String>,
    /// Collection of learning mechanisms
    pub learning_mechanisms: Vec<String>,
    /// The decision adaptation value
    pub decision_adaptation: DecisionAdaptation,
}
