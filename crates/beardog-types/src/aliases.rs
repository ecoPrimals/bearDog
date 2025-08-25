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


/// # Canonical Type Aliases - Centralized Type System
///
/// **SINGLE SOURCE OF TRUTH** for all type aliases used across the BearDog ecosystem.
/// This module consolidates scattered type aliases to eliminate duplication and improve maintainability.
/// 
/// ## Consolidated Aliases
/// This file replaces scattered aliases found in:
/// - `beardog-errors/src/types/core.rs::BearDogResult<T>`
/// - `beardog-traits/src/canonical.rs::ProviderMetrics`
/// - `tests/common/mod.rs::TestResult<T>`
/// - `tests/common/assertions.rs::AssertionResult<T>`
/// - `unwrap-migrator/src/*::*Result<T>`
/// - Various cache type aliases across modules
/// - Performance testing type aliases
/// 
/// ## Benefits
/// - **Zero duplication** - Each alias defined exactly once
/// - **Consistent naming** - Standardized alias patterns
/// - **Easy maintenance** - Single location for all aliases
/// - **Better IDE support** - Centralized type information

use std::collections::HashMap;
use std::time::Duration;

// Re-export the canonical BearDogResult from beardog-errors
pub use beardog_errors::BearDogResult;

// ============================================================================
// CORE RESULT TYPES - Primary error handling aliases
// ============================================================================

/// **CANONICAL RESULT TYPE** - Re-exported from beardog-errors
/// 
/// This is the standard Result type used throughout the BearDog ecosystem.
/// Primary definition is in beardog-errors::BearDogResult<T>.

/// **TEST RESULT TYPE** - Specialized for testing operations
/// 
/// Used in test modules and testing frameworks for consistent error handling.
pub type TestResult<T = ()> = BearDogResult<T>;

/// **ASSERTION RESULT TYPE** - For test assertions and validations
/// 
/// Used in assertion functions and validation operations.
pub type AssertionResult<T = ()> = BearDogResult<T>;

// ============================================================================
// PERFORMANCE AND METRICS TYPES - System monitoring aliases
// ============================================================================

/// **PROVIDER METRICS TYPE** - Performance and operational metrics
/// 
/// Standard metrics collection type for tracking provider performance,
/// health, and operational statistics across the ecosystem.
pub type ProviderMetrics = HashMap<String, f64>;

/// **SYSTEM METRICS TYPE** - System-wide performance metrics
/// 
/// Used for collecting and reporting system-level performance data.
pub type SystemMetrics = HashMap<String, f64>;

/// **BENCHMARK METRICS TYPE** - Performance benchmarking data
/// 
/// Specialized metrics type for performance testing and benchmarking.
pub type BenchmarkMetrics = HashMap<String, Duration>;

// ============================================================================
// CACHE TYPES - Memory and storage caching aliases
// ============================================================================

/// **CACHE RESULT TYPE** - Cache operation results
/// 
/// Standard result type for cache operations (get, set, delete, etc.).
pub type CacheResult<T> = BearDogResult<T>;

/// **CACHE STATS TYPE** - Cache performance statistics
/// 
/// Used for tracking cache hit rates, memory usage, and performance metrics.
pub type CacheStats = HashMap<String, u64>;

/// **MEMORY CACHE TYPE** - Generic in-memory cache
/// 
/// Standard type alias for in-memory caching operations.
pub type MemoryCache<K, V> = HashMap<K, V>;

// ============================================================================
// CONFIGURATION TYPES - System configuration aliases
// ============================================================================

/// **CONFIG RESULT TYPE** - Configuration operation results
/// 
/// Standard result type for configuration loading, validation, and updates.
pub type ConfigResult<T> = BearDogResult<T>;

/// **CONFIG MAP TYPE** - Configuration key-value storage
/// 
/// Standard type for storing configuration parameters as key-value pairs.
pub type ConfigMap = HashMap<String, String>;

/// **SETTINGS MAP TYPE** - Application settings storage
/// 
/// Used for storing application-specific settings and preferences.
pub type SettingsMap = HashMap<String, String>;

// ============================================================================
// NETWORK TYPES - Networking and communication aliases
// ============================================================================

/// **NETWORK RESULT TYPE** - Network operation results
/// 
/// Standard result type for network operations (connect, send, receive, etc.).
pub type NetworkResult<T> = BearDogResult<T>;

/// **CONNECTION POOL TYPE** - Network connection pool
/// 
/// Standard type for managing pools of network connections.
pub type ConnectionPool<T> = Vec<T>;

/// **ENDPOINT MAP TYPE** - Service endpoint registry
/// 
/// Used for mapping service names to their network endpoints.
pub type EndpointMap = HashMap<String, String>;

// ============================================================================
// SECURITY TYPES - Cryptographic and security aliases
// ============================================================================

/// **SECURITY RESULT TYPE** - Security operation results
/// 
/// Standard result type for security operations (encrypt, decrypt, sign, verify).
pub type SecurityResult<T> = BearDogResult<T>;

/// **CRYPTO RESULT TYPE** - Cryptographic operation results
/// 
/// Specialized result type for cryptographic operations.
pub type CryptoResult<T> = BearDogResult<T>;

/// **KEY STORE TYPE** - Cryptographic key storage
/// 
/// Standard type for storing cryptographic keys and metadata.
pub type KeyStore = HashMap<String, Vec<u8>>;

// ============================================================================
// WORKFLOW TYPES - Workflow and process management aliases
// ============================================================================

/// **WORKFLOW RESULT TYPE** - Workflow operation results
/// 
/// Standard result type for workflow operations (create, execute, monitor).
pub type WorkflowResult<T> = BearDogResult<T>;

/// **WORKFLOW REGISTRY TYPE** - Active workflow registry
/// 
/// Used for tracking active workflows and their states.
pub type WorkflowRegistry = HashMap<String, String>;

/// **PROCESS RESULT TYPE** - Process execution results
/// 
/// Standard result type for process execution and management.
pub type ProcessResult<T> = BearDogResult<T>;

// ============================================================================
// HSM TYPES - Hardware Security Module aliases
// ============================================================================

/// **HSM RESULT TYPE** - HSM operation results
/// 
/// Standard result type for HSM operations (key generation, signing, etc.).
pub type HsmResult<T> = BearDogResult<T>;

/// **KEY RESULT TYPE** - Key management operation results
/// 
/// Specialized result type for key management operations.
pub type KeyResult<T> = BearDogResult<T>;

/// **HSM REGISTRY TYPE** - HSM provider registry
/// 
/// Used for tracking available HSM providers and their capabilities.
pub type HsmRegistry = HashMap<String, String>;

// ============================================================================
// MIGRATION TYPES - Code migration and refactoring aliases
// ============================================================================

/// **MIGRATION RESULT TYPE** - Code migration operation results
/// 
/// Used by migration tools and refactoring utilities.
pub type MigrationResult<T> = BearDogResult<T>;

/// **REFACTOR RESULT TYPE** - Code refactoring operation results
/// 
/// Used by refactoring tools and code transformation utilities.
pub type RefactorResult<T> = BearDogResult<T>;

// ============================================================================
// VALIDATION AND COMPATIBILITY
// ============================================================================

/// Validate that all type aliases are properly defined and accessible
/// 
/// This function can be used in tests to ensure all aliases are working correctly.
pub fn validate_type_aliases() -> BearDogResult<()> {
    // Test basic result types
    let _test_result: BearDogResult<()> = Ok(());
    let _test_result2: TestResult<()> = Ok(());
    let _assertion_result: AssertionResult<()> = Ok(());
    
    // Test metrics types
    let _provider_metrics: ProviderMetrics = HashMap::new();
    let _system_metrics: SystemMetrics = HashMap::new();
    
    // Test cache types
    let _cache_stats: CacheStats = HashMap::new();
    let _memory_cache: MemoryCache<String, String> = HashMap::new();
    
    // Test configuration types
    let _config_map: ConfigMap = HashMap::new();
    let _settings_map: SettingsMap = HashMap::new();
    
    // Test network types
    let _endpoint_map: EndpointMap = HashMap::new();
    
    // Test security types
    let _key_store: KeyStore = HashMap::new();
    
    // Test workflow types
    let _workflow_registry: WorkflowRegistry = HashMap::new();
    
    // Test HSM types
    let _hsm_registry: HsmRegistry = HashMap::new();
    
    Ok(())
}

/// Get information about all available type aliases
/// 
/// Returns a list of (alias_name, description) tuples for documentation purposes.
pub fn get_alias_info() -> Vec<(&'static str, &'static str)> {
    vec![
        ("BearDogResult<T>", "Primary result type for all BearDog operations"),
        ("TestResult<T>", "Specialized result type for testing operations"),
        ("AssertionResult<T>", "Result type for test assertions and validations"),
        ("ProviderMetrics", "Performance and operational metrics collection"),
        ("SystemMetrics", "System-wide performance metrics"),
        ("BenchmarkMetrics", "Performance benchmarking data"),
        ("CacheResult<T>", "Cache operation results"),
        ("CacheStats", "Cache performance statistics"),
        ("MemoryCache<K,V>", "Generic in-memory cache"),
        ("ConfigResult<T>", "Configuration operation results"),
        ("ConfigMap", "Configuration key-value storage"),
        ("SettingsMap", "Application settings storage"),
        ("NetworkResult<T>", "Network operation results"),
        ("ConnectionPool<T>", "Network connection pool"),
        ("EndpointMap", "Service endpoint registry"),
        ("SecurityResult<T>", "Security operation results"),
        ("CryptoResult<T>", "Cryptographic operation results"),
        ("KeyStore", "Cryptographic key storage"),
        ("WorkflowResult<T>", "Workflow operation results"),
        ("WorkflowRegistry", "Active workflow registry"),
        ("ProcessResult<T>", "Process execution results"),
        ("HsmResult<T>", "HSM operation results"),
        ("KeyResult<T>", "Key management operation results"),
        ("HsmRegistry", "HSM provider registry"),
        ("MigrationResult<T>", "Code migration operation results"),
        ("RefactorResult<T>", "Code refactoring operation results"),
    ]
} 