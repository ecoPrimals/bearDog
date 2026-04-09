// SPDX-License-Identifier: AGPL-3.0-or-later

//! Alert Handling Test Types

use super::behavior::ThreatType;
use super::pattern::ThreatSeverity;
use std::time::Instant;

#[derive(Debug, Clone)]
#[allow(
    dead_code,
    reason = "alerting scenario fixtures and reserved enum-like stubs; expect unfulfilled when variants are referenced"
)]
pub struct ThreatEvent {
    pub threat_type: ThreatType,
    pub severity: ThreatSeverity,
    pub description: String,
    pub timestamp: Instant,
    pub is_threat: bool,
}

#[allow(
    dead_code,
    reason = "alerting scenario fixtures and reserved enum-like stubs; expect unfulfilled when variants are referenced"
)]
impl ThreatEvent {
    pub fn new(
        threat_type: ThreatType,
        severity: ThreatSeverity,
        description: impl Into<String>,
    ) -> Self {
        Self {
            threat_type,
            severity,
            description: description.into(),
            timestamp: Instant::now(),
            is_threat: threat_type != ThreatType::None,
        }
    }

    pub fn severity(&self) -> ThreatSeverity {
        self.severity
    }
}

#[derive(Debug, Clone)]
#[allow(
    dead_code,
    reason = "alerting scenario fixtures and reserved enum-like stubs; expect unfulfilled when variants are referenced"
)]
pub struct ThreatAlert {
    pub event: ThreatEvent,
    pub notified: bool,
}

#[allow(
    dead_code,
    reason = "alerting scenario fixtures and reserved enum-like stubs; expect unfulfilled when variants are referenced"
)]
pub struct AlertHandler {
    alerts: Vec<ThreatAlert>,
}

#[allow(
    dead_code,
    reason = "alerting scenario fixtures and reserved enum-like stubs; expect unfulfilled when variants are referenced"
)]
impl AlertHandler {
    pub fn new() -> Self {
        Self { alerts: Vec::new() }
    }

    pub fn handle_event(&mut self, event: ThreatEvent) {
        self.alerts.push(ThreatAlert {
            event,
            notified: true,
        });
    }

    pub fn handle_alert(&mut self, alert: ThreatAlert) {
        self.alerts.push(alert);
    }

    pub fn get_alerts(&self) -> &[ThreatAlert] {
        &self.alerts
    }

    pub fn alert_count(&self) -> usize {
        self.alerts.len()
    }
}
