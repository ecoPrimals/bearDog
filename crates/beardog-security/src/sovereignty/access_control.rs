//! # Access Control
//!
//! This module provides access control and policy management for the BearDog ecosystem.

use crate::BearDogSecurityError;
use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

// ============================================================
// Configuration
// ============================================================

/// Access control configuration
#[derive(Debug, Clone)]
pub struct AccessControlConfig {
    /// Session timeout in minutes
    pub session_timeout_minutes: u64,

    /// Maximum concurrent sessions
    pub max_concurrent_sessions: usize,

    /// Require MFA for access
    pub require_mfa: bool,

    /// Audit all access attempts
    pub audit_all_access: bool,
}

impl Default for AccessControlConfig {
    fn default() -> Self {
        Self {
            session_timeout_minutes: 60,
            max_concurrent_sessions: 100,
            require_mfa: true,
            audit_all_access: true,
        }
    }
}

// ============================================================
// Permission Types
// ============================================================

/// Permission for access control
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permission {
    /// Read access
    Read,

    /// Write access
    Write,

    /// Execute access
    Execute,

    /// Delete access
    Delete,

    /// Admin access
    Admin,

    /// Audit access
    Audit,
}

/// Access condition type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionType {
    /// Time range restriction
    TimeRange,

    /// IP address restriction
    IpAddress,

    /// User agent restriction
    UserAgent,

    /// MFA required
    MfaRequired,

    /// Trust level required
    TrustLevel,
}

/// Access condition
#[derive(Debug, Clone)]
pub struct AccessCondition {
    /// Condition type
    pub condition_type: ConditionType,

    /// Condition value
    pub value: String,
}

// ============================================================
// Policy and Session Types
// ============================================================

/// Access policy
#[derive(Debug, Clone)]
pub struct AccessPolicy {
    /// Policy identifier
    pub policy_id: String,

    /// Subject (user/entity)
    pub subject: String,

    /// Granted permissions
    pub permissions: HashSet<Permission>,

    /// Conditions for access
    pub conditions: Vec<AccessCondition>,

    /// When the policy was created
    pub created_at: DateTime<Utc>,

    /// When the policy expires (if applicable)
    pub expires_at: Option<DateTime<Utc>>,
}

/// Security session
#[derive(Debug, Clone)]
pub struct SecuritySession {
    /// Session identifier
    pub session_id: String,

    /// Subject (user/entity)
    pub subject: String,

    /// Active permissions
    pub permissions: HashSet<Permission>,

    /// When the session started
    pub started_at: DateTime<Utc>,

    /// Last activity timestamp
    pub last_activity: DateTime<Utc>,

    /// Whether MFA has been verified
    pub mfa_verified: bool,
}

// ============================================================
// Access Controller
// ============================================================

/// Access controller service
#[derive(Debug)]
pub struct AccessController {
    /// Access policies by policy ID
    access_policies: Arc<RwLock<HashMap<String, AccessPolicy>>>,

    /// Active sessions by session ID
    active_sessions: Arc<RwLock<HashMap<String, SecuritySession>>>,

    /// Configuration
    config: AccessControlConfig,
}

impl AccessController {
    /// Create a new access controller
    pub fn new(config: AccessControlConfig) -> Self {
        Self {
            access_policies: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            active_sessions: Arc::new(RwLock::new(HashMap::with_capacity(
                config.max_concurrent_sessions,
            ))),
            config,
        }
    }

    /// Create an access policy
    ///
    /// # Arguments
    /// * `subject` - Subject (user/entity)
    /// * `permissions` - Permissions to grant
    /// * `conditions` - Conditions for access
    pub async fn create_access_policy(
        &self,
        subject: &str,
        permissions: HashSet<Permission>,
        conditions: Vec<AccessCondition>,
    ) -> Result<String, BearDogSecurityError> {
        let policy_id = uuid::Uuid::new_v4().to_string();

        info!("📝 Creating access policy {} for {}", policy_id, subject);

        let policy = AccessPolicy {
            policy_id: policy_id.clone(),
            subject: subject.to_string(),
            permissions,
            conditions,
            created_at: Utc::now(),
            expires_at: None,
        };

        let mut policies = self.access_policies.write().await;
        policies.insert(policy_id.clone(), policy);

        Ok(policy_id)
    }

    /// Check if a subject has access to a permission
    ///
    /// # Arguments
    /// * `subject` - Subject to check
    /// * `permission` - Permission to check
    pub async fn check_access(
        &self,
        subject: &str,
        permission: Permission,
    ) -> Result<bool, BearDogSecurityError> {
        let policies = self.access_policies.read().await;

        for policy in policies.values() {
            if policy.subject == subject && policy.permissions.contains(&permission) {
                // Check conditions
                if self.evaluate_conditions(&policy.conditions).await? {
                    if self.config.audit_all_access {
                        info!("✅ Access granted for {} to {:?}", subject, permission);
                    }
                    return Ok(true);
                }
            }
        }

        if self.config.audit_all_access {
            warn!("❌ Access denied for {} to {:?}", subject, permission);
        }
        Ok(false)
    }

    /// Delete an access policy
    ///
    /// # Arguments
    /// * `policy_id` - Policy to delete
    pub async fn delete_policy(&self, policy_id: &str) -> Result<bool, BearDogSecurityError> {
        let mut policies = self.access_policies.write().await;
        let removed = policies.remove(policy_id).is_some();
        if removed {
            info!("🗑️ Deleted access policy: {}", policy_id);
        }
        Ok(removed)
    }

    /// Create a security session
    ///
    /// # Arguments
    /// * `subject` - Subject for the session
    /// * `permissions` - Initial permissions
    pub async fn create_session(
        &self,
        subject: &str,
        permissions: HashSet<Permission>,
    ) -> Result<String, BearDogSecurityError> {
        // Check session limit
        let sessions = self.active_sessions.read().await;
        if sessions.len() >= self.config.max_concurrent_sessions {
            return Err(BearDogSecurityError::ResourceExhausted(
                "Maximum concurrent sessions reached".to_string(),
            ));
        }
        drop(sessions);

        let session_id = uuid::Uuid::new_v4().to_string();

        let session = SecuritySession {
            session_id: session_id.clone(),
            subject: subject.to_string(),
            permissions,
            started_at: Utc::now(),
            last_activity: Utc::now(),
            mfa_verified: false,
        };

        let mut sessions = self.active_sessions.write().await;
        sessions.insert(session_id.clone(), session);

        info!("🔐 Created session {} for {}", session_id, subject);
        Ok(session_id)
    }

    /// End a security session
    ///
    /// # Arguments
    /// * `session_id` - Session to end
    pub async fn end_session(&self, session_id: &str) -> Result<bool, BearDogSecurityError> {
        let mut sessions = self.active_sessions.write().await;
        let removed = sessions.remove(session_id).is_some();
        if removed {
            info!("🔓 Ended session: {}", session_id);
        }
        Ok(removed)
    }

    /// Update session activity
    pub async fn update_session_activity(
        &self,
        session_id: &str,
    ) -> Result<(), BearDogSecurityError> {
        let mut sessions = self.active_sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.last_activity = Utc::now();
            Ok(())
        } else {
            Err(BearDogSecurityError::NotFound(format!(
                "Session not found: {}",
                session_id
            )))
        }
    }

    /// Verify MFA for a session
    pub async fn verify_mfa(&self, session_id: &str) -> Result<(), BearDogSecurityError> {
        let mut sessions = self.active_sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.mfa_verified = true;
            info!("✅ MFA verified for session: {}", session_id);
            Ok(())
        } else {
            Err(BearDogSecurityError::NotFound(format!(
                "Session not found: {}",
                session_id
            )))
        }
    }

    // Private helper methods

    async fn evaluate_conditions(
        &self,
        conditions: &[AccessCondition],
    ) -> Result<bool, BearDogSecurityError> {
        for condition in conditions {
            match condition.condition_type {
                ConditionType::MfaRequired => {
                    // MFA check would be done at session level
                    continue;
                }
                ConditionType::TimeRange => {
                    // Time range validation would be implemented here
                    continue;
                }
                ConditionType::IpAddress => {
                    // IP address validation would be implemented here
                    continue;
                }
                ConditionType::UserAgent => {
                    // User agent validation would be implemented here
                    continue;
                }
                ConditionType::TrustLevel => {
                    // Trust level validation would be implemented here
                    continue;
                }
            }
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_access_controller() -> Result<(), BearDogError> {
        let controller = AccessController::new(AccessControlConfig::default());

        let mut permissions = HashSet::new();
        permissions.insert(Permission::Read);
        permissions.insert(Permission::Write);

        let policy_id = controller
            .create_access_policy("test-user", permissions, vec![])
            .await
            .map_err(|e| BearDogError::internal(format!("Operation failed: {e:?}")))?;

        assert!(!policy_id.is_empty());

        let has_access = controller
            .check_access("test-user", Permission::Read)
            .await
            .map_err(|e| BearDogError::internal(format!("Operation failed: {e:?}")))?;

        assert!(has_access);
        Ok(())
    }

    #[tokio::test]
    async fn test_access_denied() -> Result<(), BearDogError> {
        let controller = AccessController::new(AccessControlConfig::default());

        let mut permissions = HashSet::new();
        permissions.insert(Permission::Read);

        controller
            .create_access_policy("test-user", permissions, vec![])
            .await
            .map_err(|e| BearDogError::internal(format!("Operation failed: {e:?}")))?;

        // Should be denied for Write permission
        let has_access = controller
            .check_access("test-user", Permission::Write)
            .await
            .map_err(|e| BearDogError::internal(format!("Operation failed: {e:?}")))?;

        assert!(!has_access);
        Ok(())
    }

    #[tokio::test]
    async fn test_session_management() -> Result<(), BearDogError> {
        let controller = AccessController::new(AccessControlConfig::default());

        let permissions = HashSet::new();
        let session_id = controller
            .create_session("test-user", permissions)
            .await
            .map_err(|e| BearDogError::internal(format!("Operation failed: {e:?}")))?;

        assert!(!session_id.is_empty());

        let ended = controller
            .end_session(&session_id)
            .await
            .map_err(|e| BearDogError::internal(format!("Operation failed: {e:?}")))?;

        assert!(ended);
        Ok(())
    }

    #[test]
    fn test_config_default() {
        let config = AccessControlConfig::default();
        assert_eq!(config.session_timeout_minutes, 60);
        assert!(config.require_mfa);
        assert!(config.audit_all_access);
    }
}
