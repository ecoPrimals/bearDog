// SPDX-License-Identifier: AGPL-3.0-or-later

//! Core canonical trait and foundational types (`HealthStatus`, `SessionConfig`, `SecurityContext`).

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

use super::capabilities;

// Core canonical traits that all BearDog types should implement
/// Core trait that all canonical `BearDog` types should implement
///
/// and versioning across all canonical types in the `BearDog` ecosystem.
pub trait CanonicalType: Send + Sync + Clone + Serialize + for<'de> Deserialize<'de> {
    /// Get the canonical type name
    fn canonical_type_name() -> &'static str;

    /// Validate the canonical type
    /// Validates input
    ///
    /// # Errors
    ///
    /// Returns an error if the value fails domain-specific validation.
    fn validate(&self) -> Result<(), BearDogError>;

    /// Get the canonical version
    #[must_use]
    fn canonical_version() -> &'static str {
        "3.0.0"
    }
}

/// Core health status - canonical across all `BearDog` systems
///
/// Represents the operational health of any `BearDog` component, service, or system.
/// This type is used throughout the ecosystem for consistent health tracking and reporting.
///
/// # Health States
///
/// - [`Healthy`](Self::Healthy) - System is fully operational with no issues
/// - [`Degraded`](Self::Degraded) - System is operational but with reduced performance
/// - [`Unhealthy`](Self::Unhealthy) - System is not operational or has critical issues
/// - [`Unknown`](Self::Unknown) - System status cannot be determined
///
/// # Usage
///
/// ```rust,no_run
/// use beardog_types::canonical::HealthStatus;
///
/// // Check if system is operational
/// let status = HealthStatus::Healthy;
/// assert!(matches!(status, HealthStatus::Healthy | HealthStatus::Degraded));
///
/// // Use in monitoring
/// fn should_alert(status: &HealthStatus) -> bool {
///     matches!(status, HealthStatus::Unhealthy)
/// }
///
/// assert!(!should_alert(&HealthStatus::Healthy));
/// assert!(should_alert(&HealthStatus::Unhealthy));
/// ```
///
/// # Integration
///
/// This type integrates with:
/// - System monitoring and alerting
/// - Load balancers for routing decisions
/// - Health check endpoints
/// - Service discovery for availability
///
/// # Thread Safety
///
/// `HealthStatus` is `Send + Sync` and can be safely shared across threads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum HealthStatus {
    /// System is fully operational
    Healthy,
    /// System is operational but with reduced performance or functionality
    Degraded,
    /// System is not operational
    Unhealthy,
    /// System status is unknown
    #[default]
    Unknown,
}

impl CanonicalType for HealthStatus {
    fn canonical_type_name() -> &'static str {
        "HealthStatus"
    }

    /// Validates input
    fn validate(&self) -> Result<(), BearDogError> {
        // Health status is always valid
        Ok(())
    }
}

/// Canonical session configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Session timeout in seconds
    pub timeout_seconds: u64,
    /// Whether to use secure cookies
    /// Whether `secure_cookies` is enabled
    pub secure_cookies: bool,
    /// The same site policy value
    pub same_site_policy: String,
    /// The storage backend value
    pub storage_backend: String,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 3600, // 1 hour
            secure_cookies: true,
            same_site_policy: "Strict".to_string(),
            storage_backend: "database".to_string(),
        }
    }
}

impl CanonicalType for SessionConfig {
    fn canonical_type_name() -> &'static str {
        "SessionConfig"
    }

    /// Validates input
    fn validate(&self) -> Result<(), BearDogError> {
        if self.timeout_seconds == 0 {
            return Err(BearDogError::business(
                "Session timeout must be greater than 0".to_string(),
            ));
        }
        Ok(())
    }
}

/// Security context containing authentication and authorization information
///
/// Represents the complete security state for a user, session, or operation. This type
/// is used throughout `BearDog` to make authorization decisions and audit security events.
///
/// # Overview
///
/// `SecurityContext` provides:
/// - **Authentication State** - Who is making the request
/// - **Authorization Data** - What they're allowed to do
/// - **Security Level** - How much trust to grant
/// - **Audit Trail** - When and how they authenticated
///
/// # Usage
///
/// ```rust,no_run
/// use beardog_types::canonical::{SecurityContext, SecurityLevel, CanonicalType};
///
/// // Create a security context
/// let context = SecurityContext {
///     user_id: Some("user123".to_string()),
///     session_id: Some("session_abc".to_string()),
///     permissions: vec!["read".to_string(), "write".to_string()],
///     security_level: SecurityLevel::High,
///     authentication_method: "oauth2".to_string(),
///     timestamp: chrono::Utc::now(),
/// };
///
/// // Check permissions
/// fn can_write(ctx: &SecurityContext) -> bool {
///     ctx.permissions.contains(&"write".to_string())
/// }
///
/// assert!(can_write(&context));
///
/// // Validate the context
/// assert!(context.validate().is_ok());
/// ```
///
/// # Security Levels
///
/// Different operations require different security levels:
/// - **Low** - Public or read-only operations
/// - **Standard** - Normal authenticated operations
/// - **High** - Sensitive data access
/// - **Critical** - System administration or key operations
///
/// # Authentication Methods
///
/// Common authentication methods:
/// - `"password"` - Username/password authentication
/// - `"oauth2"` - OAuth 2.0 token
/// - `"jwt"` - JSON Web Token
/// - `"api_key"` - API key authentication
/// - `"certificate"` - Client certificate
/// - `"hsm"` - Hardware security module
///
/// # Validation
///
/// Always validate security contexts before use:
///
/// ```rust,no_run
/// use beardog_types::canonical::{SecurityContext, CanonicalType};
///
/// let context = SecurityContext::default();
/// // Validation will fail if authentication_method is empty
/// assert!(context.validate().is_err());
/// ```
///
/// # Thread Safety
///
/// `SecurityContext` is `Send + Sync` and can be safely shared across threads
/// for concurrent authorization checks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityContext {
    /// User identifier (if authenticated)
    pub user_id: Option<String>,
    /// Session identifier (if session-based auth)
    pub session_id: Option<String>,
    /// List of granted permissions
    /// Collection of permissions
    pub permissions: Vec<String>,
    /// The security level value
    pub security_level: capabilities::SecurityLevel,
    /// The authentication method value
    pub authentication_method: String,
    /// Timestamp when context was created
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Default for SecurityContext {
    fn default() -> Self {
        Self {
            user_id: None,
            session_id: None,
            permissions: Vec::new(),
            security_level: capabilities::SecurityLevel::Standard,
            authentication_method: "none".to_string(),
            timestamp: chrono::Utc::now(),
        }
    }
}

impl CanonicalType for SecurityContext {
    fn canonical_type_name() -> &'static str {
        "SecurityContext"
    }

    /// Validates input
    fn validate(&self) -> Result<(), BearDogError> {
        if self.authentication_method.is_empty() {
            return Err(BearDogError::security(
                "Authentication method cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}
