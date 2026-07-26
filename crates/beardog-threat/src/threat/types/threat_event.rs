// SPDX-License-Identifier: AGPL-3.0-or-later

//! Incident-centric threat event and constructors.

use super::mitigation::MitigationStep;
use super::source_target::{ThreatSource, ThreatTarget};
use super::taxonomy::{DetectionMethod, ThreatAction, ThreatSeverity, ThreatStatus, ThreatType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Threat event representing a detected security incident
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatEvent {
    /// Stable identifier for this threat instance (often a UUID string).
    pub id: String,
    /// Type of threat detected
    /// The threat type value
    pub threat_type: ThreatType,
    /// Severity level of the threat
    /// The severity value
    pub severity: ThreatSeverity,
    /// Current status of the threat
    /// Current status of the component
    pub status: ThreatStatus,
    /// Source of the threat
    /// The source value
    pub source: ThreatSource,
    /// Target of the threat
    /// The target value
    pub target: ThreatTarget,
    /// Timestamp when threat was detected
    /// The detected at value
    pub detected_at: SystemTime,
    /// Wall-clock time associated with the event for correlation and ordering.
    pub timestamp: SystemTime,
    /// Threat confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Threat score (0-100)
    /// Number of score
    pub score: u8,
    /// Threat description
    /// The description value
    pub description: String,
    /// Detection method used
    /// The detection method value
    pub detection_method: DetectionMethod,
    /// Human-readable or structured evidence strings supporting the finding.
    pub evidence: Vec<String>,
    /// Recommended actions
    /// Collection of recommended actions
    pub recommended_actions: Vec<ThreatAction>,
    /// Assigned analyst
    /// Optional assigned analyst
    pub assigned_analyst: Option<String>,
    /// Related events
    /// Collection of related events
    pub related_events: Vec<String>,
    /// Raw event data
    /// Optional raw data
    pub raw_data: Option<serde_json::Value>,
    /// Whether threat has been mitigated
    /// Whether mitigated is enabled
    pub mitigated: bool,
    /// Mitigation actions taken
    /// Collection of mitigation actions
    pub mitigation_actions: Vec<ThreatAction>,
    /// Additional metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
    /// Mitigation steps taken
    /// Collection of mitigation steps
    pub mitigation_steps: Vec<MitigationStep>,
}

impl ThreatEvent {
    /// Create a new threat event
    /// Creates a new instance
    #[must_use]
    pub fn new(
        id: String,
        threat_type: ThreatType,
        severity: ThreatSeverity,
        source: ThreatSource,
        target: ThreatTarget,
    ) -> Self {
        let now = SystemTime::now();
        Self {
            id,
            threat_type,
            severity,
            status: ThreatStatus::Detected,
            source,
            target,
            detected_at: now,
            timestamp: now,
            confidence: 0.5,
            score: 50,
            description: String::new(),
            detection_method: DetectionMethod::RuleBased,
            evidence: Vec::new(),
            recommended_actions: Vec::new(),
            assigned_analyst: None,
            related_events: Vec::new(),
            raw_data: None,
            mitigated: false,
            mitigation_actions: Vec::new(),
            metadata: HashMap::new(),
            mitigation_steps: Vec::new(),
        }
    }

    /// Appends a [`MitigationStep`] to the running containment timeline.
    pub fn add_mitigation_step(&mut self, step: MitigationStep) {
        self.mitigation_steps.push(step);
    }

    /// Update threat status
    /// Updates status
    pub const fn update_status(&mut self, status: ThreatStatus) {
        self.status = status;
    }

    /// Check if threat is active
    /// Checks if active
    #[must_use]
    pub const fn is_active(&self) -> bool {
        matches!(
            self.status,
            ThreatStatus::Detected | ThreatStatus::Analyzing | ThreatStatus::Mitigating
        )
    }
}
