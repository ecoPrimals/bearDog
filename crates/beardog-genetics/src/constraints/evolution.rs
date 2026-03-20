// Copyright 2025 EcoPrimals BearDog Team
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Constraint evolution - constraints adapt based on key behavior

use super::types::*;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Engine for evolving constraints based on key behavior
pub struct ConstraintEvolutionEngine {
    /// Trust score for the key (0.0 - 1.0)
    trust_score: f64,

    /// Usage statistics
    usage_stats: UsageStatistics,
}

/// Statistics about key usage
#[derive(Debug, Clone, Default)]
pub struct UsageStatistics {
    /// Total operations performed
    pub total_operations: u64,

    /// Operations by type
    pub operations_by_type: HashMap<OperationType, u64>,

    /// Failed operations (constraint violations)
    pub failed_operations: u64,

    /// Domains accessed
    pub domains_accessed: HashMap<String, u64>,

    /// Last operation timestamp
    pub last_operation: Option<DateTime<Utc>>,

    /// First operation timestamp
    pub first_operation: Option<DateTime<Utc>>,
}

/// Triggers for constraint evolution
#[derive(Debug, Clone)]
pub enum EvolutionTrigger {
    /// Time-based evolution
    TimeElapsed {
        /// Months since key creation after which policy may renew or tighten.
        months: u32,
    },

    /// Trust score reached threshold
    TrustThreshold {
        /// Trust score boundary that was crossed (e.g. high-trust relaxation).
        threshold: f64,
    },

    /// Usage pattern detected
    UsagePattern {
        /// Named pattern label (e.g. consistent compliance) driving policy change.
        pattern: String,
    },

    /// Manual evolution request
    ManualRequest,
}

impl ConstraintEvolutionEngine {
    /// Create new evolution engine
    pub fn new() -> Self {
        Self {
            trust_score: 0.5, // Start neutral
            usage_stats: UsageStatistics::default(),
        }
    }

    /// Record a successful operation
    pub fn record_operation(&mut self, operation: &KeyOperation) {
        self.usage_stats.total_operations += 1;

        let op_type = operation.operation_type();
        *self
            .usage_stats
            .operations_by_type
            .entry(op_type)
            .or_insert(0) += 1;

        if let Some(domain) = operation.domain() {
            *self
                .usage_stats
                .domains_accessed
                .entry(domain.to_string())
                .or_insert(0) += 1;
        }

        let now = Utc::now();
        if self.usage_stats.first_operation.is_none() {
            self.usage_stats.first_operation = Some(now);
        }
        self.usage_stats.last_operation = Some(now);

        // Increase trust score for successful operations
        self.trust_score = (self.trust_score + 0.001).min(1.0);
    }

    /// Record a failed operation (constraint violation)
    pub fn record_failure(&mut self) {
        self.usage_stats.failed_operations += 1;

        // Decrease trust score for violations
        self.trust_score = (self.trust_score - 0.01).max(0.0);
    }

    /// Check if evolution should be triggered
    pub fn should_evolve(
        &self,
        constraints: &KeyConstraints,
        created_at: DateTime<Utc>,
    ) -> Option<EvolutionTrigger> {
        // Check time-based evolution
        if let LifetimeConstraint::Duration {
            evolution_trigger: Some(trigger_months),
            ..
        } = &constraints.lifetime
        {
            let now = Utc::now();
            let elapsed_months = (now - created_at).num_days() / 30;
            if elapsed_months >= *trigger_months as i64 {
                return Some(EvolutionTrigger::TimeElapsed {
                    months: *trigger_months,
                });
            }
        }

        // Check trust threshold
        if self.trust_score >= 0.9 {
            return Some(EvolutionTrigger::TrustThreshold { threshold: 0.9 });
        }

        // Check usage patterns
        if self.usage_stats.total_operations > 1000 && self.usage_stats.failed_operations == 0 {
            return Some(EvolutionTrigger::UsagePattern {
                pattern: "consistent_compliance".to_string(),
            });
        }

        None
    }

    /// Evolve constraints based on behavior
    pub fn evolve_constraints(
        &self,
        current: &KeyConstraints,
        trigger: &EvolutionTrigger,
    ) -> KeyConstraints {
        let mut evolved = current.clone();

        match trigger {
            EvolutionTrigger::TrustThreshold { threshold } => {
                if *threshold >= 0.9 {
                    // High trust: relax some restrictions
                    evolved = self.relax_constraints(evolved);
                }
            }

            EvolutionTrigger::TimeElapsed { .. } => {
                // Time-based evolution: prepare for expiry or renewal
                evolved = self.prepare_for_renewal(evolved);
            }

            EvolutionTrigger::UsagePattern { pattern } => {
                if pattern == "consistent_compliance" {
                    // Consistent good behavior: relax restrictions
                    evolved = self.relax_constraints(evolved);
                }
            }

            EvolutionTrigger::ManualRequest => {
                // Manual evolution: no automatic changes
            }
        }

        evolved
    }

    /// Relax constraints for trusted keys
    fn relax_constraints(&self, mut constraints: KeyConstraints) -> KeyConstraints {
        // Example: if key consistently accessed only certain domains, limit to those
        if let ScopeConstraint::Limited { ref mut domains } = constraints.scope {
            // Keep only actually used domains
            let used_domains: Vec<String> =
                self.usage_stats.domains_accessed.keys().cloned().collect();
            *domains = used_domains;
        }

        constraints
    }

    /// Prepare constraints for renewal
    fn prepare_for_renewal(&self, mut constraints: KeyConstraints) -> KeyConstraints {
        // Extend lifetime for well-behaved keys
        if let LifetimeConstraint::Duration { ref mut months, .. } = constraints.lifetime {
            if self.trust_score > 0.8 {
                *months += 6; // Add 6 months
            }
        }

        constraints
    }

    /// Get current trust score
    pub const fn trust_score(&self) -> f64 {
        self.trust_score
    }

    /// Get usage statistics
    pub const fn usage_stats(&self) -> &UsageStatistics {
        &self.usage_stats
    }
}

impl Default for ConstraintEvolutionEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_trust_score_increases() {
        let mut engine = ConstraintEvolutionEngine::new();
        let initial_trust = engine.trust_score();

        let op = KeyOperation::Sign { domain: None };
        engine.record_operation(&op);

        assert!(engine.trust_score() > initial_trust);
    }

    #[test]
    fn test_trust_score_decreases_on_failure() {
        let mut engine = ConstraintEvolutionEngine::new();
        let initial_trust = engine.trust_score();

        engine.record_failure();

        assert!(engine.trust_score() < initial_trust);
    }

    #[test]
    fn test_time_based_evolution_trigger() {
        let engine = ConstraintEvolutionEngine::new();
        let constraints = KeyConstraints {
            lifetime: LifetimeConstraint::Duration {
                months: 18,
                evolution_trigger: Some(12),
            },
            ..Default::default()
        };

        // 13 months ago
        let created_at = Utc::now() - Duration::days(390);

        let trigger = engine.should_evolve(&constraints, created_at);
        assert!(trigger.is_some());
        assert!(matches!(
            trigger.unwrap(),
            EvolutionTrigger::TimeElapsed { .. }
        ));
    }

    #[test]
    fn test_usage_pattern_trigger() {
        let mut engine = ConstraintEvolutionEngine::new();

        // Simulate many successful operations
        for _ in 0..1001 {
            engine.record_operation(&KeyOperation::Sign { domain: None });
        }

        let constraints = KeyConstraints::default();
        let created_at = Utc::now();

        let trigger = engine.should_evolve(&constraints, created_at);
        assert!(
            trigger.is_some(),
            "Should trigger evolution after consistent usage"
        );
        // Either trust threshold or usage pattern should trigger
        assert!(matches!(
            trigger.unwrap(),
            EvolutionTrigger::TrustThreshold { .. } | EvolutionTrigger::UsagePattern { .. }
        ));
    }
}
