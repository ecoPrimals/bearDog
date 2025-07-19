//! SIMD Cryptographic Acceleration Configuration
//!
//! This module provides comprehensive SIMD cryptographic acceleration configuration for BearDog,
//! including hardware detection, instruction sets, cryptographic operations, and performance optimization.

use serde::{Deserialize, Serialize};

pub mod hardware;
pub mod instruction_sets;
pub mod operations;
pub mod parallel;
pub mod performance;

pub use hardware::*;
pub use instruction_sets::*;
pub use operations::*;
pub use parallel::*;
pub use performance::*;

/// SIMD cryptographic acceleration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIMDCryptoConfig {
    /// Enable SIMD acceleration
    pub enabled: bool,
    /// Hardware detection configuration
    pub hardware_detection: HardwareDetectionConfig,
    /// Instruction set configuration
    pub instruction_sets: InstructionSetConfig,
    /// Cryptographic operations configuration
    pub operations: CryptoOperationsConfig,
    /// Performance optimization configuration
    pub performance: CryptoPerformanceConfig,
    /// Parallel processing configuration
    pub parallel_processing: ParallelCryptoConfig,
}

impl Default for SIMDCryptoConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            hardware_detection: HardwareDetectionConfig::default(),
            instruction_sets: InstructionSetConfig::default(),
            operations: CryptoOperationsConfig::default(),
            performance: CryptoPerformanceConfig::default(),
            parallel_processing: ParallelCryptoConfig::default(),
        }
    }
}

impl SIMDCryptoConfig {
    /// Create a production SIMD crypto configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            hardware_detection: HardwareDetectionConfig::production(),
            instruction_sets: InstructionSetConfig::production(),
            operations: CryptoOperationsConfig::production(),
            performance: CryptoPerformanceConfig::production(),
            parallel_processing: ParallelCryptoConfig::production(),
        }
    }

    /// Create a development SIMD crypto configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            hardware_detection: HardwareDetectionConfig::development(),
            instruction_sets: InstructionSetConfig::development(),
            operations: CryptoOperationsConfig::development(),
            performance: CryptoPerformanceConfig::development(),
            parallel_processing: ParallelCryptoConfig::development(),
        }
    }

    /// Validate the SIMD crypto configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.enabled {
            if !self.hardware_detection.auto_detection
                && self.hardware_detection.supported_architectures.is_empty()
            {
                return Err(
                    "Hardware detection disabled but no supported architectures specified"
                        .to_string(),
                );
            }

            if self.parallel_processing.thread_pool.max_threads == 0 {
                return Err("Thread pool max threads must be greater than 0".to_string());
            }

            if self.operations.hash.enabled && self.operations.hash.algorithms.is_empty() {
                return Err("Hash operations enabled but no algorithms specified".to_string());
            }
        }

        Ok(())
    }

    /// Get estimated memory usage in bytes
    pub fn estimated_memory_usage(&self) -> u64 {
        if !self.enabled {
            return 0;
        }

        let thread_count = self.parallel_processing.thread_pool.max_threads as u64;
        let batch_size = self.parallel_processing.batch_processing.batch_size as u64;

        // Rough estimate: thread count * batch size * 1KB per operation
        thread_count * batch_size * 1024
    }

    /// Check if a specific instruction set is supported
    pub fn supports_instruction_set(&self, instruction_set: &InstructionSet) -> bool {
        if !self.enabled {
            return false;
        }

        self.instruction_sets.enabled_sets.contains(instruction_set)
    }

    /// Get the optimal thread count for the current configuration
    pub fn optimal_thread_count(&self) -> u32 {
        if !self.enabled {
            return 1;
        }

        let max_threads = self.parallel_processing.thread_pool.max_threads;
        let cpu_cores = std::thread::available_parallelism()
            .map(|n| n.get() as u32)
            .unwrap_or(1);

        max_threads.min(cpu_cores)
    }
}
