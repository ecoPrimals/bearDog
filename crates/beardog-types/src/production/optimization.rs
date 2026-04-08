// SPDX-License-Identifier: AGPL-3.0-or-later

// Performance Optimization Engine
//
// This module provides advanced performance optimization capabilities
// including auto-scaling, resource optimization, and performance tuning.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

/// Knobs for automatic scaling, profiling, and periodic optimization passes.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OptimizationConfig {
    /// Enable auto-scaling
    /// Whether `enable_auto_scaling` is enabled
    pub enable_auto_scaling: bool,
    /// Whether continuous or sampled CPU/memory profiling is enabled
    pub enable_profiling: bool,
    /// Optimization interval in seconds
    /// Number of `optimization_interval_seconds`
    pub optimization_interval_seconds: u64,
}

/// Configuration holder for the `beardog-types` production ecosystem struct.
///
/// This is a structural component in the types layer. Real optimization
/// behaviour lives in `beardog-core::zero_knowledge_bootstrap::PerformanceOptimizer`
/// and `beardog-core::ecosystem_integration::EcosystemPerformanceOptimizer`.
#[derive(Debug)]
pub struct PerformanceOptimizer {
    config: OptimizationConfig,
}

impl PerformanceOptimizer {
    /// Wrap an [`OptimizationConfig`] for the production ecosystem struct.
    ///
    /// # Errors
    ///
    /// Reserved for future config validation; currently infallible.
    pub fn new(config: &OptimizationConfig) -> Result<Self, BearDogError> {
        Ok(Self {
            config: config.clone(),
        })
    }

    /// Access the underlying optimization config.
    #[must_use]
    pub const fn config(&self) -> &OptimizationConfig {
        &self.config
    }
}
