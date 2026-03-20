// SPDX-License-Identifier: AGPL-3.0-only

//! Standalone analysis engine with coarse byte-length heuristics and signature maps.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// Removed unused import: tracing::info

use crate::threat::types::{DetectionRule, ThreatType};
use beardog_errors::BearDogError;

// Use canonical threat detection configuration
use beardog_types::canonical::config::domains::threat::ThreatDetectionConfig;

/// Counters updated while running [`ThreatDetectionEngine::analyze_threat`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAnalysisMetrics {
    /// Total `analyze_threat` invocations.
    pub analyses_performed: u64,
    /// Number of `threats_detected`
    /// Number of `threats_detected`
    pub threats_detected: u64,
    /// Number of `false_positives`
    /// Number of `false_positives`
    pub false_positives: u64,
    /// The detection accuracy value
    /// The detection accuracy value
    pub detection_accuracy: f64,
}

impl Default for ThreatAnalysisMetrics {
    fn default() -> Self {
        Self {
            analyses_performed: 0,
            threats_detected: 0,
            false_positives: 0,
            detection_accuracy: 0.0,
        }
    }
}

/// Lightweight engine separate from `handlers::core`; useful for offline scans.
pub struct ThreatDetectionEngine {
    /// Canonical threat configuration controlling thresholds and features.
    pub config: ThreatDetectionConfig,
    /// Collection of detection rules
    /// Collection of detection rules
    pub detection_rules: Vec<DetectionRule>,
    /// Mapping of threat signatures
    /// Mapping of threat signatures
    pub threat_signatures: HashMap<String, String>,
    /// The metrics value
    /// The metrics value
    pub metrics: ThreatAnalysisMetrics,
}

impl ThreatDetectionEngine {
    /// Creates a new instance
    #[must_use]
    pub fn new(config: ThreatDetectionConfig) -> Self {
        Self {
            config,
            detection_rules: Vec::new(),
            threat_signatures: HashMap::new(),
            metrics: ThreatAnalysisMetrics::default(),
        }
    }

    /// Appends a rule to the in-memory list (no deduplication).
    pub fn add_detection_rule(&mut self, rule: DetectionRule) {
        self.detection_rules.push(rule);
    }

    /// Removes rule
    /// Removes rule
    pub fn remove_rule(&mut self, rule_id: &str) -> bool {
        let initial_len = self.detection_rules.len();
        self.detection_rules.retain(|rule| rule.id != rule_id);
        self.detection_rules.len() < initial_len
    }

    /// Updates `threat_signatures`
    /// Updates `threat_signatures`
    pub fn update_threat_signatures(
        &mut self,
        signatures: HashMap<String, String>,
    ) -> Result<(), BearDogError> {
        self.threat_signatures.extend(signatures);
        Ok(())
    }

    /// Runs a trivial size-based heuristic and updates [`ThreatAnalysisMetrics`].
    pub fn analyze_threat(
        &mut self,
        threat_data: &[u8],
    ) -> Result<ThreatAnalysisResult, BearDogError> {
        self.metrics.analyses_performed += 1;

        // Simplified threat analysis
        let threat_detected = threat_data.len() > 1000; // Simple size-based detection

        if threat_detected {
            self.metrics.threats_detected += 1;
        }

        Ok(ThreatAnalysisResult {
            threat_detected,
            confidence_score: if threat_detected { 0.8 } else { 0.1 },
            threat_type: if threat_detected {
                ThreatType::Malware
            } else {
                ThreatType::Unknown
            },
            details: "Basic threat analysis completed".to_string(),
        })
    }
}

impl Default for ThreatDetectionEngine {
    fn default() -> Self {
        Self::new(ThreatDetectionConfig::default())
    }
}

/// Output of [`ThreatDetectionEngine::analyze_threat`] including coarse classification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAnalysisResult {
    /// Whether `threat_detected` is enabled
    /// Whether `threat_detected` is enabled
    pub threat_detected: bool,
    /// Score returned by the heuristic (not calibrated to a global scale).
    pub confidence_score: f64,
    /// The threat type value
    /// The threat type value
    pub threat_type: ThreatType,
    /// The details value
    /// The details value
    pub details: String,
}
