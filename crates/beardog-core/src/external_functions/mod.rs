// SPDX-License-Identifier: AGPL-3.0-only

//! # External Functions Interface
//!
//! Provides comprehensive Foreign Function Interface (FFI) capabilities for BearDog,
//! enabling seamless integration with external libraries, native code, and system-level
//! operations with enterprise-grade safety and performance.
//!
//! ## Overview
//!
//! This module bridges Rust with external code through safe FFI wrappers:
//! - C library integration
//! - System-level operations
//! - Hardware abstraction
//! - Native performance optimization
//!
//! ## Architecture
//!
//! The external functions system is organized into focused modules:
//! - **types**: Core type definitions and data structures
//! - **registry**: Function registry and library management
//! - **safety**: Safety checking and validation policies

// Module declarations
/// Function registry and library management
pub mod registry;
/// Safety checking and validation policies
pub mod safety;
/// FFI type definitions
pub mod types;

// Re-export main types and functions
pub use registry::ExternalFunctionRegistry;
pub use types::{
    AccessRestriction, CallingConvention, CpuIntensity, ExternalFunctionsRegistryConfig,
    FunctionAttribute, FunctionHandle, FunctionMetadata, FunctionParameter, FunctionResult,
    FunctionValue, LibraryHandle, LibraryMetadata, ParameterType, PerformanceInfo, SafetyLevel,
    SecurityClearance, SecurityInfo,
};

pub use safety::{ParameterValue, SafetyChecker};

/// Returns the default configuration for external function registry
#[must_use]
pub fn default_config() -> ExternalFunctionsRegistryConfig {
    ExternalFunctionsRegistryConfig::default()
}

/// Creates a new external function registry with default configuration
/// Creates registry
#[must_use]
pub fn create_registry() -> ExternalFunctionRegistry {
    ExternalFunctionRegistry::default()
}

/// Creates a new external function registry with custom configuration
/// Creates `registry_with_config`
#[must_use]
pub fn create_registry_with_config(
    config: ExternalFunctionsRegistryConfig,
) -> ExternalFunctionRegistry {
    ExternalFunctionRegistry::new(config)
}

#[allow(
    unused_imports,
    clippy::float_cmp,
    clippy::useless_vec,
    clippy::needless_range_loop,
    clippy::uninlined_format_args,
    dead_code
)]
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_registry_creation() {
        let config = ExternalFunctionsRegistryConfig::default();
        let registry = ExternalFunctionRegistry::new(config);
        let libraries = registry
            .list_libraries()
            .expect("empty registry should list libraries");
        assert!(libraries.is_empty());
    }

    #[tokio::test]
    async fn test_registry_library_management() {
        let config = ExternalFunctionsRegistryConfig::default();
        let registry = ExternalFunctionRegistry::new(config);
        let libraries = registry
            .list_libraries()
            .expect("empty registry should list libraries");
        assert!(libraries.is_empty());
    }

    #[tokio::test]
    async fn test_safety_checker() {
        use super::safety::{SafetyChecker, SafetyPolicy};
        use super::types::SecurityClearance;
        // Note: PolicyEngine may need alternative implementation
        // use beardog_types::canonical::providers_unified::traits::PolicyEngine;

        let mut checker = SafetyChecker::new(SecurityClearance::Internal);

        // Add some test policies
        let policy1 = SafetyPolicy {
            policy_id: "parameter_validation".to_string(),
            function_patterns: vec!["validate_*".to_string()],
            required_clearance: SecurityClearance::Public,
            parameter_validation: true,
        };

        let policy2 = SafetyPolicy {
            policy_id: "memory_bounds".to_string(),
            function_patterns: vec!["memory_*".to_string()],
            required_clearance: SecurityClearance::Internal,
            parameter_validation: true,
        };

        // Load policies
        checker
            .load_policy(policy1)
            .expect("parameter_validation policy should load");
        checker
            .load_policy(policy2)
            .expect("memory_bounds policy should load");

        // Test list_policies
        let policies = checker.list_policies().expect("policies after load");
        assert!(!policies.is_empty());
        assert!(policies.contains(&"parameter_validation".to_string()));
        assert!(policies.contains(&"memory_bounds".to_string()));
    }
}
