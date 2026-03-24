// SPDX-License-Identifier: AGPL-3.0-only

// Ecosystem Membership - Spectrum-based Access Control
//
// This module implements the EcosystemMembership pattern from the parent reference,
// replacing binary allowlist/blocklist patterns with a nuanced spectrum of 
// relationship levels that reflect biological ecosystem dynamics.

use crate::constants::time;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

/// Ecosystem membership levels - replaces binary allowlist/blocklist patterns
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EcosystemMembership {
    /// Trusted ecosystem maintainer with stewardship responsibilities
    CoreSteward {
        stewardship_areas: Vec<StewardshipArea>,
        trust_level: f64, // 0.8-1.0
        responsibilities: Vec<String>,
        granted_at: SystemTime,
    },
    
    /// Regular positive participant in the ecosystem
    ActiveContributor {
        contribution_types: Vec<ContributionType>,
        trust_level: f64, // 0.6-0.9
        interaction_history: InteractionSummary,
        contribution_score: f64,
    },
    
    /// Growing understanding, learning from the ecosystem
    LearningParticipant {
        learning_path: LearningPath,
        trust_level: f64, // 0.4-0.7
        mentorship_connections: Vec<MentorConnection>,
        learning_progress: f64,
    },
    
    /// Temporary partnership for specific collaboration
    VisitingCollaborator {
        collaboration_scope: CollaborationScope,
        visit_duration: Duration,
        trust_level: f64, // 0.5-0.8
        collaboration_terms: CollaborationTerms,
    },
    
    /// Requires careful monitoring due to concerning patterns
    CautiousInteraction {
        concern_factors: Vec<ConcernFactor>,
        monitoring_level: MonitoringLevel,
        trust_level: f64, // 0.2-0.5
        review_schedule: ReviewSchedule,
    },
    
    /// Temporarily restricted to protect ecosystem health
    EcosystemProtection {
        protection_reason: ProtectionReason,
        protection_level: ProtectionLevel,
        trust_level: f64, // 0.0-0.3
        restoration_path: Option<RestorationPath>,
    },
}

/// Areas of ecosystem stewardship responsibility
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StewardshipArea {
    SecurityMonitoring,
    ResourceAllocation,
    CommunitySupport,
    TechnicalMaintenance,
    EcosystemHealth,
    ConflictResolution,
    KnowledgePreservation,
}

/// Types of ecosystem contributions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContributionType {
    CodeContribution,
    DocumentationImprovement,
    CommunitySupport,
    BugReporting,
    FeatureSuggestions,
    EcosystemTesting,
    SecurityAuditing,
    PerformanceOptimization,
}

/// Summary of interaction patterns
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InteractionSummary {
    pub total_interactions: u64,
    pub positive_interactions: u64,
    pub neutral_interactions: u64,
    pub concerning_interactions: u64,
    pub last_interaction: SystemTime,
    pub interaction_trend: InteractionTrend,
}

/// Trend in interaction patterns
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InteractionTrend {
    StronglyPositive,
    Positive,
    Stable,
    Concerning,
    Deteriorating,
}

/// Learning path for participants
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LearningPath {
    pub current_focus: Vec<LearningArea>,
    pub completed_milestones: Vec<String>,
    pub next_objectives: Vec<String>,
    pub estimated_completion: Option<SystemTime>,
}

/// Areas of ecosystem learning
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LearningArea {
    EcosystemProtocols,
    SecurityPrinciples,
    CommunityGuidelines,
    TechnicalSkills,
    CollaborationPatterns,
}

/// Mentorship connection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MentorConnection {
    pub mentor_id: String,
    pub mentorship_areas: Vec<LearningArea>,
    pub established_at: SystemTime,
    pub interaction_frequency: Duration,
}

/// Scope of collaboration for visitors
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollaborationScope {
    pub project_areas: Vec<String>,
    pub access_permissions: Vec<Permission>,
    pub duration_limit: Duration,
    pub resource_limits: ResourceLimits,
}

/// Collaboration terms and agreements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollaborationTerms {
    pub agreements: Vec<String>,
    pub deliverables: Vec<String>,
    pub success_criteria: Vec<String>,
    pub exit_conditions: Vec<String>,
}

/// Permission types for ecosystem access
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Permission {
    ReadAccess(Vec<String>),
    WriteAccess(Vec<String>),
    ExecuteAccess(Vec<String>),
    AdminAccess(Vec<String>),
}

/// Resource usage limits
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub cpu_limit: Option<f64>,
    pub memory_limit: Option<u64>,
    pub storage_limit: Option<u64>,
    pub network_limit: Option<u64>,
}

/// Factors causing concern about an entity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConcernFactor {
    RepeatedPolicyViolations,
    SuspiciousActivityPatterns,
    ResourceAbuse,
    CommunityDisruption,
    SecurityRiskBehavior,
    UnresponsiveToGuidance,
}

/// Level of monitoring for concerning entities
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MonitoringLevel {
    Light,      // Basic activity tracking
    Moderate,   // Enhanced monitoring with alerts
    Intensive,  // Comprehensive monitoring and logging
    Critical,   // Real-time monitoring with immediate response
}

/// Schedule for reviewing concerning entities
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviewSchedule {
    pub review_frequency: Duration,
    pub next_review: SystemTime,
    pub escalation_triggers: Vec<String>,
}

/// Reason for ecosystem protection measures
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProtectionReason {
    SecurityThreat,
    ResourceAttack,
    CommunityHarm,
    SystemAbuse,
    PolicyViolation,
    LegalCompliance,
}

/// Level of protection applied
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProtectionLevel {
    /// Limited access to non-critical resources
    Restricted,
    /// Access only to public, read-only resources
    ReadOnly,
    /// Complete isolation from ecosystem
    Quarantined,
    /// Permanent exclusion from ecosystem
    Excluded,
}

/// Path for restoring ecosystem access
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RestorationPath {
    pub required_actions: Vec<String>,
    pub timeline: Duration,
    pub verification_steps: Vec<String>,
    pub support_available: Vec<String>,
}

/// Registry for managing ecosystem memberships
#[derive(Debug, Clone)]
pub struct MembershipRegistry {
    memberships: HashMap<String, EcosystemMembership>,
    transitions: Vec<MembershipTransition>,
    /// Configuration for membership management behavior
    config: MembershipConfig,
}

/// Configuration for membership management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipConfig {
    pub auto_promotion_enabled: bool,
    pub trust_decay_rate: f64,
    pub review_intervals: HashMap<String, Duration>,
    pub escalation_thresholds: HashMap<String, f64>,
}

/// Record of membership level transitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipTransition {
    pub entity_id: String,
    pub from_membership: EcosystemMembership,
    pub to_membership: EcosystemMembership,
    pub transition_reason: String,
    pub timestamp: SystemTime,
    pub approved_by: Option<String>,
}

impl EcosystemMembership {
    /// Get the trust level for this membership
    pub fn trust_level(&self) -> f64 {
        match self {
            EcosystemMembership::CoreSteward { trust_level, .. } => *trust_level,
            EcosystemMembership::ActiveContributor { trust_level, .. } => *trust_level,
            EcosystemMembership::LearningParticipant { trust_level, .. } => *trust_level,
            EcosystemMembership::VisitingCollaborator { trust_level, .. } => *trust_level,
            EcosystemMembership::CautiousInteraction { trust_level, .. } => *trust_level,
            EcosystemMembership::EcosystemProtection { trust_level, .. } => *trust_level,
        }
    }
    
    /// Check if this membership allows a specific permission
    pub fn allows_permission(&self, permission: &Permission) -> bool {
        match self {
            EcosystemMembership::CoreSteward { .. } => true, // Stewards have all permissions
            EcosystemMembership::ActiveContributor { .. } => {
                // Contributors have most permissions except admin
                !matches!(permission, Permission::AdminAccess(_))
            }
            EcosystemMembership::LearningParticipant { .. } => {
                // Learners have read and limited write access
                matches!(permission, Permission::ReadAccess(_) | Permission::WriteAccess(_))
            }
            EcosystemMembership::VisitingCollaborator { collaboration_scope, .. } => {
                collaboration_scope.access_permissions.contains(permission)
            }
            EcosystemMembership::CautiousInteraction { .. } => {
                // Cautious entities have limited read access only
                matches!(permission, Permission::ReadAccess(_))
            }
            EcosystemMembership::EcosystemProtection { protection_level, .. } => {
                match protection_level {
                    ProtectionLevel::Restricted => matches!(permission, Permission::ReadAccess(_)),
                    ProtectionLevel::ReadOnly => matches!(permission, Permission::ReadAccess(_)),
                    ProtectionLevel::Quarantined | ProtectionLevel::Excluded => false,
                }
            }
        }
    }
    
    /// Get the display name for this membership level
    pub fn display_name(&self) -> &'static str {
        match self {
            EcosystemMembership::CoreSteward { .. } => "Core Steward",
            EcosystemMembership::ActiveContributor { .. } => "Active Contributor",
            EcosystemMembership::LearningParticipant { .. } => "Learning Participant",
            EcosystemMembership::VisitingCollaborator { .. } => "Visiting Collaborator",
            EcosystemMembership::CautiousInteraction { .. } => "Cautious Interaction",
            EcosystemMembership::EcosystemProtection { .. } => "Ecosystem Protection",
        }
    }
}

impl MembershipRegistry {
    /// Create a new membership registry
    pub fn new(config: MembershipConfig) -> Self {
        Self {
            memberships: HashMap::new(),
            transitions: Vec::new(),
            config,
        }
    }
    
    /// Register a new membership
    pub fn register_membership(
        &mut self,
        entity_id: String,
        membership: EcosystemMembership,
    ) -> Result<()> {
        // Validate membership based on configuration
        if !self.config.auto_promotion_enabled {
            match membership {
                EcosystemMembership::CoreSteward { .. } => {
                    return Err(BearDogError::business("Core steward membership requires manual approval".to_string()));
                }
                _ => {} // Other memberships are allowed
            }
        }
        
        self.memberships.insert(entity_id, membership);
        Ok(())
    }
    
    /// Get membership for an entity
    pub fn get_membership(&self, entity_id: &str) -> Option<&EcosystemMembership> {
        self.memberships.get(entity_id)
    }
    
    /// Transition an entity to a new membership level
    pub fn transition_membership(
        &mut self,
        entity_id: String,
        new_membership: EcosystemMembership,
        reason: String,
        approved_by: Option<String>,
    ) -> Result<()> {
        let old_membership = self.memberships.get(&entity_id)
            .ok_or_else(|| BearDogError::business("Entity not found in registry".to_string()))?
            .clone();
        
        let transition = MembershipTransition {
            entity_id: entity_id.clone(),
            from_membership: old_membership,
            to_membership: new_membership.clone(),
            transition_reason: reason,
            timestamp: SystemTime::now(),
            approved_by,
        };
        
        self.memberships.insert(entity_id, new_membership);
        self.transitions.push(transition);
        
        Ok(())
    }
}

impl Default for MembershipConfig {
    fn default() -> Self {
        let mut review_intervals = HashMap::new();
        review_intervals.insert(
            "CautiousInteraction".to_string(),
            Duration::from_secs(time::SECONDS_PER_DAY),
        ); // Daily
        review_intervals.insert("EcosystemProtection".to_string(), Duration::from_secs(604800)); // Weekly
        
        let mut escalation_thresholds = HashMap::new();
        escalation_thresholds.insert("trust_decay".to_string(), 0.3);
        escalation_thresholds.insert("violation_count".to_string(), 3.0);
        
        Self {
            auto_promotion_enabled: true,
            trust_decay_rate: 0.01,
            review_intervals,
            escalation_thresholds,
        }
    }
} 