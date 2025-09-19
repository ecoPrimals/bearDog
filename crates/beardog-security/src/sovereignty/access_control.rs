

use crate::BearDogSecurityError;
use std::collections::{HashMap, HashSet};
use tracing::{info, warn};
use beardog_errors::BearDogError;

#[derive(std::sync::Arc<tokio::sync::RwLock<HashMap<String, AccessPolicy>>>,

    active_sessions: std::sync::Arc<tokio::sync::RwLock<HashMap<String, SecuritySession>>>,

    config: AccessControlConfig,
}

#[derive(Debug, Clone)]
    /// Number of max_concurrent_sessions
    pub max_concurrent_sessions: usize,
    /// Whether require_mfa is enabled
    pub require_mfa: bool,
    /// Whether audit_all_access is enabled
    pub audit_all_access: bool,
}

impl Default for AccessControlConfig {
    fn default(60,
            max_concurrent_sessions: 100,
            require_mfa: true,
            audit_all_access: true,
}

#[derive(Debug, Clone)]
    /// The subject value
    pub subject: String,
    /// The permissions value
    pub permissions: HashSet<Permission>,
    /// Collection of conditions
    pub conditions: Vec<AccessCondition>,
    /// The created at value
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Optional expires at
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone)]
    /// The subject value
    pub subject: String,
    /// The permissions value
    pub permissions: HashSet<Permission>,
    /// The started at value
    pub started_at: chrono::DateTime<chrono::Utc>,
    /// The last activity value
    pub last_activity: chrono::DateTime<chrono::Utc>,
    /// Whether mfa_verified is enabled
    pub mfa_verified: bool,
}

#[derive(Debug, Clone)]
    /// The value value
    pub value: String,
}

#[derive(Debug, Clone)]
/// Types of condition
pub enum ConditionType {
    /// Represents time range variant
    TimeRange,
    /// Represents ip address variant
    IpAddress,
    /// Represents user agent variant
    UserAgent,
    /// State indicating mfarequired
    MfaRequired,
    /// Represents trust level variant
    TrustLevel,
}

impl AccessController {

/// New operation.
    /// Creates a new instance
    pub fn new(config: AccessControlConfig) -> Self {
        Self {
            access_policies: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::with_capacity(16))),
            active_sessions: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::with_capacity(&str,
        permissions: HashSet<Permission>,
        conditions: Vec<AccessCondition>,
    ) -> Result<String, BearDogSecurityError> {
        let policy_id = uuid::Uuid::new_v4({} for {}", policy_id, subject);

        let policy = AccessPolicy {
            policy_id: policy_id.clone(),
            subject: subject.to_string(),
            permissions,
            conditions,
            created_at: chrono::Utc::now(None,
        };

        let mut policies = self.access_policies.write(&str,
        permission: Permission,
    ) -> Result<bool, BearDogSecurityError> {
        let policies = self.access_policies.read();

        for policy in policies.values() {
            if policy.subject == subject && policy.permissions.contains(&permission) {

                if self.evaluate_conditions(&policy.conditions)? {
                    info!("✅ Access granted for {} to {:?}", subject, permission);
                    return Ok(true);
        }

        warn!("❌ Access denied for {} to {:?}", subject, permission);
        Ok(&[AccessCondition],
    ) -> Result<bool, BearDogSecurityError> {

        for condition in conditions {
            match condition.condition_type {
                ConditionType::MfaRequired => {

                    continue;
                }
                _ => continue,
        }
        Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;}

    #[tokio::test]
    fn test_access_controller() -> Result<(), beardog_errors::BearDogError> {
        let controller = AccessController::new(AccessControlConfig::default());

        let mut permissions = HashSet::new();
        permissions.insert(Permission::Read);
        permissions.insert(Permission::Write);

        let policy_id = controller
            .create_access_policy("test-user", permissions, vec![])
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(
                    format!("Operation failed: {e:?}"))
            })?;

        assert!(!policy_id.is_empty());

        let has_access = controller
            .check_access("test-user", Permission::Read)
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(
                    format!("Operation failed: {e:?}"))
            })?;

        assert!(has_access);
        Ok(())
}
