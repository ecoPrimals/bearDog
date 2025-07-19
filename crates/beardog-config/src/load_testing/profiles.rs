//! Load Profiles and Patterns
//!
//! This module defines load profiles and various testing patterns
//! for different types of performance testing.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Load profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadProfile {
    /// Profile name
    pub name: String,
    /// Profile type
    pub profile_type: LoadProfileType,
    /// Load parameters
    pub parameters: LoadParameters,
}

/// Load profile type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadProfileType {
    /// Constant load
    Constant { load: f64 },
    /// Ramp-up load
    RampUp {
        start: f64,
        end: f64,
        duration: Duration,
    },
    /// Spike load
    Spike {
        base: f64,
        peak: f64,
        duration: Duration,
    },
    /// Step load
    Step { steps: Vec<LoadStep> },
    /// Cyclic load
    Cyclic {
        pattern: Vec<f64>,
        cycle_duration: Duration,
    },
}

/// Load step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadStep {
    /// Load level
    pub load: f64,
    /// Step duration
    pub duration: Duration,
}

/// Load parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadParameters {
    /// Virtual users
    pub virtual_users: usize,
    /// Requests per second
    pub requests_per_second: f64,
    /// Think time
    pub think_time: Duration,
    /// Data size
    pub data_size: usize,
}

/// Connection pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionPattern {
    /// Pattern name
    pub name: String,
    /// Pattern type
    pub pattern_type: ConnectionPatternType,
    /// Connection parameters
    pub parameters: ConnectionParameters,
}

/// Connection pattern type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionPatternType {
    /// Sequential connections
    Sequential,
    /// Parallel connections
    Parallel { max_connections: usize },
    /// Connection pooling
    Pooled { pool_size: usize },
    /// Connection per request
    PerRequest,
}

/// Connection parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionParameters {
    /// Connection timeout
    pub timeout: Duration,
    /// Keep alive
    pub keep_alive: bool,
    /// Connection retry count
    pub retry_count: usize,
    /// Connection retry delay
    pub retry_delay: Duration,
}

/// Memory pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPattern {
    /// Pattern name
    pub name: String,
    /// Pattern type
    pub pattern_type: MemoryPatternType,
    /// Memory parameters
    pub parameters: MemoryParameters,
}

/// Memory pattern type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryPatternType {
    /// Sequential allocation
    Sequential,
    /// Random allocation
    Random,
    /// Fragmented allocation
    Fragmented,
    /// Bulk allocation
    Bulk,
}

/// Memory parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryParameters {
    /// Allocation size
    pub allocation_size: usize,
    /// Allocation count
    pub allocation_count: usize,
    /// Deallocation delay
    pub deallocation_delay: Duration,
    /// Memory pressure
    pub memory_pressure: f64,
}

/// Cache pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePattern {
    /// Pattern name
    pub name: String,
    /// Pattern type
    pub pattern_type: CachePatternType,
    /// Cache parameters
    pub parameters: CacheParameters,
}

/// Cache pattern type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CachePatternType {
    /// Read-heavy pattern
    ReadHeavy,
    /// Write-heavy pattern
    WriteHeavy,
    /// Read-write balanced
    Balanced,
    /// Cache warming
    Warming,
}

/// Cache parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheParameters {
    /// Cache size
    pub cache_size: usize,
    /// TTL (Time To Live)
    pub ttl: Duration,
    /// Hit ratio target
    pub hit_ratio_target: f64,
    /// Eviction policy
    pub eviction_policy: String,
}

/// Crypto pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoPattern {
    /// Pattern name
    pub name: String,
    /// Pattern type
    pub pattern_type: CryptoPatternType,
    /// Crypto parameters
    pub parameters: CryptoParameters,
}

/// Crypto pattern type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CryptoPatternType {
    /// Encryption heavy
    EncryptionHeavy,
    /// Decryption heavy
    DecryptionHeavy,
    /// Signing heavy
    SigningHeavy,
    /// Verification heavy
    VerificationHeavy,
}

/// Crypto parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoParameters {
    /// Key size
    pub key_size: usize,
    /// Algorithm
    pub algorithm: String,
    /// Data size
    pub data_size: usize,
    /// Parallel operations
    pub parallel_operations: usize,
}

/// Genetic pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticPattern {
    /// Pattern name
    pub name: String,
    /// Pattern type
    pub pattern_type: GeneticPatternType,
    /// Genetic parameters
    pub parameters: GeneticParameters,
}

/// Genetic pattern type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GeneticPatternType {
    /// Spawning heavy
    SpawningHeavy,
    /// Evolution heavy
    EvolutionHeavy,
    /// Crossover heavy
    CrossoverHeavy,
    /// Mutation heavy
    MutationHeavy,
}

/// Genetic parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticParameters {
    /// Population size
    pub population_size: usize,
    /// Generation count
    pub generation_count: usize,
    /// Mutation rate
    pub mutation_rate: f64,
    /// Crossover rate
    pub crossover_rate: f64,
}

/// Workflow pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowPattern {
    /// Pattern name
    pub name: String,
    /// Pattern type
    pub pattern_type: WorkflowPatternType,
    /// Workflow parameters
    pub parameters: WorkflowParameters,
}

/// Workflow pattern type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowPatternType {
    /// Sequential workflow
    Sequential,
    /// Parallel workflow
    Parallel,
    /// Conditional workflow
    Conditional,
    /// Loop workflow
    Loop,
}

/// Workflow parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowParameters {
    /// Step count
    pub step_count: usize,
    /// Parallelism
    pub parallelism: usize,
    /// Condition evaluation
    pub condition_evaluation: bool,
    /// Loop iteration count
    pub loop_iteration_count: usize,
}

/// Stress pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StressPattern {
    /// Pattern name
    pub name: String,
    /// Pattern type
    pub pattern_type: StressPatternType,
    /// Stress parameters
    pub parameters: StressParameters,
}

/// Stress pattern type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StressPatternType {
    /// CPU stress
    CPU,
    /// Memory stress
    Memory,
    /// IO stress
    IO,
    /// Network stress
    Network,
}

/// Stress parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StressParameters {
    /// Stress intensity
    pub intensity: f64,
    /// Stress duration
    pub duration: Duration,
    /// Stress ramp up
    pub ramp_up: Duration,
    /// Stress cool down
    pub cool_down: Duration,
}

/// Scaling pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingPattern {
    /// Pattern name
    pub name: String,
    /// Pattern type
    pub pattern_type: ScalingPatternType,
    /// Scaling parameters
    pub parameters: ScalingParameters,
}

/// Scaling pattern type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScalingPatternType {
    /// Horizontal scaling
    Horizontal,
    /// Vertical scaling
    Vertical,
    /// Auto scaling
    Auto,
    /// Manual scaling
    Manual,
}

/// Scaling parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingParameters {
    /// Initial instances
    pub initial_instances: usize,
    /// Maximum instances
    pub max_instances: usize,
    /// Scaling threshold
    pub scaling_threshold: f64,
    /// Scaling cooldown
    pub scaling_cooldown: Duration,
}

impl Default for LoadProfile {
    fn default() -> Self {
        Self {
            name: "Default Load Profile".to_string(),
            profile_type: LoadProfileType::Constant { load: 1.0 },
            parameters: LoadParameters::default(),
        }
    }
}

impl Default for LoadParameters {
    fn default() -> Self {
        Self {
            virtual_users: 10,
            requests_per_second: 10.0,
            think_time: Duration::from_secs(1),
            data_size: 1024,
        }
    }
}
