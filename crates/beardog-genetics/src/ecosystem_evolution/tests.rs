// SPDX-License-Identifier: AGPL-3.0-only

//! Tests for ecosystem evolution genetics
//!
//! Comprehensive test suite covering all ecosystem evolution functionality:
//! engine initialization, pattern evolution, migration utilities, and defaults.

use super::engine::{migrate_from_binary_patterns, EcosystemGeneticEngine};
use super::engine::{AdaptiveSecurity, EvolutionAlgorithms, TraitInheritance};
use super::support::*;
use super::types::{CoordinationModel, EcosystemMembership, TrustEvolution};
use beardog_types::canonical::HealthStatus;

// ============================================================================
// EcosystemGeneticEngine Tests
// ============================================================================

#[test]
fn test_engine_initialization() {
    let engine = EcosystemGeneticEngine::new();
    assert!(engine.is_ok());

    let engine = engine.unwrap();

    // Verify all components are initialized
    assert!(matches!(
        engine.evolution_algorithms,
        EvolutionAlgorithms { .. }
    ));
    assert!(matches!(engine.trait_inheritance, TraitInheritance { .. }));
    assert!(matches!(engine.adaptive_security, AdaptiveSecurity { .. }));
}

#[test]
fn test_evolve_access_pattern_allowlist() {
    let engine = EcosystemGeneticEngine::new().unwrap();
    let pattern = BinaryAccessPattern::Allowlist {
        allowed_entities: vec!["entity1".to_string(), "entity2".to_string()],
    };
    let context = EcosystemContext {
        current_health: HealthStatus::Healthy,
        active_relationships: 5,
        ecosystem_load: 0.5,
        recent_events: vec![],
    };

    let result = engine.evolve_access_pattern(pattern, context);
    assert!(result.is_ok());

    let membership = result.unwrap();
    match membership {
        EcosystemMembership::ActiveContributor {
            trust_level,
            contribution_types,
            ..
        } => {
            assert_eq!(trust_level, 0.7);
            assert!(!contribution_types.is_empty());
        }
        _ => panic!("Expected ActiveContributor membership"),
    }
}

#[test]
fn test_evolve_access_pattern_blocklist() {
    let engine = EcosystemGeneticEngine::new().unwrap();
    let pattern = BinaryAccessPattern::Blocklist {
        blocked_entities: vec!["entity1".to_string()],
    };
    let context = EcosystemContext {
        current_health: HealthStatus::Healthy,
        active_relationships: 5,
        ecosystem_load: 0.5,
        recent_events: vec![],
    };

    let result = engine.evolve_access_pattern(pattern, context);
    assert!(result.is_ok());

    let membership = result.unwrap();
    match membership {
        EcosystemMembership::EcosystemProtection {
            trust_level,
            restoration_path,
            ..
        } => {
            assert_eq!(trust_level, 0.1);
            assert!(restoration_path.is_some());
        }
        _ => panic!("Expected EcosystemProtection membership"),
    }
}

#[test]
fn test_evolve_access_pattern_simple_allowed() {
    let engine = EcosystemGeneticEngine::new().unwrap();
    let pattern = BinaryAccessPattern::Simple { is_allowed: true };
    let context = EcosystemContext {
        current_health: HealthStatus::Healthy,
        active_relationships: 5,
        ecosystem_load: 0.5,
        recent_events: vec![],
    };

    let result = engine.evolve_access_pattern(pattern, context);
    assert!(result.is_ok());

    let membership = result.unwrap();
    match membership {
        EcosystemMembership::LearningParticipant { trust_level, .. } => {
            assert_eq!(trust_level, 0.5);
        }
        _ => panic!("Expected LearningParticipant membership"),
    }
}

#[test]
fn test_evolve_access_pattern_simple_denied() {
    let engine = EcosystemGeneticEngine::new().unwrap();
    let pattern = BinaryAccessPattern::Simple { is_allowed: false };
    let context = EcosystemContext {
        current_health: HealthStatus::Healthy,
        active_relationships: 5,
        ecosystem_load: 0.5,
        recent_events: vec![],
    };

    let result = engine.evolve_access_pattern(pattern, context);
    assert!(result.is_ok());

    let membership = result.unwrap();
    match membership {
        EcosystemMembership::CautiousInteraction { trust_level, .. } => {
            assert_eq!(trust_level, 0.3);
        }
        _ => panic!("Expected CautiousInteraction membership"),
    }
}

#[test]
fn test_evolve_trust_pattern_trusted() {
    let engine = EcosystemGeneticEngine::new().unwrap();
    let trust = BinaryTrust::Trusted;
    let history = RelationshipHistory {
        interactions: vec![],
        trust_changes: vec![],
        collaboration_outcomes: vec![],
    };
    let context = EcosystemContext {
        current_health: HealthStatus::Healthy,
        active_relationships: 5,
        ecosystem_load: 0.5,
        recent_events: vec![],
    };

    let result = engine.evolve_trust_pattern(trust, history, context);
    assert!(result.is_ok());

    let evolution = result.unwrap();
    match evolution {
        TrustEvolution::Flourishing {
            stability_metrics, ..
        } => {
            assert_eq!(stability_metrics, "high");
        }
        _ => panic!("Expected Flourishing trust evolution"),
    }
}

#[test]
fn test_evolve_trust_pattern_untrusted() {
    let engine = EcosystemGeneticEngine::new().unwrap();
    let trust = BinaryTrust::Untrusted;
    let history = RelationshipHistory {
        interactions: vec![],
        trust_changes: vec![],
        collaboration_outcomes: vec![],
    };
    let context = EcosystemContext {
        current_health: HealthStatus::Healthy,
        active_relationships: 5,
        ecosystem_load: 0.5,
        recent_events: vec![],
    };

    let result = engine.evolve_trust_pattern(trust, history, context);
    assert!(result.is_ok());

    let evolution = result.unwrap();
    match evolution {
        TrustEvolution::Building { progress_rate, .. } => {
            assert_eq!(progress_rate, 0.1);
        }
        _ => panic!("Expected Building trust evolution"),
    }
}

#[test]
fn test_evolve_coordination_pattern_primary_replica() {
    let engine = EcosystemGeneticEngine::new().unwrap();
    let hierarchy = HierarchicalPattern::PrimaryReplica {
        primary: "node1".to_string(),
        replicas: vec!["node2".to_string(), "node3".to_string()],
    };

    let result = engine.evolve_coordination_pattern(hierarchy, vec![], "test".to_string());
    assert!(result.is_ok());

    let model = result.unwrap();
    match model {
        CoordinationModel::Collaborative {
            decision_protocol, ..
        } => {
            assert!(decision_protocol.contains("collaborative"));
            assert!(decision_protocol.contains("node1"));
        }
        _ => panic!("Expected Collaborative coordination model"),
    }
}

#[test]
fn test_evolve_coordination_pattern_client_server() {
    let engine = EcosystemGeneticEngine::new().unwrap();
    let hierarchy = HierarchicalPattern::ClientServer {
        server: "server1".to_string(),
        clients: vec!["client1".to_string(), "client2".to_string()],
    };

    let result = engine.evolve_coordination_pattern(hierarchy, vec![], "test".to_string());
    assert!(result.is_ok());

    let model = result.unwrap();
    match model {
        CoordinationModel::Distributed {
            consensus_type,
            participation_weights,
            ..
        } => {
            assert_eq!(consensus_type, "weighted_voting");
            assert!(!participation_weights.is_empty());
            assert!(participation_weights.contains_key("server1"));
        }
        _ => panic!("Expected Distributed coordination model"),
    }
}

#[test]
fn test_evolve_coordination_pattern_primary_secondary() {
    let engine = EcosystemGeneticEngine::new().unwrap();
    let hierarchy = HierarchicalPattern::PrimarySecondary {
        primary: "primary1".to_string(),
        secondaries: vec!["secondary1".to_string()],
    };

    let result = engine.evolve_coordination_pattern(hierarchy, vec![], "test".to_string());
    assert!(result.is_ok());

    let model = result.unwrap();
    match model {
        CoordinationModel::Rotational { current_leader, .. } => {
            assert_eq!(current_leader, Some("primary1".to_string()));
        }
        _ => panic!("Expected Rotational coordination model"),
    }
}

#[test]
fn test_generate_emergent_behaviors() {
    let engine = EcosystemGeneticEngine::new().unwrap();
    let result = engine.generate_emergent_behaviors();

    assert!(result.is_ok());
    let behaviors = result.unwrap();
    assert!(!behaviors.is_empty());
    assert!(behaviors.iter().all(|b| b.beneficial));
}

#[test]
fn test_assess_ecosystem_health() {
    let engine = EcosystemGeneticEngine::new().unwrap();
    let result = engine.assess_ecosystem_health();

    assert!(result.is_ok());
    let report = result.unwrap();
    assert_eq!(report.overall_health, HealthStatus::Healthy);
    assert_eq!(report.relationship_health, 0.8);
    assert_eq!(report.trust_health, 0.8);
    assert_eq!(report.coordination_health, 0.8);
    assert!(!report.recommendations.is_empty());
}

// ============================================================================
// migrate_from_binary_patterns Tests
// ============================================================================

#[test]
fn test_migrate_empty_lists() {
    let result = migrate_from_binary_patterns(vec![], vec![]);
    assert!(result.is_ok());
    let memberships = result.unwrap();
    assert_eq!(memberships.len(), 0);
}

#[test]
fn test_migrate_allowlist_only() {
    let allowlist = vec!["entity1".to_string(), "entity2".to_string()];
    let result = migrate_from_binary_patterns(allowlist, vec![]);

    assert!(result.is_ok());
    let memberships = result.unwrap();
    assert_eq!(memberships.len(), 2);

    for membership in memberships {
        match membership {
            EcosystemMembership::ActiveContributor { trust_level, .. } => {
                assert_eq!(trust_level, 0.7);
            }
            _ => panic!("Expected ActiveContributor membership"),
        }
    }
}

#[test]
fn test_migrate_blocklist_only() {
    let blocklist = vec!["entity1".to_string()];
    let result = migrate_from_binary_patterns(vec![], blocklist);

    assert!(result.is_ok());
    let memberships = result.unwrap();
    assert_eq!(memberships.len(), 1);

    match &memberships[0] {
        EcosystemMembership::EcosystemProtection {
            trust_level,
            restoration_path,
            ..
        } => {
            assert_eq!(*trust_level, 0.1);
            assert!(restoration_path.is_some());
        }
        _ => panic!("Expected EcosystemProtection membership"),
    }
}

#[test]
fn test_migrate_both_lists() {
    let allowlist = vec!["allowed1".to_string()];
    let blocklist = vec!["blocked1".to_string()];
    let result = migrate_from_binary_patterns(allowlist, blocklist);

    assert!(result.is_ok());
    let memberships = result.unwrap();
    assert_eq!(memberships.len(), 2);

    // First should be ActiveContributor, second should be EcosystemProtection
    assert!(matches!(
        memberships[0],
        EcosystemMembership::ActiveContributor { .. }
    ));
    assert!(matches!(
        memberships[1],
        EcosystemMembership::EcosystemProtection { .. }
    ));
}

#[test]
fn test_migrate_large_lists() {
    let allowlist: Vec<String> = (0..100).map(|i| format!("allowed{i}")).collect();
    let blocklist: Vec<String> = (0..50).map(|i| format!("blocked{i}")).collect();

    let result = migrate_from_binary_patterns(allowlist, blocklist);
    assert!(result.is_ok());
    let memberships = result.unwrap();
    assert_eq!(memberships.len(), 150);
}

// ============================================================================
// Default Implementation Tests
// ============================================================================

#[test]
fn test_evolution_algorithms_default() {
    let algorithms = EvolutionAlgorithms::default();
    assert!(!algorithms.genetic_operators.is_empty());
    assert!(!algorithms.fitness_functions.is_empty());
    assert!(!algorithms.selection_methods.is_empty());
    assert_eq!(algorithms.genetic_operators.len(), 2);
    assert_eq!(algorithms.fitness_functions.len(), 2);
    assert_eq!(algorithms.selection_methods.len(), 2);
}

#[test]
fn test_trait_inheritance_default() {
    let inheritance = TraitInheritance::default();
    assert!(!inheritance.inheritance_patterns.is_empty());
    assert!(!inheritance.expression_rules.is_empty());
    assert_eq!(inheritance.inheritance_patterns.len(), 2);
    assert_eq!(inheritance.expression_rules.len(), 1);
}

#[test]
fn test_adaptive_security_default() {
    let security = AdaptiveSecurity::default();
    assert!(!security.security_adaptations.is_empty());
    assert!(!security.learning_mechanisms.is_empty());
    assert!(!security.adaptation_triggers.is_empty());
    assert_eq!(security.security_adaptations.len(), 2);
    assert_eq!(security.learning_mechanisms.len(), 2);
    assert_eq!(security.adaptation_triggers.len(), 1);
}
