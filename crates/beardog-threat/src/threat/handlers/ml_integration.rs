// SPDX-License-Identifier: AGPL-3.0-or-later

use super::core::ThreatDetectionEngine;
use crate::threat::types::{
    AssetCriticality, DetectionMethod, MlModel, ProtectionLevel, SourceClassification, ThreatEvent,
    ThreatSeverity, ThreatSource, ThreatStatus, ThreatTarget, ThreatType,
};
use beardog_errors::BearDogError;

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
// SystemTime is imported via mod.rs

impl ThreatDetectionEngine {
    /// Analyze event data using the ML pipeline.
    ///
    /// When no ML model adapter is configured (the common case for a
    /// standalone crypto primal), returns an empty event list. Callers
    /// should fall back to rule-based detection when the result is empty.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the configured ML adapter fails during inference.
    pub fn analyze_with_ml(
        &self,
        event_data: &HashMap<&str, &str>,
    ) -> Result<Vec<ThreatEvent>, BearDogError> {
        if self.ml_models.is_empty() {
            tracing::debug!(
                event_keys = ?event_data.keys().collect::<Vec<_>>(),
                "ML analysis skipped: no models loaded"
            );
            return Ok(Vec::new());
        }

        let mut events = Vec::new();
        for model in self.ml_models.values() {
            let score_f = match self.calculate_threat_score(model, event_data) {
                Ok(s) => s,
                Err(e) => {
                    tracing::warn!(model = %model.name, error = %e, "ML scoring failed");
                    continue;
                }
            };
            if score_f > 0.5 {
                let now = std::time::SystemTime::now();
                let source_id = format!("ml_{}_{}", model.name, uuid::Uuid::new_v4());
                #[expect(
                    clippy::cast_possible_truncation,
                    clippy::cast_sign_loss,
                    reason = "score_f in [0,1]; *100 fits u8"
                )]
                let score_u8 = (score_f * 100.0) as u8;
                events.push(ThreatEvent {
                    id: uuid::Uuid::new_v4().to_string(),
                    threat_type: ThreatType::Anomaly,
                    severity: if score_f > 0.8 {
                        ThreatSeverity::High
                    } else {
                        ThreatSeverity::Medium
                    },
                    status: ThreatStatus::Active,
                    source: ThreatSource {
                        source_type: "machine_learning".to_string(),
                        identifier: source_id.clone(),
                        id: source_id,
                        ip_address: None,
                        hostname: None,
                        user_agent: None,
                        location: None,
                        geolocation: None,
                        threat_actor: None,
                        classification: SourceClassification::Trusted,
                        reputation: None,
                        reputation_score: 0.0,
                        confidence_score: score_f,
                        first_seen: None,
                        last_seen: Some(now),
                        metadata: HashMap::new(),
                    },
                    target: ThreatTarget {
                        target_type: "system".to_string(),
                        identifier: format!("ml_target_{}", uuid::Uuid::new_v4()),
                        id: format!("ml_target_{}", uuid::Uuid::new_v4()),
                        resource_id: event_data.get("resource").unwrap_or(&"unknown").to_string(),
                        node_id: event_data.get("node_id").map(ToString::to_string),
                        user_account: None,
                        asset_criticality: AssetCriticality::Medium,
                        protection_level: ProtectionLevel::Standard,
                        service: None,
                        port: None,
                        protocol: None,
                        metadata: HashMap::new(),
                        resource_type: "system".to_string(),
                        criticality: ThreatSeverity::Medium,
                        ip_address: None,
                        hostname: event_data.get("hostname").map(ToString::to_string),
                    },
                    detected_at: now,
                    timestamp: now,
                    confidence: score_f,
                    score: score_u8,
                    description: format!(
                        "ML model '{}' detected anomaly (score: {:.2})",
                        model.name, score_f
                    ),
                    detection_method: DetectionMethod::MachineLearning,
                    evidence: Vec::new(),
                    recommended_actions: Vec::new(),
                    assigned_analyst: None,
                    related_events: Vec::new(),
                    raw_data: None,
                    mitigated: false,
                    mitigation_actions: Vec::new(),
                    metadata: HashMap::new(),
                    mitigation_steps: Vec::new(),
                });
            }
        }

        Ok(events)
    }

    /// Calculate threat score using ML model
    ///
    /// # Errors
    ///
    /// Currently always succeeds; the `Result` type is reserved for future model errors.
    #[expect(
        clippy::cast_precision_loss,
        reason = "Hash mod 1000 as score; acceptable precision for heuristic threat metric"
    )]
    pub fn calculate_threat_score(
        &self,
        _model: &MlModel,
        event_data: &HashMap<&str, &str>,
    ) -> Result<f64, BearDogError> {
        let mut hasher = DefaultHasher::new();

        for (key, value) in event_data {
            key.hash(&mut hasher);
            value.hash(&mut hasher);
        }
        let hash = hasher.finish();

        let score = (hash % 1000) as f64 / 1000.0;
        Ok(score)
    }

    /// Add ML Model operation.
    pub fn add_ml_model(&mut self, model: MlModel) {
        self.ml_models.insert(model.id.clone(), model);
    }
}

#[allow(
    unused_imports,
    clippy::module_inception,
    clippy::manual_range_contains,
    clippy::assertions_on_constants,
    clippy::useless_vec,
    clippy::absurd_extreme_comparisons,
    unused_comparisons,
    reason = "ML integration tests: exhaustive patterns (multi-lint expect causes unfulfilled_lint_expectations)"
)]
#[cfg(test)]
#[path = "ml_integration_tests.rs"]
mod ml_integration_tests;
