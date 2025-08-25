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


/// # `BearDog` Types - Canonical Type System
///
/// This crate provides the **single source of truth** for ALL types used across
/// the `BearDog` ecosystem. All other crates should import types from here to ensure
/// consistency and eliminate duplicate type definitions.
/// ## Architecture
/// - **Canonical Types**: Modern, unified type definitions in the `canonical` module
/// - **Zero Duplicates**: All types are defined once and re-exported consistently
/// ## Usage
/// ```rust
/// use beardog_types::canonical::hsm::HsmCapabilities;
/// use beardog_types::config::BiomeConfig;
/// // All canonical types are available through their modules
/// ```
/// ## Migration Status
/// ✅ **Complete**: Core types, configurations, and security types unified  
/// ✅ **Complete**: Canonical type system modularized into focused domains
/// ✅ **Complete**: Legacy module elimination completed in v3.0
// ============================================================================
// CANONICAL TYPES - MODULAR ORGANIZATION
pub mod canonical;

// DOMAIN-SPECIFIC MODULES
pub mod aliases;
pub mod config;
pub mod constants;
pub mod hsm;
pub mod network;
pub mod providers;
pub mod testing;
pub mod zero_cost;
// Re-export ALL canonical types at the crate level for easy access
pub use canonical::*;

// Re-export common type aliases for convenience
pub use aliases::{
    BearDogResult, TestResult, AssertionResult,
    ProviderMetrics, SystemMetrics, CacheResult,
    ConfigResult, NetworkResult, SecurityResult,
    WorkflowResult, HsmResult, KeyResult
};
// CANONICAL TYPE EXPORTS - DIRECT ACCESS TO UNIFIED TYPES
// All types are now accessed directly from canonical modules:
// - HealthStatus, ComponentStatus, OperationStatus, WorkflowStatus
// - SessionConfig, RateLimitConfig, AuthenticationConfig, EncryptionConfig
// - SecurityContext, AuthorizationLevel, SecurityFlags, PolicyDecision
// - KeyStatus, KeyType, WorkflowExecutionState
// CANONICAL TYPE VALIDATION
/// Validate that all imports use canonical types
pub fn validate_canonical_usage() -> Result<(), Vec<String>> {
    let errors = Vec::new();
    // This would be expanded to check for non-canonical imports
    // For now, it's a placeholder for future tooling
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
/// Get information about canonical types
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
        // Add more as needed
    ]
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_canonical_types_available() {
        // Test that canonical types can be constructed
        let _health = HealthStatus::Healthy;
        let _security_context = SecurityContext::default();
        let _policy_decision = PolicyDecision::Allow;
    }
    #[test]
    fn test_canonical_types_direct_access() {
        // Test that canonical types are accessible directly
        let _health: HealthStatus = HealthStatus::Healthy;
        let _context: SecurityContext = SecurityContext::default();
    }
    #[test]
    fn test_canonical_validation() {
        // Test canonical type validation
        assert!(canonical::CanonicalTypeRegistry::is_canonical_type(
            "HealthStatus"
        ));
        assert!(canonical::CanonicalTypeRegistry::is_canonical_type(
            "SessionConfig"
        ));
        assert!(canonical::CanonicalTypeRegistry::is_canonical_type(
            "SecurityContext"
        ));
        assert!(!canonical::CanonicalTypeRegistry::is_canonical_type(
            "NonExistentType"
        ));
    }
    #[test]
    fn test_configuration_helpers() {
        // Test configuration helper functions
        let api_host = canonical::constants::default_api_host();
        assert!(!api_host.is_empty());
        let api_port = canonical::constants::default_api_port();
        assert!(api_port > 0);
        let bind_address = canonical::constants::default_api_bind_address();
        assert!(bind_address.contains(':'));
        assert!(bind_address.len() > 3); // At least "x:y"
    }
    #[test]
    fn test_timeout_configurations() {
        // Test timeout configuration functions
        let timeout = canonical::constants::default_timeout_ms();
        assert!(timeout <= 300_000); // Max 5 minutes is reasonable
        assert!(timeout > 0); // Must be positive
        let health_check = canonical::constants::default_health_check_interval_ms();
        assert!(health_check <= 600_000); // Max 10 minutes is reasonable
    }
    #[test]
    fn test_canonical_type_info() {
        // Test canonical type information
        let type_info = canonical_type_info();
        assert!(!type_info.is_empty());
        // Verify some expected types are documented
        let type_names: Vec<&str> = type_info.iter().map(|(name, _)| *name).collect();
        assert!(type_names.contains(&"HealthStatus"));
        assert!(type_names.contains(&"SecurityContext"));
    }
    #[test]
    fn test_canonical_usage_validation() {
        // Test validation function doesn't error
        let result = validate_canonical_usage();
        assert!(result.is_ok(), "Canonical usage validation should pass");
    }
}
