// SPDX-License-Identifier: AGPL-3.0-or-later

//! Serializable threat-domain types: events, sources, rules, intelligence feeds, and response payloads.

/// Incident-centric records used by higher-level workflows.
pub mod incidents;

/// Engine-facing rule conditions, ML metadata, and the core engine snapshot type.
pub mod engine;

pub use engine::threat_engine::ThreatDetectionEngine;

pub use beardog_types::canonical::config::domains::threat::ThreatDetectionConfig;

mod detection_rules;
mod incident_response;
mod intel;
mod mitigation;
mod ml_model;
mod security_telemetry;
mod source_target;
mod taxonomy;
mod threat_event;

pub use detection_rules::{DetectionRule, RuleCondition};
pub use incident_response::{IncidentResponse, ResponseAction, ResponseStatus};
pub use intel::{ThreatIndicator, ThreatIntelligenceFeed};
pub use mitigation::MitigationStep;
pub use ml_model::{MlModel, MlModelType};
pub use security_telemetry::SecurityEvent;
pub use source_target::{ThreatSource, ThreatTarget};
pub use taxonomy::{
    AssetCriticality, DetectionMethod, IndicatorType, ProtectionLevel, SourceClassification,
    ThreatAction, ThreatRuleType, ThreatSeverity, ThreatStatus, ThreatType,
};
pub use threat_event::ThreatEvent;
