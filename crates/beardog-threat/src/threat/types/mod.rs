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

pub use detection_rules::*;
pub use incident_response::*;
pub use intel::*;
pub use mitigation::*;
pub use ml_model::*;
pub use security_telemetry::*;
pub use source_target::*;
pub use taxonomy::*;
pub use threat_event::*;
