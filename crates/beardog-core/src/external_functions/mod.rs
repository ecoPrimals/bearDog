// **ULTRA-PEDANTIC**: External Functions Interface for BearDog Ecosystem
//
// This module provides comprehensive Foreign Function Interface (FFI) capabilities
// for BearDog, enabling seamless integration with external libraries, native code,
// and system-level operations with enterprise-grade safety and performance.
//
// ## Architecture
//
// The external functions system is organized into focused modules:
// - **types**: Core type definitions and data structures
// - **registry**: Function registry and library management
// - **safety**: Safety checking and validation policies
//
// ## Usage
//
// ```rust
// use beardog_core::external_functions::{
//     ExternalFunctionRegistry, RegistryConfig, FunctionSignature, ParameterType
// };
//
// // Create registry with default config
// let registry = ExternalFunctionRegistry::default();
//
// // Load a library
// let lib_id = registry.load_library("/path/to/library.so")?;
//
// // Cache a function
// let signature = FunctionSignature {
//     parameters: vec![ParameterType::Int32, ParameterType::CString],
//     return_type: ReturnType::Type(ParameterType::Int32),
//     calling_convention: CallingConvention::C,
//     attributes: vec![],
// };
//
// let func_id = registry.cache_function(&lib_id, "my_function", signature)?;
//
// // Call the function
// let result = registry.call_cached_function(&func_id, parameters)?;
// ```

// Module declarations
pub mod registry;
pub mod safety;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_registry_creation() {
        let config = ExternalFunctionsRegistryConfig::default();
        let registry = ExternalFunctionRegistry::new(config);
        let libraries = registry.list_libraries().unwrap();
        assert!(libraries.is_empty());
    }

    #[tokio::test]
    async fn test_registry_library_management() {
        let config = ExternalFunctionsRegistryConfig::default();
        let registry = ExternalFunctionRegistry::new(config);
        let libraries = registry.list_libraries().unwrap();
        assert!(libraries.is_empty());
    }

    #[tokio::test]
    async fn test_safety_checker() {
        use super::safety::{SafetyChecker, SafetyPolicy};
        use super::types::SecurityClearance;
        use beardog_traits::unified::PolicyEngine;

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
        checker.load_policy(policy1).await.unwrap();
        checker.load_policy(policy2).await.unwrap();

        // Test list_policies
        let policies = checker.list_policies().await.unwrap();
        assert!(!policies.is_empty());
        assert!(policies.contains(&"parameter_validation".to_string()));
        assert!(policies.contains(&"memory_bounds".to_string()));
    }
}
