//! # Specialized Systems Configuration Module
//!
//! This module contains specialized system configurations:
//! HSM, Genetics, and Performance configs.

use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::canonical::providers_unified::hsm_unified::security::SecurityLevel;

// ============================================================================
// HSM Configuration
// ============================================================================

/// **HSM CONFIGURATION** - Hardware Security Module settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmConfig {
    /// HSM core settings
    pub core: HsmCoreConfig,
    /// Key management settings
    pub key_management: HsmKeyManagementConfig,
    /// Performance settings
    pub performance: HsmPerformanceConfig,
    /// Security settings
    pub security: HsmSecurityConfig,
}

/// HSM core configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmCoreConfig {
    /// Enable HSM
    pub enabled: bool,
    /// HSM provider
    pub provider: String,
    /// Connection string
    pub connection_string: String,
}

/// HSM key management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmKeyManagementConfig {
    /// Key generation algorithm
    pub key_generation_algorithm: String,
    /// Key rotation interval
    pub rotation_interval: Duration,
    /// Enable key escrow
    pub enable_escrow: bool,
}

/// HSM performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmPerformanceConfig {
    /// Maximum concurrent operations
    pub max_concurrent_operations: u32,
    /// Operation timeout
    pub operation_timeout: Duration,
}

/// HSM security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmSecurityConfig {
    /// Security level
    pub security_level: SecurityLevel,
    /// Enable hardware attestation
    pub enable_attestation: bool,
}

// ============================================================================
// Genetics Configuration
// ============================================================================

/// **GENETICS CONFIGURATION** - AI and genetic algorithm settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticsConfig {
    /// Core genetics settings
    pub core: GeneticsCoreConfig,
    /// Algorithm configuration
    pub algorithms: GeneticAlgorithmConfig,
    /// Entropy configuration
    pub entropy: EntropyConfig,
    /// Performance optimization settings
    pub optimization: GeneticsOptimizationConfig,
}

/// Genetics core configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticsCoreConfig {
    /// Enable genetics features
    pub enabled: bool,
    /// Population size
    pub population_size: usize,
    /// Number of generations
    pub generations: usize,
}

/// Genetic algorithm configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticAlgorithmConfig {
    /// Mutation rate
    pub mutation_rate: f64,
    /// Crossover rate
    pub crossover_rate: f64,
    /// Selection method
    pub selection_method: String,
}

/// Entropy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyConfig {
    /// Enable entropy collection
    pub enabled: bool,
    /// Entropy sources
    pub sources: Vec<String>,
    /// Collection interval
    pub collection_interval: Duration,
}

/// Genetics optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticsOptimizationConfig {
    /// Enable SIMD optimization
    pub enable_simd: bool,
    /// Number of worker threads
    pub worker_threads: usize,
}

// ============================================================================
// Performance Configuration
// ============================================================================

/// **PERFORMANCE CONFIGURATION** - System performance settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Core performance settings
    pub core: PerformanceCoreConfig,
    /// CPU optimization settings
    pub cpu: CpuOptimizationConfig,
    /// Memory optimization settings
    pub memory: MemoryOptimizationConfig,
    /// I/O optimization settings
    pub io: IoOptimizationConfig,
    /// Caching configuration
    pub caching: CachingConfig,
}

/// Performance core configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceCoreConfig {
    /// Enable performance monitoring
    pub monitoring_enabled: bool,
    /// Profiling enabled
    pub profiling_enabled: bool,
}

/// CPU optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuOptimizationConfig {
    /// CPU affinity
    pub affinity: Option<Vec<usize>>,
    /// Enable CPU optimization
    pub enabled: bool,
}

/// Memory optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryOptimizationConfig {
    /// Memory pool size
    pub pool_size_mb: usize,
    /// Enable memory optimization
    pub enabled: bool,
}

/// I/O optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoOptimizationConfig {
    /// I/O buffer size
    pub buffer_size: usize,
    /// Enable I/O optimization
    pub enabled: bool,
}

/// Caching configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachingConfig {
    /// Enable caching
    pub enabled: bool,
    /// Cache size in MB
    pub cache_size_mb: usize,
    /// Cache TTL
    pub ttl: Duration,
}

// ============================================================================
// Default Implementations
// ============================================================================

impl Default for HsmConfig {
    fn default() -> Self {
        Self {
            core: HsmCoreConfig::default(),
            key_management: HsmKeyManagementConfig::default(),
            performance: HsmPerformanceConfig::default(),
            security: HsmSecurityConfig::default(),
        }
    }
}

impl Default for HsmCoreConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: "software".to_string(),
            connection_string: "memory://".to_string(),
        }
    }
}

impl Default for HsmKeyManagementConfig {
    fn default() -> Self {
        Self {
            key_generation_algorithm: "RSA-2048".to_string(),
            rotation_interval: Duration::from_secs(86400 * 30), // 30 days
            enable_escrow: false,
        }
    }
}

impl Default for HsmPerformanceConfig {
    fn default() -> Self {
        Self {
            max_concurrent_operations: 10,
            operation_timeout: Duration::from_secs(30),
        }
    }
}

impl Default for HsmSecurityConfig {
    fn default() -> Self {
        Self {
            security_level: SecurityLevel::Medium,
            enable_attestation: false,
        }
    }
}

impl Default for GeneticsConfig {
    fn default() -> Self {
        Self {
            core: GeneticsCoreConfig::default(),
            algorithms: GeneticAlgorithmConfig::default(),
            entropy: EntropyConfig::default(),
            optimization: GeneticsOptimizationConfig::default(),
        }
    }
}

impl Default for GeneticsCoreConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            population_size: 100,
            generations: 1000,
        }
    }
}

impl Default for GeneticAlgorithmConfig {
    fn default() -> Self {
        Self {
            mutation_rate: 0.01,
            crossover_rate: 0.8,
            selection_method: "tournament".to_string(),
        }
    }
}

impl Default for EntropyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            sources: vec!["hardware".to_string(), "system".to_string()],
            collection_interval: Duration::from_secs(60),
        }
    }
}

impl Default for GeneticsOptimizationConfig {
    fn default() -> Self {
        Self {
            enable_simd: true,
            worker_threads: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1)
                .max(1),
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            core: PerformanceCoreConfig::default(),
            cpu: CpuOptimizationConfig::default(),
            memory: MemoryOptimizationConfig::default(),
            io: IoOptimizationConfig::default(),
            caching: CachingConfig::default(),
        }
    }
}

impl Default for PerformanceCoreConfig {
    fn default() -> Self {
        Self {
            monitoring_enabled: true,
            profiling_enabled: false,
        }
    }
}

impl Default for CpuOptimizationConfig {
    fn default() -> Self {
        Self {
            affinity: None,
            enabled: true,
        }
    }
}

impl Default for MemoryOptimizationConfig {
    fn default() -> Self {
        Self {
            pool_size_mb: 256,
            enabled: true,
        }
    }
}

impl Default for IoOptimizationConfig {
    fn default() -> Self {
        Self {
            buffer_size: 8192,
            enabled: true,
        }
    }
}

impl Default for CachingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cache_size_mb: 128,
            ttl: Duration::from_secs(3600),
        }
    }
}
