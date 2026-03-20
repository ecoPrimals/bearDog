// SPDX-License-Identifier: AGPL-3.0-only

//! Coverage: constraint evolution engine.

use chrono::Utc;

// ═══════════════════════════════════════════════════════════════════
// constraints/evolution.rs - Additional coverage
// ═══════════════════════════════════════════════════════════════════

mod evolution_tests {
    use super::Utc;
    use crate::constraints::*;

    #[test]
    fn test_evolution_engine_default() {
        let engine = ConstraintEvolutionEngine::default();
        assert_eq!(engine.trust_score(), 0.5);
    }

    #[test]
    fn test_record_operation_with_domain() {
        let mut engine = ConstraintEvolutionEngine::new();
        let op = KeyOperation::Sign {
            domain: Some("climate".to_string()),
        };
        engine.record_operation(&op);

        assert_eq!(engine.usage_stats().total_operations, 1);
        assert!(
            engine
                .usage_stats()
                .domains_accessed
                .contains_key("climate")
        );
        assert!(engine.usage_stats().first_operation.is_some());
        assert!(engine.usage_stats().last_operation.is_some());
    }

    #[test]
    fn test_record_multiple_operations() {
        let mut engine = ConstraintEvolutionEngine::new();
        engine.record_operation(&KeyOperation::Sign {
            domain: Some("climate".to_string()),
        });
        engine.record_operation(&KeyOperation::Encrypt {
            recipients: vec!["peer".to_string()],
        });
        engine.record_operation(&KeyOperation::Read {
            path: "/data".to_string(),
        });

        assert_eq!(engine.usage_stats().total_operations, 3);
    }

    #[test]
    fn test_evolve_constraints_trust_threshold() {
        let engine = ConstraintEvolutionEngine::new();
        let constraints = KeyConstraints {
            scope: ScopeConstraint::Limited {
                domains: vec!["domain1".to_string(), "domain2".to_string()],
            },
            ..Default::default()
        };
        let trigger = EvolutionTrigger::TrustThreshold { threshold: 0.9 };
        let evolved = engine.evolve_constraints(&constraints, &trigger);
        // Relaxed: domains should be filtered to only actually used ones
        if let ScopeConstraint::Limited { domains } = &evolved.scope {
            assert!(domains.is_empty()); // No domains used yet
        }
    }

    #[test]
    fn test_evolve_constraints_trust_threshold_low() {
        let engine = ConstraintEvolutionEngine::new();
        let constraints = KeyConstraints {
            scope: ScopeConstraint::Limited {
                domains: vec!["domain1".to_string()],
            },
            ..Default::default()
        };
        let trigger = EvolutionTrigger::TrustThreshold { threshold: 0.5 };
        let evolved = engine.evolve_constraints(&constraints, &trigger);
        // Low threshold: no relaxation
        if let ScopeConstraint::Limited { domains } = &evolved.scope {
            assert_eq!(domains.len(), 1);
        }
    }

    #[test]
    fn test_evolve_constraints_time_elapsed_high_trust() {
        let mut engine = ConstraintEvolutionEngine::new();
        // Build up high trust: starts at 0.5, +0.001 per op, need >0.8 → 301 ops
        for _ in 0..350 {
            engine.record_operation(&KeyOperation::Sign { domain: None });
        }
        assert!(engine.trust_score() > 0.8);

        let constraints = KeyConstraints {
            lifetime: LifetimeConstraint::Duration {
                months: 12,
                evolution_trigger: Some(6),
            },
            ..Default::default()
        };
        let trigger = EvolutionTrigger::TimeElapsed { months: 6 };
        let evolved = engine.evolve_constraints(&constraints, &trigger);
        if let LifetimeConstraint::Duration { months, .. } = &evolved.lifetime {
            assert_eq!(*months, 18); // 12 + 6
        }
    }

    #[test]
    fn test_evolve_constraints_time_elapsed_low_trust() {
        let mut engine = ConstraintEvolutionEngine::new();
        // Keep trust low
        engine.record_failure();
        engine.record_failure();

        let constraints = KeyConstraints {
            lifetime: LifetimeConstraint::Duration {
                months: 12,
                evolution_trigger: Some(6),
            },
            ..Default::default()
        };
        let trigger = EvolutionTrigger::TimeElapsed { months: 6 };
        let evolved = engine.evolve_constraints(&constraints, &trigger);
        if let LifetimeConstraint::Duration { months, .. } = &evolved.lifetime {
            assert_eq!(*months, 12); // No extension (trust < 0.8)
        }
    }

    #[test]
    fn test_evolve_constraints_usage_pattern() {
        let engine = ConstraintEvolutionEngine::new();
        let constraints = KeyConstraints::default();
        let trigger = EvolutionTrigger::UsagePattern {
            pattern: "consistent_compliance".to_string(),
        };
        let _evolved = engine.evolve_constraints(&constraints, &trigger);
    }

    #[test]
    fn test_evolve_constraints_manual_request() {
        let engine = ConstraintEvolutionEngine::new();
        let constraints = KeyConstraints::default();
        let trigger = EvolutionTrigger::ManualRequest;
        let evolved = engine.evolve_constraints(&constraints, &trigger);
        // Manual request doesn't change constraints
        assert_eq!(evolved, constraints);
    }

    #[test]
    fn test_should_evolve_no_trigger() {
        let engine = ConstraintEvolutionEngine::new();
        let constraints = KeyConstraints::default();
        let created_at = Utc::now();
        assert!(engine.should_evolve(&constraints, created_at).is_none());
    }

    #[test]
    fn test_trust_score_capped() {
        let mut engine = ConstraintEvolutionEngine::new();
        let op = KeyOperation::Sign { domain: None };
        for _ in 0..2000 {
            engine.record_operation(&op);
        }
        assert!(engine.trust_score() <= 1.0);
    }

    #[test]
    fn test_trust_score_floor() {
        let mut engine = ConstraintEvolutionEngine::new();
        for _ in 0..200 {
            engine.record_failure();
        }
        assert!(engine.trust_score() >= 0.0);
    }
}
