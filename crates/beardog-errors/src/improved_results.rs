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


/// Improved Result Types and Error Handling
///
/// This module provides idiomatic Rust error handling patterns with rich context,
/// moving away from `Result<(), E>` to meaningful return types that provide value.
// use beardog_types::canonical::hsm::{HsmCapabilities, KeyMetadata}; // Circular dependency - define locally
use crate::BearDogResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Operation outcome with rich context - replaces bare `Result<(), E>`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationOutcome<T = OperationSummary> {
    /// The successful result data
    pub result: T,
    /// Operation metadata and context
    pub context: OperationContext,
    /// Performance metrics
    pub metrics: OperationMetrics,
    /// Any warnings or non-fatal issues
    pub warnings: Vec<OperationWarning>,
}
/// Default operation summary for functions that previously returned `()`
pub struct OperationSummary {
    /// What operation was performed
    pub operation: String,
    /// Outcome description
    pub outcome: String,
    /// Number of items processed/affected
    pub items_affected: u64,
    /// Additional outcome details
    pub details: HashMap<String, serde_json::Value>,
}

/// Rich operation context
pub struct OperationContext {
    /// Unique operation ID for tracing
    pub operation_id: String,
    /// When the operation started
    pub started_at: DateTime<Utc>,
    /// When the operation completed
    pub completed_at: DateTime<Utc>,
    /// Component that performed the operation
    pub component: String,
    /// User or system that initiated the operation
    pub initiator: String,
    /// Request ID if applicable
    pub request_id: Option<String>,
    /// Additional context metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Performance metrics for operations
pub struct OperationMetrics {
    /// Total duration of the operation
    pub duration: Duration,
    /// Memory used during operation (bytes)
    pub memory_used: Option<u64>,
    /// Number of items processed
    pub items_processed: u64,
    /// Success rate (0.0 to 100.0)
    pub success_rate: f64,
    /// Additional performance metrics
    pub additional_metrics: HashMap<String, serde_json::Value>,
}

/// Operation warning for non-fatal issues
pub struct OperationWarning {
}

    /// Type of warning
    pub warning_type: WarningType,
    /// Warning message
    pub message: String,
    /// Additional warning context
    pub context: serde_json::Value,
/// Types of warnings that can occur during operations
pub enum WarningType {
}

    /// Performance degradation detected
    PerformanceDegradation,
    /// Partial processing failure
    ProcessingFailure,
    /// Configuration issue (non-fatal)
    ConfigurationIssue,
    /// Resource constraint
    ResourceConstraint,
    /// Security concern (non-fatal)
    SecurityConcern,
    /// Deprecated feature usage
    DeprecatedFeature,
// ============================================================================
// DOMAIN-SPECIFIC OUTCOME TYPES
/// Authentication outcome with rich context}


pub struct AuthenticationOutcome {
}

    /// Generated session identifier
    pub session_id: String,
    /// Authenticated user information
    pub user_info: UserInfo,
    /// Security level achieved
    pub security_level: SecurityLevel,
    /// When the session expires
    pub expires_at: DateTime<Utc>,
    /// Method used for authentication
    pub authentication_method: AuthenticationMethod,
    /// Operation context
    /// Any warnings during authentication
/// User information from authentication
pub struct UserInfo {
}

    /// Username
    pub username: String,
    /// User roles/permissions
    pub roles: Vec<String>,
    /// Additional user attributes
    pub attributes: HashMap<String, serde_json::Value>,
/// Security levels for authentication
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SecurityLevel {
}

    /// Basic security (password only)
    Basic,
    /// Enhanced security (MFA)
    Enhanced,
    /// High security (hardware token)
    High,
    /// Maximum security (HSM-backed)
    Maximum,
/// Authentication methods}


pub enum AuthenticationMethod {
}

    /// Password-based authentication
    Password,
    /// Multi-factor authentication
    MultiFactor,
    /// Hardware token
    HardwareToken,
    /// Biometric authentication
    Biometric,
    /// HSM-backed authentication
    HsmBacked,
/// HSM initialization outcome
pub struct HsmInitializationOutcome {
}

    /// HSM capabilities discovered
    pub hsm_capabilities: HsmCapabilities,
    /// Provider information
    pub provider_info: ProviderInfo,
    /// Performance baseline established
    pub performance_baseline: PerformanceBaseline,
    /// Any warnings during initialization
/// HSM capabilities (simplified to avoid circular dependency)
pub struct HsmCapabilities {
}

    /// HSM vendor
    pub vendor: String,
    /// HSM model}


    pub model: String,
    /// Supported algorithms
    pub supported_algorithms: Vec<String>,
    /// Additional capabilities
    pub additional: HashMap<String, serde_json::Value>,
/// Key metadata (simplified to avoid circular dependency)
pub struct KeyMetadata {
}

    /// Key ID
    pub id: String,
    /// Key type
    pub key_type: LocalKeyType,
    /// Key purpose
    pub purpose: String,
    /// Additional metadata
/// HSM provider information}


pub struct ProviderInfo {
}

    /// Provider name
    pub name: String,
    /// Provider version
    pub version: String,
    /// Hardware model
    pub hardware_model: Option<String>,
    /// Firmware version
    pub firmware_version: Option<String>,
    /// Additional provider details
/// Performance baseline for HSM operations
pub struct PerformanceBaseline {
}

    /// Key generation time (milliseconds)
    pub key_generation_ms: f64,
    /// Signing operation time (milliseconds)
    pub signing_ms: f64,
    /// Verification time (milliseconds)
    pub verification_ms: f64,
    /// Throughput (operations per second)
    pub throughput_ops_per_sec: f64,
/// Key generation outcome
pub struct KeyGenerationOutcome {
}

    /// Generated key handle
    pub key_handle: KeyHandle,
    /// Key metadata
    pub key_metadata: KeyMetadata,
    /// Security properties of the key
    pub security_properties: SecurityProperties,
    /// Any warnings during key generation
/// Key handle for referencing generated keys
pub struct KeyHandle {
}

    /// Unique key identifier
    pub key_id: String,
    /// Handle to the key (implementation-specific)
    pub handle: String,
/// Key types supported (simplified to avoid conflicts)
pub enum LocalKeyType {
}

    /// RSA key
    Rsa,
    /// Elliptic Curve key
    EllipticCurve,
    /// Ed25519 key
    Ed25519,
    /// AES symmetric key
    Aes,
    /// Custom key type
    Custom(String),
/// Key usage permissions}


pub enum KeyUsage {
}

    /// Digital signatures
    Signing,
    /// Encryption/decryption
    Encryption,
    /// Key agreement/exchange
    KeyAgreement,
    /// Authentication
    Authentication,
    /// Custom usage
/// Key strength/size}


pub enum KeyStrength {
}

    /// 128-bit security
    Bits128,
    /// 192-bit security
    Bits192,
    /// 256-bit security
    Bits256,
    /// 384-bit security
    Bits384,
    /// 512-bit security
    Bits512,
    /// Custom strength
    Custom(u32),
/// Security properties of a key
pub struct SecurityProperties {
}

    /// Hardware-backed key
    pub hardware_backed: bool,
    /// Key is extractable
    pub extractable: bool,
    /// Supports attestation
    pub attestation: bool,
    /// Additional security properties
    pub properties: HashMap<String, serde_json::Value>,
/// Configuration loading outcome
pub struct ConfigurationOutcome<T> {
}

    /// Loaded configuration
    /// Validation results
    pub validation_results: ValidationResults,
    /// Any warnings during configuration loading
/// Configuration validation results
pub struct ValidationResults {
}

    /// Validation findings
    pub findings: Vec<ValidationFinding>,
    /// Overall validation status
    pub status: ValidationStatus,
    /// Validation criteria applied
    pub criteria: Vec<ValidationCriteria>,
/// Individual validation finding
pub struct ValidationFinding {
}

    /// Severity of the finding
    pub severity: FindingSeverity,
    /// Finding message
    /// Configuration path that triggered the finding
    pub config_path: Option<String>,
    /// Suggested fix
    pub suggested_fix: Option<String>,
    /// Field name that triggered the finding (for backward compatibility)
    pub field: String,
    /// Suggestion for fixing the issue (alias for suggested_fix)
    pub suggestion: Option<String>,
    /// Error code for the finding
    pub code: String,
/// Validation finding severity
pub enum FindingSeverity {
}

    /// Information only
    Info,
    /// Warning (non-blocking)
    Warning,
    /// Error (blocking)
    Error,
    /// Critical error
    Critical,
/// Validation outcome for configuration validation}


pub struct ValidationOutcome {
}

    /// Validation criteria
    /// Any warnings during validation
/// Processing configuration
pub struct ProcessingConfiguration {
}

    /// Configuration name
    /// Configuration description
    pub description: String,
    /// Batch size for processing
    pub batch_size: usize,
    /// Maximum retries
    pub max_retries: u32,
    /// Timeout in seconds
    pub timeout_seconds: u64,
    /// Enable parallel processing
    pub parallel_processing: bool,
/// Overall validation status
pub enum ValidationStatus {
}

    /// All validations passed
    Passed,
    /// Passed with warnings
    PassedWithWarnings,
    /// Failed validation
    Failed,
/// Validation criteria}


pub struct ValidationCriteria {
}

    /// Criteria name
    /// Criteria description
    /// Whether this criteria is required
    pub required: bool,
    /// Weight of this criteria (0.0 to 1.0)
    pub weight: f64,
    /// Whether this criteria passed validation
    pub passed: bool,
/// Processing outcome for batch operations
pub struct ProcessingOutcome<T> {
}

    /// Successfully processed results
    /// Processing failures
    pub failures: Vec<ProcessingFailure>,
    /// Processing statistics
    pub statistics: ProcessingStatistics,
    /// Any warnings during processing
    /// Items processed (for backward compatibility)
    pub items: Vec<T>,
    /// Failed items list
    pub failed_items: Vec<ProcessingFailure>,
    /// Processing configuration
    pub configuration: ProcessingConfiguration,
/// Individual processing failure
pub struct ProcessingFailure {
}

    /// Index of the failed item
    pub item_index: usize,
    /// Value that failed processing
    pub item_value: serde_json::Value,
    /// Error that occurred
    pub error: String,
    /// When the failure occurred
    pub attempted_at: DateTime<Utc>,
    /// Item ID for the failed item (for backward compatibility)
    pub item_id: String,
    /// Error code for the failure
    pub error_code: String,
    /// Error message (alias for error)
    pub error_message: String,
    /// Number of retry attempts
    pub retry_count: u32,
    /// When the failure occurred (alias for attempted_at)
    pub failed_at: DateTime<Utc>,
/// Processing statistics
pub struct ProcessingStatistics {
}

    /// Total items attempted
    pub total_items: usize,
    /// Successfully processed items
    pub successful_items: usize,
    /// Failed items
    pub failed_items: usize,
    /// Success rate percentage
    /// Average processing time per item
    pub avg_processing_time: Duration,
    /// Skipped items (for backward compatibility)
    pub skipped_items: usize,
    /// Processing rate per second
    pub processing_rate_per_second: f64,
    /// Average item processing time (alias for avg_processing_time)
    pub average_item_processing_time: Duration,
/// Genetics registration outcome
pub struct GeneticsRegistrationOutcome {
}

    /// Registered genetics ID
    pub genetics_id: String,
    /// Genetics capabilities summary
    pub capabilities_summary: GeneticsCapabilitiesSummary,
    /// Registration validation results
    pub validation_results: GeneticsValidationResults,
    /// Any warnings during registration
/// Summary of genetics capabilities
pub struct GeneticsCapabilitiesSummary {
}

    /// Number of crypto chromosomes
    pub crypto_chromosomes_count: usize,
    /// Security traits strength
    pub security_strength: f64,
    /// Available capabilities
    pub capabilities: Vec<String>,
    /// Security clearance level
    pub security_clearance: String,
    /// Generation number
    pub generation: u32,
    /// Fitness score
    pub fitness_score: f64,
/// Genetics validation results
pub struct GeneticsValidationResults {
}

    /// Security assessment
    pub security_assessment: SecurityAssessment,
/// Security assessment for genetics
pub struct SecurityAssessment {
}

    /// Overall security score (0.0 to 100.0)
    pub security_score: f64,
    /// Risk level assessment
    pub risk_level: String,
    /// Security recommendations
    pub recommendations: Vec<String>,
/// BearDog spawning outcome
pub struct SpawningOutcome {
}

    /// Spawned BearDog information
    pub spawned_beardog: SpawnedBearDogInfo,
    /// Genetic inheritance details
    pub genetic_inheritance: GeneticInheritanceInfo,
    /// Spawn validation results
    pub spawn_validation: SpawnValidationResults,
    /// Any warnings during spawning
/// Information about the spawned BearDog
pub struct SpawnedBearDogInfo {
}

    /// Unique spawn ID
    pub spawn_id: String,
    /// Parent BearDog IDs
    pub parent_ids: Vec<String>,
    /// Spawn purpose
    pub spawn_purpose: String,
    /// Expected lifetime
    pub expected_lifetime: Option<chrono::DateTime<chrono::Utc>>,
    /// Current status
    pub current_status: String,
    /// Resource limits
    pub resource_limits: ResourceLimits,
/// Resource limits for spawned BearDog
pub struct ResourceLimits {
}

    /// Maximum memory usage (bytes)
    pub max_memory_bytes: Option<u64>,
    /// Maximum CPU usage (percentage)
    pub max_cpu_percent: Option<f64>,
    /// Maximum network bandwidth (bytes/sec)
    pub max_network_bandwidth: Option<u64>,
    /// Maximum storage usage (bytes)
    pub max_storage_bytes: Option<u64>,
/// Genetic inheritance information
pub struct GeneticInheritanceInfo {
}

    /// Combined genetics summary
    pub combined_genetics: GeneticsCapabilitiesSummary,
    /// Inheritance method used
    pub inheritance_method: String,
    /// Genetic mutations applied
    pub mutations_applied: Vec<String>,
    /// Fitness improvement
    pub fitness_improvement: f64,
/// Spawn validation results
pub struct SpawnValidationResults {
}

    /// Resource validation
    pub resource_validation: ResourceValidation,
    /// Security validation
    pub security_validation: SecurityValidation,
/// Resource validation results
pub struct ResourceValidation {
}

    /// Memory validation passed
    pub memory_valid: bool,
    /// CPU validation passed
    pub cpu_valid: bool,
    /// Network validation passed
    pub network_valid: bool,
    /// Storage validation passed
    pub storage_valid: bool,
    /// Resource warnings
    pub resource_warnings: Vec<String>,
/// Security validation results
pub struct SecurityValidation {
}

    /// Security clearance valid
    pub clearance_valid: bool,
    /// Genetic integrity verified
    pub genetic_integrity_valid: bool,
    /// Spawn authorization valid
    pub authorization_valid: bool,
    /// Security warnings
    pub security_warnings: Vec<String>,
/// Spawn termination outcome
pub struct SpawnTerminationOutcome {
}

    /// Terminated spawn ID
    /// Termination reason
    pub termination_reason: String,
    /// Final performance metrics
    pub final_metrics: SpawnPerformanceMetrics,
    /// Resource cleanup results
    pub cleanup_results: ResourceCleanupResults,
    /// Any warnings during termination
/// Performance metrics for spawned BearDog
pub struct SpawnPerformanceMetrics {
}

    /// Total runtime
    pub total_runtime: std::time::Duration,
    /// Average CPU usage
    pub avg_cpu_usage: f64,
    /// Peak memory usage
    pub peak_memory_usage: u64,
    /// Network bytes transferred
    pub network_bytes_transferred: u64,
    /// Operations completed
    pub operations_completed: u64,
    /// Success rate
/// Resource cleanup results
pub struct ResourceCleanupResults {
}

    /// Memory cleanup successful
    pub memory_cleaned: bool,
    /// Network connections closed
    pub network_connections_closed: u32,
    /// Files cleaned up
    pub files_cleaned: u32,
    /// Cleanup warnings
    pub cleanup_warnings: Vec<String>,
// IMPLEMENTATION HELPERS}


impl OperationContext {
    /// Create a new operation context
    pub fn new(component: &str) -> Self {
        let now = Utc::now();
        Self {
            operation_id: uuid::Uuid::new_v4().to_string(),
            started_at: now,
            completed_at: now,
            component: component.to_string(),
            initiator: "system".to_string(),
            request_id: None,
            metadata: HashMap::new(),
        }
    }

    /// Create operation context with metadata
    pub fn with_metadata(component: &str, metadata: HashMap<String, serde_json::Value>) -> Self {
        let mut context = Self::new(component);
        context.metadata = metadata;
        context
    }

    /// Set the completion time
    pub fn complete(&mut self) {
        self.completed_at = Utc::now();
    }
}

impl Default for OperationMetrics {
    fn default() -> Self {
        Self {
            duration: Duration::from_millis(0),
            memory_used: None,
            items_processed: 0,
            success_rate: 100.0,
            additional_metrics: HashMap::new(),
        }
    }
}

impl<T> OperationOutcome<T> {
    /// Create a successful outcome
    pub fn success(result: T, component: &str) -> Self {
        Self {
            result: Ok(result),
            context: OperationContext::new(component),
            metrics: OperationMetrics::default(),
            warnings: Vec::new(),
        }
    }
}

// Type aliases for common operation outcomes
pub type AuthenticationOutcome = BearDogResult<super::AuthenticationOutcome>;
pub type ValidationOutcome = BearDogResult<super::ValidationOutcome>;
pub type ConfigurationOutcome<T> = BearDogResult<super::ConfigurationOutcome<T>>;
pub type ProcessingOutcome<T> = BearDogResult<super::ProcessingOutcome<T>>;
pub type GeneticsOutcome = BearDogResult<super::GeneticsRegistrationOutcome>;
pub type SpawnOutcome = BearDogResult<super::SpawningOutcome>;
pub type TerminationOutcome = BearDogResult<super::SpawnTerminationOutcome>;
