// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use serde::{Deserialize, Serialize};

pub struct ZeroCostArchitecture {
    pub performance_metrics: PerformanceMetrics,
    /// Collection of optimization patterns
    pub optimization_patterns: Vec<OptimizationPattern>,
    /// The compilation strategy value
    pub compilation_strategy: CompilationStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceMetrics {
    /// Whether async trait boxing has been eliminated
    /// Whether `async_trait_eliminated` is enabled
    pub async_trait_eliminated: bool,
    /// Whether boxing overhead has been removed
    /// Whether `boxing_overhead_removed` is enabled
    pub boxing_overhead_removed: bool,
    /// Whether compile-time dispatch is being used
    pub compile_time_dispatch: bool,
    /// Number of memory allocations that have been reduced
    /// Number of `memory_allocations_reduced`
    pub memory_allocations_reduced: u64,
}

/// Optimization patterns available in the zero-cost architecture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationPattern {
    /// Use enum-based dispatch instead of dynamic dispatch
    EnumDispatch,
    ConstGenerics,
    /// Use static dispatch instead of dynamic dispatch
    StaticDispatch,
    SimdAcceleration,
    /// Use lock-free data structures and algorithms
    LockFree,
    /// Implement zero-copy data handling
    ZeroCopy,
    CompileTimeEvaluation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompilationStrategy {
    /// Conservative optimization with minimal risk
    Conservative,
    Balanced,
    Aggressive,
    /// Maximum optimization regardless of compile time
    Maximum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImplementationStatus {
    Planned,
    /// Optimization is currently being implemented
    InProgress,
    /// Optimization has been fully implemented
    Complete,
    /// Optimization has been deprecated and should not be used
    Deprecated,
}

/// Migration complexity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MigrationComplexity {
    /// Simple migration with minimal changes required
    Low,
    /// Moderate migration requiring some refactoring
    Medium,
    /// Complex migration requiring significant changes
    High,
}

/// Cryptographic key types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
/// Types of key
pub enum KeyType {
    /// RSA 2048-bit key
    Rsa2048,
    /// ECDSA P-256 curve key
    EcdsaP256,
    /// AES 256-bit symmetric key
    Aes256,
}

/// Security level classifications
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SecurityLevel {
    Standard,
    High,
    Critical,
}

/// HSM key representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmKey {
    pub key_id: String,
    /// Type of cryptographic key
    /// The key type value
    pub key_type: KeyType,
    /// When this key was created
    /// The created at value
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Additional metadata associated with the key
    /// The metadata value
    pub metadata: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    /// Current status of the component
    /// Current status of the component
    pub status: String,
    /// When this status was recorded
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Additional details about the health status
    /// The details value
    pub details: std::collections::HashMap<String, String>,
}

/// Workflow definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: String,
    /// Type or category of this workflow
    /// The workflow type value
    pub workflow_type: String,
    /// Ordered list of steps in this workflow
    /// Collection of steps
    pub steps: Vec<WorkflowStep>,
    /// The metadata value
    pub metadata: std::collections::HashMap<String, String>,
}

/// Individual workflow step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub id: String,
    /// The action value
    pub action: String,
    /// The parameters value
    pub parameters: std::collections::HashMap<String, String>,
}

/// Workflow execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResult {
    /// ID of the workflow that was executed
    pub workflow_id: String,
    /// Final status of the workflow execution
    /// Current status of the component
    pub status: String,
    /// Results from the workflow execution
    /// The results value
    pub results: std::collections::HashMap<String, String>,
    /// When the workflow execution completed
    /// The completed at value
    pub completed_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    /// Name of the item
    pub name: std::sync::Arc<str>,
    /// The endpoint value
    pub endpoint: std::sync::Arc<str>,
    /// Port number the service is listening on
    /// Number of port
    pub port: u16,
    /// The metadata value
    pub metadata: std::collections::HashMap<std::sync::Arc<str>, std::sync::Arc<str>>,
    /// Optional health check path
    pub health_check_path: Option<std::sync::Arc<str>>,
    /// Collection of capabilities
    pub capabilities: Vec<std::sync::Arc<str>>,
}

impl ServiceInfo {
    /// Creates a new instance
    pub fn new(
        name: impl Into<String>,
        endpoint: impl Into<String>,
        port: u16,
        metadata: std::collections::HashMap<String, String>,
        health_check_path: Option<String>,
        capabilities: Vec<String>,
    ) -> Self {
        Self {
            name: name.into().into(),
            endpoint: endpoint.into().into(),
            port,
            metadata: metadata
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            health_check_path: health_check_path.map(std::convert::Into::into),
            capabilities: capabilities
                .into_iter()
                .map(std::convert::Into::into)
                .collect(),
        }
    }
}

/// `Arc<dyn>` pattern analysis report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcDynAnalysisReport {
    /// Name of the crate being analyzed
    /// Name of the crate
    pub crate_name: String,
    /// Total number of `Arc<dyn>` patterns found
    /// Number of `total_arc_dyn_patterns`
    pub total_arc_dyn_patterns: usize,
    /// Number of patterns that can be migrated
    /// Number of `migratable_patterns`
    pub migratable_patterns: usize,
    pub estimated_performance_gain: f32,
    /// Overall complexity of the migration
    /// The migration complexity value
    pub migration_complexity: MigrationComplexity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcDynPattern {
    /// Name of the trait being used with `Arc<dyn>`
    /// Name of the trait
    pub trait_name: String,
    /// File path where the pattern is found
    /// The file path value
    pub file_path: String,
    /// Line number of the pattern
    /// Number of `line_number`
    pub line_number: usize,
    /// Context in which the pattern is used
    /// The usage context value
    pub usage_context: String,
    /// The migration difficulty value
    pub migration_difficulty: MigrationComplexity,
}

/// Zero-cost migration plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostMigrationPlan {
    /// List of `Arc<dyn>` patterns to migrate
    /// Collection of patterns
    pub patterns: Vec<ArcDynPattern>,
    /// Expected increase in compilation time (percentage)
    pub estimated_compilation_time_increase: f32,
    pub estimated_runtime_improvement: f32,
    /// Step-by-step migration instructions
    /// Collection of migration steps
    pub migration_steps: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct MemoryPoolMetrics {
    /// Total memory allocated by the pool
    /// Number of `total_allocated`
    pub total_allocated: usize,
    /// Peak memory usage recorded
    /// Number of `peak_usage`
    pub peak_usage: usize,
    /// Efficiency of the memory pool (0.0-1.0)
    /// The pool efficiency value
    pub pool_efficiency: f64,
    /// Memory fragmentation ratio (0.0-1.0)
    /// The fragmentation ratio value
    pub fragmentation_ratio: f64,
}

#[derive(Debug, Clone)]
pub struct SimdCapabilities {
    /// Whether AVX2 instructions are supported
    /// Whether `avx2_supported` is enabled
    pub avx2_supported: bool,
    /// Whether SSE4.2 instructions are supported
    /// Whether `sse42_supported` is enabled
    pub sse42_supported: bool,
    /// Number of `optimal_chunk_size`
    pub optimal_chunk_size: usize,
}
