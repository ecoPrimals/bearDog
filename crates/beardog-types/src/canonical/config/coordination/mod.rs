// SPDX-License-Identifier: AGPL-3.0-only

//! Coordination Configuration - Evolved from Primary/Replica Patterns
//!
//! This module implements the evolved coordination configuration system that replaces
//! traditional primary/replica hierarchical patterns with symbiotic coordination models
//! based on biological ecosystem principles.
//!
//! ## Evolution from Binary to Spectrum
//!
//! **OLD**: Primary/Replica binary hierarchy  
//! **NEW**: Symbiotic coordination spectrum with contextual authority
//!
//! This evolution enables:
//! - Distributed decision making
//! - Rotational leadership based on expertise
//! - Contextual authority delegation
//! - Collaborative mutual cooperation
//! - Emergent natural leadership
//!
//! ## Horizontal Gene Transfer Integration
//!
//! This configuration integrates genetic material from cross-primal
//! ecosystem evolution initiative, enabling biological coordination patterns.

mod collaboration;
mod config_impl;
mod consensus_rotation;
mod context_authority;
mod ecosystem;
mod model;
mod supporting;
mod transition_health;

pub use collaboration::{
    AccountabilityAgreement, AdvancementRequirement, CollaborationFramework, CommunicationProtocol,
    DecisionProtocol, DecisionStep, MutualAccountability, ResponseRequirements, Role,
    SuccessMetric,
};
pub use config_impl::{assess_coordination_health, migrate_from_primary_replica};
pub use consensus_rotation::{
    ConsensusType, DecisionThresholds, ExpertiseEntry, ExpertiseMapping, LeaderInfo,
    RotationCriteria, RotationSchedule,
};
pub use context_authority::{
    AuthorityDelegation, ContextAssessment, ContextEvaluation, ContextFactor, Delegation,
    DelegationRule, DelegationStatus,
};
pub use ecosystem::EcosystemIntegrationConfig;
pub use model::{CoordinationConfig, CoordinationModel};
pub use supporting::{
    AdaptiveHierarchy, CollaborationSession, ContextChange, DelegationEvent, EmergenceEvent,
    EmergenceFactor, ExpertiseRotation, FeedbackSystem, MonitoringMechanism, NaturalLeadership,
    PerformanceThresholds, RollbackStrategy, ValidationCheck, WorkloadThresholds,
};
pub use transition_health::{
    CoordinationHealthConfig, CoordinationTransitionConfig, HealthMetric, HealthTrend,
    TransitionAction, TransitionStep, TransitionStrategy, TransitionTrigger, TriggerCondition,
};

#[cfg(test)]
#[path = "coordination_tests.rs"]
mod tests;
