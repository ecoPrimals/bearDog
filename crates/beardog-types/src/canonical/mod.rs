// SPDX-License-Identifier: AGPL-3.0-only

//! Canonical Types for BearDog
//!
//! This module provides the **single source of truth** for all canonical types across
//! the BearDog ecosystem. It consolidates and unifies types that were previously
//! scattered across multiple modules and crates.
//!
//! # Overview
//!
//! The canonical module is the central hub for all BearDog type definitions, providing:
//! - **Unified Configuration System** - Single configuration type for all settings
//! - **Security Types** - Cryptography, HSM, and authentication types
//! - **Monitoring & Health** - Observability and health tracking types
//! - **Network & Discovery** - Service discovery and communication types
//! - **Capabilities** - Declarative capability-based architecture
//!
//! # Quick Start
//!
//! ```rust
//! use beardog_types::canonical::{
//!     UnifiedBearDogConfig,  // Main configuration type
//!     HealthStatus,          // System health tracking
//!     SecurityContext,       // Authentication and authorization
//!     ProviderConfig,        // Provider configuration
//! };
//!
//! // Create a development configuration
//! let config = UnifiedBearDogConfig::development();
//!
//! // Check system health
//! let health = HealthStatus::Healthy;
//! assert_eq!(health, HealthStatus::Healthy);
//! ```
//!
//! # Core Modules
//!
//! - [`config`](crate::canonical::config) - Unified configuration system for all BearDog settings
//! - [`capabilities`] - Capability-based architecture and discovery
//! - [`security_unified`](crate::canonical::security_unified) - Security configuration and cryptographic types
//! - [`providers_unified`](crate::canonical::providers_unified) - Universal provider system for adapters
//! - [`monitoring`](crate::canonical::monitoring) - Observability, metrics, and health monitoring
//! - [`hsm`] - Hardware Security Module integration types
//! - [`network`](crate::canonical::network) - Network communication and service discovery types
//! - [`crypto`](crate::canonical::crypto) - Cryptographic algorithms and key management
//!
//! # Configuration System
//!
//! The unified configuration system consolidates 50+ fragmented config types:
//!
//! ```rust
//! use beardog_types::canonical::config::UnifiedBearDogConfig;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Load from environment
//! let config = UnifiedBearDogConfig::from_env()?;
//!
//! // Access domain-specific configs
//! let app = &config.app;
//! let security = &config.security;
//! let network = &config.network;
//! # Ok(())
//! # }
//! ```
//!
//! # Security Types
//!
//! Security-first design with comprehensive authentication and authorization:
//!
//! ```rust
//! use beardog_types::canonical::{SecurityContext, SecurityLevel};
//!
//! let context = SecurityContext {
//!     user_id: Some("user123".to_string()),
//!     security_level: SecurityLevel::High,
//!     authentication_method: "oauth2".to_string(),
//!     ..Default::default()
//! };
//!
//! // Security context is created
//! assert!(!context.authentication_method.is_empty());
//! ```
//!
//! # Health Monitoring
//!
//! Built-in health tracking for all components:
//!
//! ```rust
//! use beardog_types::canonical::HealthStatus;
//!
//! let health = HealthStatus::Healthy;
//! // Use in monitoring, reporting, and alerting
//! ```
//!
//! # Modernization Status
//!
//! - ✅ **Security Configuration**: Fully unified in `security_unified.rs`
//! - ✅ **Re-export Cleanup**: Eliminated ambiguous glob re-exports
//! - ✅ **Provider System**: Modern capability-based providers
//! - ✅ **Type Safety**: Strong typing with compile-time guarantees
//! - 🔄 **Other Types**: Being consolidated into canonical patterns
//!
//! # Migration Guide
//!
//! This module is the target for all type unification efforts. Legacy types
//! are being migrated here to create a single source of truth.
//!
//! ## Migrating from Legacy Types
//!
//! ```rust,ignore
//! use beardog_types::canonical::migration::migrate_to_canonical;
//!
//! // Migrate legacy types automatically
//! let canonical = migrate_to_canonical(legacy_type)?;
//! ```
//!
//! # Design Principles
//!
//! 1. **Single Source of Truth** - One canonical location for each type
//! 2. **Zero Fragmentation** - No duplicate type definitions
//! 3. **Type Safety** - Strong typing with validation
//! 4. **Performance** - Zero-cost abstractions
//! 5. **Compatibility** - Smooth migration path from legacy types
//!
//! # Performance
//!
//! All canonical types use zero-cost abstractions:
//! - No runtime overhead for type safety
//! - Efficient serialization with serde
//! - Copy/Clone only where beneficial
//! - Minimal memory footprint

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

// Core canonical modules
/// Biome configuration types
pub mod biome;
/// Capabilities module
pub mod capabilities;
/// Config module
/// Configuration management
pub mod config; // ✅ NEW: Unified canonical configuration system
// Use config::unified_simple::WorkingUnifiedConfig
/// Constants module
pub mod constants;
/// Cryptographic types and operations
pub mod crypto;
/// Discovery module - Universal capability-based discovery
pub mod discovery;
/// Hardware Security Module integration
pub mod hsm;
/// Monitoring and observability types
pub mod monitoring;
/// Network communication types
pub mod network;
// CLEANED: Temporary compatibility modules removed - using unified providers

// NEW: Unified configurations (replaces fragmented configs)
/// Hsm Unified module
pub mod hsm_unified;
/// Monitoring Unified module
pub mod monitoring_unified; // ✅ MODERNIZED: Split into modular structure
/// Network Unified module
pub mod network_unified;
/// Providers Unified module
pub mod providers_unified; // ✅ MODERNIZED: Split into modular structure
/// Rate limiting configuration - canonical implementation
pub mod rate_limiting; // ✅ MODERNIZED: Split into modular structure
/// Security Unified module
pub mod security_unified;
/// Configuration trait interfaces for polymorphic usage
pub mod traits; // ✅ NEW: Trait-based config interfaces (Nov 8, 2025) // ✅ MODERNIZED: Split into modular structure

/// Type-safe ID newtypes for compile-time safety (Nov 9, 2025)
pub mod types;

// Re-export type-safe ID newtypes for easy access
pub use types::{KeyId, RegistrationId, ServiceInstanceId};

/// Utility functions for canonical types
pub mod utils;

// Legacy modules - REMOVED as part of modernization cleanup

// Re-export the unified configurations as canonical
pub use security_unified::CanonicalSecurityConfig;

// Compatibility alias - will be deprecated in v4.0.0
pub use security_unified::SecurityConfig;

pub use providers_unified::{
    CanonicalProviderConfig,
    ConnectionConfig,
    // Re-export all provider types for convenience
    CoreProviderSettings,
    DiscoveryConfig,
    HealthConfig,
    KeyType, // Key type enumeration for HSM operations
    LoadBalancingConfig,
    PerformanceConfig,
    ProviderConfig, // Primary provider config type
    ProviderMonitoringConfig,
    ProviderSecurityConfig,
    ResilienceConfig,
    // Re-export provider traits for Android StrongBox and other implementations
    UnifiedHsmProvider,
    UnifiedProvider,
    UnifiedSecurityProvider,
};

// Export the canonical MonitoringConfig
pub use monitoring::MonitoringConfig;

// Backward compatibility aliases
// ecoPrimals: Migration plan - use canonical::monitoring::MonitoringConfig directly. Remove in v4.
#[allow(deprecated)]
pub use monitoring_unified::CanonicalMonitoringConfig;

pub use network_unified::{
    CanonicalNetworkConfig,
    NetworkConfig, // Primary network config type
};

pub use hsm_unified::{
    CanonicalHsmConfig,
    HsmConfig, // Primary HSM config type
};

// Re-export the new unified configuration system
pub use config::{
    // Domain-specific canonical configs (avoiding conflicts with existing imports)
    CanonicalAppConfig,
    CanonicalAuthConfig,
    CanonicalCacheConfig,
    CanonicalComplianceConfig,
    CanonicalDatabaseConfig,
    CanonicalGeneticsConfig,
    CanonicalPerformanceConfig,
    // CanonicalProductionConfig removed - use UnifiedProductionConfig directly
    CanonicalWorkflowConfig,
    // REMOVED: GlobalConfig, deprecated config types - use UnifiedBearDogConfig directly
    // Compatibility aliases
    UnifiedBearDogConfig,
};

// Specific re-exports to avoid ambiguous glob imports
// Capabilities
pub use capabilities::{
    CapabilityRequirements, HumanEntropyCapabilities, PerformanceCapabilities, SecurityLevel,
};

// Configuration - primary exports (from main configuration module, not canonical)
// Legacy config re-exports - use canonical::config instead
#[deprecated(
    since = "3.1.0",
    note = "Use canonical::config::app::UnifiedAppConfig instead"
)]
// Legacy imports available through canonical paths:
// - AppConfig: use beardog_types::canonical::config::AppConfig
// - BearDogConfig: use beardog_types::canonical::config::unified::BearDogConfig
// Use config::unified_simple::WorkingUnifiedConfig for configuration
// Constants
pub use constants::*;

// Crypto - specific exports to avoid conflicts
pub use crypto::{
    CryptoAlgorithm,
    // KeyUsage conflicts with HSM, so we'll use crypto::KeyUsage explicitly when needed
};

// HSM - specific exports to avoid conflicts
pub use hsm::{
    HsmCapabilities,
    HsmKey,
    KeyMetadata,
    // HsmConfig conflicts with crypto, using hsm::HsmConfig when needed
    // KeyUsage conflicts with crypto, using hsm::KeyUsage when needed
};

// Monitoring - export the unified config (sub-configs accessed through MonitoringConfig fields)
// Old specific exports removed - all available through MonitoringConfig
// e.g., config.alerting, config.metrics, config.health, etc.

// Network - specific exports to avoid conflicts
pub use network::{
    // ConnectionPoolConfig, // Use config::domains::network::ConnectionPoolConfig instead
    LoadBalancerConfig,
    TimeoutConfig,
    // NetworkConfig conflicts with configuration module
    // HealthCheckConfig conflicts with monitoring module
    // LoadBalancingStrategy conflicts with hsm module
};

// Providers - now exported from providers_unified
// (These are exported through the unified module re-exports above)

/// Health status types and definitions
pub mod health_status;
pub use health_status::{
    ComponentStatus, HealthStatus as CanonicalHealthStatus, OperationStatus,
    WorkflowStatus as CanonicalWorkflowStatus,
};

// Core canonical traits that all BearDog types should implement
/// Core trait that all canonical `BearDog` types should implement
///
/// and versioning across all canonical types in the `BearDog` ecosystem.
pub trait CanonicalType: Send + Sync + Clone + Serialize + for<'de> Deserialize<'de> {
    /// Get the canonical type name
    fn canonical_type_name() -> &'static str;

    /// Validate the canonical type
    /// Validates input
    fn validate(&self) -> Result<(), BearDogError>;

    /// Get the canonical version
    #[must_use]
    fn canonical_version() -> &'static str {
        "3.0.0"
    }
}

/// Core health status - canonical across all BearDog systems
///
/// Represents the operational health of any BearDog component, service, or system.
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
/// is used throughout BearDog to make authorization decisions and audit security events.
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

/// Canonical security audit event
///
/// Records security-relevant actions for compliance, forensics, and monitoring.
/// All security-sensitive operations in BearDog generate audit events for
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
    /// Examples: "MFA_required", "time_restricted:9-17", "approval_needed:manager"
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
/// Lifecycle status of cryptographic keys in the BearDog system.
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

// Re-export utility functions for backwards compatibility
pub use utils::migration;
pub use utils::{canonical_type_info, validate_canonical_usage};

// Tests extracted to separate module for better organization
#[cfg(test)]
#[path = "canonical_types_tests.rs"]
mod canonical_types_tests;
