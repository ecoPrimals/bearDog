//! BearDog configuration management
//!
//! This crate provides comprehensive configuration management for BearDog Security Manager,
//! including support for multiple configuration sources, secret management, and environment-specific settings.

#![allow(clippy::derivable_impls)]
#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::useless_format)]
#![allow(clippy::large_enum_variant)]

pub mod async_optimization;
pub mod caching;
pub mod constants;
pub mod core;
pub mod database;
pub mod final_validation;
pub mod genetic_optimization;
pub mod integration;
pub mod load_testing;
pub mod memory;
pub mod monitoring;
pub mod network;
pub mod performance;
pub mod production_deployment;
pub mod runtime;
pub mod secrets;
pub mod security;
pub mod simd_crypto;

use serde::{Deserialize, Serialize};

// Re-export main configuration types
pub use async_optimization::AsyncOptimizationConfig;
pub use caching::DistributedCachingConfig;
pub use database::OptimizedDatabaseConfig;
pub use final_validation::FinalValidationConfig;
pub use genetic_optimization::GeneticOptimizationConfig;
pub use load_testing::LoadTestingConfig;
pub use memory::MemoryOptimizationConfig;
pub use monitoring::ProductionMonitoringConfig;
pub use network::NetworkConfig;
pub use performance::PerformanceConfig;
pub use production_deployment::ProductionDeploymentConfig;
pub use runtime::{get_config, init_config, RuntimeConfig};
pub use simd_crypto::SIMDCryptoConfig;

// Re-export core configuration
pub use constants::*;
pub use core::BearDogConfig;

/// Version information module
pub mod version {
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
    pub const NAME: &str = env!("CARGO_PKG_NAME");
}

/// Configuration utility functions
pub mod utils {
    use super::*;

    /// Create a production-optimized configuration
    pub fn create_production_config() -> OptimizedBearDogConfig {
        OptimizedBearDogConfig {
            database: OptimizedDatabaseConfig::production(),
            memory: MemoryOptimizationConfig::production(),
            async_optimization: AsyncOptimizationConfig::production(),
            performance: PerformanceConfig::production(),
            caching: DistributedCachingConfig::production(),
            simd_crypto: SIMDCryptoConfig::production(),
            genetic_optimization: GeneticOptimizationConfig::production(),
            load_testing: LoadTestingConfig::production(),
            monitoring: ProductionMonitoringConfig::production(),
            deployment: ProductionDeploymentConfig::production(),
            validation: FinalValidationConfig::production(),
        }
    }

    /// Create a development-optimized configuration
    pub fn create_development_config() -> OptimizedBearDogConfig {
        OptimizedBearDogConfig {
            database: OptimizedDatabaseConfig::development(),
            memory: MemoryOptimizationConfig::development(),
            async_optimization: AsyncOptimizationConfig::development(),
            performance: PerformanceConfig::development(),
            caching: DistributedCachingConfig::development(),
            simd_crypto: SIMDCryptoConfig::development(),
            genetic_optimization: GeneticOptimizationConfig::development(),
            load_testing: LoadTestingConfig::development(),
            monitoring: ProductionMonitoringConfig::development(),
            deployment: ProductionDeploymentConfig::development(),
            validation: FinalValidationConfig::development(),
        }
    }
}

/// Extended configuration with advanced optimizations
/// This is separate from the core BearDogConfig to avoid conflicts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizedBearDogConfig {
    /// Database optimization configuration
    pub database: OptimizedDatabaseConfig,
    /// Memory optimization configuration
    pub memory: MemoryOptimizationConfig,
    /// Async optimization configuration
    pub async_optimization: AsyncOptimizationConfig,
    /// Performance monitoring configuration
    pub performance: PerformanceConfig,
    /// Distributed caching configuration
    pub caching: DistributedCachingConfig,
    /// SIMD cryptographic acceleration configuration
    pub simd_crypto: SIMDCryptoConfig,
    /// Genetic algorithm optimization configuration
    pub genetic_optimization: GeneticOptimizationConfig,
    /// Load testing configuration
    pub load_testing: LoadTestingConfig,
    /// Production monitoring configuration
    pub monitoring: ProductionMonitoringConfig,
    /// Production deployment configuration
    pub deployment: ProductionDeploymentConfig,
    /// Final validation configuration
    pub validation: FinalValidationConfig,
}
pub mod validation;
