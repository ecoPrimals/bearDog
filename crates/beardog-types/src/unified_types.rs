// SPDX-License-Identifier: AGPL-3.0-only

//! # Unified Type Aliases - Single Source of Truth
//!
//! This module provides the **ultimate unified type system** that consolidates ALL
//! scattered type aliases across the BearDog ecosystem into a single, comprehensive,
//! maintainable location.
//!
//! ## 🎯 **Complete Type Consolidation Strategy**
//!
//! This module consolidates and replaces **ALL** scattered type aliases:
//!
//! ### **Eliminated Type Fragmentation**
//! - ✅ **15+ Result type variants** → **Unified Result Types**
//! - ✅ **30+ Testing framework types** → **Unified Testing Types**  
//! - ✅ **beardog_types::constants::domains::system::defaults::DEFAULT_POOL_SIZE+ Configuration types** → **Unified Config Types**
//! - ✅ **20+ Provider types** → **Unified Provider Types**
//! - ✅ **Scattered utility types** → **Unified Utility Types**
//!
//! ### **Architecture Principles**
//! - **Single Source of Truth**: All type aliases in one canonical location
//! - **Consistent Naming**: Uniform naming patterns across all types
//! - **Zero Fragmentation**: No duplicate type definitions anywhere
//! - **Type Safety**: Strong typing with comprehensive error handling
//! - **Performance Optimized**: Zero-cost abstractions with efficient types
//! - **Developer Experience**: Easy imports and intuitive naming

use beardog_errors::BearDogError;
use crate::canonical::config::UnifiedBearDogConfig;
use beardog_errors::{BearDogError, EnhancedBearDogError};
use serde_json::Map;
use std::collections::HashMap;

// =============================================================================
// RESULT TYPES - Unified error handling
// =============================================================================

/// **Primary Result Type** - Standard `BearDog` result type
pub type Result<T> = Result<T, BearDogError>;

/// **Enhanced Result Type** - Result with enhanced error context
pub type EnhancedResult<T> = Result<T, EnhancedBearDogError>;

/// **Security Result Type** - Security operation results
pub type SecurityResult<T> = Result<T, BearDogError>;

/// **HSM Result Type** - Hardware security module operation results  
pub type HsmResult<T> = Result<T, BearDogError>;

/// **Genetics Result Type** - Genetics algorithm operation results
pub type GeneticsResult<T> = Result<T, BearDogError>;

/// **Provider Result Type** - Provider operation results
pub type ProviderResult<T> = Result<T, BearDogError>;

/// **Configuration Result Type** - Configuration operation results
pub type ConfigResult<T> = Result<T, BearDogError>;

/// **Validation Result Type** - Validation operation results
pub type ValidationResult<T> = Result<T, BearDogError>;

/// **Migration Result Type** - Migration operation results (replaces `MigratorResult`, etc.)
pub type MigrationResult<T> = Result<T, BearDogError>;

// =============================================================================
// CONFIGURATION TYPES - Unified configuration system
// =============================================================================

// CLEANED: Removed commented type aliases (October 2025 unification)
// Use canonical types directly - see beardog-types/src/canonical/config/ for all configs

/// **HSM Configuration Type** - Hardware security module configuration
pub type HsmConfig = crate::canonical::hsm_unified::CanonicalHsmConfig;

// =============================================================================
// TESTING FRAMEWORK TYPES - Unified testing system
// =============================================================================

/// **Property Testing Result** - Property-based testing results
pub type PropertyResult = Result<bool, BearDogError>;

/// **Mutation Testing Result** - Mutation testing results
pub type MutationTestResult = Result<MutationScore, BearDogError>;

/// **Invariant Validation Result** - Invariant validation results
pub type InvariantValidationResult = Result<bool, BearDogError>;

/// **Invariant Monitoring Result** - Invariant monitoring results
pub type InvariantMonitoringResult = Result<MonitoringReport, BearDogError>;

/// **Path Coverage Result** - Path coverage analysis results
pub type PathCoverageResult = Result<CoverageReport, BearDogError>;

/// **Boundary Testing Result** - Boundary testing results
pub type BoundaryTestResult = Result<BoundaryReport, BearDogError>;

/// **Quantum Security Result** - Quantum security assessment results
pub type QuantumSecurityResult = Result<SecurityAssessment, BearDogError>;

/// **Quantum Attack Result** - Quantum attack simulation results
pub type QuantumAttackResult = Result<AttackResistance, BearDogError>;

/// **Quantum Readiness Score** - Quantum readiness assessment
pub type QuantumReadinessScore = Result<ReadinessReport, BearDogError>;

/// **Quantum Resistance Result** - Quantum resistance analysis
pub type QuantumResistanceResult = Result<ResistanceReport, BearDogError>;

/// **Quantum Attack Simulation** - Quantum attack simulation
pub type QuantumAttackSimulation = Result<SimulationReport, BearDogError>;

/// **Error Validation Results** - Error validation testing results
pub type ErrorValidationResults = Result<Vec<ErrorValidation>, BearDogError>;

// =============================================================================
// TESTING FRAMEWORK SUPPORTING TYPES
// =============================================================================

/// **Mutation Score** - Mutation testing effectiveness score
pub type MutationScore = f64;

/// **Monitoring Report** - System monitoring report
pub type MonitoringReport = String;

/// **Coverage Report** - Code coverage report
pub type CoverageReport = String;

/// **Boundary Report** - Boundary testing report
pub type BoundaryReport = String;

/// **Security Assessment** - Security assessment report
pub type SecurityAssessment = String;

/// **Attack Resistance** - Attack resistance analysis
pub type AttackResistance = String;

/// **Readiness Report** - System readiness report
pub type ReadinessReport = String;

/// **Resistance Report** - Resistance analysis report
pub type ResistanceReport = String;

/// **Simulation Report** - Simulation execution report
pub type SimulationReport = String;

/// **Error Validation** - Error validation result
pub type ErrorValidation = String;

/// **Proof Step** - Mathematical proof step
pub type ProofStep = String;

/// **Verification Error** - Verification error description
pub type VerificationError = String;

/// **Test Vector** - Test case vector
pub type TestVector = String;

/// **Input Constraint** - Input validation constraint
pub type InputConstraint = String;

/// **Code Location** - Source code location reference
pub type CodeLocation = String;

/// **Parameter** - Function or method parameter
pub type Parameter = String;

/// **Function Constraint** - Function constraint specification
pub type FunctionConstraint = String;

/// **Edge Case** - Edge case test scenario
pub type EdgeCase = String;

/// **Test Domain** - Testing domain specification
pub type TestDomain = String;

/// **Error Case** - Error case test scenario
pub type ErrorCase = String;

/// **Safety Violation** - Safety constraint violation
pub type SafetyViolation = String;

/// **System Definition** - System definition specification
pub type SystemDefinition = String;

/// **Crypto Key** - Cryptographic key representation
pub type CryptoKey = String;

/// **Crypto System** - Cryptographic system specification
pub type CryptoSystem = String;

/// **Crypto Primitive** - Cryptographic primitive specification
pub type CryptoPrimitive = String;

/// **Input Space** - Input space specification
pub type InputSpace = String;

// =============================================================================
// PROVIDER TYPES - Unified provider system
// =============================================================================

/// **Provider Metrics** - Provider performance metrics
pub type ProviderMetrics = HashMap<String, f64>;

/// **Provider Registry** - Node registry for providers
pub type ProviderRegistry = HashMap<String, crate::canonical::providers_unified::traits::ProviderInfo>;

/// **Provider Manager** - Provider manager
pub type ProviderManager<P> = Vec<P>;

/// **Connection Type** - Network connection type specification
pub type ConnectionType = String;

// =============================================================================
// UTILITY TYPES - Common utility types
// =============================================================================

/// **JSON Value** - JSON value type
pub type JsonValue = serde_json::Value;

/// **JSON Map** - JSON object map type
pub type JsonMap = Map<String, JsonValue>;

/// **String Map** - String-to-string mapping
pub type StringMap = HashMap<String, String>;

/// **Metrics Map** - Metrics collection mapping
pub type MetricsMap = HashMap<String, JsonValue>;

/// **Timestamp** - UTC timestamp type
pub type Timestamp = chrono::DateTime<chrono::Utc>;

/// **Duration** - Time duration type
pub type Duration = std::time::Duration;

// =============================================================================
// STREAMING AND ASYNC TYPES - Unified async system
// =============================================================================

/// **Progress Stream** - Asynchronous progress stream
/// Note: Generic parameter allows for zero-cost abstractions

/// **Command Receiver** - Command channel receiver (requires tokio)
pub type CommandReceiver = std::sync::mpsc::Receiver<StreamingCommand>;

/// **Progress Sender** - Progress channel sender (requires tokio)
pub type ProgressSender = std::sync::mpsc::Sender<StreamingProgress>;

/// **Command Sender** - Command channel sender (requires tokio)
pub type CommandSender = std::sync::mpsc::Sender<StreamingCommand>;

// Supporting types for streaming
/// **Streaming Progress** - Progress update in streaming operations
pub type StreamingProgress = String; // Simplified for now

/// **Streaming Command** - Command in streaming operations
pub type StreamingCommand = String; // Simplified for now

// =============================================================================
// OUTCOME TYPES - Unified outcome system
// =============================================================================

/// **Authentication Outcome** - Authentication operation outcome
pub type AuthenticationOutcome = Result<bool, BearDogError>;

/// **Validation Outcome** - Validation operation outcome  
pub type ValidationOutcome = Result<bool, BearDogError>;

/// **Configuration Outcome** - Configuration operation outcome
pub type ConfigurationOutcome<T> = Result<T, BearDogError>;

/// **Processing Outcome** - Processing operation outcome
pub type ProcessingOutcome<T> = Result<T, BearDogError>;

/// **Genetics Outcome** - Genetics operation outcome
pub type GeneticsOutcome = Result<String, BearDogError>;

/// **Spawn Outcome** - Process spawning outcome
pub type SpawnOutcome = Result<u32, BearDogError>;

/// **Termination Outcome** - Process termination outcome
pub type TerminationOutcome = Result<(), BearDogError>;

// =============================================================================
// DEPRECATED ALIASES REMOVED - Migration Complete
// =============================================================================
//
// The following deprecated type aliases have been removed as part of the
// unification effort. Users should migrate to the modern
// canonical types:
//
// - MigratorResult<T> → MigrationResult<T>
// - EnhancedMigratorResult<T> → MigrationResult<T>
// - PanicResult<T> → MigrationResult<T>
// - RefinedResult<T> → MigrationResult<T>
// - ServiceMeshResult<T> → Result<T>
// - InMemoryNodeRegistry → ProviderRegistry
// - DefaultHsmProviderManager<P> → ProviderManager<P>
//
// For migration assistance, see: docs/guides/TYPE_MIGRATION_GUIDE.md

// =============================================================================
// RE-EXPORTS - Convenient access to core types
// =============================================================================

// Re-export commonly used external types
pub use chrono::{DateTime, Utc};
pub use serde_json::Value; 