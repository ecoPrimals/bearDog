// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


//! Audit system for BearDog compliance tracking
//!
//! This module provides comprehensive audit logging and compliance monitoring
//! capabilities for the BearDog ecosystem.

use beardog_errors::BearDogResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Severity levels for audit events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuditSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for AuditSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditSeverity::Low => write!(f, "LOW"),
            AuditSeverity::Medium => write!(f, "MEDIUM"),
            AuditSeverity::High => write!(f, "HIGH"),
            AuditSeverity::Critical => write!(f, "CRITICAL"),
        }
    }
}

/// Types of audit events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuditEventType {
    UserAction,
    SystemEvent,
    SecurityEvent,
    ConfigurationChange,
    DataAccess,
    AuthenticationAttempt,
    AuthorizationCheck,
    ComplianceViolation,
}

impl std::fmt::Display for AuditEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditEventType::UserAction => write!(f, "USER_ACTION"),
            AuditEventType::SystemEvent => write!(f, "SYSTEM_EVENT"),
            AuditEventType::SecurityEvent => write!(f, "SECURITY_EVENT"),
            AuditEventType::ConfigurationChange => write!(f, "CONFIG_CHANGE"),
            AuditEventType::DataAccess => write!(f, "DATA_ACCESS"),
            AuditEventType::AuthenticationAttempt => write!(f, "AUTH_ATTEMPT"),
            AuditEventType::AuthorizationCheck => write!(f, "AUTHZ_CHECK"),
            AuditEventType::ComplianceViolation => write!(f, "COMPLIANCE_VIOLATION"),
        }
    }
}

/// Individual audit event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub id: Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub event_type: AuditEventType,
    pub severity: AuditSeverity,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub source_ip: Option<String>,
    pub resource: String,
    pub action: String,
    pub outcome: String,
    pub details: HashMap<String, String>,
    pub compliance_tags: Vec<String>,
}

impl AuditEvent {
    /// Create a new audit event
    pub fn new(
        event_type: AuditEventType,
        severity: AuditSeverity,
        resource: String,
        action: String,
        outcome: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            event_type,
            severity,
            user_id: None,
            session_id: None,
            source_ip: None,
            resource,
            action,
            outcome,
            details: HashMap::new(),
            compliance_tags: Vec::new(),
        }
    }

    /// Add user context to the event
    pub fn with_user(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Add session context to the event
    pub fn with_session(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
        self
    }

    /// Add source IP to the event
    pub fn with_source_ip(mut self, ip: String) -> Self {
        self.source_ip = Some(ip);
        self
    }

    /// Add additional details to the event
    pub fn with_detail(mut self, key: String, value: String) -> Self {
        self.details.insert(key, value);
        self
    }

    /// Add compliance tags to the event
    pub fn with_compliance_tags(mut self, tags: Vec<String>) -> Self {
        self.compliance_tags = tags;
        self
    }
}

/// Main audit engine for logging and managing audit events
pub struct AuditEngine {
    events: Arc<RwLock<Vec<AuditEvent>>>,
}

impl AuditEngine {
    /// Create a new audit engine
    pub fn new() -> Self {
        Self {
            events: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Log an audit event
    pub async fn log_event(&self, event: AuditEvent) -> BearDogResult<()> {
        self.events.write().await.push(event);
        Ok(())
    }

    /// Get recent events with optional limit
    pub async fn get_recent_events(&self, limit: usize) -> BearDogResult<Vec<AuditEvent>> {
        let events = self.events.read().await;
        let start = events.len().saturating_sub(limit);
        Ok(events[start..].to_vec())
    }

    /// Query events by criteria
    pub async fn query_events(
        &self,
        event_type: Option<AuditEventType>,
        severity: Option<AuditSeverity>,
        user_id: Option<String>,
        limit: Option<usize>,
    ) -> BearDogResult<Vec<AuditEvent>> {
        let events = self.events.read().await;
        let mut filtered: Vec<AuditEvent> = events
            .iter()
            .filter(|event| {
                if let Some(ref et) = event_type {
                    if &event.event_type != et {
                        return false;
                    }
                }
                if let Some(ref sev) = severity {
                    if &event.severity != sev {
                        return false;
                    }
                }
                if let Some(ref uid) = user_id {
                    if let Some(ref event_uid) = event.user_id {
                        if event_uid != uid {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();

        // Sort by timestamp (newest first)
        filtered.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        // Apply limit if specified
        if let Some(limit) = limit {
            filtered.truncate(limit);
        }

        Ok(filtered)
    }

    /// Get event count
    pub async fn event_count(&self) -> usize {
        self.events.read().await.len()
    }

    /// Clear all events (for testing)
    pub async fn clear_events(&self) {
        self.events.write().await.clear();
    }
}

impl Default for AuditEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Audit log collection
pub struct AuditLog {
    events: Vec<AuditEvent>,
}

impl AuditLog {
    /// Create a new audit log
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }

    /// Add an event to the log
    pub fn add_event(&mut self, event: AuditEvent) {
        self.events.push(event);
    }

    /// Get all events
    pub fn events(&self) -> &[AuditEvent] {
        &self.events
    }

    /// Filter events by criteria
    pub fn filter_events(
        &self,
        event_type: Option<AuditEventType>,
        severity: Option<AuditSeverity>,
        user_id: Option<String>,
    ) -> BearDogResult<Vec<AuditEvent>> {
        let filtered: Vec<AuditEvent> = self
            .events
            .iter()
            .filter(|event| {
                if let Some(ref et) = event_type {
                    if &event.event_type != et {
                        return false;
                    }
                }
                if let Some(ref sev) = severity {
                    if &event.severity != sev {
                        return false;
                    }
                }
                if let Some(ref uid) = user_id {
                    if let Some(ref event_uid) = event.user_id {
                        if event_uid != uid {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();

        Ok(filtered)
    }

    /// Create a placeholder instance for initialization
    pub fn placeholder() -> Self {
        Self {
            events: Vec::new(),
        }
    }
}

impl Default for AuditLog {
    fn default() -> Self {
        Self::placeholder()
    }
}
