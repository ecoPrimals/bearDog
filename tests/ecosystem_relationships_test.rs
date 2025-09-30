//! Tests for Ecosystem Relationships Module
//!
//! This test suite validates the spectrum-based relationship patterns
//! that replace binary access control with biological ecosystem modeling.

use beardog_types::ecosystem_relationships::*;
use std::time::Duration;

#[tokio::test]
async fn test_ecosystem_membership_spectrum() {
    // Test CoreSteward - highest trust level
    let steward = EcosystemMembership::CoreSteward { stewardship_areas: vec!["security" .to_string(),"architecture".to_string()],
        trust_level: 0.95 };

    assert!(steward.allows_operation("read"));
    assert!(steward.allows_operation("write"));
    assert!(steward.allows_operation("admin"));
    assert_eq!(steward.trust_level(), 0.95);

    // Test ActiveContributor - established participant
    let contributor = EcosystemMembership::ActiveContributor { contribution_types: vec!["code".to_string(), "documentation".to_string()],
        contribution_score: 0.8 };

    assert!(contributor.allows_operation("code"));
    assert!(!contributor.allows_operation("admin"));
    assert_eq!(contributor.trust_level(), 0.64); // 0.8 * 0.8

    // Test LearningParticipant - growing member
    let learner = EcosystemMembership::LearningParticipant { learning_path: LearningPath {
            current_stage: "beginner".to_string(),
            next_milestones: vec!["first_contribution".to_string()],
            completion_percentage: 25.0 },
        progress_metrics: ProgressMetrics { skills_acquired: vec!["rust_basics".to_string()],
            contributions_made: 2,
            community_feedback_score: 0.7 },
    };

    assert!(learner.allows_operation("read"));
    assert!(learner.allows_operation("learn"));
    assert!(!learner.allows_operation("write"));
    assert_eq!(learner.trust_level(), 0.42); // 0.7 * 0.6
}

#[tokio::test]
async fn test_trust_evolution_states() {
    // Test Building trust
    let building_trust = TrustEvolution::Building { progress_rate: 0.5,
        milestone_achievements: vec!["first_commit".to_string()],
        confidence_indicators: vec![TrustIndicator {
            indicator_type: "consistency".to_string(),
            value: 0.7,
            trend: TrendDirection::Improving }],
    };

    assert_eq!(building_trust.trust_score(), 0.5); // 0.3 + (0.5 * 0.4)

    // Test Flourishing trust
    let flourishing_trust = TrustEvolution::Flourishing { stability_metrics: StabilityProfile {
            consistency_score: 0.9,
            reliability_metrics: [("uptime".to_string(), 0.99)].into(),
            predictability_index: 0.85 },
        mutual_benefit_indicators: vec![BenefitMetric { metric_name: "code_quality".to_string(),
            value: 0.8,
            beneficiary: "ecosystem".to_string() }],
    };

    assert_eq!(flourishing_trust.trust_score(), 0.97); // 0.7 + (0.9 * 0.3)

    // Test Healing trust
    let healing_trust = TrustEvolution::Healing { recovery_progress: 0.6,
        rebuilding_milestones: vec![Milestone {
            name: "apology_accepted".to_string(),
            description: "Community accepted the apology".to_string(),
            completion_criteria: vec!["positive_feedback".to_string()] }],
        trust_repair_actions: vec![RepairAction { action_type: "community_service".to_string(),
            description: "Contributing to documentation".to_string(),
            responsible_party: "participant".to_string() }],
    };

    assert_eq!(healing_trust.trust_score(), 0.5); // 0.2 + (0.6 * 0.5)
}

#[tokio::test]
async fn test_coordination_models() {
    // Test Distributed coordination
    let distributed = CoordinationModel::Distributed(ConsensusType::Majority);
    assert_eq!(
        distributed,
        CoordinationModel::Distributed(ConsensusType::Majority)
    );

    // Test Rotational coordination
    let rotational = CoordinationModel::Rotational(RotationCriteria { rotation_period: Duration::from_secs(3600), // 1 hour
        expertise_requirements: vec!["rust".to_string(), "security".to_string()],
        selection_method: "merit_based".to_string() });

    match rotational {
        CoordinationModel::Rotational(criteria) => {
            assert_eq!(criteria.expertise_requirements.len(), 2);
            assert_eq!(criteria.selection_method, "merit_based");
        }
        _ => panic!("Expected Rotational coordination model"),
    }

    // Test Collaborative coordination
    let collaborative = CoordinationModel::Collaborative(DecisionProtocol { decision_method: "consensus".to_string(),
        required_participants: vec!["steward".to_string(), "contributors".to_string()],
        conflict_resolution: "mediation".to_string() });

    match collaborative {
        CoordinationModel::Collaborative(protocol) => {
            assert_eq!(protocol.decision_method, "consensus");
            assert_eq!(protocol.required_participants.len(), 2);
        }
        _ => panic!("Expected Collaborative coordination model"),
    }
}

#[tokio::test]
async fn test_symbiosis_types() {
    let symbiosis_types = vec![
        SymbiosisType::Mutualistic,  // Both benefit
        SymbiosisType::Commensal,    // One benefits, other neutral
        SymbiosisType::Facilitative, // One helps other thrive
        SymbiosisType::Protective,   // One provides security
        SymbiosisType::Competitive,  // Healthy competition
   ];

    assert_eq!(symbiosis_types.len(), 5);
    assert_eq!(symbiosis_types[0], SymbiosisType::Mutualistic);
    assert_eq!(symbiosis_types[4], SymbiosisType::Competitive);
}

#[tokio::test]
async fn test_ecosystem_protection_with_restoration() {
    let protection = EcosystemMembership::EcosystemProtection { protection_level: ProtectionLevel::LimitedInteraction,
        restoration_path: Some(RestorationStrategy {
            required_actions: vec![
                 "acknowledge_violation".to_string(),
                 "complete_training".to_string(),
                 "demonstrate_improvement".to_string(),
           ],
            timeline_weeks: 4,
            success_criteria: vec![
                 "zero_violations_for_30_days".to_string(),
                 "positive_community_feedback".to_string(),
           ] }),
    };

    assert!(!protection.allows_operation("write"));
    assert!(!protection.allows_operation("read"));
    assert_eq!(protection.trust_level(), 0.0);

    match protection {
        EcosystemMembership::EcosystemProtection {
            restoration_path, ..
        } => {
            assert!(restoration_path.is_some());
            let strategy = restoration_path.unwrap();
            assert_eq!(strategy.required_actions.len(), 3);
            assert_eq!(strategy.timeline_weeks, 4);
        }
        _ => panic!( "Expected"EcosystemProtection),
    }
}

#[tokio::test]
async fn test_concern_factors_and_monitoring() {
    let cautious_interaction = EcosystemMembership::CautiousInteraction { concern_factors: vec![
            ConcernFactor::SecurityIncident,
            ConcernFactor::PolicyViolation,
       ],
        monitoring_level: MonitoringLevel::Enhanced };

    assert!(cautious_interaction.allows_operation("read"));
    assert!(!cautious_interaction.allows_operation("write"));
    assert_eq!(cautious_interaction.trust_level(), 0.2);

    match cautious_interaction { EcosystemMembership::CautiousInteraction {
            concern_factors,
            monitoring_level } => {
            assert_eq!(concern_factors.len(), 2);
            assert_eq!(monitoring_level, MonitoringLevel::Enhanced);
        }
        _ => panic!( "Expected"CautiousInteraction),
    }
}

#[tokio::test]
async fn test_visiting_collaborator() {
    let visitor = EcosystemMembership::VisitingCollaborator { collaboration_scope: CollaborationScope {
            allowed_operations: vec!["read".to_string(),  "comment".to_string()],
            resource_limits: [("api_calls_per_hour".to_string(), 100)].into(),
            interaction_boundaries: vec!["public_repos_only".to_string()] },
        visit_duration: Duration::from_secs(86400 * 7), // 1 week
    };

    assert!(visitor.allows_operation("read"));
    assert!(!visitor.allows_operation("admin"));
    assert_eq!(visitor.trust_level(), 0.5);

    match visitor {
        EcosystemMembership::VisitingCollaborator {
            collaboration_scope,
            ..
        } => {
            assert_eq!(collaboration_scope.allowed_operations.len(), 2);
            assert_eq!(
                collaboration_scope
                    .resource_limits
                    .get("api_calls_per_hour"),
                Some(&100)
            );
        }
        _ => panic!( "Expected"VisitingCollaborator),
    }
}

#[tokio::test]
async fn test_trust_evolution_questioning_and_resolution() {
    let questioning_trust = TrustEvolution::Questioning {
        concern_factors: vec![ConcernType::Performance, ConcernType::Communication],
        clarification_needed: vec![
             "explain_recent_downtime".to_string(),
             "improve_response_time".to_string(),
       ],
        resolution_path: Some(ResolutionStrategy {
            approach: "collaborative_improvement".to_string(),
            steps: vec![
                 "performance_analysis".to_string(),
                 "communication_training".to_string(),
                 "monitoring_implementation".to_string(),
           ],
            expected_duration: Duration::from_secs(86400 * 14), // 2 weeks
        }),
    };

    assert_eq!(questioning_trust.trust_score(), 0.4);

    match questioning_trust {
        TrustEvolution::Questioning {
            concern_factors,
            resolution_path,
            ..
        } => {
            assert_eq!(concern_factors.len(), 2);
            assert!(resolution_path.is_some());
            let strategy = resolution_path.unwrap();
            assert_eq!(strategy.steps.len(), 3);
            assert_eq!(strategy.approach,  "collaborative_improvement");
        }
        _ => panic!("Expected  Questioning trust evolution"),
    }
}
