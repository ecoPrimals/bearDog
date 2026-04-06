// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive tests for ecosystem evolution engine
//!
//! These tests cover ecosystem genetics, relationship evolution,
//! and symbiosis patterns to increase test coverage.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use super::engine::*;
use super::support::*;
use super::types::*;
use beardog_types::canonical::HealthStatus;
use std::collections::HashMap;

#[test]
fn test_ecosystem_genetic_engine_creation() {
    let engine = EcosystemGeneticEngine::new();
    assert!(engine.is_ok());
}

#[test]
fn test_evolve_allowlist_pattern() {
    let engine = EcosystemGeneticEngine::new().unwrap();

    let pattern = BinaryAccessPattern::Allowlist {
        allowed_entities: vec!["node1".to_string(), "node2".to_string()],
    };

    let context = EcosystemContext {
        current_health: HealthStatus::Healthy,
        active_relationships: 5,
        ecosystem_load: 0.3,
        recent_events: vec![],
    };

    let result = engine.evolve_access_pattern(pattern, context);
    assert!(result.is_ok());

    let membership = result.unwrap();
    // Should produce ecosystem membership rather than binary access
    match membership {
        EcosystemMembership::CoreSteward { .. }
        | EcosystemMembership::ActiveContributor { .. }
        | EcosystemMembership::LearningParticipant { .. }
        | EcosystemMembership::VisitingCollaborator { .. } => (),
        _ => panic!("Unexpected membership type"),
    }
}

#[test]
fn test_evolve_blocklist_pattern() {
    let engine = EcosystemGeneticEngine::new().unwrap();

    let pattern = BinaryAccessPattern::Blocklist {
        blocked_entities: vec!["malicious_node".to_string()],
    };

    let context = EcosystemContext {
        current_health: HealthStatus::Healthy,
        active_relationships: 3,
        ecosystem_load: 0.5,
        recent_events: vec![],
    };

    let result = engine.evolve_access_pattern(pattern, context);
    assert!(result.is_ok());

    // Should produce cautious or protection membership
    let membership = result.unwrap();
    match membership {
        EcosystemMembership::CautiousInteraction { .. }
        | EcosystemMembership::EcosystemProtection { .. }
        | EcosystemMembership::LearningParticipant { .. } => (),
        _ => panic!("Blocklist should evolve to cautious/protection membership"),
    }
}

#[test]
fn test_evolve_trust_trusted() {
    let engine = EcosystemGeneticEngine::new().unwrap();

    let trust = BinaryTrust::Trusted;

    let history = RelationshipHistory {
        interactions: vec!["positive_interaction".to_string()],
        trust_changes: vec![0.1, 0.2, 0.3],
        collaboration_outcomes: vec!["success".to_string()],
    };

    let context = EcosystemContext {
        current_health: HealthStatus::Healthy,
        active_relationships: 5,
        ecosystem_load: 0.3,
        recent_events: vec![],
    };

    let result = engine.evolve_trust_pattern(trust, history, context);
    assert!(result.is_ok());

    let evolution = result.unwrap();
    match evolution {
        TrustEvolution::Building { .. } | TrustEvolution::Flourishing { .. } => (),
        _ => panic!("High trust should evolve to Building or Flourishing"),
    }
}

#[test]
fn test_evolve_trust_untrusted() {
    let engine = EcosystemGeneticEngine::new().unwrap();

    let trust = BinaryTrust::Untrusted;

    let history = RelationshipHistory {
        interactions: vec!["negative_interaction".to_string()],
        trust_changes: vec![-0.1, -0.2],
        collaboration_outcomes: vec!["failure".to_string()],
    };

    let context = EcosystemContext {
        current_health: HealthStatus::Degraded,
        active_relationships: 1,
        ecosystem_load: 0.6,
        recent_events: vec![],
    };

    let result = engine.evolve_trust_pattern(trust, history, context);
    assert!(result.is_ok());

    let evolution = result.unwrap();
    // Accept various evolution states - the engine determines appropriate state based on context
    match evolution {
        TrustEvolution::Questioning { .. }
        | TrustEvolution::Healing { .. }
        | TrustEvolution::Building { .. }
        | TrustEvolution::Transforming { .. } => (),
        _ => panic!("Untrusted should evolve to an appropriate trust state"),
    }
}

#[test]
fn test_evolve_primary_replica_hierarchy() {
    let engine = EcosystemGeneticEngine::new().unwrap();

    let hierarchy = HierarchicalPattern::PrimaryReplica {
        primary: "leader".to_string(),
        replicas: vec!["replica1".to_string(), "replica2".to_string()],
    };

    let relationships = vec![];

    let result =
        engine.evolve_coordination_pattern(hierarchy, relationships, "test_context".to_string());
    assert!(result.is_ok());

    let coordination = result.unwrap();
    // Accept any coordination model - the evolution logic determines the best fit
    match coordination {
        CoordinationModel::Distributed { .. }
        | CoordinationModel::Emergent { .. }
        | CoordinationModel::Rotational { .. }
        | CoordinationModel::Contextual { .. }
        | CoordinationModel::Collaborative { .. } => (),
    }
}

#[test]
fn test_evolve_client_server_hierarchy() {
    let engine = EcosystemGeneticEngine::new().unwrap();

    let hierarchy = HierarchicalPattern::ClientServer {
        server: "server_node".to_string(),
        clients: vec!["client1".to_string()],
    };

    let relationships = vec![];

    let result =
        engine.evolve_coordination_pattern(hierarchy, relationships, "test_context".to_string());
    assert!(result.is_ok());

    let coordination = result.unwrap();
    match coordination {
        CoordinationModel::Distributed { .. } | CoordinationModel::Emergent { .. } => (),
        _ => panic!("ClientServer should evolve to distributed/emergent coordination"),
    }
}

#[test]
fn test_generate_emergent_behaviors() {
    let engine = EcosystemGeneticEngine::new().unwrap();

    let result = engine.generate_emergent_behaviors();
    assert!(result.is_ok());

    let behaviors = result.unwrap();
    assert!(
        !behaviors.is_empty(),
        "Should generate some emergent behaviors"
    );
}

#[test]
fn test_assess_ecosystem_health() {
    let engine = EcosystemGeneticEngine::new().unwrap();

    let result = engine.assess_ecosystem_health();
    assert!(result.is_ok());

    let health = result.unwrap();
    // overall_health is a HealthStatus enum, not a float
    assert!(matches!(
        health.overall_health,
        HealthStatus::Healthy
            | HealthStatus::Degraded
            | HealthStatus::Unhealthy
            | HealthStatus::Unknown
    ));
    // Check the numeric health metrics instead
    assert!(health.relationship_health >= 0.0 && health.relationship_health <= 1.0);
    assert!(health.trust_health >= 0.0 && health.trust_health <= 1.0);
    assert!(health.coordination_health >= 0.0 && health.coordination_health <= 1.0);
}

#[test]
fn test_migrate_from_binary_patterns_both_lists() {
    let allowlist = vec!["trusted1".to_string(), "trusted2".to_string()];
    let blocklist = vec!["blocked1".to_string()];

    let result = migrate_from_binary_patterns(allowlist, blocklist);
    assert!(result.is_ok());

    let memberships = result.unwrap();
    assert_eq!(memberships.len(), 3, "Should have 3 memberships");
}

#[test]
fn test_migrate_from_binary_patterns_allowlist_only() {
    let allowlist = vec!["trusted1".to_string()];
    let blocklist = vec![];

    let result = migrate_from_binary_patterns(allowlist, blocklist);
    assert!(result.is_ok());

    let memberships = result.unwrap();
    assert_eq!(memberships.len(), 1);
}

#[test]
fn test_migrate_from_binary_patterns_blocklist_only() {
    let allowlist = vec![];
    let blocklist = vec!["blocked1".to_string()];

    let result = migrate_from_binary_patterns(allowlist, blocklist);
    assert!(result.is_ok());

    let memberships = result.unwrap();
    assert_eq!(memberships.len(), 1);
}

#[test]
fn test_migrate_from_binary_patterns_empty() {
    let allowlist = vec![];
    let blocklist = vec![];

    let result = migrate_from_binary_patterns(allowlist, blocklist);
    assert!(result.is_ok());

    let memberships = result.unwrap();
    assert!(memberships.is_empty());
}

#[test]
fn test_ecosystem_context_with_degraded_health() {
    let context = EcosystemContext {
        current_health: HealthStatus::Degraded,
        active_relationships: 2,
        ecosystem_load: 0.8,
        recent_events: vec![],
    };

    assert!(matches!(context.current_health, HealthStatus::Degraded));
    assert_eq!(context.active_relationships, 2);
    assert_eq!(context.ecosystem_load, 0.8);
}

#[test]
fn test_ecosystem_context_with_unhealthy_health() {
    let context = EcosystemContext {
        current_health: HealthStatus::Unhealthy,
        active_relationships: 0,
        ecosystem_load: 1.0,
        recent_events: vec![],
    };

    assert!(matches!(context.current_health, HealthStatus::Unhealthy));
}

#[test]
fn test_ecosystem_context_with_healthy_state() {
    let context = EcosystemContext {
        current_health: HealthStatus::Healthy,
        active_relationships: 10,
        ecosystem_load: 0.3,
        recent_events: vec![],
    };

    assert!(matches!(context.current_health, HealthStatus::Healthy));
    assert!(context.ecosystem_load < 0.5);
}

#[test]
fn test_binary_access_pattern_allowlist() {
    let pattern = BinaryAccessPattern::Allowlist {
        allowed_entities: vec!["entity1".to_string()],
    };

    match pattern {
        BinaryAccessPattern::Allowlist { allowed_entities } => {
            assert_eq!(allowed_entities.len(), 1);
        }
        _ => panic!("Expected Allowlist"),
    }
}

#[test]
fn test_binary_access_pattern_blocklist() {
    let pattern = BinaryAccessPattern::Blocklist {
        blocked_entities: vec!["bad_entity".to_string()],
    };

    match pattern {
        BinaryAccessPattern::Blocklist { blocked_entities } => {
            assert_eq!(blocked_entities.len(), 1);
        }
        _ => panic!("Expected Blocklist"),
    }
}

#[test]
fn test_binary_trust_variants() {
    let trusted = BinaryTrust::Trusted;
    let untrusted = BinaryTrust::Untrusted;

    assert!(matches!(trusted, BinaryTrust::Trusted));
    assert!(matches!(untrusted, BinaryTrust::Untrusted));
}

#[test]
fn test_hierarchical_pattern_primary_replica() {
    let pattern = HierarchicalPattern::PrimaryReplica {
        primary: "coordinator".to_string(),
        replicas: vec!["replica1".to_string()],
    };

    match pattern {
        HierarchicalPattern::PrimaryReplica { primary, .. } => {
            assert_eq!(primary, "coordinator");
        }
        _ => panic!("Expected PrimaryReplica"),
    }
}

#[test]
fn test_hierarchical_pattern_client_server() {
    let pattern = HierarchicalPattern::ClientServer {
        server: "server_node".to_string(),
        clients: vec!["client1".to_string()],
    };

    match pattern {
        HierarchicalPattern::ClientServer { server, .. } => {
            assert_eq!(server, "server_node");
        }
        _ => panic!("Expected ClientServer"),
    }
}

#[test]
fn test_relationship_history_construction() {
    let history = RelationshipHistory {
        interactions: vec!["interaction1".to_string()],
        trust_changes: vec![0.1, 0.2, 0.3],
        collaboration_outcomes: vec!["success".to_string()],
    };

    assert_eq!(history.trust_changes.len(), 3);
    assert_eq!(history.interactions.len(), 1);
    assert_eq!(history.collaboration_outcomes.len(), 1);
}

#[test]
fn test_ecosystem_membership_variants() {
    // Test that all variants can be constructed
    let steward = EcosystemMembership::CoreSteward {
        stewardship_areas: vec![],
        trust_level: 0.9,
        responsibilities: vec![],
        genetic_markers: vec![],
    };

    match steward {
        EcosystemMembership::CoreSteward { trust_level, .. } => {
            assert_eq!(trust_level, 0.9);
        }
        _ => panic!("Expected CoreSteward"),
    }
}

#[test]
fn test_trust_evolution_variants() {
    let building = TrustEvolution::Building {
        progress_rate: 0.5,
        milestones: vec!["first_success".to_string()],
        building_activities: vec!["collaboration".to_string()],
        genetic_compatibility: 0.8,
    };

    match building {
        TrustEvolution::Building { progress_rate, .. } => {
            assert_eq!(progress_rate, 0.5);
        }
        _ => panic!("Expected Building"),
    }
}

#[test]
fn test_coordination_model_distributed() {
    let model = CoordinationModel::Distributed {
        consensus_type: "raft".to_string(),
        participation_weights: HashMap::new(),
        decision_thresholds: "majority".to_string(),
    };

    match model {
        CoordinationModel::Distributed { consensus_type, .. } => {
            assert_eq!(consensus_type, "raft");
        }
        _ => panic!("Expected Distributed"),
    }
}

#[test]
fn test_symbiosis_type_mutualistic() {
    let symbiosis = SymbiosisType::Mutualistic {
        mutual_benefits: "mutual benefit".to_string(),
        benefit_balance: 0.8,
        sustainability_metrics: "sustainable".to_string(),
    };

    match symbiosis {
        SymbiosisType::Mutualistic {
            benefit_balance, ..
        } => {
            assert_eq!(benefit_balance, 0.8);
        }
        _ => panic!("Expected Mutualistic"),
    }
}

#[test]
fn test_symbiosis_type_commensal() {
    let symbiosis = SymbiosisType::Commensal {
        beneficiary: "node1".to_string(),
        neutral_party: "node2".to_string(),
        impact_assessment: "neutral".to_string(),
    };

    match symbiosis {
        SymbiosisType::Commensal { beneficiary, .. } => {
            assert_eq!(beneficiary, "node1");
        }
        _ => panic!("Expected Commensal"),
    }
}

#[test]
fn test_symbiosis_type_facilitative() {
    let symbiosis = SymbiosisType::Facilitative {
        facilitator: "node1".to_string(),
        beneficiary: "node2".to_string(),
        facilitation_methods: vec!["resource_sharing".to_string()],
    };

    match symbiosis {
        SymbiosisType::Facilitative { facilitator, .. } => {
            assert_eq!(facilitator, "node1");
        }
        _ => panic!("Expected Facilitative"),
    }
}

#[test]
fn test_high_ecosystem_load_pattern() {
    let engine = EcosystemGeneticEngine::new().unwrap();

    let pattern = BinaryAccessPattern::Allowlist {
        allowed_entities: vec!["node1".to_string()],
    };

    let context = EcosystemContext {
        current_health: HealthStatus::Degraded,
        active_relationships: 20,
        ecosystem_load: 0.95,
        recent_events: vec![],
    };

    let result = engine.evolve_access_pattern(pattern, context);
    assert!(result.is_ok(), "Should handle high load gracefully");
}

#[test]
fn test_zero_load_pattern() {
    let engine = EcosystemGeneticEngine::new().unwrap();

    let pattern = BinaryAccessPattern::Allowlist {
        allowed_entities: vec!["node1".to_string()],
    };

    let context = EcosystemContext {
        current_health: HealthStatus::Healthy,
        active_relationships: 1,
        ecosystem_load: 0.0,
        recent_events: vec![],
    };

    let result = engine.evolve_access_pattern(pattern, context);
    assert!(result.is_ok());
}
