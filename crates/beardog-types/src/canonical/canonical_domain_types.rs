// SPDX-License-Identifier: AGPL-3.0-only

//! Audit, policy, key lifecycle, and workflow status types used across canonical APIs.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

use super::canonical_core::CanonicalType;

/// Canonical security audit event
///
/// Records security-relevant actions for compliance, forensics, and monitoring.
/// All security-sensitive operations in `BearDog` generate audit events for
/// accountability and incident response.
///
/// # Overview
///
/// Audit events provide:
/// - **Complete Audit Trail** - Who did what, when, and why
/// - **Compliance Support** - GDPR, HIPAA, SOC 2 requirements
/// - **Security Monitoring** - Detect suspicious activity
/// - **Forensic Analysis** - Investigate security incidents
///
/// # Usage
///
/// ```rust,no_run
/// use beardog_types::canonical::{SecurityAuditEvent, AuditOutcome};
/// use std::collections::HashMap;
///
/// // Record a successful login
/// let event = SecurityAuditEvent {
///     event_id: uuid::Uuid::new_v4().to_string(),
///     event_type: "authentication".to_string(),
///     user_id: Some("user123".to_string()),
///     resource: "/api/login".to_string(),
///     action: "login".to_string(),
///     outcome: AuditOutcome::Success,
///     timestamp: chrono::Utc::now(),
///     metadata: HashMap::from([
///         ("ip_address".to_string(), serde_json::json!("192.168.1.1")),
///         ("user_agent".to_string(), serde_json::json!("Mozilla/5.0")),
///     ]),
/// };
///
/// // Event is ready to log
/// assert!(!event.event_id.is_empty());
/// ```
///
/// # Event Types
///
/// Common event types:
/// - `"authentication"` - Login, logout, token refresh
/// - `"authorization"` - Permission checks, access denials
/// - `"data_access"` - Read operations on sensitive data
/// - `"data_modification"` - Create, update, delete operations
/// - `"configuration"` - System configuration changes
/// - `"key_operation"` - Cryptographic key usage
///
/// # Audit Outcomes
///
/// - [`Success`](AuditOutcome::Success) - Operation completed successfully
/// - [`Failure`](AuditOutcome::Failure) - Operation failed (error)
/// - [`Denied`](AuditOutcome::Denied) - Operation denied (authorization)
/// - [`Error`](AuditOutcome::Error) - System error occurred
///
/// # Metadata
///
/// Include relevant context in metadata:
///
/// ```rust,no_run
/// use beardog_types::canonical::SecurityAuditEvent;
/// use std::collections::HashMap;
///
/// let mut metadata = HashMap::new();
/// metadata.insert("ip_address".to_string(), serde_json::json!("192.168.1.1"));
/// metadata.insert("session_id".to_string(), serde_json::json!("abc123"));
/// metadata.insert("risk_score".to_string(), serde_json::json!(0.2));
/// ```
///
/// # Compliance
///
/// Audit events support regulatory requirements:
/// - **GDPR** - Right to access, data processing logs
/// - **HIPAA** - Access to PHI, audit controls
/// - **SOC 2** - Logging and monitoring controls
/// - **PCI DSS** - Payment card data access logs
///
/// # Thread Safety
///
/// `SecurityAuditEvent` is `Send + Sync` for concurrent audit logging.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAuditEvent {
    /// Unique identifier for the audit event
    pub event_id: String,
    /// Type of security event (e.g., login, access, modification)
    /// The event type value
    pub event_type: String,
    /// User identifier associated with the event (if authenticated)
    pub user_id: Option<String>,
    /// Resource that was accessed or modified
    /// The resource value
    pub resource: String,
    /// The action value
    pub action: String,
    /// Outcome of the audited action
    /// The outcome value
    pub outcome: AuditOutcome,
    /// When the event occurred
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Additional event-specific metadata
    /// The metadata value
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
}

/// Audit outcome
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditOutcome {
    /// Operation completed successfully
    Success,
    /// Operation failed due to an error
    Failure,
    /// Operation was denied due to insufficient permissions
    Denied,
    /// Operation encountered a system error
    Error,
}

impl Default for SecurityAuditEvent {
    fn default() -> Self {
        Self {
            event_id: uuid::Uuid::new_v4().to_string(),
            event_type: "unknown".to_string(),
            user_id: None,
            resource: "unknown".to_string(),
            action: "unknown".to_string(),
            outcome: AuditOutcome::Error,
            timestamp: chrono::Utc::now(),
            metadata: std::collections::HashMap::new(),
        }
    }
}

impl CanonicalType for SecurityAuditEvent {
    fn canonical_type_name() -> &'static str {
        "SecurityAuditEvent"
    }

    /// Validates input
    fn validate(&self) -> Result<(), BearDogError> {
        if self.event_id.is_empty() {
            return Err(BearDogError::business(
                "Event ID cannot be empty".to_string(),
            ));
        }
        if self.event_type.is_empty() {
            return Err(BearDogError::business(
                "Event type cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}

/// Canonical policy decision
///
/// Result of an access control or authorization policy evaluation.
///
/// # Variants
///
/// * `Allow` - Permission granted unconditionally
/// * `Deny` - Permission explicitly denied (default for security)
/// * `Conditional(String)` - Permission granted with conditions
///
/// # Examples
///
/// ```rust,no_run
/// use beardog_types::canonical::PolicyDecision;
///
/// // Unconditional allow
/// let decision = PolicyDecision::Allow;
/// assert!(matches!(decision, PolicyDecision::Allow));
///
/// // Explicit deny (most secure default)
/// let decision = PolicyDecision::default();
/// assert!(matches!(decision, PolicyDecision::Deny));
///
/// // Conditional access
/// let decision = PolicyDecision::Conditional("MFA_required".to_string());
/// match decision {
///     PolicyDecision::Conditional(condition) => {
///         println!("Access granted with condition: {}", condition);
///     }
///     _ => {}
/// }
/// ```
///
/// # Use Cases
///
/// - **API Access Control**: Determine if a user can call an endpoint
/// - **Resource Permissions**: Check if an operation is allowed on a resource
/// - **Policy Engines**: Result of complex policy evaluation
/// - **Conditional Access**: Grant access with additional requirements (MFA, time-based, etc.)
///
/// # Security Best Practices
///
/// - Default to `Deny` for security (fail-closed)
/// - Use `Conditional` for step-up authentication scenarios
/// - Always validate conditions before granting access
/// - Log all `Deny` decisions for security auditing
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PolicyDecision {
    /// Allow the requested action
    ///
    /// Permission is granted unconditionally. Use when all policy checks pass.
    Allow,

    /// Deny the requested action
    ///
    /// Permission is explicitly denied. This is the default for security (fail-closed).
    #[default]
    Deny,

    /// Allow with conditions (condition details in string)
    ///
    /// Permission granted but additional requirements must be met.
    /// Examples: "`MFA_required`", "time_restricted:9-17", "`approval_needed:manager`"
    Conditional(String),
}

impl CanonicalType for PolicyDecision {
    fn canonical_type_name() -> &'static str {
        "PolicyDecision"
    }

    /// Validates input
    fn validate(&self) -> Result<(), BearDogError> {
        match self {
            Self::Conditional(condition) if condition.is_empty() => Err(BearDogError::business(
                "Conditional policy decision must have a condition".to_string(),
            )),
            _ => Ok(()),
        }
    }
}

/// Canonical key status
///
/// Lifecycle status of cryptographic keys in the `BearDog` system.
///
/// # Key Lifecycle
///
/// ```text
/// Pending → Active → Inactive → Revoked
///     ↓         ↓         ↓
///    (can be activated) (can be reactivated)
///     ↓
/// Expired (time-based)
/// ```
///
/// # Examples
///
/// ```rust,no_run
/// use beardog_types::canonical::KeyStatus;
///
/// // New key starts as pending
/// let status = KeyStatus::default();
/// assert!(matches!(status, KeyStatus::Pending));
///
/// // Activate key for use
/// let status = KeyStatus::Active;
///
/// // Check if key can be used
/// let can_use = matches!(status, KeyStatus::Active);
/// assert!(can_use);
///
/// // Revoked keys cannot be reactivated
/// let status = KeyStatus::Revoked;
/// assert!(!matches!(status, KeyStatus::Active | KeyStatus::Inactive));
/// ```
///
/// # Status Meanings
///
/// * **Pending**: Key created but not yet approved/activated
/// * **Active**: Key is valid and can be used for operations
/// * **Inactive**: Key temporarily disabled, can be reactivated
/// * **Expired**: Key exceeded its validity period
/// * **Revoked**: Key permanently disabled (security breach, compromise, etc.)
///
/// # Security Implications
///
/// - **Active**: Only status where key operations are allowed
/// - **Inactive**: Temporary disablement, can be restored
/// - **Revoked**: Permanent disablement, cannot be restored
/// - **Expired**: Time-based disablement, may need rotation
/// - **Pending**: Awaiting approval, not yet trusted
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum KeyStatus {
    /// Key is currently active and valid for use
    ///
    /// This is the only status where cryptographic operations are permitted.
    Active,

    /// Key is inactive but not revoked
    ///
    /// Temporarily disabled; can be reactivated if needed.
    /// Use for maintenance or temporary suspension.
    Inactive,

    /// Key has been revoked and cannot be used
    ///
    /// **Permanent disablement**. Use when key is compromised or no longer trusted.
    /// Revoked keys cannot be reactivated.
    Revoked,

    /// Key has expired and is no longer valid
    ///
    /// Time-based expiration. Key exceeded its validity period.
    /// May need to rotate to a new key.
    Expired,

    /// Key is pending activation or approval
    ///
    /// Initial status for newly created keys. Awaiting approval or activation.
    /// Default status for security (keys are not trusted until explicitly activated).
    #[default]
    Pending,
}

impl CanonicalType for KeyStatus {
    fn canonical_type_name() -> &'static str {
        "KeyStatus"
    }

    /// Validates input
    fn validate(&self) -> Result<(), BearDogError> {
        // All key statuses are valid
        Ok(())
    }
}

/// Canonical workflow status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum WorkflowStatus {
    /// Workflow is pending execution
    #[default]
    Pending,
    /// Workflow is currently executing
    InProgress,
    /// Workflow has completed successfully
    Completed,
    /// Workflow execution failed
    Failed,
    /// Workflow was cancelled before completion
    Cancelled,
}

impl CanonicalType for WorkflowStatus {
    fn canonical_type_name() -> &'static str {
        "WorkflowStatus"
    }

    /// Validates input
    fn validate(&self) -> Result<(), BearDogError> {
        // All workflow statuses are valid
        Ok(())
    }
}
