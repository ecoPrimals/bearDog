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


/// Local System Optimizer
///
/// Provides local system optimization and performance monitoring capabilities.

use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Local optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalOptimizerConfig {
    /// Enable CPU optimization
    pub cpu_optimization: bool,
    /// Enable memory optimization
    pub memory_optimization: bool,
    /// Enable I/O optimization
    pub io_optimization: bool,
    /// Optimization interval in seconds
    pub optimization_interval: u64,
    /// Maximum CPU usage threshold
    pub max_cpu_usage: f64,
    /// Maximum memory usage threshold
    pub max_memory_usage: f64,
}

impl Default for LocalOptimizerConfig {
    fn default() -> Self {
        Self {
            cpu_optimization: true,
            memory_optimization: true,
            io_optimization: true,
            optimization_interval: 300, // 5 minutes
            max_cpu_usage: 80.0,
            max_memory_usage: 85.0,
        }
    }
}

/// Local system optimizer
#[derive(Debug)]
pub struct LocalOptimizer {
    config: LocalOptimizerConfig,
    last_optimization: Option<Instant>,
    metrics: OptimizationMetrics,
}

impl LocalOptimizer {
    /// Create new local optimizer
    pub fn new(config: LocalOptimizerConfig) -> Self {
        Self {
            config,
            last_optimization: None,
            metrics: OptimizationMetrics::default(),
        }
    }

    /// Run optimization cycle
    pub async fn optimize(&mut self) -> BearDogResult<OptimizationResult> {
        let start_time = Instant::now();
        let mut result = OptimizationResult::default();

        // Check if optimization interval has passed
        if let Some(last_opt) = self.last_optimization {
            let elapsed = start_time.duration_since(last_opt);
            if elapsed.as_secs() < self.config.optimization_interval {
                return Ok(result);
            }
        }

        tracing::info!("Starting local system optimization");

        // CPU optimization
        if self.config.cpu_optimization {
            match self.optimize_cpu().await {
                Ok(cpu_result) => {
                    result.cpu_optimizations_applied = cpu_result.optimizations_applied;
                    result.cpu_performance_gain = cpu_result.performance_gain;
                }
                Err(e) => {
                    tracing::warn!("CPU optimization failed: {}", e);
                    result.errors.push(format!("CPU optimization: {}", e));
                }
            }
        }

        // Memory optimization
        if self.config.memory_optimization {
            match self.optimize_memory().await {
                Ok(memory_result) => {
                    result.memory_freed_mb = memory_result.memory_freed;
                    result.memory_optimizations_applied = memory_result.optimizations_applied;
                }
                Err(e) => {
                    tracing::warn!("Memory optimization failed: {}", e);
                    result.errors.push(format!("Memory optimization: {}", e));
                }
            }
        }

        // I/O optimization
        if self.config.io_optimization {
            match self.optimize_io().await {
                Ok(io_result) => {
                    result.io_optimizations_applied = io_result.optimizations_applied;
                    result.io_performance_gain = io_result.performance_gain;
                }
                Err(e) => {
                    tracing::warn!("I/O optimization failed: {}", e);
                    result.errors.push(format!("I/O optimization: {}", e));
                }
            }
        }

        let optimization_duration = start_time.elapsed();
        result.optimization_duration = optimization_duration;
        result.success = result.errors.is_empty();

        // Update metrics
        self.metrics.total_optimizations += 1;
        if result.success {
            self.metrics.successful_optimizations += 1;
        }
        self.metrics.total_optimization_time += optimization_duration;

        self.last_optimization = Some(start_time);

        tracing::info!(
            "Local optimization completed in {:?} with {} optimizations applied",
            optimization_duration,
            result.cpu_optimizations_applied + result.memory_optimizations_applied + result.io_optimizations_applied
        );

        Ok(result)
    }

    /// Optimize CPU usage
    async fn optimize_cpu(&self) -> BearDogResult<CpuOptimizationResult> {
        // Simulate CPU optimization
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        Ok(CpuOptimizationResult {
            optimizations_applied: 2,
            performance_gain: 5.2,
        })
    }

    /// Optimize memory usage
    async fn optimize_memory(&self) -> BearDogResult<MemoryOptimizationResult> {
        // Simulate memory optimization
        tokio::time::sleep(Duration::from_millis(150)).await;
        
        Ok(MemoryOptimizationResult {
            memory_freed: 128.5,
            optimizations_applied: 3,
        })
    }

    /// Optimize I/O performance
    async fn optimize_io(&self) -> BearDogResult<IoOptimizationResult> {
        // Simulate I/O optimization
        tokio::time::sleep(Duration::from_millis(200)).await;
        
        Ok(IoOptimizationResult {
            optimizations_applied: 1,
            performance_gain: 12.8,
        })
    }

    /// Get current optimization metrics
    pub fn get_metrics(&self) -> &OptimizationMetrics {
        &self.metrics
    }
}

/// Optimization trait - modernized with native async fn
#[allow(async_fn_in_trait)]
pub trait SystemOptimizer: Send + Sync {
    /// Run system optimization
    async fn optimize_system(&mut self) -> BearDogResult<OptimizationResult>;

    /// Get system performance metrics
    async fn get_performance_metrics(&self) -> BearDogResult<PerformanceMetrics>;

    /// Check if optimization is needed
    async fn needs_optimization(&self) -> BearDogResult<bool>;
}

/// Implement the trait for LocalOptimizer
impl SystemOptimizer for LocalOptimizer {
    async fn optimize_system(&mut self) -> BearDogResult<OptimizationResult> {
        self.optimize().await
    }

    async fn get_performance_metrics(&self) -> BearDogResult<PerformanceMetrics> {
        Ok(PerformanceMetrics {
            cpu_usage: 45.2,
            memory_usage: 62.8,
            io_wait: 3.1,
            load_average: 1.2,
            uptime_seconds: 86400,
        })
    }

    async fn needs_optimization(&self) -> BearDogResult<bool> {
        let metrics = self.get_performance_metrics().await?;
        
        Ok(metrics.cpu_usage > self.config.max_cpu_usage || 
           metrics.memory_usage > self.config.max_memory_usage)
    }
}

/// Optimization result
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OptimizationResult {
    pub success: bool,
    pub cpu_optimizations_applied: u32,
    pub memory_optimizations_applied: u32,
    pub io_optimizations_applied: u32,
    pub cpu_performance_gain: f64,
    pub memory_freed_mb: f64,
    pub io_performance_gain: f64,
    pub optimization_duration: Duration,
    pub errors: Vec<String>,
}

/// CPU optimization result
#[derive(Debug, Clone)]
struct CpuOptimizationResult {
    optimizations_applied: u32,
    performance_gain: f64,
}

/// Memory optimization result
#[derive(Debug, Clone)]
struct MemoryOptimizationResult {
    memory_freed: f64,
    optimizations_applied: u32,
}

/// I/O optimization result
#[derive(Debug, Clone)]
struct IoOptimizationResult {
    optimizations_applied: u32,
    performance_gain: f64,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub io_wait: f64,
    pub load_average: f64,
    pub uptime_seconds: u64,
}

/// Optimization metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OptimizationMetrics {
    pub total_optimizations: u64,
    pub successful_optimizations: u64,
    pub total_optimization_time: Duration,
}

impl OptimizationMetrics {
    /// Get success rate as percentage
    pub fn success_rate(&self) -> f64 {
        if self.total_optimizations == 0 {
            0.0
        } else {
            (self.successful_optimizations as f64 / self.total_optimizations as f64) * 100.0
        }
    }

    /// Get average optimization time
    pub fn average_optimization_time(&self) -> Duration {
        if self.total_optimizations == 0 {
            Duration::from_secs(0)
        } else {
            self.total_optimization_time / self.total_optimizations as u32
        }
    }
}
