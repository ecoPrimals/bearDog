// SPDX-License-Identifier: AGPL-3.0-only

//! Ecosystem Genetic Engine - Core evolution engine
//!
//! This module contains the main `EcosystemGeneticEngine` that orchestrates
//! ecosystem evolution, integrating all genetics modules to evolve binary
//! patterns into rich spectrum-based relationships.

use super::genetics::*;
use super::support::*;
use super::types::{CoordinationModel, EcosystemMembership, SymbiosisType, TrustEvolution};
use beardog_errors::BearDogError;
use beardog_types::canonical::HealthStatus;
use chrono::Utc;
use std::collections::HashMap;
use std::time::Duration;
use tracing::{debug, info};

// ============================================================================
// Main Ecosystem Genetic Engine
// ============================================================================

/// Enhanced genetic engine that integrates ecosystem evolution capabilities
#[derive(Debug, Clone)]
pub struct EcosystemGeneticEngine {
    /// Evolution algorithms
    /// The evolution algorithms value
    pub evolution_algorithms: EvolutionAlgorithms,
    /// The trait inheritance value
    pub trait_inheritance: TraitInheritance,
    /// The adaptive security value
    pub adaptive_security: AdaptiveSecurity,

    /// Integrated via horizontal gene transfer from AI capability providers
    /// (capability-based integration, no hardcoded primal names)
    /// The relationship evolution value
    pub relationship_evolution: RelationshipEvolutionGenetics,
    /// The ecosystem membership value
    pub ecosystem_membership: EcosystemMembershipGenetics,
    /// The symbiotic coordination value
    pub symbiotic_coordination: SymbioticCoordinationGenetics,
    /// The trust dynamics value
    pub trust_dynamics: TrustEvolutionGenetics,

    /// Emergent capabilities
    /// The ecosystem intelligence value
    pub ecosystem_intelligence: EcosystemIntelligenceGenetics,
    /// The adaptive relationships value
    pub adaptive_relationships: AdaptiveRelationshipGenetics,
    /// The contextual decision making value
    pub contextual_decision_making: ContextualDecisionGenetics,
}

impl EcosystemGeneticEngine {
    /// Create new ecosystem genetic engine
    pub fn new() -> Result<Self, BearDogError> {
        info!("Initializing ecosystem genetic engine");

        Ok(Self {
            evolution_algorithms: EvolutionAlgorithms::default(),
            trait_inheritance: TraitInheritance::default(),
            adaptive_security: AdaptiveSecurity::default(),
            relationship_evolution: RelationshipEvolutionGenetics::default(),
            ecosystem_membership: EcosystemMembershipGenetics::default(),
            symbiotic_coordination: SymbioticCoordinationGenetics::default(),
            trust_dynamics: TrustEvolutionGenetics::default(),
            ecosystem_intelligence: EcosystemIntelligenceGenetics::default(),
            adaptive_relationships: AdaptiveRelationshipGenetics::default(),
            contextual_decision_making: ContextualDecisionGenetics::default(),
        })
    }

    /// Evolve binary access patterns (allowlist/blocklist) to ecosystem membership spectrum
    pub fn evolve_access_pattern(
        &self,
        pattern: BinaryAccessPattern,
        context: EcosystemContext,
    ) -> Result<EcosystemMembership, BearDogError> {
        info!("🧬 Evolving binary access pattern to ecosystem membership");

        match pattern {
            BinaryAccessPattern::Allowlist { allowed_entities } => {
                self.evolve_allowlist_to_membership(allowed_entities, context)
            }
            BinaryAccessPattern::Blocklist { blocked_entities } => {
                self.evolve_blocklist_to_protection(blocked_entities, context)
            }
            BinaryAccessPattern::Simple { is_allowed } => {
                self.evolve_simple_to_spectrum(is_allowed, context)
            }
        }
    }

    /// Evolve binary trust (trusted/untrusted) to trust evolution dynamics
    pub fn evolve_trust_pattern(
        &self,
        trust: BinaryTrust,
        _history: RelationshipHistory,
        _context: EcosystemContext,
    ) -> Result<TrustEvolution, BearDogError> {
        info!("🧬 Evolving binary trust to trust dynamics");

        match trust {
            BinaryTrust::Trusted => {
                debug!("Evolving trusted state to flourishing trust");
                Ok(TrustEvolution::Flourishing {
                    stability_metrics: "high".to_string(),
                    flourishing_indicators: vec!["consistent_positive_interactions".to_string()],
                    symbiotic_benefits: "active_participation".to_string(),
                })
            }
            BinaryTrust::Untrusted => {
                debug!("Evolving untrusted state to building trust");
                Ok(TrustEvolution::Building {
                    progress_rate: 0.1,
                    milestones: vec!["first_interaction".to_string()],
                    building_activities: vec!["mentorship".to_string()],
                    genetic_compatibility: 0.5,
                })
            }
        }
    }

    /// Evolve hierarchical patterns (primary/replica, client/server) to coordination models
    pub fn evolve_coordination_pattern(
        &self,
        hierarchy: HierarchicalPattern,
        _relationships: Vec<SymbiosisType>,
        _context: String,
    ) -> Result<CoordinationModel, BearDogError> {
        info!("🧬 Evolving hierarchical pattern to coordination model");

        match hierarchy {
            HierarchicalPattern::PrimaryReplica { primary, replicas } => {
                debug!("Evolving primary-replica to collaborative coordination");
                Ok(CoordinationModel::Collaborative {
                    decision_protocol: format!("collaborative decision-making with {primary}"),
                    collaboration_frameworks: vec![
                        "consensus_building".to_string(),
                        "shared_decision_making".to_string(),
                    ],
                    mutual_accountability: "all_participants_accountable".to_string(),
                    active_sessions: replicas
                        .iter()
                        .map(|r| format!("session_with_{r}"))
                        .collect(),
                })
            }
            HierarchicalPattern::ClientServer { server, clients } => {
                debug!("Evolving client-server to distributed coordination");
                let mut weights = HashMap::new();
                weights.insert(server, 1.0);
                for client in clients {
                    weights.insert(client, 0.5);
                }
                Ok(CoordinationModel::Distributed {
                    consensus_type: "weighted_voting".to_string(),
                    participation_weights: weights,
                    decision_thresholds: "majority_with_weights".to_string(),
                })
            }
            HierarchicalPattern::PrimarySecondary { primary, .. } => {
                debug!("Evolving primary-secondary to rotational leadership");
                Ok(CoordinationModel::Rotational {
                    rotation_schedule: "periodic".to_string(),
                    current_leader: Some(primary),
                    rotation_criteria: "based_on_availability_and_trust".to_string(),
                    expertise_mapping: "general_leadership".to_string(),
                })
            }
        }
    }

    /// Generate emergent behaviors from ecosystem state
    pub fn generate_emergent_behaviors(&self) -> Result<Vec<EmergentBehavior>, BearDogError> {
        info!("🌱 Generating emergent behaviors");

        Ok(vec![
            EmergentBehavior {
                behavior_type: "trust_network_formation".to_string(),
                emergence_strength: 0.8,
                participants: vec![
                    "core_stewards".to_string(),
                    "active_contributors".to_string(),
                ],
                beneficial: true,
            },
            EmergentBehavior {
                behavior_type: "knowledge_sharing_patterns".to_string(),
                emergence_strength: 0.7,
                participants: vec!["all_membership_levels".to_string()],
                beneficial: true,
            },
            EmergentBehavior {
                behavior_type: "collaborative_problem_solving".to_string(),
                emergence_strength: 0.7,
                participants: vec![
                    "learning_participants".to_string(),
                    "visiting_collaborators".to_string(),
                ],
                beneficial: true,
            },
        ])
    }

    /// Assess ecosystem health and adaptation
    pub fn assess_ecosystem_health(&self) -> Result<EcosystemHealthReport, BearDogError> {
        info!("🏥 Assessing ecosystem health");

        Ok(EcosystemHealthReport {
            overall_health: HealthStatus::Healthy,
            relationship_health: 0.8,
            trust_health: 0.8,
            coordination_health: 0.8,
            emergent_capabilities: self.generate_emergent_behaviors()?,
            recommendations: vec!["Continue monitoring trust evolution patterns".to_string()],
        })
    }

    fn evolve_allowlist_to_membership(
        &self,
        _allowed_entities: Vec<String>,
        _context: EcosystemContext,
    ) -> Result<EcosystemMembership, BearDogError> {
        debug!("Evolving allowlist entries to ecosystem membership spectrum");

        Ok(EcosystemMembership::ActiveContributor {
            contribution_types: vec![ContributionType::CommunitySupport],
            trust_level: 0.7,
            interaction_history: InteractionSummary {
                total_interactions: 0,
                positive_interactions: 0,
                neutral_interactions: 0,
                concerning_interactions: 0,
                last_interaction: Utc::now(),
                interaction_trend: InteractionTrend::Stable,
                relationship_quality: 0.7,
            },
            evolutionary_potential: 0.8,
        })
    }

    fn evolve_blocklist_to_protection(
        &self,
        _blocked_entities: Vec<String>,
        _context: EcosystemContext,
    ) -> Result<EcosystemMembership, BearDogError> {
        debug!("Evolving blocklist entries to ecosystem protection with restoration");

        Ok(EcosystemMembership::EcosystemProtection {
            protection_reason: ProtectionReason::PreviousConcerns,
            protection_level: ProtectionLevel::Moderate,
            trust_level: 0.1,
            restoration_path: Some(RestorationPath {
                steps: vec!["demonstrate_positive_behavior".to_string()],
                timeline: Duration::from_secs(30 * 24 * 3600), // 30 days
                success_criteria: vec!["consistent_positive_interactions".to_string()],
            }),
        })
    }

    fn evolve_simple_to_spectrum(
        &self,
        is_allowed: bool,
        _context: EcosystemContext,
    ) -> Result<EcosystemMembership, BearDogError> {
        debug!("Evolving simple binary pattern to ecosystem spectrum");

        if is_allowed {
            Ok(EcosystemMembership::LearningParticipant {
                learning_path: LearningPath {
                    current_stage: "newcomer".to_string(),
                    next_milestones: vec!["first_positive_contribution".to_string()],
                    learning_resources: vec!["ecosystem_guidelines".to_string()],
                },
                trust_level: 0.5,
                mentorship_connections: vec![],
                adaptation_rate: 0.8,
            })
        } else {
            Ok(EcosystemMembership::CautiousInteraction {
                concern_factors: vec![ConcernFactor::UnknownEntity],
                monitoring_level: MonitoringLevel::Basic,
                trust_level: 0.3,
                review_schedule: ReviewSchedule {
                    next_review: Utc::now() + chrono::Duration::days(7),
                    review_frequency: Duration::from_secs(7 * 24 * 3600), // 7 days
                },
                healing_protocols: vec![],
            })
        }
    }
}

// ============================================================================
// Default Implementations for Core Components
// ============================================================================

impl Default for EvolutionAlgorithms {
    fn default() -> Self {
        Self {
            genetic_operators: vec!["crossover".to_string(), "mutation".to_string()],
            fitness_functions: vec![
                "ecosystem_health".to_string(),
                "relationship_quality".to_string(),
            ],
            selection_methods: vec!["tournament".to_string(), "roulette".to_string()],
        }
    }
}

impl Default for TraitInheritance {
    fn default() -> Self {
        Self {
            inheritance_patterns: vec!["dominant".to_string(), "recessive".to_string()],
            trait_combinations: HashMap::new(),
            expression_rules: vec!["context_dependent".to_string()],
        }
    }
}

impl Default for AdaptiveSecurity {
    fn default() -> Self {
        Self {
            security_adaptations: vec![
                "threat_response".to_string(),
                "trust_adjustment".to_string(),
            ],
            learning_mechanisms: vec!["reinforcement".to_string(), "evolutionary".to_string()],
            adaptation_triggers: vec!["anomaly_detection".to_string()],
        }
    }
}

// Supporting struct types
/// Named genetic operators, fitness evaluators, and selection strategies used by the evolution engine.
#[derive(Debug, Clone)]
pub struct EvolutionAlgorithms {
    /// Collection of genetic operators
    pub genetic_operators: Vec<String>,
    /// Collection of fitness functions
    pub fitness_functions: Vec<String>,
    /// Collection of selection methods
    pub selection_methods: Vec<String>,
}

/// How traits combine across generations (dominance, co-expression) in the ecosystem metaphor.
#[derive(Debug, Clone)]
pub struct TraitInheritance {
    /// Collection of inheritance patterns
    pub inheritance_patterns: Vec<String>,
    /// The trait combinations value
    pub trait_combinations: HashMap<String, String>,
    /// Collection of expression rules
    pub expression_rules: Vec<String>,
}

/// Security posture adaptations (threat response, trust tuning) that co-evolve with relationships.
#[derive(Debug, Clone)]
pub struct AdaptiveSecurity {
    /// Collection of security adaptations
    pub security_adaptations: Vec<String>,
    /// Collection of learning mechanisms
    pub learning_mechanisms: Vec<String>,
    /// Collection of adaptation triggers
    pub adaptation_triggers: Vec<String>,
}

// ============================================================================
// Migration Utility
// ============================================================================

/// Migration utility to evolve from binary patterns to ecosystem models
pub fn migrate_from_binary_patterns(
    allowlist_entries: Vec<String>,
    blocklist_entries: Vec<String>,
) -> Result<Vec<EcosystemMembership>, BearDogError> {
    let mut memberships = Vec::new();

    // Migrate allowlist entries to active contributors
    for _entity in allowlist_entries {
        memberships.push(EcosystemMembership::ActiveContributor {
            contribution_types: vec![ContributionType::CommunitySupport],
            trust_level: 0.7,
            interaction_history: InteractionSummary {
                total_interactions: 0,
                positive_interactions: 0,
                neutral_interactions: 0,
                concerning_interactions: 0,
                last_interaction: Utc::now(),
                interaction_trend: InteractionTrend::Stable,
                relationship_quality: 0.7,
            },
            evolutionary_potential: 0.8,
        });
    }

    // Migrate blocklist entries to ecosystem protection
    for _entity in blocklist_entries {
        memberships.push(EcosystemMembership::EcosystemProtection {
            protection_reason: ProtectionReason::PreviousConcerns,
            protection_level: ProtectionLevel::Moderate,
            trust_level: 0.1,
            restoration_path: Some(RestorationPath {
                steps: vec!["demonstrate_positive_behavior".to_string()],
                timeline: Duration::from_secs(30 * 24 * 3600), // 30 days
                success_criteria: vec!["consistent_positive_interactions".to_string()],
            }),
        });
    }

    Ok(memberships)
}
