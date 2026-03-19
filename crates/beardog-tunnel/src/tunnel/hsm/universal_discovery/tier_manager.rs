// SPDX-License-Identifier: AGPL-3.0-only

//! HSM Tier Manager
//!
//! This module manages HSM tier assignments based on capabilities and requirements.

use super::DiscoveredHsm;
use crate::tunnel::hsm::types::HsmTier;
use beardog_errors::BearDogError;
use tracing::{debug, info};

/// Tier manager for HSM classification
pub struct TierManager {
    tier_scoring_weights: TierScoringWeights,
}

/// Tier scoring weights
#[derive(Debug, Clone)]
pub struct TierScoringWeights {
    /// Weight for security evaluation (0.0 to 1.0)
    pub security_weight: f64,
    /// Weight for performance evaluation (0.0 to 1.0)
    pub performance_weight: f64,
    /// Weight for availability evaluation (0.0 to 1.0)
    pub availability_weight: f64,
    /// Weight for compliance evaluation (0.0 to 1.0)
    pub compliance_weight: f64,
}

impl Default for TierScoringWeights {
    fn default() -> Self {
        Self {
            security_weight: 0.4,
            performance_weight: 0.3,
            availability_weight: 0.2,
            compliance_weight: 0.1,
        }
    }
}

impl TierManager {
    /// Creates a new TierManager instance
    ///
    /// # Errors
    /// Returns an error if initialization fails.
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            tier_scoring_weights: TierScoringWeights::default(),
        })
    }

    /// Assigns tier to an HSM based on capabilities
    pub fn assign_tier(&self, hsm: &DiscoveredHsm) -> HsmTier {
        debug!("📊 Assigning tier to HSM: {} {}", hsm.vendor, hsm.model);

        let score = self.calculate_tier_score(hsm);

        let tier = if score >= 0.9 {
            HsmTier::Hardware
        } else if score >= 0.7 {
            HsmTier::SecureEnclave
        } else if score >= 0.5 {
            HsmTier::Mobile
        } else {
            HsmTier::Software
        };

        info!(
            "✅ Assigned tier {:?} to {} {} (score: {:.2})",
            tier, hsm.vendor, hsm.model, score
        );
        tier
    }

    /// Calculates tier score for an HSM
    fn calculate_tier_score(&self, hsm: &DiscoveredHsm) -> f64 {
        let security_score = self.calculate_security_score(hsm);
        let performance_score = self.calculate_performance_score(hsm);
        let availability_score = self.calculate_availability_score(hsm);
        let compliance_score = self.calculate_compliance_score(hsm);

        let weighted_score: f64 = security_score * self.tier_scoring_weights.security_weight
            + performance_score * self.tier_scoring_weights.performance_weight
            + availability_score * self.tier_scoring_weights.availability_weight
            + compliance_score * self.tier_scoring_weights.compliance_weight;

        weighted_score.min(1.0_f64)
    }

    /// Calculates security score
    fn calculate_security_score(&self, hsm: &DiscoveredHsm) -> f64 {
        let mut score: f64 = 0.5; // Base score

        if hsm.capabilities.key_generation.hardware_backed {
            score += 0.2;
        }

        if hsm.capabilities.key_generation.true_rng {
            score += 0.15;
        }

        if hsm.capabilities.advanced_features.key_attestation {
            score += 0.1;
        }

        if hsm.capabilities.security.tamper_evident_logs {
            score += 0.05;
        }

        score.min(1.0_f64)
    }

    /// Calculates performance score
    fn calculate_performance_score(&self, hsm: &DiscoveredHsm) -> f64 {
        let ops_per_sec = hsm.capabilities.performance.operations_per_second;
        let latency = hsm.capabilities.performance.average_latency_ms;

        let ops_score = (ops_per_sec / 10000.0).min(1.0);
        let latency_score = (1.0 - (latency / 1000.0).min(1.0)).max(0.0);

        (ops_score + latency_score) / 2.0
    }

    /// Calculates availability score
    fn calculate_availability_score(&self, hsm: &DiscoveredHsm) -> f64 {
        match hsm.health_status {
            super::HsmHealthStatus::Healthy => 1.0,
            super::HsmHealthStatus::Warning => 0.7,
            super::HsmHealthStatus::Critical => 0.3,
            super::HsmHealthStatus::Unavailable => 0.0,
            super::HsmHealthStatus::Unknown => 0.5,
        }
    }

    /// Calculates compliance score
    fn calculate_compliance_score(&self, hsm: &DiscoveredHsm) -> f64 {
        let compliance_count = hsm.capabilities.security.compliance_reporting.len();
        (compliance_count as f64 / 5.0).min(1.0)
    }

    /// Selects best HSM for an operation
    pub fn select_best_hsm_for_operation<'a>(
        &self,
        hsms: &[&'a DiscoveredHsm],
        _operation_type: &str,
    ) -> Result<Option<&'a DiscoveredHsm>, BearDogError> {
        if hsms.is_empty() {
            return Ok(None);
        }

        let mut best_hsm = hsms[0];
        let mut best_score = self.calculate_tier_score(best_hsm);

        for hsm in hsms.iter().skip(1) {
            let score = self.calculate_tier_score(hsm);
            if score > best_score {
                best_score = score;
                best_hsm = hsm;
            }
        }

        Ok(Some(best_hsm))
    }

    /// Updates tier scoring weights
    pub fn update_scoring_weights(&mut self, weights: TierScoringWeights) {
        info!("📊 Updating tier scoring weights");
        self.tier_scoring_weights = weights;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier_manager_creation() {
        let manager = TierManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_default_weights() {
        let weights = TierScoringWeights::default();
        assert_eq!(weights.security_weight, 0.4);
        assert_eq!(weights.performance_weight, 0.3);
    }
}
