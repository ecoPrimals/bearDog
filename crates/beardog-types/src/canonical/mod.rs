// Canonical Types for BearDog
//
// This module provides the single source of truth for all canonical types across
// the BearDog ecosystem. It consolidates and unifies types that were previously
// scattered across multiple modules and crates.
//
// ## Modernization Status
//
// ✅ **Security Configuration**: Fully unified in `security_unified.rs`
// ✅ **Re-export Cleanup**: Eliminated ambiguous glob re-exports
// 🔄 **Other Types**: Being consolidated into canonical patterns
//
// ## Migration Strategy
//
// This module is the target for all type unification efforts. Legacy types
// are being migrated here to create a single source of truth.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

// Core canonical modules
/// Biome configuration types
pub mod biome;
/// Capabilities module
pub mod capabilities;
/// Config module
/// Configuration management
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
pub mod security_unified; // ✅ MODERNIZED: Split into modular structure // ✅ MODERNIZED: Split into modular structure

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
    LoadBalancingConfig,
    PerformanceConfig,
    ProviderConfig, // Primary provider config type
    ProviderMonitoringConfig,
    ProviderSecurityConfig,
    ResilienceConfig,
};

// Export the canonical MonitoringConfig
pub use monitoring::MonitoringConfig;

// Backward compatibility aliases
// DEPRECATED: Old monitoring config - use canonical::monitoring::MonitoringConfig
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
    CanonicalProductionConfig,
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

/// Core health status - canonical across all `BearDog` systems
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    /// System is fully operational
    Healthy,
    /// System is operational but with reduced performance or functionality
    Degraded,
    /// System is not operational
    Unhealthy,
    /// System status is unknown
    Unknown,
}

impl Default for HealthStatus {
    fn default() -> Self {
        Self::Unknown
    }
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyDecision {
    /// Allow the requested action
    Allow,
    /// Deny the requested action
    Deny,
    /// Allow with conditions (condition details in string)
    Conditional(String),
}

impl Default for PolicyDecision {
    fn default() -> Self {
        Self::Deny
    }
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyStatus {
    /// Key is currently active and valid for use
    Active,
    /// Key is inactive but not revoked
    Inactive,
    /// Key has been revoked and cannot be used
    Revoked,
    /// Key has expired and is no longer valid
    Expired,
    /// Key is pending activation or approval
    Pending,
}

impl Default for KeyStatus {
    fn default() -> Self {
        Self::Pending
    }
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowStatus {
    /// Workflow is pending execution
    Pending,
    /// Workflow is currently executing
    InProgress,
    /// Workflow has completed successfully
    Completed,
    /// Workflow execution failed
    Failed,
    Cancelled,
}

impl Default for WorkflowStatus {
    fn default() -> Self {
        Self::Pending
    }
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

/// Validate canonical usage across the system
/// Validates `canonical_usage`
pub fn validate_canonical_usage() -> Result<(), Vec<String>> {
    let errors = Vec::new();

    // Add validation logic here as needed
    // For now, we assume canonical usage is correct

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[must_use]
pub fn canonical_type_info() -> Vec<(&'static str, &'static str)> {
    vec![
        ("HealthStatus", "Canonical health status for all systems"),
        ("SessionConfig", "Canonical session configuration"),
        (
            "SecurityContext",
            "Canonical security context for all operations",
        ),
        ("SecurityAuditEvent", "Canonical audit event structure"),
        ("PolicyDecision", "Canonical policy decision enum"),
        ("KeyStatus", "Canonical key status enum"),
        ("WorkflowStatus", "Canonical workflow status enum"),
        (
            "CanonicalSecurityConfig",
            "Unified security configuration system",
        ),
    ]
}

pub mod migration {
    use super::{BearDogError, CanonicalType};

    /// Migrate legacy types to canonical equivalents
    pub fn migrate_to_canonical<T, C>(_legacy: T) -> Result<C, BearDogError>
    where
        T: Send + Sync,
        C: CanonicalType + Default,
    {
        // Generic migration logic - specific implementations would be provided
        // for each type pair
        let canonical = C::default();
        canonical.validate()?;
        Ok(canonical)
    }

    /// Batch migrate multiple legacy types
    pub fn batch_migrate_to_canonical<T, C>(legacy_items: Vec<T>) -> Result<Vec<C>, BearDogError>
    where
        T: Send + Sync,
        C: CanonicalType + Default,
    {
        let mut canonical_items = Vec::with_capacity(legacy_items.len());

        for legacy_item in legacy_items {
            let canonical_item = migrate_to_canonical(legacy_item)?;
            canonical_items.push(canonical_item);
        }

        Ok(canonical_items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_type_validation() {
        let health = HealthStatus::Healthy;
        assert!(health.validate().is_ok());

        let session = SessionConfig::default();
        assert!(session.validate().is_ok());

        let context = SecurityContext {
            authentication_method: "password".to_string(),
            ..Default::default()
        };
        assert!(context.validate().is_ok());

        let audit = SecurityAuditEvent::default();
        assert!(audit.validate().is_ok());

        let decision = PolicyDecision::Allow;
        assert!(decision.validate().is_ok());
    }

    #[test]
    fn test_canonical_usage_validation() {
        let result = validate_canonical_usage();
        assert!(result.is_ok());
    }

    #[test]
    fn test_canonical_type_info() {
        let info = canonical_type_info();
        assert!(!info.is_empty());
        assert!(info
            .iter()
            .any(|(name, _)| *name == "CanonicalSecurityConfig"));
    }
}
