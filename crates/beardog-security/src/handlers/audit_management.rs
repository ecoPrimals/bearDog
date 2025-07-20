//! Audit Management Module
//!
//! Handles security audit events, compliance logging, and audit trails.

use super::*;
use serde::{Deserialize, Serialize};

impl BearDogSecurityProvider {
    /// Create a comprehensive audit event
    pub async fn create_audit_event(&mut self, event: AuditEvent) -> BearDogResult<()> {
        if !self.config.audit_logging_enabled {
            return Ok(());
        }

        // Create detailed audit entry
        let audit_entry = SecurityAuditEvent {
            id: Uuid::new_v4().to_string(),
            event_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            event_type: format!("{event:?}"),
            subject: "system".to_string(), // Will be extracted from event
            resource: "unknown".to_string(), // Will be extracted from event
            action: Action {
                action_type: ActionType::Read,
                description: "Audit event".to_string(),
                risk_level: RiskLevel::Low,
                timestamp: Utc::now(),
            },
            success: true,
            result: true,               // Will be determined by event type
            risk_level: RiskLevel::Low, // Will be calculated
            details: self.extract_audit_details(&event).await?,
            metadata: HashMap::new(),
        };

        // Store audit event
        self.audit_manager.log_event(audit_entry).await?;

        // Update metrics
        self.metrics.audit_events_generated += 1;

        Ok(())
    }

    /// Create audit event for authentication attempts
    pub async fn create_auth_audit_event(
        &mut self,
        user_id: &str,
        success: bool,
        failure_reason: Option<String>,
    ) -> BearDogResult<()> {
        let event = AuditEvent::Authentication {
            user_id: user_id.to_string(),
            success,
            timestamp: Utc::now(),
            failure_reason,
            ip_address: None, // TODO: Extract from context
            user_agent: None, // TODO: Extract from context
        };

        self.create_audit_event(event).await
    }

    /// Create audit event for authorization decisions
    pub async fn create_authz_audit_event(
        &mut self,
        user_id: &str,
        resource: &str,
        action: &str,
        granted: bool,
        risk_level: RiskLevel,
    ) -> BearDogResult<()> {
        let event = AuditEvent::Authorization {
            user_id: user_id.to_string(),
            resource: resource.to_string(),
            action: action.to_string(),
            granted,
            risk_level: format!("{risk_level:?}"),
            timestamp: Utc::now(),
        };

        self.create_audit_event(event).await
    }

    /// Get audit trail for a user
    pub async fn get_user_audit_trail(
        &self,
        user_id: &str,
        from_time: Option<chrono::DateTime<Utc>>,
        to_time: Option<chrono::DateTime<Utc>>,
    ) -> BearDogResult<Vec<SecurityAuditEvent>> {
        self.audit_manager
            .get_user_events(user_id, from_time, to_time)
            .await
    }

    /// Get audit statistics
    pub async fn get_audit_statistics(&self, period_hours: u32) -> BearDogResult<AuditStatistics> {
        let from_time = Utc::now() - Duration::hours(period_hours as i64);
        let events = self.audit_manager.get_events_since(from_time).await?;

        let total_events = events.len();
        let auth_events = events
            .iter()
            .filter(|e| e.event_type.contains("Authentication"))
            .count();
        let authz_events = events
            .iter()
            .filter(|e| e.event_type.contains("Authorization"))
            .count();
        let high_risk_events = events
            .iter()
            .filter(|e| matches!(e.risk_level, RiskLevel::High | RiskLevel::Critical))
            .count();

        Ok(AuditStatistics {
            period_hours,
            total_events,
            auth_events,
            authz_events,
            high_risk_events,
            success_rate: if total_events > 0 {
                events.iter().filter(|e| e.success).count() as f64 / total_events as f64
            } else {
                0.0
            },
        })
    }

    /// Extract audit details from event
    async fn extract_audit_details(
        &self,
        event: &AuditEvent,
    ) -> BearDogResult<HashMap<String, String>> {
        let mut details = HashMap::new();

        match event {
            AuditEvent::Authentication {
                user_id,
                success,
                failure_reason,
                ..
            } => {
                details.insert("user_id".to_string(), user_id.clone());
                details.insert("success".to_string(), success.to_string());
                if let Some(reason) = failure_reason {
                    details.insert("failure_reason".to_string(), reason.clone());
                }
            }
            AuditEvent::Authorization {
                user_id,
                resource,
                action,
                granted,
                risk_level,
                ..
            } => {
                details.insert("user_id".to_string(), user_id.clone());
                details.insert("resource".to_string(), resource.clone());
                details.insert("action".to_string(), action.clone());
                details.insert("granted".to_string(), granted.to_string());
                details.insert("risk_level".to_string(), risk_level.clone());
            }
            AuditEvent::SessionCreated {
                user_id,
                session_id,
                ..
            } => {
                details.insert("user_id".to_string(), user_id.clone());
                details.insert("session_id".to_string(), session_id.clone());
            }
            AuditEvent::SessionRevoked {
                user_id,
                session_id,
                ..
            } => {
                details.insert("user_id".to_string(), user_id.clone());
                details.insert("session_id".to_string(), session_id.clone());
            }
            AuditEvent::AccountLocked {
                user_id, reason, ..
            } => {
                details.insert("user_id".to_string(), user_id.clone());
                details.insert("reason".to_string(), reason.clone());
            }
            AuditEvent::AccountUnlocked {
                user_id, admin_id, ..
            } => {
                details.insert("user_id".to_string(), user_id.clone());
                details.insert("admin_id".to_string(), admin_id.clone());
            }
            AuditEvent::MfaTokenGenerated {
                user_id, method, ..
            } => {
                details.insert("user_id".to_string(), user_id.clone());
                details.insert("method".to_string(), method.clone());
            }
            AuditEvent::MfaTokenVerified {
                user_id, success, ..
            } => {
                details.insert("user_id".to_string(), user_id.clone());
                details.insert("success".to_string(), success.to_string());
            }
        }

        Ok(details)
    }
}

/// Audit statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditStatistics {
    pub period_hours: u32,
    pub total_events: usize,
    pub auth_events: usize,
    pub authz_events: usize,
    pub high_risk_events: usize,
    pub success_rate: f64,
}
