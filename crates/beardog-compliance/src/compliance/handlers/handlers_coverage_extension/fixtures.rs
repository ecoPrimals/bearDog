// SPDX-License-Identifier: AGPL-3.0-or-later

//! Shared test fixtures for coverage extension tests.

use crate::compliance::types::{
    ComplianceEvent, ComplianceEventType, ComplianceSeverity, ComplianceStandard,
};
use chrono::Utc;
use uuid::Uuid;

pub(super) fn create_event(
    event_type: ComplianceEventType,
    standard: ComplianceStandard,
) -> ComplianceEvent {
    let description = format!("Test event: {event_type:?}");
    ComplianceEvent {
        id: Uuid::new_v4().to_string(),
        event_type,
        timestamp: Utc::now(),
        description,
        severity: ComplianceSeverity::Medium,
        standard,
        metadata: serde_json::json!({}),
    }
}

pub(super) fn event_with_simulation(
    event_type: ComplianceEventType,
    standard: ComplianceStandard,
    simulation: serde_json::Value,
) -> ComplianceEvent {
    let mut event = create_event(event_type, standard);
    event.metadata = simulation;
    event
}
