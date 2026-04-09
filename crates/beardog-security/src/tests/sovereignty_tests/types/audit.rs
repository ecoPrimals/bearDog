// SPDX-License-Identifier: AGPL-3.0-or-later

//! Audit and Compliance Types
//!
//! Test helper types for audit events, trails, and compliance reporting.

use beardog_errors::BearDogError;
use std::time::Duration;
use std::time::Instant;

/// Audit event types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(
    dead_code,
    reason = "sovereignty test type stubs for scenario scaffolding"
)]
pub enum EventType {
    DataAccess,
    KeyOperation,
    PolicyChange,
    AuthenticationAttempt,
}

/// Event outcome
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(
    dead_code,
    reason = "sovereignty test type stubs for scenario scaffolding"
)]
pub enum Outcome {
    Success,
    Failure,
}

/// Audit event
#[derive(Debug, Clone)]
#[expect(
    dead_code,
    reason = "sovereignty test type stubs for scenario scaffolding"
)]
pub struct AuditEvent {
    event_type: EventType,
    user: String,
    resource: String,
    region: String,
    outcome: Outcome,
    timestamp: Instant,
}

impl AuditEvent {
    pub fn new(event_type: EventType) -> Self {
        Self {
            event_type,
            user: String::new(),
            resource: String::new(),
            region: String::new(),
            outcome: Outcome::Success,
            timestamp: Instant::now(),
        }
    }

    pub fn with_user(mut self, user: &str) -> Self {
        self.user = user.to_string();
        self
    }

    pub fn with_resource(mut self, resource: &str) -> Self {
        self.resource = resource.to_string();
        self
    }

    pub fn with_region(mut self, region: &str) -> Self {
        self.region = region.to_string();
        self
    }

    pub fn with_outcome(mut self, outcome: Outcome) -> Self {
        self.outcome = outcome;
        self
    }

    pub fn with_timestamp_nanos(self, _nanos: u64) -> Self {
        // For testing old events
        self
    }

    pub fn event_type(&self) -> EventType {
        self.event_type
    }

    pub fn region(&self) -> &str {
        &self.region
    }

    pub fn user(&self) -> &str {
        &self.user
    }

    pub fn outcome(&self) -> Outcome {
        self.outcome
    }
}

/// Audit trail
#[derive(Debug, Clone)]
#[expect(
    dead_code,
    reason = "sovereignty test type stubs for scenario scaffolding"
)]
pub struct AuditTrail {
    name: String,
    events: Vec<AuditEvent>,
    corrupted: bool,
}

impl AuditTrail {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            events: Vec::new(),
            corrupted: false,
        }
    }

    pub fn with_event(mut self, event: AuditEvent) -> Self {
        self.events.push(event);
        self
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn log_event(&mut self, event: AuditEvent) -> Result<(), BearDogError> {
        self.events.push(event);
        Ok(())
    }

    pub fn events(&self) -> &[AuditEvent] {
        &self.events
    }

    pub fn verify_integrity(&self) -> Result<(), BearDogError> {
        // Simplified integrity check
        if self.corrupted {
            return Err(BearDogError::security("Audit trail corrupted".to_string()));
        }
        Ok(())
    }

    pub fn corrupt_event(&mut self, _index: usize) {
        // Simulate corruption for testing
        self.corrupted = true;
    }

    pub fn filter_by_region(&self, region: &str) -> Vec<&AuditEvent> {
        self.events
            .iter()
            .filter(|e| e.region() == region)
            .collect()
    }

    pub fn search_by_type(&self, event_type: EventType) -> Vec<&AuditEvent> {
        self.events
            .iter()
            .filter(|e| e.event_type() == event_type)
            .collect()
    }

    pub fn search_by_user(&self, user: &str) -> Vec<&AuditEvent> {
        self.events.iter().filter(|e| e.user() == user).collect()
    }

    pub fn generate_compliance_report(&self) -> ComplianceReport {
        let total = self.events.len();
        let successful = self
            .events
            .iter()
            .filter(|e| e.outcome() == Outcome::Success)
            .count();
        let failed = total - successful;

        ComplianceReport {
            total_events: total,
            successful_events: successful,
            failed_events: failed,
        }
    }
}

#[derive(Debug)]
pub struct ComplianceReport {
    total_events: usize,
    successful_events: usize,
    failed_events: usize,
}

impl ComplianceReport {
    pub fn total_events(&self) -> usize {
        self.total_events
    }

    pub fn successful_events(&self) -> usize {
        self.successful_events
    }

    pub fn failed_events(&self) -> usize {
        self.failed_events
    }
}

/// Audit chain
#[derive(Debug, Clone)]
pub struct AuditChain {
    trails: Vec<AuditTrail>,
}

impl AuditChain {
    pub fn new() -> Self {
        Self { trails: Vec::new() }
    }

    pub fn length(&self) -> usize {
        self.trails.len()
    }

    pub fn append(&mut self, trail: AuditTrail) -> Result<(), BearDogError> {
        self.trails.push(trail);
        Ok(())
    }

    pub fn verify_chain(&self) -> Result<(), BearDogError> {
        for trail in &self.trails {
            trail.verify_integrity()?;
        }
        Ok(())
    }
}

/// Retention policy
#[derive(Debug, Clone)]
pub struct RetentionPolicy {
    duration: Duration,
}

impl RetentionPolicy {
    pub fn new() -> Self {
        Self {
            duration: Duration::from_secs(86400 * 365), // 1 year default
        }
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn should_retain(&self, _event: &AuditEvent) -> bool {
        // Simplified - always retain in test
        true
    }

    pub fn duration_days(&self) -> u64 {
        self.duration.as_secs() / 86400
    }
}
