// SPDX-License-Identifier: AGPL-3.0-or-later

// Biological Relationship Types - Ecosystem Symbiosis Patterns
//
// This module implements biological relationship patterns that enable rich
// inter-primal symbiosis based on natural ecosystem relationships like
// mutualism, commensalism, and facilitation.

// Note: BearDogError available for future extensions (use Result<T, BearDogError>)
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Biological relationship types for ecosystem interactions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BiologicalRelationship {
    /// Mutualistic relationship where both parties benefit
    Mutualistic {
        mutual_benefits: Vec<MutualBenefit>,
        benefit_balance: f64, // How balanced the benefits are (-1.0 to 1.0)
        sustainability_score: f64,
        evolution_potential: EvolutionPotential,
    },
    
    /// Commensal relationship where one benefits, other is neutral
    Commensal {
        beneficiary: String,
        neutral_party: String,
        benefit_type: BenefitType,
        impact_assessment: ImpactAssessment,
    },
    
    /// Facilitative relationship that helps ecosystem growth
    Facilitative {
        facilitator: String,
        facilitated_parties: Vec<String>,
        facilitation_methods: Vec<FacilitationMethod>,
        ecosystem_impact: EcosystemImpact,
    },
    
    /// Protective relationship that shields from threats
    Protective {
        protector: String,
        protected_parties: Vec<String>,
        protection_methods: Vec<ProtectionMethod>,
        threat_mitigation: ThreatMitigation,
    },
    
    /// Competitive relationship with healthy competition
    Competitive {
        competitors: Vec<String>,
        competition_areas: Vec<CompetitionArea>,
        competition_rules: CompetitionRules,
        ecosystem_benefits: Vec<String>,
    },
}

/// Types of mutual benefits in relationships
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MutualBenefit {
    ResourceSharing(ResourceSharingBenefit),
    KnowledgeExchange(KnowledgeExchangeBenefit),
    CapabilityAmplification(CapabilityAmplificationBenefit),
    RiskMitigation(RiskMitigationBenefit),
    EfficiencyGain(EfficiencyGainBenefit),
    InnovationSynergy(InnovationSynergyBenefit),
}

/// Resource sharing benefit details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceSharingBenefit {
    pub shared_resources: Vec<String>,
    pub sharing_ratio: HashMap<String, f64>,
    pub efficiency_gains: f64,
    pub cost_reductions: f64,
}

/// Knowledge exchange benefit details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeExchangeBenefit {
    pub knowledge_domains: Vec<String>,
    pub exchange_frequency: Duration,
    pub learning_acceleration: f64,
    pub innovation_potential: f64,
}

/// Capability amplification benefit details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapabilityAmplificationBenefit {
    pub amplified_capabilities: Vec<String>,
    pub amplification_factor: f64,
    pub synergy_effects: Vec<String>,
    pub emergence_potential: f64,
}

/// Risk mitigation benefit details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RiskMitigationBenefit {
    pub mitigated_risks: Vec<String>,
    pub risk_reduction_percentage: f64,
    pub shared_resilience: f64,
    pub recovery_acceleration: f64,
}

/// Efficiency gain benefit details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EfficiencyGainBenefit {
    pub efficiency_areas: Vec<String>,
    pub performance_improvements: HashMap<String, f64>,
    pub resource_optimization: f64,
    pub time_savings: Duration,
}

/// Innovation synergy benefit details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InnovationSynergyBenefit {
    pub innovation_areas: Vec<String>,
    pub synergy_multiplier: f64,
    pub breakthrough_potential: f64,
    pub collaborative_innovations: Vec<String>,
}

/// Evolution potential for relationships
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvolutionPotential {
    pub evolution_directions: Vec<EvolutionDirection>,
    pub adaptation_capacity: f64,
    pub learning_rate: f64,
    pub stability_factors: Vec<String>,
}

/// Direction of relationship evolution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EvolutionDirection {
    DeeperIntegration,
    BroaderCollaboration,
    SpecializedPartnership,
    EcosystemLeadership,
    CommunityBuilding,
}

/// Type of benefit in relationships
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BenefitType {
    ResourceAccess,
    KnowledgeGain,
    NetworkExpansion,
    CapabilityEnhancement,
    ProtectionProvision,
    OpportunityCreation,
}

/// Assessment of relationship impact
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImpactAssessment {
    pub positive_impacts: Vec<String>,
    pub neutral_impacts: Vec<String>,
    pub potential_concerns: Vec<String>,
    pub monitoring_indicators: Vec<String>,
}

/// Methods of facilitation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FacilitationMethod {
    ResourceProvision,
    KnowledgeTransfer,
    ConnectionFacilitation,
    PlatformProvision,
    CapacityBuilding,
    BarrierRemoval,
}

/// Impact on the broader ecosystem
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EcosystemImpact {
    pub direct_beneficiaries: Vec<String>,
    pub indirect_beneficiaries: Vec<String>,
    pub ecosystem_health_improvement: f64,
    pub sustainability_contribution: f64,
}

/// Methods of protection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProtectionMethod {
    ThreatDetection,
    ResourceBuffering,
    SecurityProvision,
    RiskAbsorption,
    RecoverySupport,
    PreventiveMeasures,
}

/// Threat mitigation strategies
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThreatMitigation {
    pub identified_threats: Vec<String>,
    pub mitigation_strategies: HashMap<String, String>,
    pub protection_effectiveness: f64,
    pub recovery_capabilities: Vec<String>,
}

/// Areas of healthy competition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompetitionArea {
    pub domain: String,
    pub competition_type: CompetitionType,
    pub success_metrics: Vec<String>,
    pub ecosystem_benefits: Vec<String>,
}

/// Type of competition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CompetitionType {
    PerformanceBased,
    InnovationDriven,
    EfficiencyFocused,
    QualityOriented,
    ServiceExcellence,
}

/// Rules governing competition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompetitionRules {
    pub fairness_principles: Vec<String>,
    pub collaboration_boundaries: Vec<String>,
    pub shared_standards: Vec<String>,
    pub conflict_resolution: Vec<String>,
}

/// Relationship type classification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RelationshipType {
    Symbiotic,
    Collaborative,
    Competitive,
    Neutral,
    Protective,
}

/// Role in the ecosystem
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EcosystemRole {
    Producer,      // Creates value/resources
    Consumer,      // Uses resources/services
    Facilitator,   // Enables others' success
    Protector,     // Shields from threats
    Connector,     // Links different parties
    Innovator,     // Drives new developments
}

/// Relationship health metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipHealth {
    pub overall_health: f64,
    pub sustainability_score: f64,
    pub mutual_satisfaction: f64,
    pub ecosystem_contribution: f64,
    pub growth_potential: f64,
    pub resilience_factor: f64,
}

/// Relationship lifecycle stage
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RelationshipLifecycle {
    Formation,
    Development,
    Maturation,
    Optimization,
    Evolution,
    Renewal,
}

impl BiologicalRelationship {
    /// Get the relationship type classification
    pub fn relationship_type(&self) -> RelationshipType {
        match self {
            BiologicalRelationship::Mutualistic { .. } => RelationshipType::Symbiotic,
            BiologicalRelationship::Commensal { .. } => RelationshipType::Symbiotic,
            BiologicalRelationship::Facilitative { .. } => RelationshipType::Collaborative,
            BiologicalRelationship::Protective { .. } => RelationshipType::Protective,
            BiologicalRelationship::Competitive { .. } => RelationshipType::Competitive,
        }
    }
    
    /// Get the primary participants in this relationship
    pub fn participants(&self) -> Vec<String> {
        match self {
            BiologicalRelationship::Mutualistic { .. } => {
                // Would extract from mutual_benefits in real implementation
                vec!["participant_a".to_string(), "participant_b".to_string()]
            }
            BiologicalRelationship::Commensal { beneficiary, neutral_party, .. } => {
                vec![beneficiary.clone(), neutral_party.clone()]
            }
            BiologicalRelationship::Facilitative { facilitator, facilitated_parties, .. } => {
                let mut participants = vec![facilitator.clone()];
                participants.extend(facilitated_parties.clone());
                participants
            }
            BiologicalRelationship::Protective { protector, protected_parties, .. } => {
                let mut participants = vec![protector.clone()];
                participants.extend(protected_parties.clone());
                participants
            }
            BiologicalRelationship::Competitive { competitors, .. } => {
                competitors.clone()
            }
        }
    }
    
    /// Calculate the relationship health score
    pub fn calculate_health(&self) -> RelationshipHealth {
        match self {
            BiologicalRelationship::Mutualistic { 
                benefit_balance, 
                sustainability_score, 
                .. 
            } => {
                RelationshipHealth {
                    overall_health: (benefit_balance.abs() + sustainability_score) / 2.0,
                    sustainability_score: *sustainability_score,
                    mutual_satisfaction: benefit_balance.abs(),
                    ecosystem_contribution: 0.8,
                    growth_potential: 0.9,
                    resilience_factor: 0.85,
                }
            }
            BiologicalRelationship::Facilitative { ecosystem_impact, .. } => {
                RelationshipHealth {
                    overall_health: ecosystem_impact.ecosystem_health_improvement,
                    sustainability_score: ecosystem_impact.sustainability_contribution,
                    mutual_satisfaction: 0.7,
                    ecosystem_contribution: ecosystem_impact.ecosystem_health_improvement,
                    growth_potential: 0.8,
                    resilience_factor: 0.75,
                }
            }
            _ => {
                // Default health metrics for other relationship types
                RelationshipHealth {
                    overall_health: 0.7,
                    sustainability_score: 0.6,
                    mutual_satisfaction: 0.7,
                    ecosystem_contribution: 0.6,
                    growth_potential: 0.7,
                    resilience_factor: 0.65,
                }
            }
        }
    }
    
    /// Get the ecosystem roles of participants
    pub fn ecosystem_roles(&self) -> HashMap<String, EcosystemRole> {
        let mut roles = HashMap::new();
        
        match self {
            BiologicalRelationship::Mutualistic { .. } => {
                // Both parties are producers in mutualistic relationships
                for participant in self.participants() {
                    roles.insert(participant, EcosystemRole::Producer);
                }
            }
            BiologicalRelationship::Commensal { beneficiary, neutral_party, .. } => {
                roles.insert(beneficiary.clone(), EcosystemRole::Consumer);
                roles.insert(neutral_party.clone(), EcosystemRole::Producer);
            }
            BiologicalRelationship::Facilitative { facilitator, facilitated_parties, .. } => {
                roles.insert(facilitator.clone(), EcosystemRole::Facilitator);
                for party in facilitated_parties {
                    roles.insert(party.clone(), EcosystemRole::Consumer);
                }
            }
            BiologicalRelationship::Protective { protector, protected_parties, .. } => {
                roles.insert(protector.clone(), EcosystemRole::Protector);
                for party in protected_parties {
                    roles.insert(party.clone(), EcosystemRole::Consumer);
                }
            }
            BiologicalRelationship::Competitive { competitors, .. } => {
                for competitor in competitors {
                    roles.insert(competitor.clone(), EcosystemRole::Innovator);
                }
            }
        }
        
        roles
    }
    
    /// Check if this relationship is sustainable
    pub fn is_sustainable(&self) -> bool {
        let health = self.calculate_health();
        health.sustainability_score > 0.6 && health.overall_health > 0.5
    }
    
    /// Get recommendations for relationship improvement
    pub fn improvement_recommendations(&self) -> Vec<String> {
        let health = self.calculate_health();
        let mut recommendations = Vec::new();
        
        if health.mutual_satisfaction < 0.7 {
            recommendations.push("Improve mutual benefit balance".to_string());
        }
        
        if health.sustainability_score < 0.6 {
            recommendations.push("Develop long-term sustainability strategies".to_string());
        }
        
        if health.ecosystem_contribution < 0.5 {
            recommendations.push("Increase positive ecosystem impact".to_string());
        }
        
        if health.resilience_factor < 0.6 {
            recommendations.push("Build relationship resilience mechanisms".to_string());
        }
        
        recommendations
    }
} 