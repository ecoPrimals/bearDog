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


/// # Canonical Types - Modular Organization
///
/// **SINGLE SOURCE OF TRUTH** for all `BearDog` types, now organized into focused modules
/// to maintain the 2000-line file limit while preserving the canonical type system.
/// ## Module Organization
/// - `health_status` - Health and status enums (~145 lines)
/// - `configuration` - Configuration structures (~360 lines)
/// - `security` - Security and authorization types (~425 lines)
/// - `workflow` - Workflow and process types (~195 lines)
/// - `network` - Network and communication types (~204 lines)
/// - `metrics` - Metrics and monitoring types (~136 lines)
/// - `providers` - Provider interface types (~145 lines)
/// - `crypto` - Cryptographic types (~102 lines)
/// - `hsm` - HSM key and hardware security module types (~250 lines)
/// - `genetics` - Genetics configuration and types (~120 lines)
/// - `constants` - System constants consolidated from across codebase (~180 lines)
/// - `capabilities` - Capability definitions and types (~120 lines)
use serde::{Deserialize, Serialize};

// Module declarations
pub mod capabilities;
pub mod configuration;
pub mod constants;
pub mod crypto;
pub mod genetics;
pub mod health_status;
pub mod hsm;
pub mod metrics;
pub mod monitoring;
pub mod network;
pub mod providers;
pub mod security;
pub mod services;
pub mod workflow;
// Re-export capabilities types (excluding conflicting ones)
pub use capabilities::{
    CapabilityRequirements, HumanEntropyCapabilities, PerformanceCapabilities,
    SecurityLevel as CapabilitiesSecurityLevel,
};
// Configuration exports with explicit conflict resolution
pub use configuration::{
    BearDogConfig,
    DatabaseConfig,
    MonitoringConfig,
    NetworkSecurityConfig,
    // Import only types that actually exist
    SecurityConfig,
};


pub use genetics::*;
pub use crate::constants::unified::network::limits::MAX_CONNECTIONS;
pub use crypto::{KeyType, EncryptionAlgorithm, KeyUsage, CryptoParams};
pub use health_status::*;
pub use hsm::{HsmCapabilities, HsmKey, KeyMetadata};
// Key types are already available from the hsm module above
pub use metrics::*;
pub use monitoring::{
    AlertConfig, AlertSeverity, HealthCheckConfig, HealthCheckResult, MonitoringMetrics,
    NetworkUsage, RequestMetrics, ServiceHealthMonitor,
};


pub use services::*;
// Network exports with explicit aliases to avoid conflicts
pub use network::{
    CircuitBreakerConfig, ConnectionPoolConfig as NetworkConnectionPoolConfig,
    FailoverConfig as NetworkFailoverConfig, HealthCheckConfig as NetworkHealthCheckConfig,
    LoadBalancingConfig as NetworkLoadBalancingConfig, ServiceDiscoveryConfig,
};


pub use providers::{
    ProviderCapability, ProviderConfig as ProviderTraitConfig, ProviderHealth,
    ProviderRegistryEntry, ProviderStatus, ProviderType,
};
pub use security::*;
// Workflow exports with explicit aliases
pub use workflow::WorkflowRetryConfig as WorkflowRetryConfiguration;
// ============================================================================
// CANONICAL TYPE REGISTRY
/// Registry of all canonical types for validation and tooling
pub struct CanonicalTypeRegistry;
impl CanonicalTypeRegistry {
    /// Get list of all canonical status enums
    #[must_use]
    pub fn status_types() -> Vec<&'static str> {
        vec![
            "HealthStatus",
            "ComponentStatus",
            "OperationStatus",
            "WorkflowStatus",
            "KeyStatus",
        ]
    }
    /// Get list of all canonical configuration structs
    pub fn config_types() -> Vec<&'static str> {
        vec![
            "SessionConfig",
            "RateLimitConfig",
            "AuthenticationConfig",
            "PasswordPolicyConfig",
            "MfaConfig",
            "EncryptionConfig",
            "ProviderConfig",
        ]
    }

    /// Validate that a type name is canonical
    pub fn is_canonical_type(type_name: &str) -> bool {
        Self::status_types().contains(&type_name)
            || Self::config_types().contains(&type_name)
            || matches!(
                type_name,
                "SecurityContext"
                    | "SecurityAuditEvent"
                    | "PolicyDecision"
                    | "AuthorizationLevel"
                    | "SecurityFlags"
                    | "RiskLevel"
            )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCapabilities {
    /// Supported authentication methods
    pub authentication_methods: Vec<String>,
    /// Role-based access control
    pub rbac: bool,
    /// Audit logging
    pub audit_logging: bool,
    /// Secure communication protocols
    pub secure_protocols: Vec<String>,
    /// Compliance certifications
    pub compliance_certifications: Vec<String>,
    /// Security assurance level
    pub security_level: String,
}

impl Default for SecurityCapabilities {
    fn default() -> Self {
        Self {
            authentication_methods: vec!["password".to_string(), "mfa".to_string()],
            rbac: true,
            audit_logging: true,
            secure_protocols: vec!["TLS".to_string(), "HTTPS".to_string()],
            compliance_certifications: vec!["ISO27001".to_string()],
            security_level: "high".to_string(),
        }
    }
}
